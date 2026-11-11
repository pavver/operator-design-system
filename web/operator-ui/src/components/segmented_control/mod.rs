//! Segment switcher: SegmentItem and SegmentedControl.

use crate::components::primitives::{Icon, PendingIndicator};
use crate::core::{
    AsyncAction, ControlState, PendingRun, Press, du, sleep_ms, spawn_ripple,
    use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim};

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
    /// Height in DU.
    #[prop(default = 9.0)]
    height: f32,
) -> impl IntoView {
    let count = items.len().max(1);
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
        pending_index.set(Some(index));
        spawn_local(async move {
            let result = action.call(index).await;
            pending_index.set(None);
            if result.is_success() {
                selected.set(index);
                feedback.set(ControlState::Success);
                sleep_ms(500).await;
            } else {
                feedback.set(ControlState::Error);
                sleep_ms(600).await;
            }
            feedback.set(ControlState::Idle);
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
                            view! { <PendingIndicator size=dim::ICON_SMALL/> }.into_any()
                        } else {
                            let item = item.clone();
                            view! {
                                <span class="op-seg-content">
                                    {item.icon.map(|i| view! { <Icon icon=i size=dim::ICON_SMALL/> })}
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
            class="op-seg"
            role="tablist"
            style:height=du(height)
            data-disabled=move || (!enabled.get()).then_some("")
        >
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
        </div>
    }
}
