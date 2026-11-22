//! Chat message bubble: ChatMessage, delivery statuses, TTL indicators, and reactions.

pub mod chat_status_badge;
pub mod content_views;
pub mod delivery_status;
pub mod emoji_text;
#[cfg(test)]
mod emoji_text_tests;
pub mod markdown;
#[cfg(test)]
mod markdown_tests;
pub mod media_views;
pub mod reactions;
pub mod ttl_indicator;
pub mod types;

pub use chat_status_badge::*;
pub use content_views::*;
pub use delivery_status::*;
pub use emoji_text::*;
pub use markdown::*;
pub use media_views::*;
pub use reactions::*;
pub use ttl_indicator::*;
pub use types::*;

use crate::core::{Metric, Slot, root_class, root_style};
use leptos::prelude::*;

pub(crate) const CSS: &str = concat!(
    include_str!("chat_message.css"),
    include_str!("media_views.css"),
    include_str!("content_views.css"),
);

/// Message bubble for conversational interfaces with sender identification, timestamp stamp,
/// directional alignment, rich content attachments, delivery status, TTL, and reactions.
#[component]
pub fn ChatMessage(
    /// Complete message data model. When supplied, overrides individual fields.
    #[prop(optional)]
    message: Option<OperatorMessage>,
    /// Message was sent by the local operator (aligned to the end).
    #[prop(optional)]
    sent: bool,
    /// Sender's name displayed above the message.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Sender's role tag badge (e.g. "OPERATOR", "ADMIN").
    #[prop(optional, into)]
    role: MaybeProp<String>,
    /// Timestamp stamp displayed below or beside the message.
    #[prop(optional, into)]
    stamp: MaybeProp<String>,
    /// Avatar image URL.
    #[prop(optional, into)]
    avatar: MaybeProp<String>,
    /// Simple plain text message body.
    #[prop(optional, into)]
    text: MaybeProp<String>,
    /// Rich polymorphic content payload.
    #[prop(optional)]
    content: Option<MessageContent>,
    /// Outgoing delivery status.
    #[prop(optional)]
    delivery_status: Option<DeliveryStatus>,
    /// Quoted message reply reference.
    #[prop(optional)]
    reply_to: Option<MessageReply>,
    /// Forwarded message attribution.
    #[prop(optional)]
    forward_info: Option<ForwardHeader>,
    /// Edited message indicator flag.
    #[prop(optional)]
    is_edited: bool,
    /// Self-destruct TTL countdown timer descriptor.
    #[prop(optional)]
    ttl: Option<MessageTtlInfo>,
    /// Consecutive message layout grouping.
    #[prop(optional)]
    grouping: Option<MessageGrouping>,
    /// Attached emoji reaction counters.
    #[prop(optional)]
    reactions: Option<Vec<MessageReaction>>,
    /// Bot interactive inline action buttons.
    #[prop(optional)]
    inline_keyboard: Option<Vec<Vec<InlineKeyboardButton>>>,
    /// Callback triggered when an inline action button is clicked.
    #[prop(optional)]
    on_action: Option<Callback<String>>,
    /// Callback triggered when a reaction chip is clicked.
    #[prop(optional)]
    on_reaction_click: Option<Callback<String>>,
    /// Callback triggered when the add reaction button is clicked.
    #[prop(optional)]
    on_add_reaction_click: Option<Callback<()>>,
    /// Callback triggered when a poll option is voted on.
    #[prop(optional, into)]
    on_poll_vote: MaybeProp<Callback<String>>,
    /// Custom name slot.
    #[prop(optional, into)]
    name_slot: Slot,
    /// Custom stamp slot.
    #[prop(optional, into)]
    stamp_slot: Slot,
    /// Custom avatar slot.
    #[prop(optional, into)]
    avatar_slot: Slot,
    /// Bubble internal padding in DU (`--op-chat-message-padding`).
    #[prop(optional, into)]
    padding: MaybeProp<f32>,
    /// Spacing between bubble, name, and stamp in DU (`--op-chat-message-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Message font size in TU (`--op-chat-message-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> AnyView {
    // Unpack from message model if provided, otherwise use explicit props
    let is_sent = message.as_ref().map(|m| m.is_outgoing).unwrap_or(sent);
    let resolved_name = message
        .as_ref()
        .map(|m| m.sender_name.clone())
        .or_else(|| name.get());
    let resolved_role = message
        .as_ref()
        .and_then(|m| m.sender_role.clone())
        .or_else(|| role.get());
    let resolved_stamp = message
        .as_ref()
        .map(|m| m.timestamp.clone())
        .or_else(|| stamp.get());
    let resolved_avatar = message
        .as_ref()
        .and_then(|m| m.avatar.clone())
        .or_else(|| avatar.get());
    let resolved_delivery = message
        .as_ref()
        .map(|m| m.delivery_status)
        .or(delivery_status);
    let resolved_reply = message
        .as_ref()
        .and_then(|m| m.reply_to.clone())
        .or(reply_to);
    let resolved_forward = message
        .as_ref()
        .and_then(|m| m.forward_info.clone())
        .or(forward_info);
    let resolved_edited = message.as_ref().map(|m| m.is_edited).unwrap_or(is_edited);
    let resolved_ttl = message.as_ref().and_then(|m| m.ttl.clone()).or(ttl);
    let resolved_grouping = message
        .as_ref()
        .map(|m| m.grouping)
        .or(grouping)
        .unwrap_or_default();
    let resolved_reactions = message
        .as_ref()
        .map(|m| m.reactions.clone())
        .or(reactions)
        .unwrap_or_default();
    let resolved_keyboard = message
        .as_ref()
        .map(|m| m.inline_keyboard.clone())
        .or(inline_keyboard)
        .unwrap_or_default();

    // Check for content payload
    let resolved_content = message
        .as_ref()
        .map(|m| m.content.clone())
        .or(content)
        .or_else(|| text.get().map(MessageContent::Text));

    // Check for isolated big emoji (1-3 emojis without attachments or reply)
    if resolved_reply.is_none()
        && resolved_forward.is_none()
        && let Some(MessageContent::Text(ref raw_txt)) = resolved_content
        && let Some(isolated_list) = get_isolated_emojis(raw_txt)
    {
        return view! {
            <IsolatedEmojiView
                emojis=isolated_list
                timestamp=resolved_stamp.unwrap_or_default()
                delivery_status=resolved_delivery
                is_outgoing=is_sent
                reactions=resolved_reactions
                class=class
                style=style
            />
        }
        .into_any();
    }

    // Check for standalone sticker
    if resolved_reply.is_none()
        && resolved_forward.is_none()
        && let Some(MessageContent::Sticker(sticker)) = resolved_content.as_ref()
    {
        return view! {
            <StickerView
                sticker=sticker.clone()
                timestamp=resolved_stamp
                is_outgoing=is_sent
                delivery_status=resolved_delivery
            />
        }
        .into_any();
    }

    // Check for standalone video note
    if resolved_reply.is_none()
        && resolved_forward.is_none()
        && let Some(MessageContent::VideoNote(vn)) = resolved_content.as_ref()
    {
        return view! {
            <VideoNoteView
                video_note=vn.clone()
                timestamp=resolved_stamp.unwrap_or_default()
                is_outgoing=is_sent
                delivery_status=resolved_delivery
            />
        }
        .into_any();
    }

    let show_header =
        resolved_grouping == MessageGrouping::Single || resolved_grouping == MessageGrouping::First;
    let show_avatar = !is_sent
        && (resolved_grouping == MessageGrouping::Single
            || resolved_grouping == MessageGrouping::Last);

    view! {
        <div
            class=root_class("op-chat-message", class)
            data-sent=is_sent.then_some("")
            data-group=resolved_grouping.as_str()
            style=root_style(
                vec![
                    Metric::du("--op-chat-message-padding", padding),
                    Metric::du("--op-chat-message-gap", gap),
                    Metric::tu("--op-chat-message-font-size", font_size),
                ],
                style,
            )
        >
            <div class="op-chat-message-container">
                {show_avatar.then(|| {
                    if avatar_slot.is_set() || resolved_avatar.is_some() {
                        view! {
                            <div class="op-chat-message-avatar-wrap">
                                {if avatar_slot.is_set() {
                                    avatar_slot.run()
                                } else {
                                    resolved_avatar.map(|src| view! {
                                        <img
                                            class="op-chat-message-avatar"
                                            src=src
                                            alt=""
                                            aria-hidden="true"
                                        />
                                    }.into_any())
                                }}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div class="op-chat-message-avatar-spacer" /> }.into_any()
                    }
                })}

                <div class="op-chat-message-body-wrap">
                    // 1. Sender name and role header (only for first/single messages)
                    {(show_header && (name_slot.is_set() || resolved_name.is_some() || resolved_role.is_some())).then(|| view! {
                        <div class="op-chat-message-header">
                            <span class="op-chat-message-name">
                                {if name_slot.is_set() {
                                    name_slot.run()
                                } else {
                                    resolved_name.map(|n| n.into_any())
                                }}
                            </span>
                            {resolved_role.map(|r| view! {
                                <span class="op-chat-role-badge">{r.to_uppercase()}</span>
                            })}
                        </div>
                    })}

                    // 2. Main Bubble Container
                    <div class="op-chat-message-bubble">
                        // Forward attribution header
                        {resolved_forward.map(|fwd| view! {
                            <div class="op-chat-forward-header">
                                <span class="op-chat-forward-icon">"↩"</span>
                                <span>{format!("Forwarded from {}", fwd.original_sender_name)}</span>
                            </div>
                        })}

                        // Quoted reply block
                        {resolved_reply.map(|reply| view! {
                            <div class="op-chat-reply-quote">
                                {reply.preview_image_url.map(|img| view! {
                                    <img class="op-chat-reply-preview-img" src=img alt="" />
                                })}
                                <div class="op-chat-reply-text-wrap">
                                    <span class="op-chat-reply-sender">{reply.sender_name}</span>
                                    <span class="op-chat-reply-excerpt">{reply.excerpt_text}</span>
                                </div>
                            </div>
                        })}

                        // Primary content / children
                        {if let Some(c) = children {
                            c().into_any()
                        } else if let Some(cnt) = resolved_content {
                            view! {
                                <RenderMessageContent content=cnt on_poll_vote=on_poll_vote />
                            }.into_any()
                        } else {
                            ().into_any()
                        }}

                        // Integrated footer: reactions and time/status
                        <div class="op-chat-footer">
                            {resolved_ttl.map(|t| view! {
                                <TtlIndicator ttl=t />
                            })}
                            {resolved_edited.then(|| view! {
                                <span class="op-chat-edited-mark">"edited"</span>
                            })}
                            {(stamp_slot.is_set() || resolved_stamp.is_some()).then(|| view! {
                                <span class="op-chat-message-stamp">
                                    {if stamp_slot.is_set() {
                                        stamp_slot.run()
                                    } else {
                                        resolved_stamp.map(|s| s.into_any())
                                    }}
                                </span>
                            })}
                            {is_sent.then(|| {
                                resolved_delivery.map(|st| view! {
                                    <DeliveryStatusView status=st />
                                })
                            })}
                        </div>
                    </div>

                    // 3. Emoji Reactions Row below the bubble
                    {(!resolved_reactions.is_empty() || on_add_reaction_click.is_some()).then(|| {
                        let sig = Signal::derive(move || resolved_reactions.clone());
                        let on_rx = on_reaction_click;
                        let on_add = on_add_reaction_click;
                        view! {
                            <ReactionsRow
                                reactions=sig
                                on_reaction_click=on_rx
                                on_add_click=on_add
                            />
                        }
                    })}

                    // 4. Tactical Inline Keyboard Buttons below reactions
                    {(!resolved_keyboard.is_empty()).then(|| view! {
                        <div class="op-chat-inline-keyboard">
                            {resolved_keyboard.into_iter().map(|row| view! {
                                <div class="op-chat-inline-keyboard-row">
                                    {row.into_iter().map(|btn| {
                                        let action_cb = on_action;
                                        let callback_payload = btn.callback_data.clone();
                                        if let Some(url) = btn.url {
                                            view! {
                                                <a
                                                    class="op-chat-inline-btn"
                                                    href=url
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                >
                                                    {btn.text}
                                                </a>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <button
                                                    type="button"
                                                    class="op-chat-inline-btn"
                                                    on:click=move |_| {
                                                        if let (Some(cb), Some(payload)) = (&action_cb, &callback_payload) {
                                                            cb.run(payload.clone());
                                                        }
                                                    }
                                                >
                                                    {btn.text}
                                                </button>
                                            }.into_any()
                                        }
                                    }).collect_view()}
                                </div>
                            }).collect_view()}
                        </div>
                    })}
                </div>
            </div>
        </div>
    }
    .into_any()
}
