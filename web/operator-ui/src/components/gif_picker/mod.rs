//! Looping animated GIF / short video clip picker with IntersectionObserver viewport playback.

use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::icons;
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
    /// Trending search tags list.
    #[prop(optional)]
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
    let default_tags = vec![
        "Trending".to_string(),
        "Reaction".to_string(),
        "Cats".to_string(),
        "Yes".to_string(),
        "No".to_string(),
        "Party".to_string(),
        "Wait".to_string(),
    ];
    let tags_list = trending_tags.unwrap_or(default_tags);

    let selected_tag = RwSignal::new(String::from("Trending"));
    let search_query = RwSignal::new(String::new());

    let search_placeholder = ui_text(None::<String>.into(), "gif-picker.search", "Search GIFs...");
    let search_icon = ui_icon(None, "gif-picker.search-icon", icons::SEARCH);

    let metrics = vec![
        Metric::du("--op-gif-picker-height", height),
        Metric::du("--op-gif-picker-cell-height", cell_height),
    ];
    let root_cls = root_class("op-gif-picker", class);
    let root_stl = root_style(metrics, style);

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
            } else if tag != "Trending" {
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

            <div class="op-gif-picker__tags" role="toolbar" aria-label="Trending GIF tags">
                {tags_list.into_iter().map(|tag| {
                    let t_name = tag.clone();
                    let t_click = tag.clone();
                    let is_sel = move || selected_tag.get() == t_name;
                    view! {
                        <button
                            type="button"
                            class="op-gif-picker__tag-btn"
                            data-selected=move || is_sel().then_some("true")
                            on:click=move |_| {
                                search_query.set(String::new());
                                selected_tag.set(t_click.clone());
                            }
                        >
                            {tag}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="op-gif-picker__grid">
                {move || {
                    let items = filtered_gifs.get();
                    items.into_iter().map(|item| {
                        let it = item.clone();
                        view! {
                            <GifCell item=it on_select=on_select />
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

#[component]
fn GifCell(item: GifItem, on_select: Callback<GifItem>) -> impl IntoView {
    let video_ref = NodeRef::<leptos::html::Video>::new();
    let it_click = item.clone();

    // IntersectionObserver to pause video when scrolled out of view and resume when visible
    Effect::new(move |_| {
        let Some(video_el) = video_ref.get() else {
            return;
        };
        let raw_video = video_el.unchecked_ref::<HtmlVideoElement>().clone();

        let closure = Closure::wrap(Box::new(
            move |entries: js_sys::Array, _obs: IntersectionObserver| {
                for entry_val in entries.iter() {
                    if let Ok(entry) = entry_val.dyn_into::<IntersectionObserverEntry>() {
                        if entry.is_intersecting() {
                            let _ = raw_video.play();
                        } else {
                            let _ = raw_video.pause();
                        }
                    }
                }
            },
        )
            as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

        if let Ok(observer) = IntersectionObserver::new(closure.as_ref().unchecked_ref()) {
            observer.observe(video_el.unchecked_ref());
            closure.forget();
            on_cleanup(move || {
                observer.disconnect();
            });
        }
    });

    let it_title = item.title.clone();

    view! {
        <div
            class="op-gif-picker__cell"
            on:click=move |_| on_select.run(it_click.clone())
            role="button"
            tabindex="0"
            aria-label=it_title.clone()
        >
            <video
                node_ref=video_ref
                class="op-gif-picker__video"
                src=item.media_url
                poster=item.preview_url
                autoplay
                muted
                loop
                playsinline
            />
            <div class="op-gif-picker__title-bar">{it_title.clone()}</div>
        </div>
    }
}
