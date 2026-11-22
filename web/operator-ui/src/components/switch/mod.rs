//! Capsule switch: AsyncSwitch.

use crate::core::{
    AsyncAction, ControlState, Metric, PendingRun, Slot, ToggleControl, root_class, root_style,
    spawn_ripple, ui_icon, ui_text, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

use crate::core::glyph::{pending_spinner, symbol};

const ICON_SIZE: &str = "var(--op-switch-icon-size, calc(var(--op-du) * 3))";
const SPINNER_SIZE: &str = "var(--op-switch-spinner-size, calc(var(--op-du) * 3.5))";

pub(crate) const CSS: &str = include_str!("switch.css");

/// Capsule switch; the thumb parks in the center with a spinner while the request is pending.
#[component]
pub fn AsyncSwitch(
    #[prop(into)] checked: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] on_toggle: Option<AsyncAction<bool>>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] thumb_size: MaybeProp<f32>,
    #[prop(optional, into)] thumb_inset: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] spinner_size: MaybeProp<f32>,
    #[prop(optional, into)] spinner_stroke: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
    #[prop(optional)] checked_icon: Option<IconData>,
    #[prop(optional)] unchecked_icon: Option<IconData>,
) -> impl IntoView {
    let loading_label = ui_text(loading_label, "switch.loading", "Loading");
    let label = ui_text(label, "switch.label", "Switch");
    let checked_icon = ui_icon(checked_icon, "switch.checked", icons::CHECK);
    let unchecked_icon = ui_icon(unchecked_icon, "switch.unchecked", icons::CLOSE);

    let root_metrics = vec![
        Metric::du("--op-switch-width", width),
        Metric::du("--op-switch-height", height),
        Metric::du("--op-switch-thumb-size", thumb_size),
        Metric::du("--op-switch-thumb-inset", thumb_inset),
        Metric::du("--op-switch-icon-size", icon_size),
        Metric::du("--op-switch-spinner-size", spinner_size),
        Metric::du("--op-switch-spinner-stroke", spinner_stroke),
    ];

    let ctrl = ToggleControl::new(checked, state, enabled, on_toggle.is_some());
    let value = ctrl.value;
    let current = ctrl.current;
    let press = ctrl.press;
    let visual = use_state_visual(current);
    let show_spinner = use_pending_indicator(move || current.get() == ControlState::Pending);
    let clickable = move || ctrl.clickable();
    let toggle = move || ctrl.toggle(on_toggle.as_ref());

    view! {
        <button
            type="button"
            role="switch"
            class=root_class("op-sb op-rippled op-switch", class)
            style=root_style(root_metrics, style)
            aria-label=move || label.get()
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
                        pending_spinner("switch", SPINNER_SIZE, loading_label)
                    } else {
                        if content.is_set() { content.run().into_any() } else {
                            let icon = if value.get() { checked_icon } else { unchecked_icon };
                            symbol(icon, ICON_SIZE, None)
                        }
                    }
                }}
            </span>
            <PendingRun/>
        </button>
    }
}
