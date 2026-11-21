//! Full-screen photo and video viewer: GalleryViewer.

mod thumbnails;
mod types;
mod zoom;

pub use types::*;
pub use zoom::ViewerZoomState;

use crate::components::icon_button::IconButton;
use crate::core::{Metric, Slot, root_class, root_style, ui_text, use_page_scroll_lock};
use leptos::prelude::*;
use operator_ui_tokens::icons;
use thumbnails::GalleryThumbnailCarousel;

pub(crate) const CSS: &str = include_str!("gallery_viewer.css");

/// Full-screen media viewer dialog with gestures, zoom, pan, thumbnail navigation,
/// video playback, and download/share actions.
///
/// Ports the comprehensive behavior of the Kotlin `OperatorGalleryViewerDialog`:
/// - Mouse wheel zoom on desktop
/// - Double-tap / double-click to zoom to 3.0x or reset
/// - Drag to pan when zoomed
/// - Arrow keys and edge clicks to switch items
/// - Expandable bottom description
/// - Close on Escape and backdrop click
/// - Focus trap and page scroll lock
#[component]
pub fn GalleryViewer(
    /// Controls viewer dialog visibility.
    open: RwSignal<bool>,
    /// Collection of media items to view.
    #[prop(into)]
    items: Signal<Vec<GalleryViewerItem>>,
    /// Active item index.
    #[prop(optional, into)]
    current_index: Option<RwSignal<usize>>,
    /// Translatable dialog accessibility label.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Translatable close button label.
    #[prop(optional, into)]
    close_label: MaybeProp<String>,
    /// Translatable previous item button label.
    #[prop(optional, into)]
    prev_label: MaybeProp<String>,
    /// Translatable next item button label.
    #[prop(optional, into)]
    next_label: MaybeProp<String>,
    /// Translatable empty media message.
    #[prop(optional, into)]
    empty_label: MaybeProp<String>,
    /// Slot for top-right download button or custom download action.
    #[prop(optional, into)]
    download_slot: Slot,
    /// Slot for top-right share button or custom share action.
    #[prop(optional, into)]
    share_slot: Slot,
    /// Slot for extra header action buttons.
    #[prop(optional, into)]
    actions_slot: Slot,
    /// Thumbnail size in DU (`--op-gallery-viewer-thumb-size`, default 12 DU).
    #[prop(optional, into)]
    thumbnail_size: MaybeProp<f32>,
    /// Callback when user triggers item change.
    #[prop(optional)]
    on_change: Option<Callback<usize>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_label = ui_text(label, "gallery-viewer.label", "Media gallery viewer");
    let resolved_close = ui_text(close_label, "gallery-viewer.close", "Close");
    let resolved_prev = ui_text(prev_label, "gallery-viewer.prev", "Previous item");
    let resolved_next = ui_text(next_label, "gallery-viewer.next", "Next item");
    let resolved_empty = ui_text(empty_label, "gallery-viewer.empty", "Media not found");

    let current = current_index.unwrap_or_else(|| RwSignal::new(0));
    let zoom = ViewerZoomState::new();
    let is_ui_visible = RwSignal::new(true);
    let is_description_expanded = RwSignal::new(false);
    let is_pointer_down = RwSignal::new(false);
    let last_pointer_x = RwSignal::new(0.0f32);
    let last_pointer_y = RwSignal::new(0.0f32);
    let last_tap_time = RwSignal::new(0.0f64);

    let panel_ref = NodeRef::<leptos::html::Div>::new();

    use_page_scroll_lock(open);

    // Reset zoom and description on page change
    Effect::new(move |_| {
        let idx = current.get();
        zoom.reset();
        is_description_expanded.set(false);
        if let Some(cb) = on_change {
            cb.run(idx);
        }
    });

    // Auto-focus panel when opened
    Effect::new(move |_| {
        if open.get() {
            zoom.reset();
            is_ui_visible.set(true);
            request_animation_frame(move || {
                if let Some(panel) = panel_ref.get() {
                    let _ = panel.focus();
                }
            });
        }
    });

    let root_metrics = vec![Metric::du("--op-gallery-viewer-thumb-size", thumbnail_size)];
    let root_style = Signal::derive(root_style(root_metrics, style));

    let total_items = Signal::derive(move || items.with(Vec::len));

    let prev_item = Callback::new(move |()| {
        if current.get_untracked() > 0 {
            current.update(|c| *c -= 1);
        }
    });

    let next_item = Callback::new(move |()| {
        if current.get_untracked() + 1 < total_items.get_untracked() {
            current.update(|c| *c += 1);
        }
    });

    let close_viewer = Callback::new(move |()| {
        open.set(false);
    });

    let download_stored = StoredValue::new(download_slot);
    let share_stored = StoredValue::new(share_slot);
    let actions_stored = StoredValue::new(actions_slot);

    view! {
        <Show when=move || open.get()>
            <div
                node_ref=panel_ref
                class=root_class("op-gallery-viewer", class)
                style=move || root_style.get()
                tabindex="-1"
                role="dialog"
                aria-modal="true"
                aria-label=move || resolved_label.get()
                on:keydown=move |ev| {
                    match ev.key().as_str() {
                        "Escape" => {
                            ev.prevent_default();
                            close_viewer.run(());
                        }
                        "ArrowLeft" if !zoom.is_zoomed() => {
                            ev.prevent_default();
                            prev_item.run(());
                        }
                        "ArrowRight" if !zoom.is_zoomed() => {
                            ev.prevent_default();
                            next_item.run(());
                        }
                        _ => {}
                    }
                }
            >
                // Floating Chrome (Top and Bottom bars)
                <div
                    class="op-gallery-viewer-chrome"
                    data-visible=move || is_ui_visible.get().to_string()
                >
                    // Top Bar
                    <div class="op-gallery-viewer-top-bar">
                        <div class="op-gallery-viewer-counter">
                            {move || format!("{} / {}", current.get() + 1, total_items.get())}
                        </div>
                        <div class="op-gallery-viewer-top-actions">
                            {download_stored.with_value(|s| s.is_set().then(|| s.run()))}
                            {share_stored.with_value(|s| s.is_set().then(|| s.run()))}
                            {actions_stored.with_value(|s| s.is_set().then(|| s.run()))}
                            <IconButton
                                icon=icons::CLOSE
                                label=resolved_close
                                on_click=Callback::new(move |()| close_viewer.run(()))
                            />
                        </div>
                    </div>

                    // Bottom Bar
                    <div class="op-gallery-viewer-bottom-bar">
                        {move || {
                            let curr_idx = current.get();
                            let media = items.with(|list| list.get(curr_idx).cloned());
                            media.and_then(|item| {
                                let title = item.title.clone();
                                let caption = item.caption.clone();
                                (title.is_some() || caption.is_some()).then(|| {
                                    view! {
                                        <div
                                            class="op-gallery-viewer-description"
                                            on:click=move |_| is_description_expanded.update(|e| *e = !*e)
                                        >
                                            {title.map(|t| view! { <div class="op-gallery-viewer-title">{t}</div> })}
                                            {caption.map(|c| view! { <div class="op-gallery-viewer-caption">{c}</div> })}
                                        </div>
                                    }
                                })
                            })
                        }}

                        // Thumbnail carousel
                        {move || (total_items.get() > 1).then(|| view! {
                            <GalleryThumbnailCarousel
                                items=items
                                current=current
                            />
                        })}
                    </div>
                </div>

                // Media Viewport
                <div
                    class="op-gallery-viewer-viewport"
                    on:wheel=move |ev| {
                        ev.prevent_default();
                        let factor = if ev.delta_y() < 0.0 { 1.15 } else { 0.87 };
                        zoom.zoom_by(factor);
                    }
                    on:pointerdown=move |ev| {
                        is_pointer_down.set(true);
                        last_pointer_x.set(ev.client_x() as f32);
                        last_pointer_y.set(ev.client_y() as f32);

                        // Double tap detection
                        let now = js_sys::Date::now();
                        let diff = now - last_tap_time.get_untracked();
                        if diff < 300.0 {
                            zoom.toggle_double_tap();
                            last_tap_time.set(0.0);
                        } else {
                            last_tap_time.set(now);
                        }
                    }
                    on:pointermove=move |ev| {
                        if is_pointer_down.get() && zoom.is_zoomed() {
                            let cx = ev.client_x() as f32;
                            let cy = ev.client_y() as f32;
                            let dx = cx - last_pointer_x.get_untracked();
                            let dy = cy - last_pointer_y.get_untracked();
                            zoom.pan_by(dx, dy);
                            last_pointer_x.set(cx);
                            last_pointer_y.set(cy);
                        }
                    }
                    on:pointerup=move |ev| {
                        is_pointer_down.set(false);
                        // Edge click navigation when not zoomed
                        if !zoom.is_zoomed() {
                            let window_width = web_sys::window()
                                .and_then(|w| w.inner_width().ok())
                                .and_then(|v| v.as_f64())
                                .unwrap_or(1000.0) as f32;
                            let x = ev.client_x() as f32;
                            if x < window_width * 0.2 {
                                prev_item.run(());
                            } else if x > window_width * 0.8 {
                                next_item.run(());
                            } else {
                                is_ui_visible.update(|v| *v = !*v);
                            }
                        }
                    }
                >
                    {move || {
                        let curr_idx = current.get();
                        let media = items.with(|list| list.get(curr_idx).cloned());
                        if let Some(item) = media {
                            if item.is_video {
                                view! {
                                    <video
                                        class="op-gallery-viewer-video"
                                        src=item.src
                                        poster=item.thumbnail
                                        controls=true
                                        autoplay=true
                                        playsinline=true
                                    ></video>
                                }.into_any()
                            } else {
                                view! {
                                    <div
                                        class="op-gallery-viewer-media-container"
                                        style=move || zoom.transform_style()
                                    >
                                        <img
                                            class="op-gallery-viewer-image"
                                            src=item.src
                                            alt=item.title.unwrap_or_default()
                                        />
                                    </div>
                                }.into_any()
                            }
                        } else {
                            view! {
                                <div style="color:white;">{move || resolved_empty.get()}</div>
                            }.into_any()
                        }
                    }}
                </div>

                // Left / Right edge buttons
                {move || (total_items.get() > 1 && current.get() > 0 && is_ui_visible.get()).then(|| view! {
                    <div class="op-gallery-viewer-nav-btn op-gallery-viewer-nav-prev">
                        <IconButton
                            icon=icons::CHEVRON_LEFT
                            label=resolved_prev
                            on_click=Callback::new(move |()| prev_item.run(()))
                        />
                    </div>
                })}

                {move || (total_items.get() > 1 && current.get() + 1 < total_items.get() && is_ui_visible.get()).then(|| view! {
                    <div class="op-gallery-viewer-nav-btn op-gallery-viewer-nav-next">
                        <IconButton
                            icon=icons::CHEVRON_RIGHT
                            label=resolved_next
                            on_click=Callback::new(move |()| next_item.run(()))
                        />
                    </div>
                })}
            </div>
        </Show>
    }
}
