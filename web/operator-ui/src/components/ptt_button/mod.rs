//! Push-to-talk button with pending, active, locked, and error states.

use crate::components::primitives::Icon;
use crate::core::{ControlState, Metric, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("ptt_button.css");

/// Drag distance in pixels to lock hands-free transmission (approx 20 DU).
const DRAG_LOCK_THRESHOLD_PX: f64 = 80.0;

/// Tactical push-to-talk button supporting press-and-hold transmission,
/// upward drag lock for hands-free speaking, and Operator state feedback.
#[component]
pub fn PttButton(
    /// Whether the button is actively in transmitting/talking state.
    #[prop(optional, into)]
    talking: MaybeProp<bool>,
    /// Whether transmission is locked into hands-free mode.
    #[prop(optional, into)]
    locked: MaybeProp<bool>,
    /// Explicit control state (e.g. Idle, Pending, Error, Disabled).
    #[prop(optional, into)]
    state: MaybeProp<ControlState>,
    /// Disables the button from receiving interaction.
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// Render in circular shape (round exception); defaults to false (square geometry).
    #[prop(optional, into)]
    circular: MaybeProp<bool>,
    /// Size in Design Units (`--op-ptt-button-size`). Default is 22.5 DU.
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    /// Border stroke width in Design Units (`--op-ptt-button-border-width`). Default is 0.5 DU.
    #[prop(optional, into)]
    border_width: MaybeProp<f32>,
    /// Custom textual label overriding default state string.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline CSS styles.
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// Callback invoked when talking or locked state changes `(is_talking, is_locked)`.
    #[prop(optional)]
    on_state_change: Option<Callback<(bool, bool)>>,
) -> impl IntoView {
    let start_y = RwSignal::new(0.0_f64);
    let is_pressed = RwSignal::new(false);

    let is_talking = Signal::derive(move || talking.get().unwrap_or(false));
    let is_locked = Signal::derive(move || locked.get().unwrap_or(false));
    let is_disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let is_circular = Signal::derive(move || circular.get().unwrap_or(false));

    let idle_text = ui_text(label, "ptt-button.idle", "PUSH TO TALK");
    let talking_text = ui_text(None::<String>.into(), "ptt-button.talking", "TALKING");
    let locked_text = ui_text(None::<String>.into(), "ptt-button.locked", "HANDS-FREE");
    let hint_text = ui_text(None::<String>.into(), "ptt-button.hint", "Slide up to lock");

    let icon_mic = ui_icon(None, "ptt-button.mic", icons::ARROW_UPWARD);
    let icon_lock = ui_icon(None, "ptt-button.lock", icons::LOCK);

    let current_state = Signal::derive(move || {
        if is_disabled.get() {
            ControlState::Disabled
        } else if let Some(s) = state.get() {
            s
        } else if is_locked.get() {
            ControlState::Success
        } else if is_talking.get() {
            ControlState::Selected
        } else {
            ControlState::Idle
        }
    });

    let display_label = Signal::derive(move || {
        if is_locked.get() {
            locked_text.get()
        } else if is_talking.get() {
            talking_text.get()
        } else {
            idle_text.get()
        }
    });

    let on_pointerdown = move |ev: ev::PointerEvent| {
        if is_disabled.get() {
            return;
        }
        if is_locked.get() {
            // Tap while locked stops hands-free transmission
            if let Some(cb) = on_state_change {
                cb.run((false, false));
            }
            return;
        }
        start_y.set(f64::from(ev.client_y()));
        is_pressed.set(true);
        if let Some(cb) = on_state_change {
            cb.run((true, false));
        }
    };

    let on_pointermove = move |ev: ev::PointerEvent| {
        if !is_pressed.get() || is_locked.get() || is_disabled.get() {
            return;
        }
        let current_y = f64::from(ev.client_y());
        let drag_delta = start_y.get() - current_y;
        if drag_delta > DRAG_LOCK_THRESHOLD_PX {
            // Locked hands-free mode
            is_pressed.set(false);
            if let Some(cb) = on_state_change {
                cb.run((true, true));
            }
        }
    };

    let on_pointerup = move |_ev: ev::PointerEvent| {
        if is_pressed.get() && !is_locked.get() {
            is_pressed.set(false);
            if let Some(cb) = on_state_change {
                cb.run((false, false));
            }
        }
    };

    let on_pointercancel = move |_ev: ev::PointerEvent| {
        if is_pressed.get() && !is_locked.get() {
            is_pressed.set(false);
            if let Some(cb) = on_state_change {
                cb.run((false, false));
            }
        }
    };

    let metrics = vec![
        Metric::du("--op-ptt-button-size", size),
        Metric::du("--op-ptt-button-border-width", border_width),
    ];
    let root_cls = root_class("op-ptt-button", class);
    let root_stl = root_style(metrics, style);

    view! {
        <button
            type="button"
            class=root_cls
            style=root_stl
            data-circular=move || is_circular.get().then_some("true")
            data-talking=move || is_talking.get().then_some("true")
            data-locked=move || is_locked.get().then_some("true")
            data-disabled=move || is_disabled.get().then_some("true")
            data-state=move || current_state.get().as_str()
            disabled=move || is_disabled.get()
            on:pointerdown=on_pointerdown
            on:pointermove=on_pointermove
            on:pointerup=on_pointerup
            on:pointercancel=on_pointercancel
            aria-pressed=move || is_talking.get().to_string()
            aria-label=move || display_label.get()
        >
            <span class="op-ptt-button__icon">
                {move || {
                    let icon_data = if is_locked.get() { icon_lock.get() } else { icon_mic.get() };
                    view! { <Icon icon=icon_data /> }
                }}
            </span>
            <span class="op-ptt-button__label">{move || display_label.get()}</span>
            {move || {
                (!is_locked.get() && is_talking.get()).then(|| view! {
                    <span class="op-ptt-button__hint">{hint_text.get()}</span>
                })
            }}
        </button>
    }
}
