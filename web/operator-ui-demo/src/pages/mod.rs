//! Live examples per documentation page.

pub mod core;
pub mod data;
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
        "components/chip" => (inputs::chip, false),
        "components/tabs" => (navigation::tabs, false),
        "components/kbd" => (data::kbd, false),
        "components/split-pane" => (layout::split_pane, false),
        _ => return None,
    };
    Some(entry)
}
