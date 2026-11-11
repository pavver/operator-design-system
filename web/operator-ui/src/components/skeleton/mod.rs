//! Content placeholder: Skeleton.

use crate::core::du;
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("skeleton.css");

/// Placeholder that holds the place of content while it loads.
#[component]
pub fn Skeleton(
    /// Width in DU; full width when omitted.
    #[prop(optional)]
    width: Option<f32>,
    /// Height in DU; one text line when omitted.
    #[prop(optional)]
    height: Option<f32>,
    /// Number of text lines; the last line is shorter.
    #[prop(default = 1)]
    lines: u32,
    /// Circle for avatar placeholders.
    #[prop(optional)]
    circle: bool,
) -> impl IntoView {
    let count = lines.max(1);
    let bars = (0..count)
        .map(|i| {
            let last = count > 1 && i + 1 == count;
            view! {
                <span
                    class="op-skeleton"
                    data-circle=circle.then_some("")
                    data-text=height.is_none().then_some("")
                    style:width=if last { Some("60%".to_string()) } else { width.map(du) }
                    style:height=height.map(du)
                ></span>
            }
        })
        .collect_view();
    view! {
        <span class="op-skeleton-group" aria-busy="true" aria-label="Loading">
            {bars}
        </span>
    }
}
