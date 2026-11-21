//! Pluggable vector and animated sticker picker with pack navigation and preview support.

use crate::components::chat_message::{StickerAttachment, StickerFormat};
use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("sticker_picker.css");

/// Individual sticker item within a pack.
#[derive(Debug, Clone, PartialEq)]
pub struct StickerItem {
    pub id: String,
    pub name: String,
    pub url: String,
    pub format: StickerFormat,
    pub emoji: String,
}

impl StickerItem {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        url: impl Into<String>,
        format: StickerFormat,
        emoji: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            url: url.into(),
            format,
            emoji: emoji.into(),
        }
    }

    pub fn to_attachment(&self, pack_id: Option<String>) -> StickerAttachment {
        StickerAttachment {
            url: self.url.clone(),
            format: self.format.clone(),
            emoji: Some(self.emoji.clone()),
            pack_id,
            sticker_id: Some(self.id.clone()),
        }
    }
}

/// Sticker pack containing grouped stickers and an icon graphic.
#[derive(Debug, Clone, PartialEq)]
pub struct StickerPack {
    pub id: String,
    pub title: String,
    pub icon_url: String,
    pub icon_format: StickerFormat,
    pub stickers: Vec<StickerItem>,
}

impl StickerPack {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        icon_url: impl Into<String>,
        icon_format: StickerFormat,
        stickers: Vec<StickerItem>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon_url: icon_url.into(),
            icon_format,
            stickers,
        }
    }
}

/// Pluggable sticker picker component supporting pack navigation tabs,
/// search queries, animated/vector formats, and enlarged hover previews.
#[component]
pub fn StickerPicker(
    /// Callback when a sticker is selected.
    on_select: Callback<StickerAttachment>,
    /// Available sticker packs provided by the host application.
    #[prop(optional, into)]
    sticker_packs: Option<Vec<StickerPack>>,
    /// Recent stickers list.
    #[prop(optional, into)]
    recent_stickers: Option<Vec<StickerItem>>,
    /// Custom sticker renderer for animated/vector formats.
    #[prop(optional)]
    render_sticker: Option<Callback<StickerAttachment, AnyView>>,
    /// Picker height in DU (`--op-sticker-picker-height`). Defaults to 60 DU.
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Sticker cell size in DU (`--op-sticker-picker-cell-size`). Defaults to 18 DU.
    #[prop(optional, into)]
    cell_size: MaybeProp<f32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline CSS styles.
    #[prop(optional, into)]
    style: MaybeProp<String>,
) -> impl IntoView {
    let packs = sticker_packs.unwrap_or_default();
    let recents = recent_stickers.unwrap_or_default();

    let selected_pack_idx = RwSignal::new(0_usize);
    let search_query = RwSignal::new(String::new());
    let preview_sticker = RwSignal::new(None::<StickerItem>);

    let search_placeholder = ui_text(
        None::<String>.into(),
        "sticker-picker.search",
        "Search stickers...",
    );
    let recent_label = ui_text(
        None::<String>.into(),
        "sticker-picker.recent",
        "Recent stickers",
    );
    let packs_aria = ui_text(
        None::<String>.into(),
        "sticker-picker.packs",
        "Sticker packs",
    );
    let search_prefix = ui_text(
        None::<String>.into(),
        "sticker-picker.search-prefix",
        "Search results",
    );
    let empty_text = ui_text(
        None::<String>.into(),
        "sticker-picker.empty",
        "No stickers found",
    );

    let search_icon = ui_icon(None, "sticker-picker.search-icon", icons::SEARCH);
    let recent_icon = ui_icon(None, "sticker-picker.recent-icon", icons::REFRESH);

    let metrics = vec![
        Metric::du("--op-sticker-picker-height", height),
        Metric::du("--op-sticker-picker-cell-size", cell_size),
    ];
    let root_cls = root_class("op-sticker-picker", class);
    let root_stl = root_style(metrics, style);

    let has_recents = !recents.is_empty();

    let filtered_packs = Memo::new({
        let packs = packs.clone();
        let recents = recents.clone();
        move |_| {
            let q = search_query.get().trim().to_lowercase();
            if q.is_empty() {
                return (None, packs.clone());
            }
            let mut matched = Vec::new();
            for item in &recents {
                if item.name.to_lowercase().contains(&q) || item.emoji.contains(&q) {
                    matched.push(item.clone());
                }
            }
            for pack in &packs {
                for item in &pack.stickers {
                    if item.name.to_lowercase().contains(&q) || item.emoji.contains(&q) {
                        matched.push(item.clone());
                    }
                }
            }
            matched.dedup_by(|a, b| a.id == b.id);
            (Some(matched), packs.clone())
        }
    });

    view! {
        <div class=root_cls style=root_stl>
            <div class="op-sticker-picker__search">
                <span class="op-sticker-picker__search-icon">
                    <Icon icon=search_icon.get() />
                </span>
                <input
                    type="text"
                    class="op-sticker-picker__search-input"
                    placeholder=move || search_placeholder.get()
                    prop:value=move || search_query.get()
                    on:input=move |ev: ev::Event| {
                        search_query.set(event_target_value(&ev));
                    }
                />
            </div>

            {move || {
                let (search_matched, ref_packs) = filtered_packs.get();
                if search_matched.is_some() {
                    ().into_any()
                } else {
                    let recent_lbl = recent_label.get();
                    let recent_ic = recent_icon.get();
                    let packs_lbl = packs_aria.get();
                    view! {
                        <div class="op-sticker-picker__packs" role="tablist" aria-label=packs_lbl>
                            {has_recents.then(|| {
                                let is_sel = move || selected_pack_idx.get() == 0;
                                view! {
                                    <button
                                        type="button"
                                        class="op-sticker-picker__pack-btn"
                                        data-selected=move || is_sel().then_some("true")
                                        role="tab"
                                        aria-selected=move || is_sel().to_string()
                                        aria-label=recent_lbl
                                        on:click=move |_| selected_pack_idx.set(0)
                                    >
                                        <Icon icon=recent_ic />
                                    </button>
                                }
                            })}

                            {ref_packs.iter().enumerate().map(|(idx, pack)| {
                                let target_idx = if has_recents { idx + 1 } else { idx };
                                let is_sel = move || selected_pack_idx.get() == target_idx;
                                let title = pack.title.clone();
                                let icon_url = pack.icon_url.clone();
                                view! {
                                    <button
                                        type="button"
                                        class="op-sticker-picker__pack-btn"
                                        data-selected=move || is_sel().then_some("true")
                                        role="tab"
                                        aria-selected=move || is_sel().to_string()
                                        aria-label=title
                                        on:click=move |_| selected_pack_idx.set(target_idx)
                                    >
                                        <img class="op-sticker-picker__pack-icon" src=icon_url alt="" />
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }
            }}

            <div class="op-sticker-picker__content">
                {move || {
                    let (search_matched, ref_packs) = filtered_packs.get();
                    if let Some(matched) = search_matched {
                        if matched.is_empty() {
                            view! {
                                <div class="op-sticker-picker__empty">
                                    {empty_text.get()}
                                </div>
                            }.into_any()
                        } else {
                            let prefix = search_prefix.get();
                            view! {
                                <div class="op-sticker-picker__section-title">
                                    {format!("{prefix} ({} found)", matched.len())}
                                </div>
                                <div class="op-sticker-picker__grid">
                                    {matched.into_iter().map(|item| {
                                        render_sticker_cell(item, None, on_select, preview_sticker, render_sticker)
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any()
                        }
                    } else {
                        let cur_idx = selected_pack_idx.get();
                        if has_recents && cur_idx == 0 {
                            let r_lbl = recent_label.get();
                            view! {
                                <div class="op-sticker-picker__section-title">{r_lbl}</div>
                                <div class="op-sticker-picker__grid">
                                    {recents.clone().into_iter().map(|item| {
                                        render_sticker_cell(item, None, on_select, preview_sticker, render_sticker)
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any()
                        } else {
                            let pack_offset = if has_recents { cur_idx.saturating_sub(1) } else { cur_idx };
                            if let Some(pack) = ref_packs.get(pack_offset) {
                                let pack_id = pack.id.clone();
                                let title = pack.title.clone();
                                let stickers = pack.stickers.clone();
                                view! {
                                    <div class="op-sticker-picker__section-title">{title}</div>
                                    <div class="op-sticker-picker__grid">
                                        {stickers.into_iter().map(|item| {
                                            render_sticker_cell(item, Some(pack_id.clone()), on_select, preview_sticker, render_sticker)
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="op-sticker-picker__empty">
                                        {empty_text.get()}
                                    </div>
                                }.into_any()
                            }
                        }
                    }
                }}
            </div>

            {move || preview_sticker.get().map(|sticker| {
                view! {
                    <div class="op-sticker-preview-overlay">
                        <img class="op-sticker-preview-img" src=sticker.url.clone() alt=sticker.name.clone() />
                        <span class="op-sticker-preview-name">{format!("{} {}", sticker.emoji, sticker.name)}</span>
                    </div>
                }
            })}
        </div>
    }
}

fn render_sticker_cell(
    item: StickerItem,
    pack_id: Option<String>,
    on_select: Callback<StickerAttachment>,
    preview_signal: RwSignal<Option<StickerItem>>,
    custom_render: Option<Callback<StickerAttachment, AnyView>>,
) -> impl IntoView {
    let attachment = item.to_attachment(pack_id);
    let item_for_click = attachment.clone();
    let item_for_hover = item.clone();
    let item_name = item.name.clone();

    view! {
        <button
            type="button"
            class="op-sticker-picker__cell"
            on:click=move |_| on_select.run(item_for_click.clone())
            on:mouseenter=move |_| preview_signal.set(Some(item_for_hover.clone()))
            on:mouseleave=move |_| preview_signal.set(None)
            aria-label=item_name.clone()
        >
            {if let Some(render_fn) = custom_render {
                render_fn.run(attachment)
            } else {
                view! {
                    <img class="op-sticker-picker__img" src=item.url alt=item_name.clone() />
                }.into_any()
            }}
        </button>
    }
}
