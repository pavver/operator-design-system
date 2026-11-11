//! Message for empty views: EmptyState.

use crate::components::primitives::Icon;
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

pub(crate) const CSS: &str = include_str!("empty_state.css");

/// Message shown in place of a list or view that has no content.
#[component]
pub fn EmptyState(
    #[prop(into)] title: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(default = icons::INBOX)] icon: IconData,
    /// Buttons under the text.
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    /// Smaller variant for panels and table bodies.
    #[prop(optional)]
    compact: bool,
) -> impl IntoView {
    let icon_size = if compact { 8.0 } else { 12.0 };
    view! {
        <div class="op-empty" data-compact=compact.then_some("")>
            <span class="op-empty-icon"><Icon icon=icon size=icon_size/></span>
            <div class="op-empty-title">{title}</div>
            {description.map(|d| view! { <div class="op-empty-text">{d}</div> })}
            {actions.map(|a| view! { <div class="op-empty-actions">{a.run()}</div> })}
        </div>
    }
}
