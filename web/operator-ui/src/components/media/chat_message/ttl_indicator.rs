//! Self-destruct TTL countdown timer badge component.

use crate::components::chat_message::types::MessageTtlInfo;
use crate::components::primitives::Icon;
use crate::core::{root_class, root_style};
use leptos::prelude::*;
use operator_ui_tokens::dim;
use operator_ui_tokens::icons;

/// Self-destruct TTL timer badge indicating expiring message content.
#[component]
pub fn TtlIndicator(
    /// TTL timing descriptor.
    ttl: MessageTtlInfo,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let is_urgent = ttl.remaining_seconds <= 10;
    let formatted_time = if ttl.remaining_seconds >= 60 {
        let mins = ttl.remaining_seconds / 60;
        let secs = ttl.remaining_seconds % 60;
        format!("{mins}:{secs:02}")
    } else {
        format!("{}s", ttl.remaining_seconds)
    };
    let time_label = formatted_time.clone();

    view! {
        <span
            class=root_class("op-ttl-indicator", class)
            data-urgent=is_urgent.then_some("")
            style=root_style(vec![], style)
            role="status"
            aria-label=format!("Self-destruct timer: {time_label} remaining")
        >
            {ttl.is_encrypted.then(|| view! {
                <span class="op-ttl-lock-icon" aria-hidden="true">
                    <Icon icon=icons::LOCK size=dim::ICON_XSMALL />
                </span>
            })}
            <span class="op-ttl-timer-glyph" aria-hidden="true">"⏱"</span>
            <span class="op-ttl-text">{formatted_time}</span>
        </span>
    }
}
