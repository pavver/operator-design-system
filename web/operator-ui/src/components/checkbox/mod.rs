//! Checkbox: AsyncCheckbox.

use crate::core::{
    AsyncAction, ControlState, Metric, PendingRun, Slot, ToggleControl, root_class, root_style,
    spawn_ripple, ui_icon, ui_text, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

use crate::core::glyph::{pending_spinner, symbol};

const ICON_SIZE: &str = "var(--op-checkbox-icon-size, calc(var(--op-du) * 3.5))";
const SPINNER_SIZE: &str = "var(--op-checkbox-spinner-size, calc(var(--op-du) * 3))";

pub(crate) const CSS: &str = include_str!("checkbox.css");

/// Rectangular checkbox with optional label; commits the value only after a successful request.
#[component]
pub fn AsyncCheckbox(
    #[prop(into)] checked: Signal<bool>,
    #[prop(optional, into)] text: Option<Signal<String>>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] on_change: Option<AsyncAction<bool>>,
    #[prop(optional, into)] size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] spinner_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] spinner_stroke: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] label_slot: Slot,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional)] checked_icon: Option<IconData>,
) -> impl IntoView {
    let loading_label = ui_text(loading_label, "checkbox.loading", "Loading");
    let label = ui_text(label, "checkbox.label", "Checkbox");
    let checked_icon = ui_icon(checked_icon, "checkbox.checked", icons::CHECK);

    let root_metrics = vec![
        Metric::du("--op-checkbox-spinner-stroke", spinner_stroke),
        Metric::du("--op-checkbox-size", size),
        Metric::du("--op-checkbox-gap", gap),
        Metric::du("--op-checkbox-padding-x", padding_x),
        Metric::du("--op-checkbox-padding-y", padding_y),
        Metric::du("--op-checkbox-icon-size", icon_size),
        Metric::du("--op-checkbox-spinner-size", spinner_size),
        Metric::tu("--op-checkbox-font-size", font_size),
        Metric::tu("--op-checkbox-line-height", line_height),
    ];

    let has_label_slot = label_slot.is_set();
    let ctrl = ToggleControl::new(checked, state, enabled, on_change.is_some());
    let value = ctrl.value;
    let current = ctrl.current;
    let press = ctrl.press;
    let clickable = move || ctrl.clickable();
    let toggle = move || ctrl.toggle(on_change.as_ref());

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

    view! {
        <button
            type="button"
            role="checkbox"
            class=root_class("op-rippled op-checkbox", class)
            style=root_style(root_metrics, style)
            aria-label=move || if text.is_none() && !has_label_slot { Some(label.get()) } else { None }
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
                            pending_spinner("checkbox", SPINNER_SIZE, loading_label)
                        } else if value.get() {
                            symbol(checked_icon, ICON_SIZE, None)
                        } else {
                            ().into_any()
                        }
                    }}
                </span>
                <PendingRun/>
            </span>
            {if label_slot.is_set() { label_slot.run() } else { text.map(|t| view! { <span class="op-checkbox-text">{move || t.get()}</span> }.into_any()) }}
        </button>
    }
}
