//! Live examples per documentation page.

pub mod core;
pub mod data;
pub mod feedback;
pub mod foundations;
pub mod inputs;
pub mod layout;
pub mod navigation;

use leptos::prelude::AnyView;

/// Live examples for a documentation page and whether they talk to the simulated backend.
pub fn examples(slug: &str) -> Option<(fn() -> AnyView, bool)> {
    let entry: (fn() -> AnyView, bool) = match slug {
        "foundations/color" => (foundations::color, false),
        "foundations/typography" => (foundations::typography, false),
        "foundations/units" => (foundations::units, false),
        "foundations/iconography" => (foundations::iconography, false),
        "foundations/motion" => (foundations::motion, false),
        "foundations/states" => (foundations::states, false),
        "components/action-button" => (core::action_button, true),
        "components/icon-button" => (core::icon_button, true),
        "components/switch" => (core::switch, true),
        "components/checkbox" => (core::checkbox, true),
        "components/checkbox-group" => (core::checkbox_group, true),
        "components/text-field" => (core::text_field, true),
        "components/slider" => (core::slider, true),
        "components/stepper" => (core::stepper, true),
        "components/segmented-control" => (core::segmented, true),
        "components/dropdown-menu" => (core::dropdown, false),
        "components/list-item" => (core::list_item, true),
        "components/status-badge" => (core::status_badge, false),
        "components/avatar" => (core::avatar, false),
        "components/header" => (core::header, false),
        "components/confirm-dialog" => (core::confirm_dialog, true),
        "components/console-terminal" => (core::console, false),
        "components/marquee-text" => (core::marquee, false),
        "components/theme-toggle" => (core::theme_toggle, false),
        "components/primitives" => (core::primitives, false),
        "components/tooltip" => (feedback::tooltip, false),
        "components/toast" => (feedback::toast, true),
        "components/banner" => (feedback::banner, false),
        "components/linear-progress" => (feedback::linear_progress, false),
        "components/skeleton" => (feedback::skeleton, false),
        "components/empty-state" => (feedback::empty_state, false),
        "components/text-area" => (inputs::text_area, false),
        "components/search-field" => (inputs::search_field, false),
        "components/radio-group" => (inputs::radio_group, true),
        "components/combobox" => (inputs::combobox, false),
        "components/chip" => (inputs::chip, false),
        "components/menu" => (navigation::menu, false),
        "components/dialog" => (navigation::dialog, true),
        "components/tabs" => (navigation::tabs, false),
        "components/sheet" => (navigation::sheet, false),
        "components/pagination" => (navigation::pagination, false),
        "components/data-table" => (data::data_table, true),
        "components/counter" => (data::counter, false),
        "components/property-list" => (data::property_list, false),
        "components/accordion" => (data::accordion, false),
        "components/code-block" => (data::code_block, false),
        "components/kbd" => (data::kbd, false),
        "components/toolbar" => (layout::toolbar, true),
        "components/split-pane" => (layout::split_pane, false),
        "components/scroll-area" => (layout::scroll_area, false),
        "components/scrollbar" => (layout::scrollbar, false),
        _ => return None,
    };
    Some(entry)
}
