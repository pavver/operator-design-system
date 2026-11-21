//! Segment switcher: SegmentItem and SegmentedControl.

use crate::core::{
    AsyncAction, ControlState, JobToken, Metric, PendingRun, Press, ScopedSlot, Slot, root_class,
    root_style, sleep_ms, spawn_ripple, ui_text, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::Icon as IconData;

mod visuals;
use visuals::{pending, symbol};

const ICON_SIZE: &str = "var(--op-segmented-control-icon-size, var(--op-dim-icon-small))";

pub(crate) const CSS: &str = include_str!("segmented_control.css");

/// Segment descriptor supporting text, icon, or icon + text.
#[derive(Clone, Debug, PartialEq)]
pub struct SegmentItem {
    pub text: Option<String>,
    pub icon: Option<IconData>,
}

impl SegmentItem {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            icon: None,
        }
    }

    pub fn icon(icon: IconData) -> Self {
        Self {
            text: None,
            icon: Some(icon),
        }
    }

    pub fn item(text: impl Into<String>, icon: IconData) -> Self {
        Self {
            text: Some(text.into()),
            icon: Some(icon),
        }
    }
}

/// Segment switcher with a sliding indicator; async mode adds pending, success flash and error rollback.
#[component]
pub fn SegmentedControl(
    items: Vec<SegmentItem>,
    selected: RwSignal<usize>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// When set, the selection commits only after the request succeeds.
    #[prop(optional, into)]
    on_select: Option<AsyncAction<usize>>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] spinner_stroke: MaybeProp<f32>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] option: Option<ScopedSlot<SegmentItem>>,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    let loading_label = ui_text(loading_label, "segmented-control.loading", "Loading");
    let label = ui_text(label, "segmented-control.label", "Options");

    let root_metrics = vec![
        Metric::du("--op-segmented-control-width", width),
        Metric::du("--op-segmented-control-spinner-stroke", spinner_stroke),
        Metric::du("--op-segmented-control-height", height),
        Metric::du("--op-segmented-control-padding", padding),
        Metric::du("--op-segmented-control-gap", gap),
        Metric::du("--op-segmented-control-icon-size", icon_size),
        Metric::tu("--op-segmented-control-font-size", font_size),
        Metric::tu("--op-segmented-control-line-height", line_height),
    ];

    let count = items.len().max(1);
    let job = JobToken::new();
    let pending_index = RwSignal::new(None::<usize>);
    let feedback = RwSignal::new(ControlState::Idle);
    let is_async = on_select.is_some();
    let on_select = StoredValue::new(on_select);

    let target = Signal::derive(move || {
        pending_index
            .get()
            .unwrap_or_else(|| selected.get().min(count - 1))
    });
    let indicator_state = Signal::derive(move || {
        if pending_index.get().is_some() {
            ControlState::Pending
        } else {
            feedback.get()
        }
    });
    let visual = use_state_visual(indicator_state);

    let pick = move |index: usize| {
        let Some(action) = on_select.get_value() else {
            selected.set(index);
            return;
        };
        if index == selected.get_untracked() || pending_index.get_untracked().is_some() {
            return;
        }
        let request = job.next();
        pending_index.set(Some(index));
        spawn_local(async move {
            let result = action.call(index).await;
            if !job.is_current(request) {
                return;
            }
            pending_index.set(None);
            if result.is_success() {
                selected.set(index);
                feedback.set(ControlState::Success);
                sleep_ms(500).await;
            } else {
                feedback.set(ControlState::Error);
                sleep_ms(600).await;
            }
            if job.is_current(request) {
                feedback.set(ControlState::Idle);
            }
        });
    };

    let buttons = items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            let press = Press::new();
            let is_selected = move || target.get() == index;
            let busy = move || pending_index.get().is_some();
            let show_spinner = use_pending_indicator(move || pending_index.get() == Some(index));
            let tone = move || {
                if !enabled.get() {
                    "disabled"
                } else if pending_index.get() == Some(index) {
                    "pending"
                } else if is_selected() {
                    match feedback.get() {
                        ControlState::Success => "success",
                        ControlState::Error => "error",
                        _ => "selected",
                    }
                } else {
                    "idle"
                }
            };
            view! {
                <button
                    type="button"
                    role="tab"
                    class="op-rippled op-seg-item"
                    aria-selected=move || is_selected().to_string()
                    data-tone=tone
                    data-pressed=move || press.pressed.get().then_some("")
                    on:pointerdown=move |ev| {
                        if enabled.get_untracked() && !busy() {
                            press.down();
                            spawn_ripple(&ev);
                        }
                    }
                    on:pointerup=move |_| press.up()
                    on:pointerleave=move |_| press.up()
                    on:pointercancel=move |_| press.up()
                    on:click=move |_| {
                        if enabled.get_untracked() && !busy() {
                            pick(index);
                        }
                    }
                >
                    {move || {
                        if show_spinner.get() {
                            pending(ICON_SIZE, loading_label)
                        } else {
                            let item = item.clone();
                            if let Some(slot) = option { return slot.run(item); }
                            view! {
                                <span class="op-seg-content">
                                    {item.icon.map(|i| view! { {symbol(Signal::stored(i), ICON_SIZE, None)} })}
                                    {item.text.map(|t| view! { <span>{t}</span> })}
                                </span>
                            }
                                .into_any()
                        }
                    }}
                </button>
            }
        })
        .collect_view();

    view! {
        <div
            class=root_class("op-seg", class)
            style=root_style(root_metrics, style)
            role="tablist"
            aria-label=move || label.get()
            data-disabled=move || (!enabled.get()).then_some("")
        >
            {before_options.run()}
            <div class="op-seg-track">
                <Show when=move || enabled.get()>
                    <div
                        class="op-seg-slide"
                        style:width=format!("{}%", 100.0 / count as f32)
                        style:transform=move || format!("translateX({}%)", target.get() * 100)
                    >
                        <div
                            class="op-sb op-seg-indicator"
                            class:op-seg-indicator-async=is_async
                            data-state=move || visual.state.get()
                            data-result=move || visual.result.get()
                            data-fading=move || visual.fading.get().then_some("")
                        >
                            <PendingRun/>
                        </div>
                    </div>
                </Show>
                {buttons}
            </div>
            {after_options.run()}
        </div>
    }
}
