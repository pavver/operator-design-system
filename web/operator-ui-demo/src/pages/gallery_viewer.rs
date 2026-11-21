//! Live examples for GalleryViewer.

use super::demo_video;
use crate::kit::{Example, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

fn demo_viewer_items() -> Vec<MediaItem> {
    vec![
        MediaItem::image(
            "v-1",
            "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='800' height='600' viewBox='0 0 800 600'><rect width='800' height='600' fill='%231e3a8a'/><circle cx='400' cy='280' r='100' fill='%233b82f6'/><text x='400' y='460' font-family='sans-serif' font-size='28' fill='%23ffffff' text-anchor='middle'>Telemetry Cam 01</text></svg>",
        )
        .with_title("Telemetry Optical Capture")
        .with_caption("High resolution sweep of sector A-01 recorded offline."),

        MediaItem::image(
            "v-2",
            "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='800' height='600' viewBox='0 0 800 600'><rect width='800' height='600' fill='%23064e3b'/><rect x='280' y='180' width='240' height='180' fill='%2310b981'/><text x='400' y='460' font-family='sans-serif' font-size='28' fill='%23ffffff' text-anchor='middle'>Perimeter Sector B</text></svg>",
        )
        .with_title("Perimeter Guard Post")
        .with_caption("Infrared boundary line surveillance log."),

        MediaItem::video(
            "v-3",
            "blob:offline-recon-video",
            Some("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='800' height='600' viewBox='0 0 800 600'><rect width='800' height='600' fill='%23311042'/><polygon points='360,220 480,300 360,380' fill='%23c084fc'/><text x='400' y='460' font-family='sans-serif' font-size='28' fill='%23ffffff' text-anchor='middle'>Recon Drone Video</text></svg>".to_string()),
        )
        .with_title("Autonomous Drone Flight")
        .with_caption("Video clip recorded offline in-browser from canvas."),
    ]
}

pub fn gallery_viewer() -> AnyView {
    let open = RwSignal::new(false);
    let items = RwSignal::new(demo_viewer_items());
    let current_index = RwSignal::new(0);

    // Record an offline video clip from canvas and assign its blob URL
    demo_video::record_canvas_clip(move |blob_url: String| {
        items.update(|list| {
            for item in list.iter_mut() {
                if item.is_video {
                    item.src = blob_url.clone();
                }
            }
        });
    });

    view! {
        <Example
            title="Full-Screen Gallery Viewer"
            note="Complete viewer dialog with multi-touch zoom, double-tap, pan, carousel navigation, and Escape dismissal."
        >
            <ActionButton
                text="Open Gallery Viewer"
                icon=icons::VISIBILITY
                on_click=instant(move || open.set(true))
            />

            <GalleryViewer
                open=open
                items=items
                current_index=current_index
            />
        </Example>
    }
    .into_any()
}
