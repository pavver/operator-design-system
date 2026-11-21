//! High-density emoji picker supporting categories, search filtering, and custom emoji packs.

use crate::components::chat_message::CustomEmojiMap;
use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("emoji_picker.css");

/// Emoji tab categories matching standard chat groupings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmojiTabCategory {
    Recent,
    #[default]
    Smileys,
    Gestures,
    Animals,
    Food,
    Activities,
    Travel,
    Objects,
    Symbols,
    Flags,
}

impl EmojiTabCategory {
    pub fn all() -> &'static [Self] {
        &[
            Self::Recent,
            Self::Smileys,
            Self::Gestures,
            Self::Animals,
            Self::Food,
            Self::Activities,
            Self::Travel,
            Self::Objects,
            Self::Symbols,
            Self::Flags,
        ]
    }

    pub fn icon_emoji(self) -> &'static str {
        match self {
            Self::Recent => "🕒",
            Self::Smileys => "😀",
            Self::Gestures => "👍",
            Self::Animals => "🐻",
            Self::Food => "🍔",
            Self::Activities => "⚽",
            Self::Travel => "🚀",
            Self::Objects => "💡",
            Self::Symbols => "❤️",
            Self::Flags => "🏳️",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Recent => "Recent",
            Self::Smileys => "Smileys",
            Self::Gestures => "Gestures",
            Self::Animals => "Animals",
            Self::Food => "Food",
            Self::Activities => "Activities",
            Self::Travel => "Travel",
            Self::Objects => "Objects",
            Self::Symbols => "Symbols",
            Self::Flags => "Flags",
        }
    }
}

pub fn standard_category_emojis(cat: EmojiTabCategory) -> &'static [&'static str] {
    match cat {
        EmojiTabCategory::Recent => &["👍", "❤️", "🔥", "🎉", "👏", "⚡", "😀", "🚀"],
        EmojiTabCategory::Smileys => &[
            "😀", "😃", "😄", "😁", "😆", "😅", "😂", "🤣", "🥲", "🥹", "😊", "😇", "🙂", "🙃",
            "😉", "😌", "😍", "🥰", "😘", "😗", "😋", "😛", "😜", "🤪", "🤨", "🧐", "🤓", "😎",
            "🥸", "🤩", "🥳", "😏", "😒", "😞", "😔", "😟", "😕", "🙁", "☹️", "😣", "😖", "😫",
            "😩", "🥺", "😢", "😭", "😤", "😠", "😡", "🤬", "🤯", "😳", "🥵", "🥶", "😱", "😨",
        ],
        EmojiTabCategory::Gestures => &[
            "👍",
            "👎",
            "👊",
            "✊",
            "🤛",
            "🤜",
            "👏",
            "🙌",
            "👐",
            "🤲",
            "🤝",
            "🙏",
            "✍️",
            "💅",
            "🤳",
            "💪",
            "🦾",
            "🦿",
            "🦵",
            "🦶",
            "👂",
            "👃",
            "🧠",
            "👀",
            "👁️",
            "🧑‍💻",
            "👨‍💻",
            "👩‍💻",
        ],
        EmojiTabCategory::Animals => &[
            "🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼", "🐨", "🐯", "🦁", "🐮", "🐷", "🐸",
            "🐵", "🐔", "🐧", "🐦", "🐤", "🦆", "🦅", "🦉", "🦇", "🐺", "🐗", "🐴", "🦄", "🐝",
        ],
        EmojiTabCategory::Food => &[
            "🍏", "🍎", "🍐", "🍊", "🍋", "🍌", "🍉", "🍇", "🍓", "🫐", "🍈", "🍒", "🍑", "🥭",
            "🍍", "🥥", "🥝", "🍅", "🍆", "🥑", "🥦", "🌽", "🌶️", "🍞", "🧀", "🍔", "🍟", "🍕",
        ],
        EmojiTabCategory::Activities => &[
            "⚽", "🏀", "🏈", "⚾", "🥎", "🎾", "🏐", "🏉", "🥏", "🎱", "🪀", "🏓", "🏸", "🏒",
            "🏑", "🥍", "🏏", "🎯", "⛳", "🪁", "🏹", "🎣", "🤿", "🥊", "🥋", "🛹", "🛼", "🏆",
        ],
        EmojiTabCategory::Travel => &[
            "🚗", "🚕", "🚙", "🚌", "🚎", "🏎️", "🚓", "🚑", "🚒", "🚐", "🛻", "🚚", "🚛", "🚜",
            "🏍️", "🛵", "🚲", "🛴", "🚨", "🚔", "✈️", "🚀", "🛸", "🚁", "🛶", "⛵", "🚤", "🚢",
        ],
        EmojiTabCategory::Objects => &[
            "⌚", "📱", "📲", "💻", "⌨️", "🖥️", "🖨️", "🖱️", "🕹️", "💽", "💾", "💿", "📀", "📷",
            "📹", "🎥", "📞", "☎️", "📟", "📠", "📺", "📻", "🎙️", "💡", "🔦", "🕯️", "📦", "🏷️",
        ],
        EmojiTabCategory::Symbols => &[
            "❤️",
            "🧡",
            "💛",
            "💚",
            "💙",
            "💜",
            "🖤",
            "🤍",
            "🤎",
            "💔",
            "❤️‍🔥",
            "❤️‍🩹",
            "❣️",
            "💕",
            "💞",
            "💓",
            "💗",
            "💖",
            "💘",
            "💝",
            "⚡",
            "🔥",
            "✨",
            "⭐",
            "🌟",
            "💫",
            "💥",
            "💯",
        ],
        EmojiTabCategory::Flags => &[
            "🏁",
            "🚩",
            "🎌",
            "🏴",
            "🏳️",
            "🏳️‍🌈",
            "🏳️‍⚧️",
            "🇺🇦",
            "🇺🇸",
            "🇬🇧",
            "🇩🇪",
            "🇫🇷",
            "🇯🇵",
            "🇨🇦",
        ],
    }
}

/// Pluggable emoji picker with category tabs, live search filtering,
/// and support for custom emoji map replacement.
#[component]
pub fn EmojiPicker(
    /// Callback invoked when an emoji or custom emoji code is selected.
    on_select: Callback<String>,
    /// Recent emojis list provided by the host app.
    #[prop(optional)]
    recent_emojis: Option<Vec<String>>,
    /// Pluggable registry of custom image-backed emojis.
    #[prop(optional, into)]
    custom_emojis: Option<CustomEmojiMap>,
    /// Pre-selected category tab. Defaults to Smileys.
    #[prop(optional)]
    initial_category: Option<EmojiTabCategory>,
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
    let selected_category = RwSignal::new(initial_category.unwrap_or(EmojiTabCategory::Smileys));
    let search_query = RwSignal::new(String::new());

    let search_placeholder = ui_text(
        None::<String>.into(),
        "emoji-picker.search",
        "Search emoji...",
    );
    let search_icon = ui_icon(None, "emoji-picker.search-icon", icons::SEARCH);

    let metrics = vec![
        Metric::du("--op-emoji-picker-height", height),
        Metric::du("--op-emoji-picker-cell-size", cell_size),
    ];
    let root_cls = root_class("op-emoji-picker", class);
    let root_stl = root_style(metrics, style);

    let custom_map = custom_emojis.unwrap_or_default();
    let custom_map_filter = custom_map.clone();
    let recents = recent_emojis.unwrap_or_else(|| {
        standard_category_emojis(EmojiTabCategory::Recent)
            .iter()
            .map(|&s| s.to_string())
            .collect()
    });

    let displayed_emojis = Memo::new(move |_| {
        let q = search_query.get().trim().to_lowercase();
        if !q.is_empty() {
            let mut all = Vec::new();
            for &cat in EmojiTabCategory::all() {
                for &em in standard_category_emojis(cat) {
                    if em.contains(&q) || cat.label().to_lowercase().contains(&q) {
                        all.push(em.to_string());
                    }
                }
            }
            for key in custom_map_filter.entries.keys() {
                if key.to_lowercase().contains(&q) {
                    all.push(key.clone());
                }
            }
            all.dedup();
            all
        } else {
            let cat = selected_category.get();
            if cat == EmojiTabCategory::Recent {
                recents.clone()
            } else {
                standard_category_emojis(cat)
                    .iter()
                    .map(|&s| s.to_string())
                    .collect()
            }
        }
    });

    let choose_emoji = move |emoji: String| {
        on_select.run(emoji);
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

            <div class="op-emoji-picker__categories" role="tablist" aria-label="Emoji categories">
                {EmojiTabCategory::all().iter().map(|&cat| {
                    let is_sel = move || selected_category.get() == cat;
                    let icon_sym = cat.icon_emoji();
                    let cat_label = cat.label();
                    view! {
                        <button
                            type="button"
                            class="op-emoji-picker__cat-btn"
                            data-selected=move || is_sel().then_some("true")
                            role="tab"
                            aria-selected=move || is_sel().to_string()
                            aria-label=cat_label
                            on:click=move |_| {
                                search_query.set(String::new());
                                selected_category.set(cat);
                            }
                        >
                            {icon_sym}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="op-emoji-picker__section-title">
                {move || {
                    let q = search_query.get();
                    if !q.is_empty() {
                        format!("Search: \"{q}\"")
                    } else {
                        selected_category.get().label().to_uppercase()
                    }
                }}
            </div>

            <div class="op-emoji-picker__grid">
                {move || {
                    let emojis = displayed_emojis.get();
                    let map_ref = &custom_map;
                    emojis.into_iter().map(|emoji| {
                        let em = emoji.clone();
                        let em_click = emoji.clone();
                        let custom_url = map_ref.get(&em).cloned();
                        view! {
                            <button
                                type="button"
                                class="op-emoji-picker__cell"
                                on:click=move |_| choose_emoji(em_click.clone())
                                aria-label=em.clone()
                            >
                                {if let Some(url) = custom_url {
                                    view! {
                                        <img class="op-emoji-picker__custom-img" src=url alt=em.clone() />
                                    }.into_any()
                                } else {
                                    view! { <span>{em.clone()}</span> }.into_any()
                                }}
                            </button>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
