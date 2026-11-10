//! Operator UI component library for Leptos (CSR).
//!
//! Wrap the application in [`ThemeRoot`] to inject the generated tokens and component styles.

pub mod basic;
pub mod buttons;
pub mod core;
pub mod inputs;
pub mod theme;

pub use operator_ui_tokens as tokens;

/// Convenient glob import of every public component and type.
pub mod prelude {
    pub use crate::basic::*;
    pub use crate::buttons::*;
    pub use crate::core::{ActionResult, AsyncAction, ControlState, du, sleep_ms, tu};
    pub use crate::inputs::*;
    pub use crate::theme::{OperatorTheme, Theme, ThemeRoot};
    pub use operator_ui_tokens::{dim, icons, motion};
}
