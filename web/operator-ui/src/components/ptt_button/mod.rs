//! Push-to-talk button with pending, active, locked, and error states.

use crate::components::primitives::Icon;
use crate::core::{ControlState, Metric, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("ptt_button.css");

/// Drag distance in pixels to lock hands-free transmission (approx 20 DU).
const DRAG_LOCK_THRESHOLD_PX: f64 = 80.0;

#[derive(Clone, Copy)]
struct PttGestureState {
    start_y: RwSignal<f64>,
    is_pressed: RwSignal<bool>,
}

impl PttGestureState {
    fn new() -> Self {
        Self {
            start_y: RwSignal::new(0.0),
            is_pressed: RwSignal::new(false),
        }
    }

    fn on_down(
        self,
        client_y: f64,
        disabled: bool,
        locked: bool,
        on_state_change: Option<Callback<(bool, bool)>>,
    ) {
        if disabled {
            return;
        }
        if locked {
            if let Some(cb) = on_state_change {
                cb.run((false, false));
            }
            return;
        }
        self.start_y.set(client_y);
        self.is_pressed.set(true);
        if let Some(cb) = on_state_change {
            cb.run((true, false));
        }
    }

    fn on_move(
        self,
        client_y: f64,
        disabled: bool,
        locked: bool,
        on_state_change: Option<Callback<(bool, bool)>>,
    ) {
        if !self.is_pressed.get() || locked || disabled {
            return;
        }
        let drag_delta = self.start_y.get() - client_y;
        if drag_delta > DRAG_LOCK_THRESHOLD_PX {
            self.is_pressed.set(false);
            if let Some(cb) = on_state_change {
                cb.run((true, true));
            }
        }
    }

    fn on_release(self, locked: bool, on_state_change: Option<Callback<(bool, bool)>>) {
        if self.is_pressed.get() && !locked {
            self.is_pressed.set(false);
            if let Some(cb) = on_state_change {
                cb.run((false, false));
            }
        }
    }
}

pub fn resolve_ptt_state(
    is_disabled: bool,
    explicit_state: Option<ControlState>,
    is_locked: bool,
    is_talking: bool,
) -> ControlState {
    if is_disabled {
        ControlState::Disabled
    } else if let Some(s) = explicit_state {
        s
    } else if is_locked {
        ControlState::Success
    } else if is_talking {
        ControlState::Selected
    } else {
        ControlState::Idle
    }
}

pub fn resolve_ptt_label(
    is_locked: bool,
    is_talking: bool,
    locked_text: &str,
    talking_text: &str,
    idle_text: &str,
) -> String {
    if is_locked {
        locked_text.to_string()
    } else if is_talking {
        talking_text.to_string()
    } else {
        idle_text.to_string()
    }
}

pub fn resolve_ptt_icon(
    is_locked: bool,
    icon_lock: operator_ui_tokens::Icon,
    icon_mic: operator_ui_tokens::Icon,
) -> operator_ui_tokens::Icon {
    if is_locked { icon_lock } else { icon_mic }
}

#[component]
fn PttButtonHint(show: Signal<bool>, hint_text: Signal<String>) -> impl IntoView {
    move || {
        show.get().then(|| {
            view! { <span class="op-ptt-button__hint">{hint_text.get()}</span> }
        })
    }
}

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
    let gesture = PttGestureState::new();

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
        resolve_ptt_state(
            is_disabled.get(),
            state.get(),
            is_locked.get(),
            is_talking.get(),
        )
    });

    let display_label = Signal::derive(move || {
        resolve_ptt_label(
            is_locked.get(),
            is_talking.get(),
            &locked_text.get(),
            &talking_text.get(),
            &idle_text.get(),
        )
    });

    let on_pointerdown = move |ev: ev::PointerEvent| {
        gesture.on_down(
            f64::from(ev.client_y()),
            is_disabled.get(),
            is_locked.get(),
            on_state_change,
        );
    };

    let on_pointermove = move |ev: ev::PointerEvent| {
        gesture.on_move(
            f64::from(ev.client_y()),
            is_disabled.get(),
            is_locked.get(),
            on_state_change,
        );
    };

    let on_pointerup = move |_ev: ev::PointerEvent| {
        gesture.on_release(is_locked.get(), on_state_change);
    };

    let on_pointercancel = move |_ev: ev::PointerEvent| {
        gesture.on_release(is_locked.get(), on_state_change);
    };

    let metrics = vec![
        Metric::du("--op-ptt-button-size", size),
        Metric::du("--op-ptt-button-border-width", border_width),
    ];
    let root_cls = root_class("op-ptt-button", class);
    let root_stl = root_style(metrics, style);

    let show_hint = Signal::derive(move || !is_locked.get() && is_talking.get());

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
                    let icon_data = resolve_ptt_icon(is_locked.get(), icon_lock.get(), icon_mic.get());
                    view! { <Icon icon=icon_data /> }
                }}
            </span>
            <span class="op-ptt-button__label">{move || display_label.get()}</span>
            <PttButtonHint show=show_hint hint_text=hint_text />
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_ptt_state() {
        assert_eq!(
            resolve_ptt_state(true, None, false, false),
            ControlState::Disabled
        );
        assert_eq!(
            resolve_ptt_state(false, Some(ControlState::Error), false, false),
            ControlState::Error
        );
        assert_eq!(
            resolve_ptt_state(false, None, true, false),
            ControlState::Success
        );
        assert_eq!(
            resolve_ptt_state(false, None, false, true),
            ControlState::Selected
        );
        assert_eq!(
            resolve_ptt_state(false, None, false, false),
            ControlState::Idle
        );
    }

    #[test]
    fn test_resolve_ptt_label() {
        assert_eq!(
            resolve_ptt_label(true, false, "LOCKED", "TALK", "IDLE"),
            "LOCKED"
        );
        assert_eq!(
            resolve_ptt_label(false, true, "LOCKED", "TALK", "IDLE"),
            "TALK"
        );
        assert_eq!(
            resolve_ptt_label(false, false, "LOCKED", "TALK", "IDLE"),
            "IDLE"
        );
    }

    #[test]
    fn test_resolve_ptt_icon() {
        assert_eq!(
            resolve_ptt_icon(true, icons::LOCK, icons::ARROW_UPWARD),
            icons::LOCK
        );
        assert_eq!(
            resolve_ptt_icon(false, icons::LOCK, icons::ARROW_UPWARD),
            icons::ARROW_UPWARD
        );
    }
}
