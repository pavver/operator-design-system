//! Content placeholder: Skeleton.

use crate::core::{Metric, du, root_class, root_style, ui_text};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("skeleton.css");

/// Placeholder that holds the place of content while it loads.
#[component]
pub fn Skeleton(
    /// Width in DU (`--op-skeleton-width`); full width when omitted.
    #[prop(optional, into)]
    width: MaybeProp<f32>,
    /// Height in DU (`--op-skeleton-height`); one text line when omitted.
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Number of text lines; the last line is shorter.
    #[prop(default = 1)]
    lines: u32,
    /// Circle for avatar placeholders.
    #[prop(optional)]
    circle: bool,
    /// Gap between text lines in DU (`--op-skeleton-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Text line height in TU (`--op-skeleton-text-height`).
    #[prop(optional, into)]
    text_height: MaybeProp<f32>,
    /// Text line spacing in TU (`--op-skeleton-text-line-height`).
    #[prop(optional, into)]
    text_line_height: MaybeProp<f32>,
    /// Accessible name; defaults to `skeleton.loading` ("Loading").
    #[prop(optional, into)]
    label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let count = lines.max(1);
    let resolved_label = ui_text(label, "skeleton.loading", "Loading");
    let bars = (0..count)
        .map(|i| {
            let last = count > 1 && i + 1 == count;
            view! {
                <span
                    class="op-skeleton"
                    data-circle=circle.then_some("")
                    data-text=height.get().is_none().then_some("")
                    style:width=if last { Some("60%".to_string()) } else { width.get().map(du) }
                    style:height=height.get().map(du)
                ></span>
            }
        })
        .collect_view();
    view! {
        <span
            class=root_class("op-skeleton-group", class)
            style=root_style(
                vec![
                    Metric::du("--op-skeleton-width", width),
                    Metric::du("--op-skeleton-height", height),
                    Metric::du("--op-skeleton-gap", gap),
                    Metric::tu("--op-skeleton-text-height", text_height),
                    Metric::tu("--op-skeleton-text-line-height", text_line_height),
                ],
                style,
            )
            aria-busy="true"
            aria-label=move || resolved_label.get()
        >
            {bars}
        </span>
    }
}
