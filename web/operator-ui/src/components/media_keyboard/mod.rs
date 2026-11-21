//! Generic Media Keyboard with unified Emoji, Sticker, and GIF dispatching.
//!
//! Provides a tabbed panel compatible with custom emoji packs, sticker sets, and animated clips (including Telegram packs).

use crate::components::chat_message::{CustomEmojiMap, StickerAttachment};
use crate::components::emoji_picker::{EmojiCategory, EmojiItem, EmojiPicker};
use crate::components::gif_picker::{GifItem, GifPicker};
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::primitives::Icon;
use crate::components::sticker_picker::{StickerPack, StickerPicker};
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("media_keyboard.css");

/// Primary tabs of the media keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaKeyboardTab {
    #[default]
    Emoji,
    Stickers,
    Gifs,
}

impl MediaKeyboardTab {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Emoji => "emoji",
            Self::Stickers => "stickers",
            Self::Gifs => "gifs",
        }
    }
}

/// Unified Media Keyboard with Emoji, Sticker, and GIF dispatching.
///
/// On desktop, mounts as a floating anchored popover adjacent to the input field.
/// On phones, acts as an inline panel that replaces the software keyboard area.
#[component]
pub fn MediaKeyboard(
    /// Callback when an emoji is picked.
    on_select_emoji: Callback<String>,
    /// Callback when a sticker is selected.
    on_select_sticker: Callback<StickerAttachment>,
    /// Callback when a GIF item is selected.
    on_select_gif: Callback<GifItem>,
    /// Optional backspace action button callback.
    #[prop(optional)]
    on_backspace: Option<Callback<()>>,
    /// Whether the keyboard is currently open/visible.
    #[prop(optional, into)]
    open: MaybeProp<bool>,
    /// Callback when keyboard should close.
    #[prop(optional)]
    on_close: Option<Callback<()>>,
    /// Desktop mode: renders anchored popover if true, docked bottom panel if false.
    #[prop(optional, into)]
    desktop: MaybeProp<bool>,
    /// Anchor coordinates / element rect for desktop popover mode.
    #[prop(optional, into)]
    anchor: Option<Signal<PopoverAnchor>>,
    /// Initial active tab (Emoji, Stickers, or GIFs).
    #[prop(optional)]
    initial_tab: Option<MediaKeyboardTab>,
    /// Pluggable emoji categories/packs.
    #[prop(optional, into)]
    emoji_categories: Option<Vec<EmojiCategory>>,
    /// Recent emojis list provided by the host app.
    #[prop(optional, into)]
    recent_emojis: Option<Vec<EmojiItem>>,
    /// Pluggable registry of custom image emojis.
    #[prop(optional, into)]
    custom_emojis: Option<CustomEmojiMap>,
    /// Pluggable sticker packs.
    #[prop(optional, into)]
    sticker_packs: Option<Vec<StickerPack>>,
    /// Pluggable GIF collection.
    #[prop(optional, into)]
    gifs: Option<Vec<GifItem>>,
    /// Trending tags list for GIFs.
    #[prop(optional, into)]
    trending_tags: Option<Vec<String>>,
    /// Width in DU (`--op-media-keyboard-width`). Defaults to 85 DU.
    #[prop(optional, into)]
    width: MaybeProp<f32>,
    /// Height in DU (`--op-media-keyboard-height`). Defaults to 70 DU.
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline CSS styles.
    #[prop(optional, into)]
    style: MaybeProp<String>,
) -> impl IntoView {
    let current_tab = RwSignal::new(initial_tab.unwrap_or_default());
    let is_desktop = Signal::derive(move || desktop.get().unwrap_or(true));

    let open_rw = RwSignal::new(open.get().unwrap_or(true));
    Effect::new(move |_| {
        if let Some(o) = open.get() {
            open_rw.set(o);
        }
    });

    Effect::new(move |_| {
        if let (false, Some(cb)) = (open_rw.get(), on_close) {
            cb.run(());
        }
    });

    let tab_emoji_text = ui_text(None::<String>.into(), "media-keyboard.emoji", "Emoji");
    let tab_stickers_text = ui_text(None::<String>.into(), "media-keyboard.stickers", "Stickers");
    let tab_gifs_text = ui_text(None::<String>.into(), "media-keyboard.gifs", "GIF");
    let backspace_aria = ui_text(
        None::<String>.into(),
        "media-keyboard.backspace",
        "Backspace",
    );
    let backspace_icon = ui_icon(None, "media-keyboard.backspace-icon", icons::CLOSE);

    let metrics = vec![
        Metric::du("--op-media-keyboard-width", width),
        Metric::du("--op-media-keyboard-height", height),
    ];
    let root_cls = StoredValue::new(root_class("op-media-keyboard", class));
    let root_stl = StoredValue::new(root_style(metrics, style));

    let emoji_cats = StoredValue::new(emoji_categories);
    let emoji_recents = StoredValue::new(recent_emojis);
    let emoji_map = StoredValue::new(custom_emojis);
    let packs = StoredValue::new(sticker_packs);
    let gif_items = StoredValue::new(gifs);
    let gif_tags = StoredValue::new(trending_tags);

    let render_panel = move || {
        let em_cats = emoji_cats.get_value().unwrap_or_default();
        let em_rec = emoji_recents.get_value().unwrap_or_default();
        let em_map = emoji_map.get_value();
        let st_packs = packs.get_value();
        let gf_list = gif_items.get_value();
        let gf_tags = gif_tags.get_value().unwrap_or_default();
        let cls = root_cls.with_value(|f| f());
        let stl = root_stl.with_value(|f| f());

        view! {
            <div
                class=cls
                style=stl
                data-phone=move || (!is_desktop.get()).then_some("true")
                role="region"
                aria-label="Media keyboard"
            >
                <div class="op-media-keyboard__content">
                    {move || match current_tab.get() {
                        MediaKeyboardTab::Emoji => {
                            view! {
                                <EmojiPicker
                                    on_select=on_select_emoji
                                    categories=em_cats.clone()
                                    recent_emojis=em_rec.clone()
                                    custom_emojis=em_map.clone().unwrap_or_default()
                                />
                            }.into_any()
                        }
                        MediaKeyboardTab::Stickers => {
                            view! {
                                <StickerPicker
                                    on_select=on_select_sticker
                                    sticker_packs=st_packs.clone().unwrap_or_default()
                                />
                            }.into_any()
                        }
                        MediaKeyboardTab::Gifs => {
                            view! {
                                <GifPicker
                                    on_select=on_select_gif
                                    gifs=gf_list.clone().unwrap_or_default()
                                    trending_tags=gf_tags.clone()
                                />
                            }.into_any()
                        }
                    }}
                </div>

                <div class="op-media-keyboard__bottom-bar">
                    <div class="op-media-keyboard__tabs" role="tablist">
                        <button
                            type="button"
                            class="op-media-keyboard__tab-btn"
                            data-selected=move || (current_tab.get() == MediaKeyboardTab::Emoji).then_some("true")
                            role="tab"
                            aria-selected=move || (current_tab.get() == MediaKeyboardTab::Emoji).to_string()
                            on:click=move |_| current_tab.set(MediaKeyboardTab::Emoji)
                        >
                            {tab_emoji_text.get()}
                        </button>

                        <button
                            type="button"
                            class="op-media-keyboard__tab-btn"
                            data-selected=move || (current_tab.get() == MediaKeyboardTab::Stickers).then_some("true")
                            role="tab"
                            aria-selected=move || (current_tab.get() == MediaKeyboardTab::Stickers).to_string()
                            on:click=move |_| current_tab.set(MediaKeyboardTab::Stickers)
                        >
                            {tab_stickers_text.get()}
                        </button>

                        <button
                            type="button"
                            class="op-media-keyboard__tab-btn"
                            data-selected=move || (current_tab.get() == MediaKeyboardTab::Gifs).then_some("true")
                            role="tab"
                            aria-selected=move || (current_tab.get() == MediaKeyboardTab::Gifs).to_string()
                            on:click=move |_| current_tab.set(MediaKeyboardTab::Gifs)
                        >
                            {tab_gifs_text.get()}
                        </button>
                    </div>

                    {on_backspace.map(|cb| {
                        let aria_bs = backspace_aria.get();
                        view! {
                            <button
                                type="button"
                                class="op-media-keyboard__backspace-btn"
                                on:click=move |_| cb.run(())
                                aria-label=aria_bs
                            >
                                <Icon icon=backspace_icon.get() />
                            </button>
                        }
                    })}
                </div>
            </div>
        }
    };

    view! {
        {move || {
            if !open_rw.get() {
                return ().into_any();
            }
            if is_desktop.get() {
                if let Some(anchor_sig) = anchor {
                    view! {
                        <Popover
                            anchor=anchor_sig
                            open=open_rw
                            placement=Placement::Above
                        >
                            {render_panel()}
                        </Popover>
                    }.into_any()
                } else {
                    render_panel().into_any()
                }
            } else {
                render_panel().into_any()
            }
        }}
    }
}
