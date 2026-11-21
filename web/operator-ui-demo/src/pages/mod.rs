//! Live examples per documentation page.

mod accordion;
mod action_button;
mod avatar;
mod banner;
mod checkbox;
mod checkbox_group;
mod chip;
mod code_block;
mod code_field;
mod combobox;
mod common;
mod confirm_dialog;
mod console_terminal;
mod counter;
mod data_table;
mod dialog;
mod dropdown_menu;
mod empty_state;
mod field;
mod foundations;
mod graph;
mod header;
mod hover_card;
mod icon_button;
mod kbd;
mod language_picker;
mod linear_progress;
mod list_item;
mod marquee_text;
mod menu;
mod navigation_shell;
mod pagination;
mod popover;
mod primitives;
mod property_list;
mod radio_group;
mod scroll_area;
mod scrollbar;
mod search_field;
mod segmented_control;
mod sheet;
mod skeleton;
mod slider;
mod split_pane;
mod status_badge;
mod stepper;
mod switch;
mod tabs;
mod text_area;
mod text_field;
mod theme_toggle;
mod toast;
mod toolbar;
mod tooltip;
mod value_fields;
mod virtual_list;

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
        "components/action-button" => (action_button::action_button, true),
        "components/icon-button" => (icon_button::icon_button, true),
        "components/switch" => (switch::switch, true),
        "components/checkbox" => (checkbox::checkbox, true),
        "components/checkbox-group" => (checkbox_group::checkbox_group, true),
        "components/field" => (field::field, false),
        "components/text-field" => (text_field::text_field, true),
        "components/slider" => (slider::slider, true),
        "components/stepper" => (stepper::stepper, true),
        "components/segmented-control" => (segmented_control::segmented, true),
        "components/dropdown-menu" => (dropdown_menu::dropdown, false),
        "components/list-item" => (list_item::list_item, true),
        "components/status-badge" => (status_badge::status_badge, false),
        "components/avatar" => (avatar::avatar, false),
        "components/header" => (header::header, false),
        "components/confirm-dialog" => (confirm_dialog::confirm_dialog, true),
        "components/console-terminal" => (console_terminal::console, false),
        "components/marquee-text" => (marquee_text::marquee, false),
        "components/theme-toggle" => (theme_toggle::theme_toggle, false),
        "components/primitives" => (primitives::primitives, false),
        "components/tooltip" => (tooltip::tooltip, false),
        "components/toast" => (toast::toast, true),
        "components/banner" => (banner::banner, false),
        "components/linear-progress" => (linear_progress::linear_progress, false),
        "components/skeleton" => (skeleton::skeleton, true),
        "components/empty-state" => (empty_state::empty_state, true),
        "components/text-area" => (text_area::text_area, false),
        "components/search-field" => (search_field::search_field, false),
        "components/radio-group" => (radio_group::radio_group, true),
        "components/combobox" => (combobox::combobox, false),
        "components/language-picker" => (language_picker::language_picker, false),
        "components/value-fields" => (value_fields::value_fields, false),
        "components/chip" => (chip::chip, false),
        "components/menu" => (menu::menu, false),
        "components/hover-card" => (hover_card::hover_card, false),
        "components/popover" => (popover::popover, false),
        "components/dialog" => (dialog::dialog, true),
        "components/tabs" => (tabs::tabs, false),
        "components/sheet" => (sheet::sheet, false),
        "components/pagination" => (pagination::pagination, false),
        "components/data-table" => (data_table::data_table, true),
        "components/counter" => (counter::counter, false),
        "components/property-list" => (property_list::property_list, false),
        "components/accordion" => (accordion::accordion, false),
        "components/code-block" => (code_block::code_block, false),
        "components/kbd" => (kbd::kbd, false),
        "components/toolbar" => (toolbar::toolbar, true),
        "components/split-pane" => (split_pane::split_pane, false),
        "components/scroll-area" => (scroll_area::scroll_area, false),
        "components/scrollbar" => (scrollbar::scrollbar, false),
        "components/virtual-list" => (virtual_list::virtual_list, false),
        "components/graph" => (graph::graph, true),
        "components/code-field" => (code_field::code_field, false),
        "components/navigation-rail" => (navigation_shell::navigation_shell, false),
        "components/side-panel" => (navigation_shell::navigation_shell, false),
        "components/tree" => (navigation_shell::navigation_shell, false),
        _ => return None,
    };
    Some(entry)
}
