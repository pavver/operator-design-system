//! Live examples for Gallery.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;
use std::collections::HashSet;

fn demo_gallery_items() -> Vec<GalleryMediaItem> {
    vec![
        GalleryMediaItem::image(
            "item-1",
            "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='600' height='400' viewBox='0 0 600 400'><rect width='600' height='400' fill='%231e3a8a'/><circle cx='300' cy='180' r='80' fill='%233b82f6'/><text x='300' y='320' font-family='sans-serif' font-size='24' fill='%23ffffff' text-anchor='middle'>Telemetry Cam 01</text></svg>",
        )
        .with_title("Telemetry Cam 01")
        .with_caption("North quadrant optical sweep"),

        GalleryMediaItem::image(
            "item-2",
            "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='600' height='400' viewBox='0 0 600 400'><rect width='600' height='400' fill='%23064e3b'/><rect x='200' y='100' width='200' height='160' fill='%2310b981'/><text x='300' y='320' font-family='sans-serif' font-size='24' fill='%23ffffff' text-anchor='middle'>Sector B Patrol</text></svg>",
        )
        .with_title("Sector B Patrol")
        .with_caption("Infrared border surveillance"),

        GalleryMediaItem::image(
            "item-3",
            "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='600' height='400' viewBox='0 0 600 400'><rect width='600' height='400' fill='%2378350f'/><polygon points='300,80 420,240 180,240' fill='%23f59e0b'/><text x='300' y='320' font-family='sans-serif' font-size='24' fill='%23ffffff' text-anchor='middle'>Thermal Sensor 04</text></svg>",
        )
        .with_title("Thermal Sensor 04")
        .with_caption("Thermal anomalies logged"),

        GalleryMediaItem::video(
            "item-4",
            "data:video/mp4;base64,",
            Some("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='600' height='400' viewBox='0 0 600 400'><rect width='600' height='400' fill='%23311042'/><polygon points='260,150 360,200 260,250' fill='%23c084fc'/><text x='300' y='320' font-family='sans-serif' font-size='24' fill='%23ffffff' text-anchor='middle'>Live Drone Feed</text></svg>".to_string()),
        )
        .with_title("Recon Flight 12")
        .with_caption("Automated drone patrol sequence"),
    ]
}

pub fn gallery() -> AnyView {
    let items = Signal::stored(demo_gallery_items());
    let selectable = RwSignal::new(false);
    let selected_ids = RwSignal::new(HashSet::new());
    let last_clicked = RwSignal::new(None::<usize>);

    let on_selection_change = Callback::new(move |set: HashSet<String>| {
        selected_ids.set(set);
    });

    let on_item_click = Callback::new(move |idx: usize| {
        last_clicked.set(Some(idx));
    });

    view! {
        <Example
            title="Responsive Media Gallery Grid"
            note="Photos and videos with play indicators, captions, and viewport-lazy loading."
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
            title="Telegram-Style Compact Layout"
            note="Edge-to-edge multi-image grid ported from chat message attachments."
            block=true
        >
            <div style="max-width:calc(var(--op-du)*70);">
                <Gallery
                    items=items
                    layout=GalleryLayout::Telegram
                />
            </div>
        </Example>
    }
    .into_any()
}
