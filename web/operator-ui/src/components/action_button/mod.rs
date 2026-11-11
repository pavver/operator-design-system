//! Asynchronous action button.

use crate::components::primitives::{Icon, PendingIndicator};
use crate::core::{
    AsyncAction, ControlState, JobToken, PendingRun, Press, flash_icon, sleep_ms, spawn_ripple,
    use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, motion};

pub(crate) const CSS: &str = include_str!("action_button.css");

/// Asynchronous action button with a pending running border and a two-phase result flash.
#[component]
pub fn ActionButton(
    #[prop(into)] text: Signal<String>,
    #[prop(optional)] icon: Option<IconData>,
    /// External state; when `Some`, it overrides the local async state.
    #[prop(optional, into)]
    state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(default = true)] show_pending_border: bool,
    #[prop(optional)] keep_flash_icon_on_success: bool,
    /// Destructive style: red content and tinted background.
    #[prop(optional)]
    dangerous: bool,
    #[prop(optional, into)] on_click: Option<AsyncAction>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] style: String,
) -> impl IntoView {
    let local = RwSignal::new(ControlState::Idle);
    let show_flash = RwSignal::new(false);
    let job = JobToken::new();
    let press = Press::new();
    let has_action = on_click.is_some();

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
    let show_spinner = use_pending_indicator(move || current.get() == ControlState::Pending);
    let border_state = Signal::derive(move || {
        let s = current.get();
        if s == ControlState::Pending && !show_pending_border {
            ControlState::Idle
        } else {
            s
        }
    });
    let visual = use_state_visual(border_state);
    let clickable = move || {
        let s = current.get();
        has_action && s != ControlState::Disabled && s != ControlState::Pending
    };

    let run = move || {
        let Some(action) = on_click.clone() else {
            return;
        };
        let s = current.get_untracked();
        if dangerous
            && matches!(
                s,
                ControlState::Pending | ControlState::Success | ControlState::Error
            )
        {
            return;
        }
        let id = job.next();
        spawn_local(async move {
            local.set(ControlState::Pending);
            let result = action.call(()).await;
            if !job.is_current(id) {
                return;
            }
            let success = result.is_success();
            local.set(result.into());
            show_flash.set(true);
            if success && keep_flash_icon_on_success {
                sleep_ms(motion::RESULT_HOLD_MS).await;
                if !job.is_current(id) {
                    return;
                }
                show_flash.set(false);
            } else {
                sleep_ms(motion::LOCAL_FLASH_ICON_MS).await;
                if !job.is_current(id) {
                    return;
                }
                show_flash.set(false);
                sleep_ms(motion::LOCAL_FLASH_TAIL_MS).await;
            }
            if job.is_current(id) {
                local.set(ControlState::Idle);
            }
        });
    };

    let label = move || {
        view! {
            <span class="op-btn-label">
                {icon.map(|i| view! { <span class="op-btn-icon"><Icon icon=i size=dim::ICON_SMALL/></span> })}
                <span>{move || text.get()}</span>
            </span>
        }
    };

    view! {
        <button
            type="button"
            class=format!("op-sb op-sb-bg op-rippled op-btn {class}")
            class:op-danger=dangerous
            style=style
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
            aria-busy=move || (current.get() == ControlState::Pending).then_some("true")
            aria-disabled=move || (!clickable()).then_some("true")
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
                    run();
                }
            }
        >
            <span class="op-stack">
                <span class="op-ghost" aria-hidden="true">{label()}</span>
                <span class="op-btn-content">
                    {move || {
                        let s = current.get();
                        if show_spinner.get() {
                            view! { <PendingIndicator size=dim::ICON_SMALL/> }.into_any()
                        } else if s.is_result() && show_flash.get() {
                            flash_icon(s, dim::ICON_SMALL)
                        } else {
                            label().into_any()
                        }
                    }}
                </span>
            </span>
            <PendingRun/>
        </button>
    }
}
