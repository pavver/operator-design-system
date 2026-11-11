//! Operator UI core component library for Leptos (CSR).
//!
//! Wrap the application in [`ThemeRoot`] to inject the generated tokens and component styles.

pub mod basic;
pub mod buttons;
pub mod core;
pub mod data;
pub mod dialog;
pub mod display;
pub mod feedback;
pub mod fields;
pub mod inputs;
pub mod layout;
pub mod nav;
pub mod overlay;
pub mod scroll;
pub mod text;
pub mod theme;
pub mod theme_toggle;
pub mod toggles;

pub use operator_ui_tokens as tokens;

/// Convenient glob import of every public component and type.
pub mod prelude {
    pub use crate::basic::*;
    pub use crate::buttons::*;
    pub use crate::core::{
        ActionResult, AsyncAction, ControlState, du, sleep_ms, tu, use_pending_indicator,
    };
    pub use crate::data::*;
    pub use crate::dialog::*;
    pub use crate::display::*;
    pub use crate::feedback::*;
    pub use crate::fields::*;
    pub use crate::inputs::*;
    pub use crate::layout::*;
    pub use crate::nav::*;
    pub use crate::overlay::Tooltip;
    pub use crate::scroll::{ScrollAxis, ScrollView, Scrollbar};
    pub use crate::text::*;
    pub use crate::theme::{OperatorTheme, Theme, ThemeRoot};
    pub use crate::theme_toggle::*;
    pub use crate::toggles::*;
    pub use operator_ui_tokens::{dim, icons, motion};
}
