//! Checkbox: AsyncCheckbox.

use crate::components::primitives::{Icon, PendingIndicator};
use crate::core::{
    AsyncAction, ControlState, JobToken, PendingRun, Press, mirrored, run_toggle, spawn_ripple,
    use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("checkbox.css");

/// Rectangular checkbox with optional label; commits the value only after a successful request.
#[component]
pub fn AsyncCheckbox(
    #[prop(into)] checked: Signal<bool>,
    #[prop(optional, into)] text: Option<Signal<String>>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] on_change: Option<AsyncAction<bool>>,
) -> impl IntoView {
    let local = RwSignal::new(ControlState::Idle);
    let value = mirrored(checked);
    let job = JobToken::new();
    let press = Press::new();
    let has_action = on_change.is_some();

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
    let border_state = Signal::derive(move || {
        let s = current.get();
        if s == ControlState::Idle && value.get() {
            ControlState::Selected
        } else {
            s
        }
    });
    let visual = use_state_visual(border_state);
    let tone = use_state_visual(current);
    let show_spinner = use_pending_indicator(move || current.get() == ControlState::Pending);
    let clickable = move || {
        has_action
            && !matches!(
                current.get(),
                ControlState::Disabled | ControlState::Pending
            )
    };
    let toggle = move || {
        if let Some(action) = on_change.clone() {
            run_toggle(action, value, local, job);
        }
    };

    view! {
        <button
            type="button"
            role="checkbox"
            class="op-rippled op-checkbox"
            aria-checked=move || value.get().to_string()
            aria-busy=move || (current.get() == ControlState::Pending).then_some("true")
            aria-disabled=move || (!clickable()).then_some("true")
            data-disabled=move || (current.get() == ControlState::Disabled).then_some("")
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
            <span
                class="op-sb op-checkbox-box"
                data-checked=move || value.get().then_some("")
                data-state=move || visual.state.get()
                data-result=move || visual.result.get()
                data-fading=move || visual.fading.get().then_some("")
            >
                <span
                    class="op-tone op-checkbox-mark"
                    data-state=move || tone.state.get()
                    data-result=move || tone.result.get()
                    data-fading=move || tone.fading.get().then_some("")
                >
                    {move || {
                        if show_spinner.get() {
                            view! { <PendingIndicator size=3.0/> }.into_any()
                        } else if value.get() {
                            view! { <Icon icon=icons::CHECK size=3.5/> }.into_any()
                        } else {
                            ().into_any()
                        }
                    }}
                </span>
                <PendingRun/>
            </span>
            {text.map(|t| view! { <span class="op-checkbox-text">{move || t.get()}</span> })}
        </button>
    }
}
