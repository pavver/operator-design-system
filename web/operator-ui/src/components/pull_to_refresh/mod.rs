//! PullToRefresh component: touch pull-down gesture triggering an asynchronous refresh.

use crate::components::primitives::Icon;
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, Metric, Slot, root_class, root_style,
    sleep_ms, ui_text,
};
use leptos::ev;
use leptos::html::Div;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{icons, motion};

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
    #[prop(optional, into)] success_label: MaybeProp<String>,
    #[prop(optional, into)] error_label: MaybeProp<String>,
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
    let success_str = ui_text(success_label, "pull-to-refresh.success", "Refreshed");
    let error_str = ui_text(error_label, "pull-to-refresh.error", "Refresh failed");

    let container_ref = NodeRef::<Div>::new();
    let pull_y = RwSignal::new(0.0f64);
    let touch_start_y = RwSignal::new(0.0f64);
    let is_pulling = RwSignal::new(false);
    let state = RwSignal::new(ControlState::Idle);
    let error_message = RwSignal::new(Option::<String>::None);
    let job = JobToken::new();

    let threshold_px = Signal::derive(move || (threshold.get().unwrap_or(15.0) * 4.0) as f64);

    let on_touchstart = move |ev: ev::TouchEvent| {
        if !enabled.get() || state.get() == ControlState::Pending {
            return;
        }
        let scroll_top = container_ref.get().map(|el| el.scroll_top()).unwrap_or(0);
        if scroll_top <= 0
            && let Some(t) = ev.touches().get(0)
        {
            state.set(ControlState::Idle);
            error_message.set(None);
            touch_start_y.set(t.client_y() as f64);
            is_pulling.set(true);
        }
    };

    let on_touchmove = move |ev: ev::TouchEvent| {
        if !is_pulling.get() || state.get() == ControlState::Pending {
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

        if curr_pull >= target_th && state.get() != ControlState::Pending {
            state.set(ControlState::Pending);
            let action = on_refresh.clone();
            let id = job.next();
            spawn_local(async move {
                let result = action.call(()).await;
                if !job.is_current(id) {
                    return;
                }
                match result {
                    ActionResult::Success => {
                        state.set(ControlState::Success);
                        error_message.set(None);
                    }
                    ActionResult::Error(msg) => {
                        state.set(ControlState::Error);
                        error_message.set(msg);
                    }
                }
                sleep_ms(motion::RESULT_HOLD_MS).await;
                if !job.is_current(id) {
                    return;
                }
                pull_y.set(0.0);
                state.set(ControlState::Idle);
                error_message.set(None);
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
            data-state=move || match state.get() {
                ControlState::Pending => Some("pending"),
                ControlState::Success => Some("success"),
                ControlState::Error => Some("error"),
                _ => None,
            }
            on:touchstart=on_touchstart
            on:touchmove=on_touchmove
            on:touchend=on_touchend
        >
            <div
                class="op-pull-to-refresh-puller"
                style=move || {
                    let s = state.get();
                    if s == ControlState::Pending || s == ControlState::Success || s == ControlState::Error {
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
                            match state.get() {
                                ControlState::Pending => view! {
                                    <span class="op-pull-to-refresh-spinner" />
                                    <span>{refreshing_str.get()}</span>
                                }.into_any(),
                                ControlState::Success => view! {
                                    <Icon icon=icons::CHECK size=18.0 />
                                    <span>{success_str.get()}</span>
                                }.into_any(),
                                ControlState::Error => {
                                    let custom_err = error_message.get();
                                    view! {
                                        <Icon icon=icons::ERROR size=18.0 />
                                        <span>{custom_err.unwrap_or_else(|| error_str.get())}</span>
                                    }.into_any()
                                }
                                _ => {
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
