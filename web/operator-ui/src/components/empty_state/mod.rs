//! Message for empty views: EmptyState.

use crate::components::primitives::Icon;
use crate::core::{Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

pub(crate) const CSS: &str = include_str!("empty_state.css");

/// Message shown in place of a list or view that has no content.
#[component]
pub fn EmptyState(
    #[prop(optional, into)] title: MaybeProp<String>,
    #[prop(optional, into)] description: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    /// Buttons under the text.
    #[prop(optional, into)]
    actions: Slot,
    /// Custom icon slot replacing the built-in icon.
    #[prop(optional, into)]
    icon_slot: Slot,
    /// Custom title slot replacing the title text.
    #[prop(optional, into)]
    title_slot: Slot,
    /// Smaller variant for panels and table bodies.
    #[prop(optional)]
    compact: bool,
    /// Icon size in DU (`--op-empty-state-icon-size`).
    #[prop(optional, into)]
    icon_size: MaybeProp<f32>,
    /// Horizontal padding in DU (`--op-empty-state-padding-x`).
    #[prop(optional, into)]
    padding_x: MaybeProp<f32>,
    /// Vertical padding in DU (`--op-empty-state-padding-y`).
    #[prop(optional, into)]
    padding_y: MaybeProp<f32>,
    /// Gap between elements in DU (`--op-empty-state-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Gap below the icon in DU (`--op-empty-state-icon-gap`).
    #[prop(optional, into)]
    icon_gap: MaybeProp<f32>,
    /// Title font size in TU (`--op-empty-state-title-size`).
    #[prop(optional, into)]
    title_size: MaybeProp<f32>,
    /// Maximum width of description in DU (`--op-empty-state-max-width`).
    #[prop(optional, into)]
    max_width: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_icon = ui_icon(icon, "empty-state.icon", icons::INBOX);
    let resolved_title = ui_text(title, "empty-state.title", "No items");
    let default_icon_size = if compact { 8.0 } else { 12.0 };
    let size = move || icon_size.get().unwrap_or(default_icon_size);

    let rendered_icon = move || {
        if icon_slot.is_set() {
            view! { <span class="op-empty-icon">{icon_slot.run()}</span> }.into_any()
        } else {
            view! {
                <span class="op-empty-icon">
                    <Icon icon=resolved_icon.get() size=size()/>
                </span>
            }
            .into_any()
        }
    };

    let rendered_title = move || {
        if title_slot.is_set() {
            view! { <div class="op-empty-title">{title_slot.run()}</div> }.into_any()
        } else {
            view! { <div class="op-empty-title">{move || resolved_title.get()}</div> }.into_any()
        }
    };

    let rendered_desc = move || {
        description
            .get()
            .map(|d| view! { <div class="op-empty-text">{d}</div> })
    };

    let rendered_actions = move || {
        actions
            .is_set()
            .then(|| view! { <div class="op-empty-actions">{actions.run()}</div> })
    };

    view! {
        <div
            class=root_class("op-empty", class)
            style=root_style(
                vec![
                    Metric::du("--op-empty-state-icon-size", icon_size),
                    Metric::du("--op-empty-state-padding-x", padding_x),
                    Metric::du("--op-empty-state-padding-y", padding_y),
                    Metric::du("--op-empty-state-gap", gap),
                    Metric::du("--op-empty-state-icon-gap", icon_gap),
                    Metric::tu("--op-empty-state-title-size", title_size),
                    Metric::du("--op-empty-state-max-width", max_width),
                ],
                style,
            )
            data-compact=compact.then_some("")
        >
            {rendered_icon}
            {rendered_title}
            {rendered_desc}
            {rendered_actions}
        </div>
    }
}
