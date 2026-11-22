//! Thumbnail carousel strip rendered at the bottom of the GalleryViewer.

use super::types::MediaItem;
use crate::core::ui_text;
use leptos::prelude::*;

/// Thumbnail strip for quick navigation across gallery items.
#[component]
pub fn GalleryThumbnailCarousel(
    items: Signal<Vec<MediaItem>>,
    current: RwSignal<usize>,
) -> impl IntoView {
    let resolved_label = ui_text(
        None::<String>.into(),
        "gallery-viewer.thumbnails",
        "Media thumbnails",
    );

    view! {
        <div class="op-gallery-viewer-carousel" role="tablist" aria-label=move || resolved_label.get()>
            {move || {
                let media_items = items.get();
                let active_idx = current.get();
                media_items.into_iter().enumerate().map(|(idx, item)| {
                    let is_active = idx == active_idx;
                    let display_src = item.thumbnail.unwrap_or(item.src);
                    view! {
                        <button
                            type="button"
                            class="op-gallery-viewer-thumb"
                            data-active=is_active.to_string()
                            role="tab"
                            aria-selected=is_active.to_string()
                            aria-label=format!("Jump to item {}", idx + 1)
                            on:click=move |_| current.set(idx)
                        >
                            <img src=display_src alt="" loading="lazy" />
                        </button>
                    }
                }).collect_view()
            }}
        </div>
    }
}
