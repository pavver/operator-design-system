//! Live examples for Gallery.

use super::demo_video;
use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;
use std::collections::HashSet;

fn demo_gallery_items() -> Vec<MediaItem> {
    vec![
        MediaItem::image(
            "item-1",
            "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='600' height='400' viewBox='0 0 600 400'><rect width='600' height='400' fill='%231e3a8a'/><circle cx='300' cy='180' r='80' fill='%233b82f6'/><text x='300' y='320' font-family='sans-serif' font-size='24' fill='%23ffffff' text-anchor='middle'>Telemetry Cam 01</text></svg>",
        )
        .with_title("Telemetry Cam 01")
        .with_caption("North quadrant optical sweep"),

        MediaItem::image(
            "item-2",
            "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='600' height='400' viewBox='0 0 600 400'><rect width='600' height='400' fill='%23064e3b'/><rect x='200' y='100' width='200' height='160' fill='%2310b981'/><text x='300' y='320' font-family='sans-serif' font-size='24' fill='%23ffffff' text-anchor='middle'>Sector B Patrol</text></svg>",
        )
        .with_title("Sector B Patrol")
        .with_caption("Infrared border surveillance"),

        MediaItem::image(
            "item-3",
            "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='600' height='400' viewBox='0 0 600 400'><rect width='600' height='400' fill='%2378350f'/><polygon points='300,80 420,240 180,240' fill='%23f59e0b'/><text x='300' y='320' font-family='sans-serif' font-size='24' fill='%23ffffff' text-anchor='middle'>Thermal Sensor 04</text></svg>",
        )
        .with_title("Thermal Sensor 04")
        .with_caption("Thermal anomalies logged"),

        MediaItem::video(
            "item-4",
            "blob:offline-drone-video",
            Some("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='600' height='400' viewBox='0 0 600 400'><rect width='600' height='400' fill='%23311042'/><polygon points='260,150 360,200 260,250' fill='%23c084fc'/><text x='300' y='320' font-family='sans-serif' font-size='24' fill='%23ffffff' text-anchor='middle'>Live Drone Feed</text></svg>".to_string()),
        )
        .with_title("Recon Flight 12")
        .with_caption("Automated drone patrol sequence recorded offline"),
    ]
}

pub fn gallery() -> AnyView {
    let items = RwSignal::new(demo_gallery_items());
    let selectable = RwSignal::new(false);
    let selected_ids = RwSignal::new(HashSet::new());
    let last_clicked = RwSignal::new(None::<usize>);

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

    let on_selection_change = Callback::new(move |set: HashSet<String>| {
        selected_ids.set(set);
    });

    let on_item_click = Callback::new(move |idx: usize| {
        last_clicked.set(Some(idx));
    });

    view! {
        <Example
            title="Responsive Media Gallery Grid"
            note="Clicking any tile opens the full-screen viewer at that item. Selection mode toggles selection instead."
            block=true
        >
            <div style="margin-bottom:calc(var(--op-du)*2);display:flex;align-items:center;gap:calc(var(--op-du)*3);">
                <AsyncCheckbox
                    checked=selectable
                    text=Signal::stored("Selection Mode".to_string())
                    on_change=AsyncAction::new(move |v: bool| async move {
                        selectable.set(v);
                        ActionResult::Success
                    })
                />
                <span style="font-size:calc(var(--op-tu)*12);color:var(--op-text-secondary);">
                    {move || format!("Selected: {} items", selected_ids.get().len())}
                </span>
                {move || last_clicked.get().map(|idx| view! {
                    <span style="font-size:calc(var(--op-tu)*12);color:var(--op-accent);">
                        {format!("Clicked item index: {idx}")}
                    </span>
                })}
            </div>

            <Gallery
                items=items
                selectable=selectable
                selected_ids=selected_ids
                on_selection_change=on_selection_change
                on_item_click=on_item_click
            />
        </Example>

        <Example
            title="Compact Attachment Layout"
            note="Edge-to-edge multi-image grid for chat message attachments."
            block=true
        >
            <div style="max-width:calc(var(--op-du)*70);">
                <Gallery
                    items=items
                    layout=GalleryLayout::Compact
                />
            </div>
        </Example>
    }
    .into_any()
}
