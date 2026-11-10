//! ActionButton and IconButton.

use crate::basic::{Icon, ProgressIndicator};
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, PendingRun, Press, du, sleep_ms,
    spawn_ripple, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};

/// Result icon shown briefly after an async action completes.
fn flash_icon(state: ControlState, size: f32) -> AnyView {
    if state == ControlState::Success {
        view! { <span class="op-c-success"><Icon icon=icons::CHECK size=size label="Success"/></span> }.into_any()
    } else {
        view! { <span class="op-c-error"><Icon icon=icons::CLOSE size=size label="Error"/></span> }
            .into_any()
    }
}

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
                        if s == ControlState::Pending {
                            view! {
                                <ProgressIndicator size=dim::ICON_SMALL stroke=dim::SPINNER_STROKE color="var(--op-pending)"/>
                            }
                                .into_any()
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

/// Square icon button with state border feedback; supports sync and async clicks.
#[component]
pub fn IconButton(
    icon: IconData,
    #[prop(optional, into)] label: Option<String>,
    /// Button size in DU.
    #[prop(default = dim::ROW_COMPACT)]
    size: f32,
    /// Icon size in DU.
    #[prop(default = dim::ICON_SMALL)]
    icon_size: f32,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] on_async_click: Option<AsyncAction>,
) -> impl IntoView {
    let local = RwSignal::new(ControlState::Idle);
    let show_flash = RwSignal::new(false);
    let job = JobToken::new();
    let press = Press::new();
    let has_action = on_click.is_some() || on_async_click.is_some();

    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if loading.get() {
            ControlState::Pending
        } else if let Some(s) = state.get() {
            s
        } else if press.pressed.get() {
            ControlState::Pressed
        } else {
            local.get()
        }
    });
    let visual = use_state_visual(current);
    let clickable = move || has_action && enabled.get() && current.get() != ControlState::Pending;

    let run = move || {
        if let Some(action) = on_async_click.clone() {
            let id = job.next();
            spawn_local(async move {
                local.set(ControlState::Pending);
                let result: ActionResult = action.call(()).await;
                if !job.is_current(id) {
                    return;
                }
                local.set(result.into());
                show_flash.set(true);
                sleep_ms(motion::LOCAL_FLASH_ICON_MS).await;
                if !job.is_current(id) {
                    return;
                }
                show_flash.set(false);
                sleep_ms(motion::LOCAL_FLASH_TAIL_MS).await;
                if job.is_current(id) {
                    local.set(ControlState::Idle);
                }
            });
        } else if let Some(cb) = on_click {
            cb.run(());
        }
    };

    view! {
        <button
            type="button"
            class="op-sb op-sb-bg op-rippled op-icon-btn"
            aria-label=label.clone()
            title=label
            style:width=du(size)
            style:height=du(size)
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
            {move || {
                let s = current.get();
                if s == ControlState::Pending {
                    view! { <ProgressIndicator size=icon_size stroke=dim::SPINNER_STROKE color="var(--op-pending)"/> }
                        .into_any()
                } else if s.is_result() && show_flash.get() {
                    flash_icon(s, icon_size)
                } else {
                    view! { <Icon icon=icon size=icon_size/> }.into_any()
                }
            }}
            <PendingRun/>
        </button>
    }
}
