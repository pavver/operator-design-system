//! Individual media cell component for Gallery.

use super::observer::use_intersection_observer;
use super::types::MediaItem;
use crate::components::checkbox::AsyncCheckbox;
use crate::components::image::Image;
use crate::components::primitives::Icon;
use crate::core::{ActionResult, AsyncAction};
use leptos::prelude::*;
use operator_ui_tokens::dim;
use operator_ui_tokens::icons;

/// Single media tile inside a Gallery.
#[component]
pub fn GalleryItemView(
    item: MediaItem,
    index: usize,
    selectable: Signal<bool>,
    is_selected: Signal<bool>,
    on_toggle_select: Callback<String>,
    on_click: Option<Callback<usize>>,
) -> impl IntoView {
    let cell_ref = NodeRef::<leptos::html::Div>::new();
    let is_visible = use_intersection_observer(cell_ref);

    let id = item.id.clone();
    let id_for_select = id.clone();
    let display_src = item.thumbnail.clone().unwrap_or_else(|| item.src.clone());
    let title_or_empty = item.title.clone().unwrap_or_default();
    let caption_text = item.caption.clone().or_else(|| item.title.clone());

    let toggle_id = Callback::new(move |()| {
        on_toggle_select.run(id_for_select.clone());
    });

    let async_toggle = AsyncAction::new(move |_| {
        toggle_id.run(());
        async { ActionResult::Success }
    });

    view! {
        <div
            node_ref=cell_ref
            class="op-gallery-item"
            data-selected=move || is_selected.get().to_string()
            on:click=move |_| {
                if selectable.get() {
                    on_toggle_select.run(id.clone());
                } else if let Some(cb) = on_click {
                    cb.run(index);
                }
            }
        >
            {move || {
                if is_visible.get() {
                    view! {
                        <Image
                            src=Signal::stored(display_src.clone())
                            alt=Signal::stored(title_or_empty.clone())
                            ratio=item.aspect_ratio.unwrap_or(1.0)
                        />
                    }.into_any()
                } else {
                    view! {
                        <div style="width:100%;height:100%;min-height:calc(var(--op-du)*20);background:var(--op-surface-subtle);" />
                    }.into_any()
                }
            }}

            // Play badge for videos
            {item.is_video.then(|| view! {
                <div class="op-gallery-play-badge">
                    <Icon icon=icons::VIDEOCAM size=dim::ICON_LARGE />
                </div>
            })}

            // Caption overlay
            {caption_text.map(|c| view! {
                <div class="op-gallery-caption-bar">{c}</div>
            })}

            // Selection checkbox
            {move || selectable.get().then(|| {
                view! {
                    <div
                        class="op-gallery-selection-checkbox"
                        on:click=move |ev| {
                            ev.stop_propagation();
                        }
                    >
                        <AsyncCheckbox
                            checked=is_selected
                            on_change=async_toggle.clone()
                        />
                    </div>
                }
            })}
        </div>
    }
}
