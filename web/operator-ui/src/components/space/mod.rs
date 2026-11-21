//! Flexible spacer for flex containers: Space.

use crate::core::{Metric, root_class, root_style};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("space.css");

/// Flexible spacer that expands to fill remaining space in a flex row or column.
#[component]
pub fn Space(
    /// Minimum width in DU (`--op-space-min-width`).
    #[prop(optional, into)]
    min_width: MaybeProp<f32>,
    /// Minimum height in DU (`--op-space-min-height`).
    #[prop(optional, into)]
    min_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            class=root_class("op-space", class)
            aria-hidden="true"
            style=root_style(
                vec![
                    Metric::du("--op-space-min-width", min_width),
                    Metric::du("--op-space-min-height", min_height),
                ],
                style,
            )
        ></div>
    }
}
