//! Operator UI core component library for Leptos (CSR).
//!
//! Wrap the application in [`ThemeRoot`] to inject the generated tokens and component styles.

pub mod basic;
pub mod buttons;
pub mod core;
pub mod dialog;
pub mod display;
pub mod inputs;
pub mod text;
pub mod theme;
pub mod theme_toggle;
pub mod toggles;

pub use operator_ui_tokens as tokens;

/// Convenient glob import of every public component and type.
pub mod prelude {
    pub use crate::basic::*;
    pub use crate::buttons::*;
    pub use crate::core::{ActionResult, AsyncAction, ControlState, du, sleep_ms, tu};
    pub use crate::dialog::*;
    pub use crate::display::*;
    pub use crate::inputs::*;
    pub use crate::text::*;
    pub use crate::theme::{OperatorTheme, Theme, ThemeRoot};
    pub use crate::theme_toggle::*;
    pub use crate::toggles::*;
    pub use operator_ui_tokens::{dim, icons, motion};
}
