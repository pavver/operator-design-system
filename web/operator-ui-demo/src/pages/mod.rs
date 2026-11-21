//! Live examples per documentation page.

mod accordion;
mod action_button;
mod aspect_ratio;
mod avatar;
mod banner;
mod breadcrumbs;
mod button_group;
mod chat_message;
mod checkbox;
mod checkbox_group;
mod chip;
mod circular_progress;
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
mod editor;
mod empty_state;
mod fab;
mod field;
mod file_field;
mod form;
mod foundations;
mod graph;
mod header;
mod hover_card;
mod icon_button;
mod image;
mod kbd;
mod knob;
mod language_picker;
mod linear_progress;
mod list_item;
mod loading_bar;
mod loading_overlay;
mod marquee_text;
mod menu;
mod navigation_shell;
mod pagination;
mod popover;
mod popup_edit;
mod primitives;
mod property_list;
mod radio_group;
mod range_slider;
mod rating;
mod scroll_area;
mod scrollbar;
mod search_field;
mod segmented_control;
mod sheet;
mod skeleton;
mod slider;
mod space;
mod split_button;
mod split_pane;
mod status_badge;
mod stepper;
mod switch;
mod tabs;
mod text_area;
mod text_field;
mod theme_toggle;
mod timeline;
mod toast;
mod toolbar;
mod tooltip;
mod uploader;
mod value_fields;
mod video;
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
        "components/aspect-ratio" => (aspect_ratio::aspect_ratio, false),
        "components/breadcrumbs" => (breadcrumbs::breadcrumbs, false),
        "components/button-group" => (button_group::button_group, false),
        "components/chat-message" => (chat_message::chat_message, false),
        "components/circular-progress" => (circular_progress::circular_progress, false),
        "components/image" => (image::image, false),
        "components/loading-bar" => (loading_bar::loading_bar, false),
        "components/loading-overlay" => (loading_overlay::loading_overlay, false),
        "components/space" => (space::space, false),
        "components/timeline" => (timeline::timeline, false),
        "components/video" => (video::video, false),
        "components/editor" => (editor::editor, false),
        "components/fab" => (fab::fab, true),
        "components/file-field" => (file_field::file_field, false),
        "components/form" => (form::form, true),
        "components/knob" => (knob::knob, true),
        "components/popup-edit" => (popup_edit::popup_edit, true),
        "components/range-slider" => (range_slider::range_slider, true),
        "components/rating" => (rating::rating, true),
        "components/split-button" => (split_button::split_button, true),
        "components/uploader" => (uploader::uploader, true),
        _ => return None,
    };
    Some(entry)
}
