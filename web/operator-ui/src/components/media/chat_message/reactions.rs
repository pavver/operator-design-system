//! Message emoji reactions row component.

use crate::components::chat_message::types::MessageReaction;
use crate::components::primitives::Icon;
use crate::core::{root_class, root_style};
use leptos::prelude::*;
use operator_ui_tokens::dim;
use operator_ui_tokens::icons;

/// Row of emoji reaction counters with selection toggle states.
#[component]
pub fn ReactionsRow(
    /// List of reaction counters.
    #[prop(into)]
    reactions: Signal<Vec<MessageReaction>>,
    /// Callback triggered when a reaction chip is clicked (passes the emoji string).
    #[prop(optional, into)]
    on_reaction_click: MaybeProp<Callback<String>>,
    /// Callback triggered when the add reaction (+) button is clicked.
    #[prop(optional, into)]
    on_add_click: MaybeProp<Callback<()>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=root_class("op-reactions-row", class) style=root_style(vec![], style)>
            {move || {
                let items = reactions.get();
                items.into_iter().map(|reaction| {
                    let emoji_for_click = reaction.emoji.clone();
                    let emoji_for_label = reaction.emoji.clone();
                    let on_click_cb = on_reaction_click.get();
                    view! {
                        <button
                            type="button"
                            class="op-reaction-chip"
                            data-selected=reaction.is_selected_by_me.then_some("")
                            on:click=move |_| {
                                if let Some(cb) = &on_click_cb {
                                    cb.run(emoji_for_click.clone());
                                }
                            }
                            aria-label=format!("React with {} ({} votes)", emoji_for_label, reaction.count)
                        >
                            <span class="op-reaction-emoji">{reaction.emoji}</span>
                            <span class="op-reaction-count">{reaction.count}</span>
                        </button>
                    }
                }).collect_view()
            }}
            {move || on_add_click.get().map(|cb| view! {
                <button
                    type="button"
                    class="op-reaction-add-btn"
                    on:click=move |_| cb.run(())
                    aria-label="Add reaction"
                >
                    <Icon icon=icons::ADD size=dim::ICON_XSMALL />
                </button>
            })}
        </div>
    }
}
