//! Chat message bubble: ChatMessage.

use crate::core::{Metric, Slot, root_class, root_style};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("chat_message.css");

/// Message bubble for conversational interfaces with sender identification, timestamp stamp, and directional alignment.
#[component]
pub fn ChatMessage(
    /// Message was sent by the local operator (aligned to the end).
    #[prop(optional)]
    sent: bool,
    /// Sender's name displayed above the message.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Timestamp stamp displayed below or beside the message.
    #[prop(optional, into)]
    stamp: MaybeProp<String>,
    /// Avatar image URL.
    #[prop(optional, into)]
    avatar: MaybeProp<String>,
    /// Simple plain text message body.
    #[prop(optional, into)]
    text: MaybeProp<String>,
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
) -> impl IntoView {
    view! {
        <div
            class=root_class("op-chat-message", class)
            data-sent=sent.then_some("")
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
                {(avatar_slot.is_set() || avatar.get().is_some()).then(|| view! {
                    <div class="op-chat-message-avatar-wrap">
                        {if avatar_slot.is_set() {
                            avatar_slot.run()
                        } else {
                            avatar.get().map(|src| view! {
                                <img
                                    class="op-chat-message-avatar"
                                    src=src
                                    alt=""
                                    aria-hidden="true"
                                />
                            }.into_any())
                        }}
                    </div>
                })}
                <div class="op-chat-message-body-wrap">
                    {(name_slot.is_set() || name.get().is_some()).then(|| view! {
                        <div class="op-chat-message-name">
                            {if name_slot.is_set() {
                                name_slot.run()
                            } else {
                                name.get().map(|n| n.into_any())
                            }}
                        </div>
                    })}
                    <div class="op-chat-message-bubble">
                        {if let Some(c) = children {
                            c().into_any()
                        } else if let Some(t) = text.get() {
                            view! {
                                <span>{t}</span>
                            }.into_any()
                        } else {
                            ().into_any()
                        }}
                    </div>
                    {(stamp_slot.is_set() || stamp.get().is_some()).then(|| view! {
                        <div class="op-chat-message-stamp">
                            {if stamp_slot.is_set() {
                                stamp_slot.run()
                            } else {
                                stamp.get().map(|s| s.into_any())
                            }}
                        </div>
                    })}
                </div>
            </div>
        </div>
    }
}
