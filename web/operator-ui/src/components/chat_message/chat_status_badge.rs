//! Chat list conversation status badge component.

use crate::components::chat_message::delivery_status::DeliveryStatusView;
use crate::components::chat_message::types::DeliveryStatus;
use crate::components::primitives::Icon;
use crate::core::{root_class, root_style};
use leptos::prelude::*;
use operator_ui_tokens::icons;

/// High-level conversation thread status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatStatus {
    /// Unread messages indicator with mention and critical priority flags.
    Unread {
        /// Number of unread messages.
        count: u32,
        /// Muted notification state.
        is_muted: bool,
        /// Personal user mention indicator.
        is_mentioned: bool,
        /// Count of personal mentions.
        mention_count: u32,
        /// Critical high-priority alert.
        is_critical: bool,
    },
    /// Outgoing delivery status.
    Delivery(DeliveryStatus),
    /// Background synchronization in progress.
    Syncing,
    /// Pinned conversation thread.
    Pinned,
    /// Silenced notifications flag.
    Muted,
    /// Neutral inactive state.
    None,
}

/// Conversation status badge displayed in chat lists and headers.
#[component]
pub fn ChatStatusBadge(
    /// Chat thread status descriptor.
    status: ChatStatus,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=root_class("op-chat-status-badge", class) style=root_style(vec![], style)>
            {match status {
                ChatStatus::Unread { count, is_muted, is_mentioned, mention_count, is_critical } => {
                    let effective_mentions = if is_mentioned { mention_count.max(1) } else { 0 };
                    view! {
                        <div class="op-chat-status-unread-wrap">
                            {(effective_mentions > 0).then(|| {
                                let mention_text = if effective_mentions > 1 {
                                    format!("@{effective_mentions}")
                                } else {
                                    "@".to_string()
                                };
                                view! {
                                    <span class="op-chat-badge-mention" role="status" aria-label="Mentioned">
                                        {mention_text}
                                    </span>
                                }
                            })}
                            {(count > 0).then(|| {
                                let count_text = if count > 99 { "99+".to_string() } else { count.to_string() };
                                view! {
                                    <span
                                        class="op-chat-badge-count"
                                        data-muted=is_muted.then_some("")
                                        data-critical=is_critical.then_some("")
                                        role="status"
                                        aria-label=format!("{count} unread messages")
                                    >
                                        {count_text}
                                    </span>
                                }
                            })}
                        </div>
                    }.into_any()
                }
                ChatStatus::Delivery(st) => view! {
                    <DeliveryStatusView status=st />
                }.into_any(),
                ChatStatus::Syncing => view! {
                    <span class="op-chat-status-syncing" role="status" aria-label="Syncing">
                        <Icon icon=icons::REFRESH size=3.75 />
                    </span>
                }.into_any(),
                ChatStatus::Pinned => view! {
                    <span class="op-chat-status-pinned" role="status" aria-label="Pinned">
                        <span class="op-chat-pin-mark">"📌"</span>
                    </span>
                }.into_any(),
                ChatStatus::Muted => view! {
                    <span class="op-chat-status-muted" role="status" aria-label="Muted">
                        <span class="op-chat-mute-mark">"🔕"</span>
                    </span>
                }.into_any(),
                ChatStatus::None => ().into_any(),
            }}
        </div>
    }
}
