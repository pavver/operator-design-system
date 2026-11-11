//! Operator UI core component library for Leptos (CSR).
//!
//! Wrap the application in [`ThemeRoot`] to inject the generated tokens and component styles.

pub mod components;
pub mod core;

pub use operator_ui_tokens as tokens;

/// Convenient glob import of every public component and type.
pub mod prelude {
    pub use crate::components::accordion::*;
    pub use crate::components::action_button::*;
    pub use crate::components::avatar::*;
    pub use crate::components::banner::*;
    pub use crate::components::checkbox::*;
    pub use crate::components::checkbox_group::*;
    pub use crate::components::chip::*;
    pub use crate::components::code_block::*;
    pub use crate::components::combobox::*;
    pub use crate::components::confirm_dialog::*;
    pub use crate::components::console_terminal::*;
    pub use crate::components::counter::*;
    pub use crate::components::data_table::*;
    pub use crate::components::dialog::*;
    pub use crate::components::dropdown_menu::*;
    pub use crate::components::empty_state::*;
    pub use crate::components::header::*;
    pub use crate::components::icon_button::*;
    pub use crate::components::kbd::*;
    pub use crate::components::linear_progress::*;
    pub use crate::components::list_item::*;
    pub use crate::components::marquee_text::*;
    pub use crate::components::menu::*;
    pub use crate::components::pagination::*;
    pub use crate::components::primitives::*;
    pub use crate::components::property_list::*;
    pub use crate::components::radio_group::*;
    pub use crate::components::scroll_area::*;
    pub use crate::components::scrollbar::*;
    pub use crate::components::search_field::*;
    pub use crate::components::segmented_control::*;
    pub use crate::components::sheet::*;
    pub use crate::components::skeleton::*;
    pub use crate::components::slider::*;
    pub use crate::components::split_pane::*;
    pub use crate::components::status_badge::*;
    pub use crate::components::stepper::*;
    pub use crate::components::switch::*;
    pub use crate::components::tabs::*;
    pub use crate::components::text_area::*;
    pub use crate::components::text_field::*;
    pub use crate::components::theme_toggle::*;
    pub use crate::components::toast::*;
    pub use crate::components::toolbar::*;
    pub use crate::components::tooltip::*;
    pub use crate::core::theme::{OperatorTheme, Theme, ThemeRoot};
    pub use crate::core::{
        ActionResult, AsyncAction, ControlState, du, sleep_ms, tu, use_pending_indicator,
    };
    pub use operator_ui_tokens::{dim, icons, motion};
}
