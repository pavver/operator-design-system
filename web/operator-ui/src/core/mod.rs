//! Shared interaction primitives: control states, async actions, state-border visuals, ripple and units.

mod action;
mod action_scope;
pub mod custom;
pub mod glyph;
#[cfg(feature = "observers")]
pub mod observers;
pub mod overlay;
mod ripple;
#[cfg(any(feature = "dialog", feature = "sheet", feature = "gallery-viewer"))]
mod scroll_lock;
mod state;
#[cfg(any(feature = "code-block", feature = "console-terminal"))]
pub(crate) mod styled_text;
pub mod theme;
mod units;
mod visual;

#[cfg(any(feature = "checkbox", feature = "switch"))]
pub(crate) use action::ToggleControl;
#[cfg(any(feature = "code-block", feature = "property-list"))]
pub(crate) use action::copy_action;
pub use action::{ActionResult, AsyncAction, JobToken, copy_text, sleep_ms};
pub use action_scope::{ActionScope, join_action_scope};
pub use custom::*;
#[cfg(feature = "observers")]
pub use observers::*;
pub use overlay::*;
pub use ripple::spawn_ripple;
#[cfg(any(feature = "dialog", feature = "sheet", feature = "gallery-viewer"))]
pub(crate) use scroll_lock::use_page_scroll_lock;
#[cfg(any(feature = "combobox", feature = "search-field", feature = "text-area"))]
pub(crate) use state::field_state;
pub use state::{ControlState, Press, link_state};
pub use theme::{OperatorTheme, Theme, ThemeRoot};
pub use units::{du, tu};
pub use visual::*;
