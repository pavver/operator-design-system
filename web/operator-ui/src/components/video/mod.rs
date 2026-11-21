//! Embedded video with responsive aspect ratio: Video.

use crate::core::{Metric, root_class, root_style, ui_text};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("video.css");

/// Responsive container for embedded videos or iframes that maintains a fixed aspect ratio.
#[component]
pub fn Video(
    /// Video stream or iframe embed URL.
    #[prop(into)]
    src: Signal<String>,
    /// Accessible title for the iframe.
    #[prop(optional, into)]
    title: MaybeProp<String>,
    /// Aspect ratio (width / height); defaults to `16.0 / 9.0` (1.7778).
    #[prop(optional, into)]
    ratio: MaybeProp<f32>,
    /// Maximum width in DU (`--op-video-max-width`).
    #[prop(optional, into)]
    max_width: MaybeProp<f32>,
    /// Maximum height in DU (`--op-video-max-height`).
    #[prop(optional, into)]
    max_height: MaybeProp<f32>,
    /// Feature policy permissions.
    #[prop(optional, into)]
    allow: MaybeProp<String>,
    /// Enables fullscreen support; defaults to `true`.
    #[prop(default = true)]
    allow_fullscreen: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_title = ui_text(title, "video.title", "Embedded video");
    let base_style = root_style(
        vec![
            Metric::du("--op-video-max-width", max_width),
            Metric::du("--op-video-max-height", max_height),
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
            class=root_class("op-video", class)
            style=full_style
        >
            <iframe
                class="op-video-frame"
                src=move || src.get()
                title=move || resolved_title.get()
                allow=move || allow.get().unwrap_or_else(|| "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture".to_string())
                allowfullscreen=allow_fullscreen.then_some("true")
            ></iframe>
        </div>
    }
}
