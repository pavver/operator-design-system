//! Looping animated GIF / short video clip picker with IntersectionObserver viewport playback.

use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::icons;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlVideoElement, IntersectionObserver, IntersectionObserverEntry};

pub(crate) const CSS: &str = include_str!("gif_picker.css");

/// Model representing a short animated GIF/clip (MP4/WebM video).
#[derive(Debug, Clone, PartialEq)]
pub struct GifItem {
    pub id: String,
    pub title: String,
    pub preview_url: String,
    pub media_url: String,
    pub tags: Vec<String>,
    pub aspect_ratio: f32,
}

impl GifItem {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        preview_url: impl Into<String>,
        media_url: impl Into<String>,
        tags: Vec<String>,
        aspect_ratio: f32,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            preview_url: preview_url.into(),
            media_url: media_url.into(),
            tags,
            aspect_ratio,
        }
    }
}

/// Pluggable GIF clip picker.
///
/// GIFs are streamed as looping MP4/WebM videos (`<video autoplay muted loop playsinline>`).
/// To optimize resource usage and CPU, videos are automatically paused when scrolled out of view
/// using a browser [`IntersectionObserver`].
#[component]
pub fn GifPicker(
    /// Callback when a GIF item is selected.
    on_select: Callback<GifItem>,
    /// Available GIF items passed by the application.
    #[prop(optional, into)]
    gifs: Option<Vec<GifItem>>,
    /// Trending search tags list provided by the application.
    #[prop(optional, into)]
    trending_tags: Option<Vec<String>>,
    /// Height in DU (`--op-gif-picker-height`). Defaults to 60 DU.
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Cell height in DU (`--op-gif-picker-cell-height`). Defaults to 30 DU.
    #[prop(optional, into)]
    cell_height: MaybeProp<f32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline CSS styles.
    #[prop(optional, into)]
    style: MaybeProp<String>,
) -> impl IntoView {
    let all_gifs = gifs.unwrap_or_default();
    let tags_list = trending_tags.unwrap_or_default();

    let selected_tag = RwSignal::new(String::new());
    let search_query = RwSignal::new(String::new());

    let search_placeholder = ui_text(None::<String>.into(), "gif-picker.search", "Search GIFs...");
    let tags_aria = ui_text(
        None::<String>.into(),
        "gif-picker.tags",
        "Trending GIF tags",
    );
    let empty_text = ui_text(None::<String>.into(), "gif-picker.empty", "No GIFs found");
    let search_icon = ui_icon(None, "gif-picker.search-icon", icons::SEARCH);

    let metrics = vec![
        Metric::du("--op-gif-picker-height", height),
        Metric::du("--op-gif-picker-cell-height", cell_height),
    ];
    let root_cls = root_class("op-gif-picker", class);
    let root_stl = root_style(metrics, style);

    let has_tags = !tags_list.is_empty();

    let filtered_gifs = Memo::new({
        let all_gifs = all_gifs.clone();
        move |_| {
            let q = search_query.get().trim().to_lowercase();
            let tag = selected_tag.get();
            if !q.is_empty() {
                all_gifs
                    .iter()
                    .filter(|item| {
                        item.title.to_lowercase().contains(&q)
                            || item.tags.iter().any(|t| t.to_lowercase().contains(&q))
                    })
                    .cloned()
                    .collect()
            } else if !tag.is_empty() {
                all_gifs
                    .iter()
                    .filter(|item| item.tags.iter().any(|t| t.eq_ignore_ascii_case(&tag)))
                    .cloned()
                    .collect()
            } else {
                all_gifs.clone()
            }
        }
    });

    view! {
        <div class=root_cls style=root_stl>
            <div class="op-gif-picker__search">
                <span class="op-gif-picker__search-icon">
                    <Icon icon=search_icon.get() />
                </span>
                <input
                    type="text"
                    class="op-gif-picker__search-input"
                    placeholder=move || search_placeholder.get()
                    prop:value=move || search_query.get()
                    on:input=move |ev: ev::Event| {
                        search_query.set(event_target_value(&ev));
                    }
                />
            </div>

            {has_tags.then(|| {
                let tags = tags_list.clone();
                let aria_lbl = tags_aria.get();
                view! {
                    <div class="op-gif-picker__tags" role="toolbar" aria-label=aria_lbl>
                        {tags.into_iter().map(|tag| {
                            let t_name = tag.clone();
                            let t_click = tag.clone();
                            let is_sel = move || selected_tag.get() == t_name;
                            view! {
                                <button
                                    type="button"
                                    class="op-gif-picker__tag-btn"
                                    data-selected=move || is_sel().then_some("true")
                                    on:click=move |_| {
                                        if selected_tag.get() == t_click {
                                            selected_tag.set(String::new());
                                        } else {
                                            selected_tag.set(t_click.clone());
                                        }
                                    }
                                >
                                    {tag}
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }
            })}

            <div class="op-gif-picker__grid">
                {move || {
                    let items = filtered_gifs.get();
                    if items.is_empty() {
                        view! {
                            <div class="op-gif-picker__empty">
                                {empty_text.get()}
                            </div>
                        }.into_any()
                    } else {
                        items.into_iter().map(|item| {
                            render_gif_cell(item, on_select)
                        }).collect::<Vec<_>>().into_any()
                    }
                }}
            </div>
        </div>
    }
}

fn render_gif_cell(item: GifItem, on_select: Callback<GifItem>) -> impl IntoView {
    let video_ref = NodeRef::<leptos::html::Video>::new();
    let item_click = item.clone();
    let aria_title = item.title.clone();
    let display_title = item.title.clone();
    let media_url = item.media_url.clone();
    let preview_url = item.preview_url.clone();

    // IntersectionObserver to pause video playback when scrolled outside the viewport
    Effect::new(move |_| {
        if let Some(video_el) = video_ref.get() {
            let raw_video: HtmlVideoElement = (*video_el).clone().unchecked_into();
            let video_clone = raw_video.clone();

            let callback = Closure::<dyn FnMut(js_sys::Array, IntersectionObserver)>::new(
                move |entries: js_sys::Array, _obs: IntersectionObserver| {
                    for i in 0..entries.length() {
                        let entry: IntersectionObserverEntry = entries.get(i).unchecked_into();
                        if entry.is_intersecting() {
                            // A clip that cannot play rejects the promise; the poster stays visible.
                            if let Ok(promise) = video_clone.play() {
                                leptos::task::spawn_local(async move {
                                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                });
                            }
                        } else {
                            let _ = video_clone.pause();
                        }
                    }
                },
            );

            if let Ok(observer) = IntersectionObserver::new(callback.as_ref().unchecked_ref()) {
                observer.observe(&raw_video);
                let keep = SendWrapper::new((observer, callback));
                on_cleanup(move || {
                    keep.0.disconnect();
                });
            }
        }
    });

    view! {
        <div
            class="op-gif-picker__cell"
            on:click=move |_| on_select.run(item_click.clone())
            role="button"
            tabindex="0"
            aria-label=aria_title
        >
            <video
                node_ref=video_ref
                class="op-gif-picker__video"
                src=media_url
                poster=preview_url
                autoplay=true
                muted=true
                loop=true
                playsinline=true
            />
            <div class="op-gif-picker__title-bar">
                {display_title}
            </div>
        </div>
    }
}
