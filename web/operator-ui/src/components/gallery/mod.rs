//! Responsive media grid of photos and videos: Gallery.

mod item;
mod observer;
mod types;

pub use types::*;

use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use item::GalleryItemView;
use leptos::prelude::*;
use std::collections::HashSet;

pub(crate) const CSS: &str = include_str!("gallery.css");

/// Responsive media grid of photos and videos with selection, captions,
/// and viewport-lazy loading via `IntersectionObserver`.
#[component]
pub fn Gallery(
    /// Collection of media items to display.
    #[prop(into)]
    items: Signal<Vec<GalleryMediaItem>>,
    /// Layout arrangement mode: `Grid` (default) or `Telegram`.
    #[prop(default = GalleryLayout::Grid)]
    layout: GalleryLayout,
    /// Minimum column width in DU for the responsive grid (`--op-gallery-column-min-width`, default 30 DU).
    #[prop(optional, into)]
    column_min_width: MaybeProp<f32>,
    /// Gap between media cells in DU (`--op-gallery-gap`, default 0.5 DU).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Enables multi-item selection mode with checkboxes.
    #[prop(default = false.into(), into)]
    selectable: Signal<bool>,
    /// Currently selected item IDs.
    #[prop(optional, into)]
    selected_ids: Signal<HashSet<String>>,
    /// Callback invoked when the selection changes.
    #[prop(optional)]
    on_selection_change: Option<Callback<HashSet<String>>>,
    /// Callback invoked when an item is clicked (passes item index).
    #[prop(optional)]
    on_item_click: Option<Callback<usize>>,
    /// Custom slot rendered when `items` is empty.
    #[prop(optional, into)]
    empty_slot: Slot,
    /// Translatable label rendered when `items` is empty.
    #[prop(optional, into)]
    empty_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_empty = ui_text(empty_label, "gallery.empty", "No media items");

    let root_metrics = vec![
        Metric::du("--op-gallery-column-min-width", column_min_width),
        Metric::du("--op-gallery-gap", gap),
    ];
    let full_style = root_style(root_metrics, style);

    let toggle_select = Callback::new(move |id: String| {
        let mut set = selected_ids.get_untracked();
        if set.contains(&id) {
            set.remove(&id);
        } else {
            set.insert(id);
        }
        if let Some(cb) = on_selection_change {
            cb.run(set);
        }
    });

    let layout_class = move || match layout {
        GalleryLayout::Grid => "op-gallery-grid",
        GalleryLayout::Telegram => {
            let count = items.with(Vec::len);
            if count <= 1 {
                "op-gallery-telegram-single"
            } else {
                "op-gallery-telegram-multi"
            }
        }
    };

    view! {
        <div
            class=root_class("op-gallery", class)
            style=full_style
        >
            {move || {
                let media_list = items.get();
                if media_list.is_empty() {
                    if empty_slot.is_set() {
                        empty_slot.run().into_any()
                    } else {
                        view! {
                            <div style="padding:calc(var(--op-du)*4);text-align:center;color:var(--op-text-secondary);">
                                {move || resolved_empty.get()}
                            </div>
                        }.into_any()
                    }
                } else {
                    view! {
                        <div class=layout_class>
                            {media_list.into_iter().enumerate().map(|(idx, it)| {
                                let it_id = it.id.clone();
                                let is_sel = Signal::derive(move || selected_ids.get().contains(&it_id));
                                view! {
                                    <GalleryItemView
                                        item=it
                                        index=idx
                                        selectable=selectable
                                        is_selected=is_sel
                                        on_toggle_select=toggle_select
                                        on_click=on_item_click
                                    />
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
