//! Control states, pointer press tracking, and derived interaction state.

use leptos::prelude::*;

/// Visual and interaction state of every Operator UI control.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ControlState {
    #[default]
    Idle,
    Pressed,
    Pending,
    Success,
    Error,
    Disabled,
    Focused,
    Selected,
}

impl ControlState {
    /// Value written to the `data-state` attribute.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Pressed => "pressed",
            Self::Pending => "pending",
            Self::Success => "success",
            Self::Error => "error",
            Self::Disabled => "disabled",
            Self::Focused => "focused",
            Self::Selected => "selected",
        }
    }

    /// Success and Error carry the pulsing result animation.
    pub const fn is_result(self) -> bool {
        matches!(self, Self::Success | Self::Error)
    }
}

/// Tracks pointer press state for a control.
#[derive(Clone, Copy)]
pub struct Press {
    pub pressed: RwSignal<bool>,
}

impl Press {
    pub fn new() -> Self {
        Self {
            pressed: RwSignal::new(false),
        }
    }

    pub fn down(&self) {
        self.pressed.set(true);
    }

    pub fn up(&self) {
        self.pressed.set(false);
    }
}

impl Default for Press {
    fn default() -> Self {
        Self::new()
    }
}

/// Control state in link mode, where only `Idle`, `Pressed`, and `Disabled` apply.
pub fn link_state(enabled: Signal<bool>, press: Press) -> Signal<ControlState> {
    Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if press.pressed.get() {
            ControlState::Pressed
        } else {
            ControlState::Idle
        }
    })
}

/// Shared field state: disabled takes priority, then explicit state, then focused, else idle.
#[cfg(any(feature = "combobox", feature = "search-field", feature = "text-area"))]
pub(crate) fn field_state(
    enabled: Signal<bool>,
    state: Signal<Option<ControlState>>,
    focused: RwSignal<bool>,
) -> Signal<ControlState> {
    Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if let Some(s) = state.get() {
            s
        } else if focused.get() {
            ControlState::Focused
        } else {
            ControlState::Idle
        }
    })
}
