//! Square icon button with state border feedback; supports sync and async clicks.

use crate::components::primitives::{Icon, PendingIndicator};
use crate::components::tooltip::{InsideTooltip, Tooltip};
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, PendingRun, Press, du, flash_icon,
    link_state, sleep_ms, spawn_ripple, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, motion};

pub(crate) const CSS: &str = include_str!("icon_button.css");

/// Square icon button with state border feedback; supports sync and async clicks.
///
/// With `href`, it renders a real link styled as an icon button and ignores the action props.
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
    /// Navigation target; switches the button to link mode.
    #[prop(optional, into)]
    href: Option<Signal<String>>,
    /// Opens the link in a new tab (link mode only).
    #[prop(optional)]
    new_tab: bool,
) -> impl IntoView {
    // The label doubles as the tooltip; an explicit parent Tooltip takes over instead.
    let tooltip_text = label
        .clone()
        .filter(|_| use_context::<InsideTooltip>().is_none());
    let with_tooltip = move |control: AnyView| match tooltip_text {
        Some(text) => view! { <Tooltip text=text>{control}</Tooltip> }.into_any(),
        None => control,
    };

    if let Some(href) = href {
        let press = Press::new();
        let current = link_state(enabled, press);
        let link = view! {
            <a
                class="op-sb op-sb-bg op-rippled op-icon-btn"
                aria-label=label
                style:width=du(size)
                style:height=du(size)
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
                <Icon icon=icon size=icon_size/>
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
            class="op-sb op-sb-bg op-rippled op-icon-btn"
            aria-label=label
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
                if show_spinner.get() {
                    view! { <PendingIndicator size=icon_size/> }.into_any()
                } else if s.is_result() && show_flash.get() {
                    flash_icon(s, icon_size)
                } else {
                    view! { <Icon icon=icon size=icon_size/> }.into_any()
                }
            }}
            <PendingRun/>
        </button>
    };
    with_tooltip(button.into_any())
}
