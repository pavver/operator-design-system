//! Tactical Reaction Picker with quick reactions row and expandable category dialog.

use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("reaction_picker.css");

/// Pluggable reaction category or section provided by the application.
#[derive(Debug, Clone, PartialEq)]
pub struct ReactionSection {
    pub id: String,
    pub title: String,
    pub emojis: Vec<String>,
}

impl ReactionSection {
    pub fn new(id: impl Into<String>, title: impl Into<String>, emojis: Vec<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            emojis,
        }
    }
}

/// Tactical reaction picker offering a quick reactions bar with an expand button
/// to open the full category picker popover/dialog.
#[component]
pub fn ReactionPicker(
    /// Callback when an emoji reaction is picked.
    on_select: Callback<String>,
    /// Quick reactions displayed in the compact row. Provided by the host app.
    #[prop(optional, into)]
    quick_reactions: Option<Vec<String>>,
    /// Pluggable reaction categories/sections provided by the host app.
    #[prop(optional, into)]
    categories: Option<Vec<ReactionSection>>,
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
    let selected_section_idx = RwSignal::new(0_usize);

    let quick_list = quick_reactions.unwrap_or_default();
    let sections = categories.unwrap_or_default();

    let header_title = ui_text(
        None::<String>.into(),
        "reaction-picker.title",
        "SELECT REACTION",
    );
    let quick_reactions_aria = ui_text(
        None::<String>.into(),
        "reaction-picker.quick-reactions",
        "Quick reactions",
    );
    let more_aria = ui_text(
        None::<String>.into(),
        "reaction-picker.more",
        "More reactions",
    );
    let close_aria = ui_text(None::<String>.into(), "reaction-picker.close", "Close");
    let empty_text = ui_text(
        None::<String>.into(),
        "reaction-picker.empty",
        "No reactions available",
    );

    let close_icon = ui_icon(None, "reaction-picker.close-icon", icons::CLOSE);
    let expand_icon = ui_icon(None, "reaction-picker.expand-icon", icons::ADD);

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
                let aria_lbl = quick_reactions_aria.get();
                let more_lbl = more_aria.get();
                view! {
                    <div class="op-quick-reactions-row" role="toolbar" aria-label=aria_lbl>
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
                            aria-label=more_lbl
                        >
                            <Icon icon=expand_icon.get() />
                        </button>
                    </div>
                }
            })}

            {move || is_open.get().then(|| {
                let sec_list = sections.clone();
                let cur_idx = selected_section_idx.get();
                let current_emojis = sec_list.get(cur_idx).map(|s| s.emojis.clone()).unwrap_or_default();
                let is_sec_empty = sec_list.is_empty();

                view! {
                    <div class="op-reaction-picker-dialog" role="dialog" aria-modal="true">
                        <div class="op-reaction-picker__header">
                            <span class="op-reaction-picker__title">{header_title.get()}</span>
                            <button
                                type="button"
                                class="op-reaction-picker__close"
                                on:click=move |_| close_dialog()
                                aria-label=move || close_aria.get()
                            >
                                <Icon icon=close_icon.get() />
                            </button>
                        </div>

                        {(!is_sec_empty).then(|| {
                            let sec_tabs = sec_list.clone();
                            view! {
                                <div class="op-reaction-picker__tabs">
                                    {sec_tabs.into_iter().enumerate().map(|(idx, sec)| {
                                        let is_sel = move || selected_section_idx.get() == idx;
                                        let title = sec.title.clone();
                                        view! {
                                            <button
                                                type="button"
                                                class="op-reaction-picker__tab"
                                                data-selected=move || is_sel().then_some("true")
                                                on:click=move |_| selected_section_idx.set(idx)
                                            >
                                                {title}
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }
                        })}

                        <div class="op-reaction-picker__grid">
                            {if current_emojis.is_empty() {
                                view! {
                                    <div class="op-reaction-picker__empty">
                                        {empty_text.get()}
                                    </div>
                                }.into_any()
                            } else {
                                current_emojis.into_iter().map(|emoji| {
                                    let em = emoji.clone();
                                    let em_click = emoji.clone();
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
                                }).collect::<Vec<_>>().into_any()
                            }}
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
