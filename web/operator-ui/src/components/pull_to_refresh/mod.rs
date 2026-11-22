//! PullToRefresh component: touch pull-down gesture triggering an asynchronous refresh.

use crate::components::primitives::Icon;
use crate::core::{AsyncAction, Metric, Slot, root_class, root_style, ui_text};
use leptos::ev;
use leptos::html::Div;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("pull_to_refresh.css");

/// Touch pull-down container for mobile or responsive touch refresh flows.
#[component]
pub fn PullToRefresh(
    on_refresh: AsyncAction,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Pull threshold in DU to trigger a refresh (`--op-pull-to-refresh-threshold`).
    #[prop(default = 15.0.into(), into)]
    threshold: MaybeProp<f32>,
    #[prop(optional, into)] pull_label: MaybeProp<String>,
    #[prop(optional, into)] release_label: MaybeProp<String>,
    #[prop(optional, into)] refreshing_label: MaybeProp<String>,
    #[prop(optional, into)] puller: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let pull_str = ui_text(pull_label, "pull-to-refresh.pull", "Pull to refresh");
    let release_str = ui_text(
        release_label,
        "pull-to-refresh.release",
        "Release to refresh",
    );
    let refreshing_str = ui_text(
        refreshing_label,
        "pull-to-refresh.refreshing",
        "Refreshing…",
    );

    let container_ref = NodeRef::<Div>::new();
    let pull_y = RwSignal::new(0.0f64);
    let touch_start_y = RwSignal::new(0.0f64);
    let is_pulling = RwSignal::new(false);
    let is_refreshing = RwSignal::new(false);

    let threshold_px = Signal::derive(move || (threshold.get().unwrap_or(15.0) * 4.0) as f64);

    let on_touchstart = move |ev: ev::TouchEvent| {
        if !enabled.get() || is_refreshing.get() {
            return;
        }
        let scroll_top = container_ref.get().map(|el| el.scroll_top()).unwrap_or(0);
        if scroll_top <= 0
            && let Some(t) = ev.touches().get(0)
        {
            touch_start_y.set(t.client_y() as f64);
            is_pulling.set(true);
        }
    };

    let on_touchmove = move |ev: ev::TouchEvent| {
        if !is_pulling.get() || is_refreshing.get() {
            return;
        }
        if let Some(t) = ev.touches().get(0) {
            let diff = t.client_y() as f64 - touch_start_y.get();
            if diff > 0.0 {
                // Apply half damping resistance
                pull_y.set(diff * 0.5);
            } else {
                pull_y.set(0.0);
            }
        }
    };

    let on_touchend = move |_| {
        if !is_pulling.get() {
            return;
        }
        is_pulling.set(false);
        let curr_pull = pull_y.get();
        let target_th = threshold_px.get();

        if curr_pull >= target_th && !is_refreshing.get() {
            is_refreshing.set(true);
            let action = on_refresh.clone();
            spawn_local(async move {
                let _ = action.call(()).await;
                is_refreshing.set(false);
                pull_y.set(0.0);
            });
        } else {
            pull_y.set(0.0);
        }
    };

    let metrics = vec![Metric::du("--op-pull-to-refresh-threshold", threshold)];
    let class_fn = root_class("op-pull-to-refresh", class);
    let style_fn = root_style(metrics, style);

    let has_puller_slot = puller.is_set();

    view! {
        <div
            class=class_fn
            style=style_fn
            node_ref=container_ref
            data-refreshing=move || is_refreshing.get().then_some("")
            on:touchstart=on_touchstart
            on:touchmove=on_touchmove
            on:touchend=on_touchend
        >
            <div
                class="op-pull-to-refresh-puller"
                style=move || {
                    if is_refreshing.get() {
                        String::new()
                    } else {
                        format!("height: {}px;", pull_y.get().min(threshold_px.get() * 1.5) as i32)
                    }
                }
            >
                {if has_puller_slot {
                    view! { {puller.run()} }.into_any()
                } else {
                    view! {
                        {move || {
                            if is_refreshing.get() {
                                view! {
                                    <span class="op-pull-to-refresh-spinner" />
                                    <span>{refreshing_str.get()}</span>
                                }.into_any()
                            } else {
                                let past_threshold = pull_y.get() >= threshold_px.get();
                                view! {
                                    <span
                                        class="op-pull-to-refresh-icon"
                                        data-flipped=past_threshold.then_some("")
                                    >
                                        <Icon icon=icons::ARROW_DOWNWARD size=18.0 />
                                    </span>
                                    <span>{if past_threshold { release_str.get() } else { pull_str.get() }}</span>
                                }.into_any()
                            }
                        }}
                    }.into_any()
                }}
            </div>

            <div class="op-pull-to-refresh-content">
                {children()}
            </div>
        </div>
    }
}
