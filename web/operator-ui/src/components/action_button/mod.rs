//! Asynchronous action button.

use crate::core::{
    AsyncAction, ControlState, JobToken, Metric, PendingRun, Press, Slot, link_state, root_class,
    root_style, sleep_ms, spawn_ripple, ui_icon, ui_text, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, icons, motion};

mod visuals;
use visuals::{pending, symbol};

const ICON_SIZE: &str = "var(--op-button-icon-size, var(--op-dim-icon-small))";

pub(crate) const CSS: &str = include_str!("action_button.css");

/// Asynchronous action button with a pending running border and a two-phase result flash.
///
/// With `href`, it renders a real link styled as a button and ignores the async props.
#[component]
pub fn ActionButton(
    #[prop(optional, into)] text: MaybeProp<String>,
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
    /// Navigation target; switches the button to link mode.
    #[prop(optional, into)]
    href: Option<Signal<String>>,
    /// Opens the link in a new tab (link mode only).
    #[prop(optional)]
    new_tab: bool,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] spinner_stroke: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] leading: Slot,
    #[prop(optional, into)] trailing: Slot,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
    #[prop(optional, into)] success_label: MaybeProp<String>,
    #[prop(optional)] success_icon: Option<IconData>,
    #[prop(optional, into)] error_label: MaybeProp<String>,
    #[prop(optional)] error_icon: Option<IconData>,
) -> impl IntoView {
    let loading_label = ui_text(loading_label, "action-button.loading", "Loading");
    let success_label = ui_text(success_label, "action-button.success", "Success");
    let success_icon = ui_icon(success_icon, "action-button.success", icons::CHECK);
    let error_label = ui_text(error_label, "action-button.error", "Error");
    let error_icon = ui_icon(error_icon, "action-button.error", icons::CLOSE);
    let text = ui_text(text, "action-button.text", "");

    let root_metrics = vec![
        Metric::du("--op-button-spinner-stroke", spinner_stroke),
        Metric::du("--op-button-height", height),
        Metric::du("--op-button-padding-x", padding_x),
        Metric::du("--op-button-gap", gap),
        Metric::du("--op-button-icon-size", icon_size),
        Metric::tu("--op-button-font-size", font_size),
        Metric::tu("--op-button-line-height", line_height),
    ];

    let label = move || {
        view! {
            <span class="op-btn-label">
                {leading.run()}
                {icon.map(|i| view! { <span class="op-btn-icon">{symbol(Signal::stored(i), ICON_SIZE, None)}</span> })}
                {if content.is_set() { content.run() } else { Some(view! { <span>{move || text.get()}</span> }.into_any()) }}
                {trailing.run()}
            </span>
        }
    };

    if let Some(href) = href {
        let press = Press::new();
        let current = link_state(enabled, press);
        return view! {
            <a
                class=root_class("op-sb op-sb-bg op-rippled op-btn", class)
                class:op-danger=dangerous
                style=root_style(root_metrics, style)
                href=move || enabled.get().then(|| href.get())
                target=new_tab.then_some("_blank")
                rel=new_tab.then_some("noopener noreferrer")
                role="link"
                data-state=move || current.get().as_str()
                aria-disabled=move || (!enabled.get()).then_some("true")
                on:pointerdown=move |ev| {
                    if enabled.get_untracked() {
                        press.down();
                        spawn_ripple(&ev);
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
            >
                {label()}
            </a>
        }
        .into_any();
    }

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

    view! {
        <button
            type="button"
            class=root_class("op-sb op-sb-bg op-rippled op-btn", class)
            class:op-danger=dangerous
            style=root_style(root_metrics, style)
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
                            pending(ICON_SIZE, loading_label)
                        } else if s.is_result() && show_flash.get() {
                            if s == ControlState::Success { symbol(success_icon, ICON_SIZE, Some(success_label)) } else { symbol(error_icon, ICON_SIZE, Some(error_label)) }
                        } else {
                            label().into_any()
                        }
                    }}
                </span>
            </span>
            <PendingRun/>
        </button>
    }
    .into_any()
}
