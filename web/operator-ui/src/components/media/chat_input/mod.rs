//! Multi-line auto-growing chat input bar with attachments, reply/edit preview, and record switcher.

use crate::components::chat_message::MessageReply;
use crate::components::media_record_button::{MediaRecordButton, MediaRecordMode};
use crate::components::primitives::Icon;
use crate::core::{ControlState, Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::icons;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlTextAreaElement};

pub(crate) const CSS: &str = include_str!("chat_input.css");

/// High-density chat input component with auto-growing textarea,
/// quoted message reply preview, attachments slot, send button with Operator async states,
/// and smooth switching between text sending and media voice/video recording.
#[component]
pub fn ChatInput(
    /// Text value binding of the input.
    #[prop(into)]
    value: Signal<String>,
    /// Callback when user changes the input text.
    on_input: Callback<String>,
    /// Callback when user sends the message.
    on_send: Callback<()>,
    /// Quoted message reply reference.
    #[prop(optional, into)]
    reply: MaybeProp<Option<MessageReply>>,
    /// Whether the message is an edit of an existing message.
    #[prop(optional, into)]
    is_editing: MaybeProp<bool>,
    /// Callback when user cancels reply or edit bar.
    #[prop(optional)]
    on_cancel_reply: Option<Callback<()>>,
    /// Callback when user clicks the attachments button.
    #[prop(optional)]
    on_attach: Option<Callback<()>>,
    /// Callback when user toggles the emoji/stickers media keyboard.
    #[prop(optional)]
    on_toggle_media: Option<Callback<()>>,
    /// Current operator state for the send button (Idle, Pending, Success, Error).
    #[prop(optional, into)]
    state: MaybeProp<ControlState>,
    /// Media recording mode (Voice audio or Video note).
    #[prop(optional, into)]
    media_mode: MaybeProp<MediaRecordMode>,
    /// Whether media recording is currently active.
    #[prop(optional, into)]
    recording: MaybeProp<bool>,
    /// Whether media recording is locked into hands-free mode.
    #[prop(optional, into)]
    recording_locked: MaybeProp<bool>,
    /// Callback when recording starts.
    #[prop(optional, into)]
    on_record_start: Option<Callback<MediaRecordMode>>,
    /// Callback when recording concludes.
    #[prop(optional, into)]
    on_record_stop: Option<Callback<()>>,
    /// Callback when recording is cancelled.
    #[prop(optional, into)]
    on_record_cancel: Option<Callback<()>>,
    /// Callback when recording is locked.
    #[prop(optional, into)]
    on_record_locked: Option<Callback<()>>,
    /// Callback when media record mode switches between voice and video note.
    #[prop(optional, into)]
    on_record_mode_change: Option<Callback<MediaRecordMode>>,
    /// Custom placeholder text.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Minimum height in DU (`--op-chat-input-min-height`). Defaults to 9 DU.
    #[prop(optional, into)]
    min_height: MaybeProp<f32>,
    /// Maximum height in DU (`--op-chat-input-max-height`). Defaults to 29 DU.
    #[prop(optional, into)]
    max_height: MaybeProp<f32>,
    /// Font size in TU (`--op-chat-input-font-size`). Defaults to 13 TU.
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    /// Slot rendered before the input (attachments, emoji button).
    #[prop(optional, into)]
    leading: Slot,
    /// Slot rendered after the input (send / record button).
    #[prop(optional, into)]
    trailing: Slot,
    /// Custom attachments button slot.
    #[prop(optional, into)]
    attachments: Slot,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline CSS styles.
    #[prop(optional, into)]
    style: MaybeProp<String>,
) -> impl IntoView {
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();

    let default_ph = ui_text(placeholder, "chat-input.placeholder", "Type a message...");
    let send_label = ui_text(None::<String>.into(), "chat-input.send", "Send");
    let attach_label = ui_text(None::<String>.into(), "chat-input.attach", "Attach");
    let media_label = ui_text(None::<String>.into(), "chat-input.media", "Media");
    let cancel_label = ui_text(None::<String>.into(), "chat-input.cancel", "Cancel");
    let edit_label = ui_text(
        None::<String>.into(),
        "chat-input.editing",
        "Editing message",
    );

    let send_icon = ui_icon(None, "chat-input.send-icon", icons::ARROW_UPWARD);
    let attach_icon = ui_icon(None, "chat-input.attach-icon", icons::ADD);
    let media_icon = ui_icon(None, "chat-input.media-icon", icons::SETTINGS);
    let close_icon = ui_icon(None, "chat-input.close-icon", icons::CLOSE);

    let metrics = vec![
        Metric::du("--op-chat-input-min-height", min_height),
        Metric::du("--op-chat-input-max-height", max_height),
        Metric::tu("--op-chat-input-font-size", font_size),
    ];
    let root_cls = root_class("op-chat-input-container", class);
    let root_stl = root_style(metrics, style);

    let current_state = Signal::derive(move || state.get().unwrap_or(ControlState::Idle));

    // Auto-grow height adjustment
    let adjust_height = move || {
        if let Some(el) = textarea_ref.get() {
            let raw: &HtmlTextAreaElement = el.unchecked_ref();
            let html_el: &HtmlElement = raw.unchecked_ref();
            let style = html_el.style();
            let _ = style.set_property("height", "auto");
            let scroll_h = raw.scroll_height();
            let _ = style.set_property("height", &format!("{scroll_h}px"));
        }
    };

    let on_textarea_input = move |ev: ev::Event| {
        let val = event_target_value(&ev);
        on_input.run(val);
        adjust_height();
    };

    let on_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();
            if !value.get().trim().is_empty() {
                on_send.run(());
                adjust_height();
            }
        }
    };

    view! {
        <div class=root_cls style=root_stl>
            // Reply / Edit preview header bar
            {move || {
                let editing = is_editing.get().unwrap_or(false);
                let reply_info = reply.get().flatten();

                if editing || reply_info.is_some() {
                    view! {
                        <div class="op-chat-input__reply-bar">
                            <div class="op-chat-input__reply-content">
                                <span class="op-chat-input__reply-title">
                                    {if editing {
                                        edit_label.get()
                                    } else if let Some(ref r) = reply_info {
                                        r.sender_name.clone()
                                    } else {
                                        String::new()
                                    }}
                                </span>
                                <span class="op-chat-input__reply-text">
                                    {if let Some(ref r) = reply_info {
                                        r.excerpt_text.clone()
                                    } else {
                                        String::new()
                                    }}
                                </span>
                            </div>

                            {on_cancel_reply.map(|cb| {
                                view! {
                                    <button
                                        type="button"
                                        class="op-chat-input__reply-cancel"
                                        on:click=move |_| cb.run(())
                                        aria-label=move || cancel_label.get()
                                    >
                                        <Icon icon=close_icon.get() />
                                    </button>
                                }
                            })}
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}

            <div class="op-chat-input">
                {leading.run()}

                // Attachments button slot or default attach button
                {if attachments.is_set() {
                    attachments.run().into_any()
                } else {
                    view! {
                        <button
                            type="button"
                            class="op-chat-input__btn"
                            on:click=move |_| {
                                if let Some(cb) = on_attach {
                                    cb.run(());
                                }
                            }
                            aria-label=move || attach_label.get()
                        >
                            <Icon icon=attach_icon.get() />
                        </button>
                    }.into_any()
                }}

                // Emoji / stickers keyboard button
                <button
                    type="button"
                    class="op-chat-input__btn"
                    on:click=move |_| {
                        if let Some(cb) = on_toggle_media {
                            cb.run(());
                        }
                    }
                    aria-label=move || media_label.get()
                >
                    <Icon icon=media_icon.get() />
                </button>

                // Auto-growing textarea
                <textarea
                    node_ref=textarea_ref
                    class="op-chat-input__textarea"
                    placeholder=move || default_ph.get()
                    prop:value=move || value.get()
                    on:input=on_textarea_input
                    on:keydown=on_keydown
                    rows="1"
                />

                {trailing.run()}

                // Trailing action: Send button when text is present, or MediaRecordButton when empty
                {move || {
                    let text = value.get();
                    let has_text = !text.trim().is_empty();

                    if has_text {
                        view! {
                            <button
                                type="button"
                                class="op-chat-input__send-btn"
                                data-state=move || current_state.get().as_str()
                                on:click=move |_| on_send.run(())
                                aria-label=move || send_label.get()
                            >
                                <Icon icon=send_icon.get() />
                            </button>
                        }.into_any()
                    } else {
                        view! {
                            <MediaRecordButton
                                mode=media_mode
                                recording=recording
                                locked=recording_locked
                                on_record_start=Callback::new(move |m| {
                                    if let Some(cb) = on_record_start {
                                        cb.run(m);
                                    }
                                })
                                on_record_stop=Callback::new(move |()| {
                                    if let Some(cb) = on_record_stop {
                                        cb.run(());
                                    }
                                })
                                on_record_cancel=Callback::new(move |()| {
                                    if let Some(cb) = on_record_cancel {
                                        cb.run(());
                                    }
                                })
                                on_record_locked=Callback::new(move |()| {
                                    if let Some(cb) = on_record_locked {
                                        cb.run(());
                                    }
                                })
                                on_mode_change=Callback::new(move |m| {
                                    if let Some(cb) = on_record_mode_change {
                                        cb.run(m);
                                    }
                                })
                            />
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
