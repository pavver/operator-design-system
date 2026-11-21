//! Group of joined buttons sharing borders: ButtonGroup.

use crate::core::{Metric, root_class, root_style, ui_text};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("button_group.css");

/// Groups adjacent action or icon buttons, joining them with shared borders.
#[component]
pub fn ButtonGroup(
    /// Stacks buttons vertically instead of horizontally.
    #[prop(optional)]
    vertical: bool,
    /// Stretches buttons equally to fill the full container width.
    #[prop(optional)]
    spread: bool,
    /// Accessible label for the button group.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Gap between buttons in DU (`--op-button-group-gap`); defaults to `0 DU`.
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let resolved_label = ui_text(label, "button-group.label", "Button group");
    view! {
        <div
            class=root_class("op-button-group", class)
            role="group"
            aria-label=move || resolved_label.get()
            data-vertical=vertical.then_some("")
            data-spread=spread.then_some("")
            style=root_style(vec![Metric::du("--op-button-group-gap", gap)], style)
        >
            {children()}
        </div>
    }
}
