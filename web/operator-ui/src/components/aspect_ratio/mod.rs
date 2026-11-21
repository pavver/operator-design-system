//! Aspect ratio box: AspectRatio.

use crate::core::{Metric, root_class, root_style};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("aspect_ratio.css");

/// Responsive container that maintains a constant width-to-height ratio for its child element.
#[component]
pub fn AspectRatio(
    /// Aspect ratio (width / height); defaults to `16.0 / 9.0` (1.7778).
    #[prop(optional, into)]
    ratio: MaybeProp<f32>,
    /// Maximum width in DU (`--op-aspect-ratio-max-width`).
    #[prop(optional, into)]
    max_width: MaybeProp<f32>,
    /// Maximum height in DU (`--op-aspect-ratio-max-height`).
    #[prop(optional, into)]
    max_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let base_style = root_style(
        vec![
            Metric::du("--op-aspect-ratio-max-width", max_width),
            Metric::du("--op-aspect-ratio-max-height", max_height),
        ],
        style,
    );
    let full_style = move || {
        let mut out = String::new();
        if let Some(r) = ratio.get() {
            out.push_str(&format!("aspect-ratio:{r};"));
        }
        if let Some(rest) = base_style() {
            out.push_str(&rest);
        }
        (!out.is_empty()).then_some(out)
    };
    view! {
        <div
            class=root_class("op-aspect-ratio", class)
            style=full_style
        >
            <div class="op-aspect-ratio-content">
                {children()}
            </div>
        </div>
    }
}
