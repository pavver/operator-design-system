//! Pluggable emoji picker supporting app-provided categories, live search filtering, and custom emoji packs.

use crate::components::chat_message::CustomEmojiMap;
use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("emoji_picker.css");

/// Individual emoji item within a category or search result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiItem {
    /// Identifier or code, e.g. ":smile:" or "😀".
    pub id: String,
    /// Unicode text glyph or fallback text representation (e.g. "😀").
    pub glyph: String,
    /// Searchable title or keywords.
    pub name: String,
    /// Optional custom image URL if this is an image-backed custom emoji.
    pub image_url: Option<String>,
}

impl EmojiItem {
    pub fn new(glyph: impl Into<String>) -> Self {
        let g = glyph.into();
        Self {
            id: g.clone(),
            glyph: g.clone(),
            name: g,
            image_url: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn custom(
        id: impl Into<String>,
        name: impl Into<String>,
        glyph: impl Into<String>,
        image_url: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            glyph: glyph.into(),
            name: name.into(),
            image_url: Some(image_url.into()),
        }
    }
}

impl From<&str> for EmojiItem {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for EmojiItem {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// Pluggable emoji category / pack containing a title, a tab icon, and a list of emoji items.
#[derive(Debug, Clone, PartialEq)]
pub struct EmojiCategory {
    pub id: String,
    /// App-provided category title (e.g. "Smileys & Emotion").
    pub title: String,
    /// Tab icon glyph (e.g. "😀") or image URL.
    pub icon: String,
    /// List of emojis in this category.
    pub emojis: Vec<EmojiItem>,
}

impl EmojiCategory {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        icon: impl Into<String>,
        emojis: Vec<EmojiItem>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: icon.into(),
            emojis,
        }
    }
}

/// Pluggable emoji picker with category tabs, live search filtering,
/// and support for custom emoji map replacement.
#[component]
pub fn EmojiPicker(
    /// Callback invoked when an emoji or custom emoji code is selected.
    on_select: Callback<String>,
    /// Categories provided by the host application.
    #[prop(optional, into)]
    categories: Option<Vec<EmojiCategory>>,
    /// Recent emojis list provided by the host app.
    #[prop(optional, into)]
    recent_emojis: Option<Vec<EmojiItem>>,
    /// Pluggable registry of custom image-backed emojis.
    #[prop(optional, into)]
    custom_emojis: Option<CustomEmojiMap>,
    /// Height in DU (`--op-emoji-picker-height`). Defaults to 60 DU.
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Cell size in DU (`--op-emoji-picker-cell-size`). Defaults to 8.5 DU.
    #[prop(optional, into)]
    cell_size: MaybeProp<f32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline CSS styles.
    #[prop(optional, into)]
    style: MaybeProp<String>,
) -> impl IntoView {
    let cats = categories.unwrap_or_default();
    let recents = recent_emojis.unwrap_or_default();
    let has_recents = !recents.is_empty();

    let selected_tab_idx = RwSignal::new(0_usize);
    let search_query = RwSignal::new(String::new());

    let search_placeholder = ui_text(
        None::<String>.into(),
        "emoji-picker.search",
        "Search emoji...",
    );
    let search_prefix = ui_text(
        None::<String>.into(),
        "emoji-picker.search-prefix",
        "Search",
    );
    let recent_title = ui_text(None::<String>.into(), "emoji-picker.recent", "Recent");
    let categories_aria = ui_text(
        None::<String>.into(),
        "emoji-picker.categories",
        "Emoji categories",
    );
    let empty_text = ui_text(
        None::<String>.into(),
        "emoji-picker.empty",
        "No emoji found",
    );

    let search_icon = ui_icon(None, "emoji-picker.search-icon", icons::SEARCH);
    let recent_icon = ui_icon(None, "emoji-picker.recent-icon", icons::REFRESH);

    let metrics = vec![
        Metric::du("--op-emoji-picker-height", height),
        Metric::du("--op-emoji-picker-cell-size", cell_size),
    ];
    let root_cls = root_class("op-emoji-picker", class);
    let root_stl = root_style(metrics, style);

    let custom_map = custom_emojis.unwrap_or_default();
    let custom_map_filter = custom_map.clone();

    let displayed_emojis = Memo::new({
        let cats = cats.clone();
        let recents = recents.clone();
        let custom_map_filter = custom_map_filter.clone();
        move |_| {
            let q = search_query.get().trim().to_lowercase();
            if !q.is_empty() {
                let mut matched = Vec::new();
                for item in &recents {
                    if item.name.to_lowercase().contains(&q)
                        || item.glyph.contains(&q)
                        || item.id.to_lowercase().contains(&q)
                    {
                        matched.push(item.clone());
                    }
                }
                for cat in &cats {
                    for item in &cat.emojis {
                        if item.name.to_lowercase().contains(&q)
                            || item.glyph.contains(&q)
                            || item.id.to_lowercase().contains(&q)
                            || cat.title.to_lowercase().contains(&q)
                        {
                            matched.push(item.clone());
                        }
                    }
                }
                for (key, url) in &custom_map_filter.entries {
                    if key.to_lowercase().contains(&q) {
                        matched.push(EmojiItem::custom(
                            key.clone(),
                            key.clone(),
                            key.clone(),
                            url.clone(),
                        ));
                    }
                }
                matched.dedup_by(|a, b| a.id == b.id);
                matched
            } else {
                let cur_idx = selected_tab_idx.get();
                if has_recents && cur_idx == 0 {
                    recents.clone()
                } else {
                    let cat_offset = if has_recents {
                        cur_idx.saturating_sub(1)
                    } else {
                        cur_idx
                    };
                    if let Some(cat) = cats.get(cat_offset) {
                        cat.emojis.clone()
                    } else {
                        Vec::new()
                    }
                }
            }
        }
    });

    let choose_emoji = move |item: EmojiItem| {
        let code = if item.glyph.is_empty() {
            item.id
        } else {
            item.glyph
        };
        on_select.run(code);
    };

    view! {
        <div class=root_cls style=root_stl>
            <div class="op-emoji-picker__search">
                <span class="op-emoji-picker__search-icon">
                    <Icon icon=search_icon.get() />
                </span>
                <input
                    type="text"
                    class="op-emoji-picker__search-input"
                    placeholder=move || search_placeholder.get()
                    prop:value=move || search_query.get()
                    on:input=move |ev: ev::Event| {
                        search_query.set(event_target_value(&ev));
                    }
                />
            </div>

            <div class="op-emoji-picker__categories" role="tablist" aria-label=move || categories_aria.get()>
                {has_recents.then(|| {
                    let is_sel = move || selected_tab_idx.get() == 0;
                    let aria_lbl = recent_title.get();
                    let icon_sym = recent_icon.get();
                    view! {
                        <button
                            type="button"
                            class="op-emoji-picker__cat-btn"
                            data-selected=move || is_sel().then_some("true")
                            role="tab"
                            aria-selected=move || is_sel().to_string()
                            aria-label=aria_lbl
                            on:click=move |_| {
                                search_query.set(String::new());
                                selected_tab_idx.set(0);
                            }
                        >
                            <Icon icon=icon_sym />
                        </button>
                    }
                })}

                {cats.iter().enumerate().map(|(idx, cat)| {
                    let target_idx = if has_recents { idx + 1 } else { idx };
                    let is_sel = move || selected_tab_idx.get() == target_idx;
                    let icon_str = cat.icon.clone();
                    let is_url = icon_str.starts_with("http://")
                        || icon_str.starts_with("https://")
                        || icon_str.starts_with("data:")
                        || icon_str.starts_with('/');
                    let cat_title = cat.title.clone();
                    view! {
                        <button
                            type="button"
                            class="op-emoji-picker__cat-btn"
                            data-selected=move || is_sel().then_some("true")
                            role="tab"
                            aria-selected=move || is_sel().to_string()
                            aria-label=cat_title
                            on:click=move |_| {
                                search_query.set(String::new());
                                selected_tab_idx.set(target_idx);
                            }
                        >
                            {if is_url {
                                view! { <img class="op-emoji-picker__cat-icon-img" src=icon_str alt="" /> }.into_any()
                            } else {
                                view! { <span>{icon_str}</span> }.into_any()
                            }}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="op-emoji-picker__section-title">
                {
                    let cats_ref = cats.clone();
                    move || {
                        let q = search_query.get();
                        if !q.is_empty() {
                            format!("{}: \"{q}\"", search_prefix.get())
                        } else {
                            let cur_idx = selected_tab_idx.get();
                            if has_recents && cur_idx == 0 {
                                recent_title.get()
                            } else {
                                let offset = if has_recents { cur_idx.saturating_sub(1) } else { cur_idx };
                                if let Some(cat) = cats_ref.get(offset) {
                                    cat.title.clone()
                                } else {
                                    String::new()
                                }
                            }
                        }
                    }
                }
            </div>

            <div class="op-emoji-picker__grid">
                {move || {
                    let emojis = displayed_emojis.get();
                    if emojis.is_empty() {
                        view! {
                            <div class="op-emoji-picker__empty">
                                {empty_text.get()}
                            </div>
                        }.into_any()
                    } else {
                        let map_ref = &custom_map;
                        emojis.into_iter().map(|item| {
                            let item_for_click = item.clone();
                            let item_name = item.name.clone();
                            let custom_url = item.image_url.clone().or_else(|| {
                                map_ref.get(&item.id).or_else(|| map_ref.get(&item.glyph)).cloned()
                            });
                            view! {
                                <button
                                    type="button"
                                    class="op-emoji-picker__cell"
                                    on:click=move |_| choose_emoji(item_for_click.clone())
                                    aria-label=item_name.clone()
                                >
                                    {if let Some(url) = custom_url {
                                        view! {
                                            <img class="op-emoji-picker__custom-img" src=url alt=item_name.clone() />
                                        }.into_any()
                                    } else {
                                        view! { <span>{item.glyph.clone()}</span> }.into_any()
                                    }}
                                </button>
                            }
                        }).collect::<Vec<_>>().into_any()
                    }
                }}
            </div>
        </div>
    }
}
