//! Document and interactive card renderers: file, contact, poll, webpage, invoice, story, code block, and blockquote.

use crate::components::chat_message::markdown::SafeMarkdown;
use crate::components::chat_message::media_views::{
    AudioAttachmentView, MediaGridView, StickerView, VideoNoteView, VoiceVisualizerView,
    format_bytes,
};
use crate::components::chat_message::types::{
    ContactAttachment, FileAttachment, InvoiceAttachment, MessageContent, PollAttachment,
    StoryAttachment, WebPagePreview,
};
use crate::components::primitives::Icon;
use leptos::prelude::*;
use operator_ui_tokens::icons;

/// Document or file attachment row.
#[component]
pub fn FileAttachmentView(
    /// File attachment metadata.
    file: FileAttachment,
    /// Click action callback.
    #[prop(optional, into)]
    on_click: MaybeProp<Callback<()>>,
) -> impl IntoView {
    let size_label = file
        .file_size
        .map(format_bytes)
        .or(file.mime_type)
        .unwrap_or_else(|| "Document".to_string());

    view! {
        <div
            class="op-chat-file-row"
            on:click=move |_| {
                if let Some(cb) = on_click.get() {
                    cb.run(());
                }
            }
            role="button"
            tabindex="0"
        >
            <div class="op-chat-file-icon-box" aria-hidden="true">
                <span class="op-chat-file-glyph">"📄"</span>
            </div>
            <div class="op-chat-file-info">
                <span class="op-chat-file-name">{file.file_name}</span>
                <span class="op-chat-file-meta">{size_label}</span>
            </div>
        </div>
    }
}

/// Contact card row.
#[component]
pub fn ContactAttachmentView(
    /// Contact details.
    contact: ContactAttachment,
) -> impl IntoView {
    view! {
        <div class="op-chat-contact-row">
            <div class="op-chat-contact-avatar-box" aria-hidden="true">
                <span class="op-chat-contact-glyph">"👤"</span>
            </div>
            <div class="op-chat-contact-info">
                <span class="op-chat-contact-name">{contact.name}</span>
                <span class="op-chat-contact-phone">{contact.phone_number}</span>
            </div>
        </div>
    }
}

/// Interactive voting poll card.
#[component]
pub fn PollAttachmentView(
    /// Poll data payload.
    poll: PollAttachment,
    /// Optional vote selection callback (passes selected option ID).
    #[prop(optional, into)]
    on_vote: MaybeProp<Callback<String>>,
) -> impl IntoView {
    let total_votes = poll.total_votes;
    let is_anonymous = poll.is_anonymous;

    view! {
        <div class="op-chat-poll-card">
            <div class="op-chat-poll-header">
                <span class="op-chat-poll-glyph" aria-hidden="true">"📊"</span>
                <span class="op-chat-poll-question">{poll.question}</span>
            </div>
            <div class="op-chat-poll-options">
                {poll.options.into_iter().map(|opt| {
                    let opt_id = opt.id.clone();
                    let on_vote_cb = on_vote.get();
                    let percentage = if total_votes > 0 {
                        (opt.vote_count as f32 / total_votes as f32 * 100.0) as u32
                    } else {
                        0
                    };
                    let bar_width = format!("width: {percentage}%;");

                    view! {
                        <div
                            class="op-chat-poll-option"
                            data-selected=opt.is_selected.then_some("")
                            on:click=move |_| {
                                if let Some(cb) = &on_vote_cb {
                                    cb.run(opt_id.clone());
                                }
                            }
                        >
                            <div class="op-chat-poll-opt-labels">
                                <span class="op-chat-poll-opt-text">{opt.text}</span>
                                <span class="op-chat-poll-opt-percent">{format!("{percentage}%")}</span>
                            </div>
                            <div class="op-chat-poll-progress-track">
                                <div class="op-chat-poll-progress-bar" style=bar_width />
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
            <div class="op-chat-poll-footer">
                <span class="op-chat-poll-meta">
                    {format!("{total_votes} votes • {}", if is_anonymous { "Anonymous" } else { "Public" })}
                </span>
            </div>
        </div>
    }
}

/// Web page preview card.
#[component]
pub fn WebPagePreviewView(
    /// Preview card data.
    web_page: WebPagePreview,
    /// Optional URL click callback.
    #[prop(optional, into)]
    on_click: MaybeProp<Callback<String>>,
) -> impl IntoView {
    let target_url = web_page.url.clone();
    let on_click_cb = on_click.get();

    view! {
        <div
            class="op-chat-webpage-card"
            on:click=move |_| {
                if let Some(cb) = &on_click_cb {
                    cb.run(target_url.clone());
                }
            }
        >
            {web_page.preview_image_url.map(|img_url| view! {
                <img class="op-chat-webpage-thumb" src=img_url alt="Link preview image" />
            })}
            <div class="op-chat-webpage-body">
                <div class="op-chat-webpage-bar" />
                <div class="op-chat-webpage-content">
                    {web_page.site_name.map(|site| view! {
                        <span class="op-chat-webpage-site">{site.to_uppercase()}</span>
                    })}
                    {web_page.title.map(|title| view! {
                        <span class="op-chat-webpage-title">{title}</span>
                    })}
                    {web_page.description.map(|desc| view! {
                        <span class="op-chat-webpage-desc">{desc}</span>
                    })}
                </div>
                <div class="op-chat-webpage-open-icon">
                    <Icon icon=icons::OPEN_IN_NEW size=3.5 />
                </div>
            </div>
        </div>
    }
}

/// Tactical payment invoice card.
#[component]
pub fn InvoiceAttachmentView(
    /// Invoice payload.
    invoice: InvoiceAttachment,
    /// Optional pay button callback.
    #[prop(optional, into)]
    on_pay: MaybeProp<Callback<String>>,
) -> impl IntoView {
    let inv_id = invoice.invoice_id.clone();
    let is_paid = invoice.is_paid;
    let on_pay_cb = on_pay.get();

    view! {
        <div class="op-chat-invoice-card">
            {invoice.photo_url.map(|img| view! {
                <img class="op-chat-invoice-photo" src=img alt=invoice.title.clone() />
            })}
            <div class="op-chat-invoice-body">
                <div class="op-chat-invoice-header">
                    <span class="op-chat-invoice-badge">"INVOICE"</span>
                    <span class="op-chat-invoice-price">
                        {format!("{} {}", invoice.currency, invoice.total_amount)}
                    </span>
                </div>
                <div class="op-chat-invoice-title">{invoice.title}</div>
                {(!invoice.description.is_empty()).then(|| view! {
                    <div class="op-chat-invoice-desc">{invoice.description}</div>
                })}
                <div class="op-chat-invoice-action-wrap">
                    {if is_paid {
                        view! {
                            <div class="op-chat-invoice-paid-badge">
                                "✓ PAID"
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <button
                                type="button"
                                class="op-chat-invoice-pay-btn"
                                on:click=move |_| {
                                    if let Some(cb) = &on_pay_cb {
                                        cb.run(inv_id.clone());
                                    }
                                }
                            >
                                "PAY"
                            </button>
                        }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}

/// Ephemeral story card.
#[component]
pub fn StoryAttachmentView(
    /// Story data.
    story: StoryAttachment,
    /// Optional story click callback.
    #[prop(optional, into)]
    on_click: MaybeProp<Callback<String>>,
) -> impl IntoView {
    let story_id = story.story_id.clone();
    let on_click_cb = on_click.get();

    view! {
        <div
            class="op-chat-story-card"
            on:click=move |_| {
                if let Some(cb) = &on_click_cb {
                    cb.run(story_id.clone());
                }
            }
        >
            <img class="op-chat-story-media" src=story.media_url alt="Story background" />
            <div class="op-chat-story-overlay" />
            <div class="op-chat-story-header">
                {if let Some(avatar) = story.author_avatar_url {
                    view! {
                        <img class="op-chat-story-avatar" src=avatar alt=story.author_name.clone() />
                    }.into_any()
                } else {
                    let initial = story.author_name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_else(|| "S".to_string());
                    view! {
                        <span class="op-chat-story-avatar-placeholder">{initial}</span>
                    }.into_any()
                }}
                <div class="op-chat-story-meta">
                    <span class="op-chat-story-author">{story.author_name}</span>
                    <span class="op-chat-story-tag">"STORY"</span>
                </div>
            </div>
            {story.is_video.then(|| view! {
                <div class="op-chat-story-play-badge">
                    <span>"▶"</span>
                </div>
            })}
        </div>
    }
}

/// Monospaced code block with language header and copy button.
#[component]
pub fn CodeBlockView(
    /// Source programming language tag.
    language: String,
    /// Code text.
    code: String,
) -> impl IntoView {
    let is_copied = RwSignal::new(false);
    let code_for_copy = code.clone();

    view! {
        <div class="op-chat-code-block">
            <div class="op-chat-code-header">
                <span class="op-chat-code-lang">{language.to_uppercase()}</span>
                <button
                    type="button"
                    class="op-chat-code-copy-btn"
                    on:click=move |_| {
                        is_copied.set(true);
                        // In browser context clipboard write can be triggered or feedback provided
                        #[cfg(target_arch = "wasm32")]
                        {
                            if let Some(window) = web_sys::window() {
                                let nav = window.navigator();
                                let clipboard = nav.clipboard();
                                let _ = clipboard.write_text(&code_for_copy);
                            }
                        }
                    }
                    aria-label="Copy code to clipboard"
                >
                    <Icon icon=if is_copied.get() { icons::CHECK } else { icons::CONTENT_COPY } size=3.5 />
                    <span>{move || if is_copied.get() { "COPIED" } else { "COPY" }}</span>
                </button>
            </div>
            <pre class="op-chat-code-pre">
                <code>{code}</code>
            </pre>
        </div>
    }
}

/// Rich blockquote with accent bar and collapsible toggle.
#[component]
pub fn BlockquoteView(
    /// Quoted text.
    text: String,
    /// Whether blockquote can be folded.
    #[prop(optional)]
    is_collapsible: bool,
) -> impl IntoView {
    let is_expanded = RwSignal::new(!is_collapsible);

    view! {
        <blockquote class="op-chat-blockquote" data-collapsed=move || (is_collapsible && !is_expanded.get()).then_some("")>
            <div class="op-chat-blockquote-bar" />
            <div class="op-chat-blockquote-body">
                <div class="op-chat-blockquote-text">
                    <SafeMarkdown text=text />
                </div>
                {is_collapsible.then(|| view! {
                    <button
                        type="button"
                        class="op-chat-blockquote-toggle"
                        on:click=move |_| is_expanded.set(!is_expanded.get())
                    >
                        <span>{move || if is_expanded.get() { "COLLAPSE" } else { "EXPAND QUOTE" }}</span>
                    </button>
                })}
            </div>
        </blockquote>
    }
}

/// Helper to render an attachment content variant.
#[component]
pub fn RenderMessageContent(
    content: MessageContent,
    #[prop(optional, into)] on_poll_vote: MaybeProp<Callback<String>>,
) -> impl IntoView {
    match content {
        MessageContent::Text(txt) => view! {
            <SafeMarkdown text=txt />
        }
        .into_any(),
        MessageContent::CodeBlock { language, code } => view! {
            <CodeBlockView language=language code=code />
        }
        .into_any(),
        MessageContent::Blockquote {
            text,
            is_collapsible,
        } => view! {
            <BlockquoteView text=text is_collapsible=is_collapsible />
        }
        .into_any(),
        MessageContent::MediaGrid(items) => view! {
            <MediaGridView items=items />
        }
        .into_any(),
        MessageContent::File(file) => view! {
            <FileAttachmentView file=file />
        }
        .into_any(),
        MessageContent::Audio(audio) => view! {
            <AudioAttachmentView audio=audio />
        }
        .into_any(),
        MessageContent::Voice(voice) => view! {
            <VoiceVisualizerView voice=voice />
        }
        .into_any(),
        MessageContent::VideoNote(vn) => view! {
            <VideoNoteView video_note=vn timestamp="".to_string() />
        }
        .into_any(),
        MessageContent::Sticker(st) => view! {
            <StickerView sticker=st />
        }
        .into_any(),
        MessageContent::Contact(contact) => view! {
            <ContactAttachmentView contact=contact />
        }
        .into_any(),
        MessageContent::Poll(poll) => view! {
            <PollAttachmentView poll=poll on_vote=on_poll_vote />
        }
        .into_any(),
        MessageContent::WebPage(wp) => view! {
            <WebPagePreviewView web_page=wp />
        }
        .into_any(),
        MessageContent::Invoice(inv) => view! {
            <InvoiceAttachmentView invoice=inv />
        }
        .into_any(),
        MessageContent::Story(story) => view! {
            <StoryAttachmentView story=story />
        }
        .into_any(),
        MessageContent::Mixed { attachments, text } => view! {
            <div class="op-chat-mixed-content">
                {attachments.into_iter().map(|att| view! {
                    <RenderMessageContent content=att on_poll_vote=on_poll_vote />
                }).collect_view()}
                {text.map(|t| view! {
                    <SafeMarkdown text=t />
                })}
            </div>
        }
        .into_any(),
    }
}
