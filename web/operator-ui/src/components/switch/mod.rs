//! Capsule switch: AsyncSwitch.

use crate::components::primitives::{Icon, PendingIndicator};
use crate::core::{
    AsyncAction, ControlState, JobToken, PendingRun, Press, mirrored, run_toggle, spawn_ripple,
    use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("switch.css");

/// Capsule switch; the thumb parks in the center with a spinner while the request is pending.
#[component]
pub fn AsyncSwitch(
    #[prop(into)] checked: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] on_toggle: Option<AsyncAction<bool>>,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let local = RwSignal::new(ControlState::Idle);
    let value = mirrored(checked);
    let job = JobToken::new();
    let press = Press::new();
    let has_action = on_toggle.is_some();

    let current = Signal::derive(move || {
        let managed = state.get().unwrap_or_else(|| local.get());
        if !enabled.get() {
            ControlState::Disabled
        } else if managed != ControlState::Idle {
            managed
        } else if press.pressed.get() {
            ControlState::Pressed
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(current);
    let show_spinner = use_pending_indicator(move || current.get() == ControlState::Pending);
    let clickable = move || {
        has_action
            && !matches!(
                current.get(),
                ControlState::Disabled | ControlState::Pending
            )
    };
    let toggle = move || {
        if let Some(action) = on_toggle.clone() {
            run_toggle(action, value, local, job);
        }
    };

    view! {
        <button
            type="button"
            role="switch"
            class="op-sb op-rippled op-switch"
            aria-label=label
            aria-checked=move || value.get().to_string()
            aria-busy=move || (current.get() == ControlState::Pending).then_some("true")
            aria-disabled=move || (!clickable()).then_some("true")
            data-checked=move || value.get().then_some("")
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
            data-spinner=move || show_spinner.get().then_some("")
            on:pointerdown=move |ev| {
                if clickable() {
                    press.down();
                    spawn_ripple(&ev);
                }
            }
            on:pointerup=move |_| press.up()
            on:pointerleave=move |_| press.up()
            on:pointercancel=move |_| press.up()
            on:click=move |_| {
                if clickable() {
                    toggle();
                }
            }
        >
            <span class="op-switch-thumb">
                {move || {
                    if show_spinner.get() {
                        view! { <PendingIndicator size=3.5/> }.into_any()
                    } else {
                        let icon = if value.get() { icons::CHECK } else { icons::CLOSE };
                        view! { <Icon icon=icon size=3.0/> }.into_any()
                    }
                }}
            </span>
            <PendingRun/>
        </button>
    }
}
