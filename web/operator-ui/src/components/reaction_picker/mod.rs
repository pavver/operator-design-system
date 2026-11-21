//! Tactical Reaction Picker with quick reactions row and expandable category dialog.

use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("reaction_picker.css");

/// Category filter for reaction emojis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReactionCategory {
    #[default]
    Popular,
    Smileys,
    Gestures,
    Symbols,
    Tactical,
}

impl ReactionCategory {
    pub fn all() -> &'static [Self] {
        &[
            Self::Popular,
            Self::Smileys,
            Self::Gestures,
            Self::Symbols,
            Self::Tactical,
        ]
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Popular => "popular",
            Self::Smileys => "smileys",
            Self::Gestures => "gestures",
            Self::Symbols => "symbols",
            Self::Tactical => "tactical",
        }
    }
}

pub fn emojis_for_category(cat: ReactionCategory) -> &'static [&'static str] {
    match cat {
        ReactionCategory::Popular => &[
            "👍", "👎", "❤️", "🔥", "👏", "🎉", "⚡", "🤔", "🤯", "💩", "🙏", "💯", "🎯", "🚀",
        ],
        ReactionCategory::Smileys => &[
            "😀", "😃", "😄", "😁", "😆", "😅", "😂", "🤣", "🥲", "🥹", "😊", "😇", "🙂", "🙃",
            "😉", "😌", "😍", "🥰", "😘", "😗", "😋", "😛", "😜", "🤪", "🤨", "🧐", "🤓", "😎",
            "🥸", "🤩", "🥳", "😏", "😒", "😞", "😔", "😟", "😕", "🙁", "☹️", "😣", "😖", "😫",
            "😩", "🥺", "😢", "😭", "😤", "😠", "😡", "🤬", "🤯", "😳", "🥵", "🥶", "😱", "😨",
        ],
        ReactionCategory::Gestures => &[
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
        ],
        ReactionCategory::Symbols => &[
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
        ReactionCategory::Tactical => &[
            "🎯", "🚀", "🛡️", "⚔️", "📡", "📻", "💻", "🕹️", "🔋", "💡", "🔧", "🔨", "📦", "🏆",
            "🥇", "🥈", "🥉", "🎲", "🎰", "🔔",
        ],
    }
}

/// Tactical reaction picker offering a quick reactions bar with an expand button
/// to open the full category picker popover/dialog.
#[component]
pub fn ReactionPicker(
    /// Callback when an emoji reaction is picked.
    on_select: Callback<String>,
    /// Quick reactions displayed in the compact row. Defaults to TOP 7.
    #[prop(optional)]
    quick_reactions: Option<Vec<String>>,
    /// Whether the quick reactions bar is shown; defaults to true.
    #[prop(default = true)]
    show_quick_bar: bool,
    /// Explicit open state for the full picker dialog.
    #[prop(optional, into)]
    open: MaybeProp<bool>,
    /// Callback invoked when full dialog is dismissed.
    #[prop(optional)]
    on_close: Option<Callback<()>>,
    /// Width of full picker dialog in DU (`--op-reaction-picker-width`). Default is 85 DU.
    #[prop(optional, into)]
    width: MaybeProp<f32>,
    /// Max height of full picker dialog in DU (`--op-reaction-picker-max-height`). Default is 110 DU.
    #[prop(optional, into)]
    max_height: MaybeProp<f32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline CSS styles.
    #[prop(optional, into)]
    style: MaybeProp<String>,
) -> impl IntoView {
    let local_open = RwSignal::new(false);
    let is_open = Signal::derive(move || open.get().unwrap_or_else(|| local_open.get()));
    let selected_category = RwSignal::new(ReactionCategory::Popular);

    let default_quick = vec![
        "👍".to_string(),
        "👎".to_string(),
        "❤️".to_string(),
        "🔥".to_string(),
        "👏".to_string(),
        "🎉".to_string(),
        "⚡".to_string(),
    ];
    let quick_list = quick_reactions.unwrap_or(default_quick);

    let header_title = ui_text(
        None::<String>.into(),
        "reaction-picker.title",
        "SELECT REACTION",
    );
    let close_icon = ui_icon(None, "reaction-picker.close", icons::CLOSE);
    let expand_icon = ui_icon(None, "reaction-picker.expand", icons::ADD);

    let metrics = vec![
        Metric::du("--op-reaction-picker-width", width),
        Metric::du("--op-reaction-picker-max-height", max_height),
    ];
    let root_cls = root_class("op-reaction-picker-root", class);
    let root_stl = root_style(metrics, style);

    let close_dialog = move || {
        local_open.set(false);
        if let Some(cb) = on_close {
            cb.run(());
        }
    };

    let choose_emoji = move |emoji: String| {
        on_select.run(emoji);
        close_dialog();
    };

    view! {
        <div class=root_cls style=root_stl>
            {show_quick_bar.then(|| {
                let q_list = quick_list.clone();
                view! {
                    <div class="op-quick-reactions-row" role="toolbar" aria-label="Quick reactions">
                        {q_list.into_iter().map(|emoji| {
                            let em = emoji.clone();
                            let em_pick = emoji.clone();
                            view! {
                                <button
                                    type="button"
                                    class="op-quick-reaction-btn"
                                    on:click=move |_| choose_emoji(em_pick.clone())
                                    aria-label=em.clone()
                                >
                                    {em.clone()}
                                </button>
                            }
                        }).collect::<Vec<_>>()}

                        <button
                            type="button"
                            class="op-quick-reaction-expand"
                            on:click=move |_| local_open.update(|o| *o = !*o)
                            aria-label="More reactions"
                        >
                            <Icon icon=expand_icon.get() />
                        </button>
                    </div>
                }
            })}

            {move || is_open.get().then(|| {
                view! {
                    <div class="op-reaction-picker-dialog" role="dialog" aria-modal="true">
                        <div class="op-reaction-picker__header">
                            <span class="op-reaction-picker__title">{header_title.get()}</span>
                            <button
                                type="button"
                                class="op-reaction-picker__close"
                                on:click=move |_| close_dialog()
                                aria-label="Close"
                            >
                                <Icon icon=close_icon.get() />
                            </button>
                        </div>

                        <div class="op-reaction-picker__tabs">
                            {ReactionCategory::all().iter().map(|&cat| {
                                let is_sel = move || selected_category.get() == cat;
                                let label = match cat {
                                    ReactionCategory::Popular => "TOP",
                                    ReactionCategory::Smileys => "SMILEYS",
                                    ReactionCategory::Gestures => "GESTURES",
                                    ReactionCategory::Symbols => "SYMBOLS",
                                    ReactionCategory::Tactical => "TACTICAL",
                                };
                                view! {
                                    <button
                                        type="button"
                                        class="op-reaction-picker__tab"
                                        data-selected=move || is_sel().then_some("true")
                                        on:click=move |_| selected_category.set(cat)
                                    >
                                        {label}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>

                        <div class="op-reaction-picker__grid">
                            {move || {
                                let emojis = emojis_for_category(selected_category.get());
                                emojis.iter().map(|&emoji| {
                                    let em = emoji.to_string();
                                    let em_click = em.clone();
                                    view! {
                                        <button
                                            type="button"
                                            class="op-reaction-picker__cell"
                                            on:click=move |_| choose_emoji(em_click.clone())
                                            aria-label=em.clone()
                                        >
                                            {em.clone()}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
