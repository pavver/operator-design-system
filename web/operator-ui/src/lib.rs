//! Operator UI core component library for Leptos (CSR).
//!
//! Wrap the application in [`ThemeRoot`] to inject the generated tokens and component styles.

pub mod components;
pub mod core;

pub use operator_ui_tokens as tokens;

/// Convenient glob import of every public component and type.
pub mod prelude {
    #[cfg(feature = "accordion")]
    pub use crate::components::accordion::*;
    #[cfg(feature = "action-button")]
    pub use crate::components::action_button::*;
    #[cfg(feature = "avatar")]
    pub use crate::components::avatar::*;
    #[cfg(feature = "banner")]
    pub use crate::components::banner::*;
    #[cfg(feature = "checkbox")]
    pub use crate::components::checkbox::*;
    #[cfg(feature = "checkbox-group")]
    pub use crate::components::checkbox_group::*;
    #[cfg(feature = "chip")]
    pub use crate::components::chip::*;
    #[cfg(feature = "code-block")]
    pub use crate::components::code_block::*;
    #[cfg(feature = "combobox")]
    pub use crate::components::combobox::*;
    #[cfg(feature = "confirm-dialog")]
    pub use crate::components::confirm_dialog::*;
    #[cfg(feature = "console-terminal")]
    pub use crate::components::console_terminal::*;
    #[cfg(feature = "counter")]
    pub use crate::components::counter::*;
    #[cfg(feature = "data-table")]
    pub use crate::components::data_table::*;
    #[cfg(feature = "dialog")]
    pub use crate::components::dialog::*;
    #[cfg(feature = "dropdown-menu")]
    pub use crate::components::dropdown_menu::*;
    #[cfg(feature = "empty-state")]
    pub use crate::components::empty_state::*;
    #[cfg(feature = "header")]
    pub use crate::components::header::*;
    #[cfg(feature = "icon-button")]
    pub use crate::components::icon_button::*;
    #[cfg(feature = "kbd")]
    pub use crate::components::kbd::*;
    #[cfg(feature = "linear-progress")]
    pub use crate::components::linear_progress::*;
    #[cfg(feature = "list-item")]
    pub use crate::components::list_item::*;
    #[cfg(feature = "marquee-text")]
    pub use crate::components::marquee_text::*;
    #[cfg(feature = "menu")]
    pub use crate::components::menu::*;
    #[cfg(feature = "pagination")]
    pub use crate::components::pagination::*;
    #[cfg(feature = "primitives")]
    pub use crate::components::primitives::*;
    #[cfg(feature = "property-list")]
    pub use crate::components::property_list::*;
    #[cfg(feature = "radio-group")]
    pub use crate::components::radio_group::*;
    #[cfg(feature = "scroll-area")]
    pub use crate::components::scroll_area::*;
    #[cfg(feature = "scrollbar")]
    pub use crate::components::scrollbar::*;
    #[cfg(feature = "search-field")]
    pub use crate::components::search_field::*;
    #[cfg(feature = "segmented-control")]
    pub use crate::components::segmented_control::*;
    #[cfg(feature = "sheet")]
    pub use crate::components::sheet::*;
    #[cfg(feature = "skeleton")]
    pub use crate::components::skeleton::*;
    #[cfg(feature = "slider")]
    pub use crate::components::slider::*;
    #[cfg(feature = "split-pane")]
    pub use crate::components::split_pane::*;
    #[cfg(feature = "status-badge")]
    pub use crate::components::status_badge::*;
    #[cfg(feature = "stepper")]
    pub use crate::components::stepper::*;
    #[cfg(feature = "switch")]
    pub use crate::components::switch::*;
    #[cfg(feature = "tabs")]
    pub use crate::components::tabs::*;
    #[cfg(feature = "text-area")]
    pub use crate::components::text_area::*;
    #[cfg(feature = "text-field")]
    pub use crate::components::text_field::*;
    #[cfg(feature = "theme-toggle")]
    pub use crate::components::theme_toggle::*;
    #[cfg(feature = "toast")]
    pub use crate::components::toast::*;
    #[cfg(feature = "toolbar")]
    pub use crate::components::toolbar::*;
    #[cfg(feature = "tooltip")]
    pub use crate::components::tooltip::*;
    pub use crate::core::theme::{OperatorTheme, Theme, ThemeRoot};
    pub use crate::core::{
        ActionResult, AsyncAction, ControlState, du, sleep_ms, tu, use_pending_indicator,
    };
    pub use operator_ui_tokens::{dim, icons, motion};
}
