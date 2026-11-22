//! Square icon button with state border feedback; supports sync and async clicks.

use crate::components::tooltip::{InsideTooltip, Tooltip};
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, Metric, PendingRun, Press, Slot, link_state,
    root_class, root_style, sleep_ms, spawn_ripple, ui_icon, ui_text, use_pending_indicator,
    use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, icons, motion};

use crate::core::glyph::{pending_spinner, symbol};

const ICON_SIZE: &str = "var(--op-button-icon-size, var(--op-dim-icon-small))";

pub(crate) const CSS: &str = include_str!("icon_button.css");

/// Square icon button with state border feedback; supports sync and async clicks.
///
/// With `href`, it renders a real link styled as an icon button and ignores the action props.
#[component]
pub fn IconButton(
    icon: IconData,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] on_async_click: Option<AsyncAction>,
    /// Navigation target; switches the button to link mode.
    #[prop(optional, into)]
    href: Option<Signal<String>>,
    /// Opens the link in a new tab (link mode only).
    #[prop(optional)]
    new_tab: bool,
    #[prop(optional, into)] size: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
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
    let has_label = label.get_untracked().is_some();
    let loading_label = ui_text(loading_label, "icon-button.loading", "Loading");
    let success_label = ui_text(success_label, "icon-button.success", "Success");
    let success_icon = ui_icon(success_icon, "icon-button.success", icons::CHECK);
    let error_label = ui_text(error_label, "icon-button.error", "Error");
    let error_icon = ui_icon(error_icon, "icon-button.error", icons::CLOSE);
    let label = ui_text(label, "icon-button.label", "Action");

    let root_metrics = vec![
        Metric::du("--op-button-spinner-stroke", spinner_stroke),
        Metric::du("--op-button-size", size),
        Metric::du("--op-button-icon-size", icon_size),
        Metric::du("--op-button-gap", gap),
    ];

    // The label doubles as the tooltip; an explicit parent Tooltip takes over instead.
    let with_tooltip = move |control: AnyView| {
        if has_label && use_context::<InsideTooltip>().is_none() {
            view! { <Tooltip text=label>{control}</Tooltip> }.into_any()
        } else {
            control
        }
    };
    let label_content = move || {
        view! {
            <span class="op-icon-btn-content">
                {leading.run()}
                {if content.is_set() { content.run() } else { Some(symbol(Signal::stored(icon), ICON_SIZE, None)) }}
                {trailing.run()}
            </span>
        }
    };

    if let Some(href) = href {
        let press = Press::new();
        let current = link_state(enabled, press);
        let link = view! {
            <a
                class=root_class("op-sb op-sb-bg op-rippled op-icon-btn", class)
            style=root_style(root_metrics, style)
                aria-label=move || label.get()
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
                {label_content()}
            </a>
        };
        return with_tooltip(link.into_any());
    }

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
    let show_spinner = use_pending_indicator(move || current.get() == ControlState::Pending);
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

    let button = view! {
        <button
            type="button"
            class=root_class("op-sb op-sb-bg op-rippled op-icon-btn", class)
            style=root_style(root_metrics, style)
            aria-label=move || label.get()
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
                if show_spinner.get() {
                    pending_spinner("button", ICON_SIZE, loading_label)
                } else if s.is_result() && show_flash.get() {
                    if s == ControlState::Success { symbol(success_icon, ICON_SIZE, Some(success_label)) } else { symbol(error_icon, ICON_SIZE, Some(error_label)) }
                } else {
                    label_content().into_any()
                }
            }}
            <PendingRun/>
        </button>
    };
    with_tooltip(button.into_any())
}
