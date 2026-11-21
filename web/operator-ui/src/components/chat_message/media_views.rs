//! Media attachment renderers: media grid, voice, audio, video note, and stickers.

use crate::components::chat_message::delivery_status::DeliveryStatusView;
use crate::components::chat_message::ttl_indicator::TtlIndicator;
use crate::components::chat_message::types::{
    AudioAttachment, ChatMediaItem, DeliveryStatus, StickerAttachment, StickerFormat,
    VideoNoteAttachment, VoiceAttachment,
};
use leptos::prelude::*;

/// Formats byte count into human-readable units.
pub fn format_bytes(bytes: u64) -> String {
    let kb = bytes as f64 / 1024.0;
    let mb = kb / 1024.0;
    if mb >= 1.0 {
        format!("{mb:.1} MB")
    } else if kb >= 1.0 {
        format!("{kb:.1} KB")
    } else {
        format!("{bytes} B")
    }
}

/// Compact multi-item photo and video media grid.
#[component]
pub fn MediaGridView(
    /// Collection of visual media attachments.
    items: Vec<ChatMediaItem>,
    /// Callback when a media item is clicked (passes media index).
    #[prop(optional, into)]
    on_media_click: Option<Callback<usize>>,
) -> impl IntoView {
    let count = items.len();
    view! {
        <div class="op-chat-media-grid" data-count=count>
            {items.into_iter().enumerate().map(|(idx, item)| {
                let has_spoiler = item.has_spoiler;
                let spoiler_revealed = RwSignal::new(!has_spoiler);
                let is_video = item.is_video;
                let duration_badge = item.duration_seconds.map(|dur| {
                    format!("{}:{:02}", dur / 60, dur % 60)
                });
                let ttl_clone = item.ttl.clone();
                let on_click = on_media_click;

                view! {
                    <div
                        class="op-chat-media-cell"
                        on:click=move |_| {
                            if has_spoiler && !spoiler_revealed.get() {
                                spoiler_revealed.set(true);
                            } else if let Some(cb) = &on_click {
                                cb.run(idx);
                            }
                        }
                    >
                        {if is_video {
                            view! {
                                <div class="op-chat-video-wrapper">
                                    <img
                                        class="op-chat-media-thumb"
                                        src=item.thumbnail_url.unwrap_or_else(|| item.url.clone())
                                        alt=item.caption.clone().unwrap_or_default()
                                    />
                                    <div class="op-chat-play-overlay">
                                        <span class="op-chat-play-icon">"▶"</span>
                                    </div>
                                    {duration_badge.map(|d| view! {
                                        <span class="op-chat-duration-badge">{d}</span>
                                    })}
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <img
                                    class="op-chat-media-thumb"
                                    src=item.url
                                    alt=item.caption.clone().unwrap_or_default()
                                />
                            }.into_any()
                        }}

                        {has_spoiler.then(|| view! {
                            <div
                                class="op-chat-media-spoiler-overlay"
                                data-revealed=move || spoiler_revealed.get().then_some("")
                            >
                                <span class="op-chat-spoiler-badge">"👁️ SPOILER"</span>
                            </div>
                        })}

                        {ttl_clone.map(|ttl| view! {
                            <div class="op-chat-media-ttl">
                                <TtlIndicator ttl=ttl />
                            </div>
                        })}
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

/// Interactive voice memo waveform visualizer.
#[component]
pub fn VoiceVisualizerView(
    /// Voice attachment payload.
    voice: VoiceAttachment,
) -> impl IntoView {
    let is_playing = RwSignal::new(false);
    let sample_waves: Vec<u8> = if voice.waveform.is_empty() {
        vec![
            6, 12, 18, 24, 16, 8, 14, 22, 28, 19, 10, 15, 25, 20, 12, 7, 14, 21, 26, 18, 11, 16,
            23, 17, 9, 13, 20, 15, 8, 12, 19, 10,
        ]
    } else {
        voice.waveform
    };

    let duration_str = format!(
        "{}:{:02}",
        voice.duration_seconds / 60,
        voice.duration_seconds % 60
    );

    view! {
        <div class="op-chat-voice-row">
            <button
                type="button"
                class="op-chat-voice-play-btn"
                on:click=move |_| is_playing.set(!is_playing.get())
                aria-label=move || if is_playing.get() { "Pause voice memo" } else { "Play voice memo" }
            >
                {move || if is_playing.get() { "⏸" } else { "▶" }}
            </button>
            <div class="op-chat-voice-waveform">
                {sample_waves.into_iter().map(|sample| {
                    let height_du = (sample as f32 / 28.0 * 6.5).clamp(1.0, 6.5);
                    let bar_style = format!("height: calc(var(--op-du) * {height_du});");
                    view! {
                        <span class="op-chat-voice-bar" style=bar_style />
                    }
                }).collect_view()}
            </div>
            <span class="op-chat-voice-duration">{duration_str}</span>
        </div>
    }
}

/// Audio track player row.
#[component]
pub fn AudioAttachmentView(
    /// Audio attachment details.
    audio: AudioAttachment,
) -> impl IntoView {
    let is_playing = RwSignal::new(false);
    let duration_label = audio
        .duration_seconds
        .map(|d| format!("{}:{:02}", d / 60, d % 60));
    let size_label = audio.file_size.map(format_bytes);

    let meta_line = match (audio.performer, duration_label, size_label) {
        (Some(p), Some(d), _) => format!("{p} • {d}"),
        (Some(p), None, Some(s)) => format!("{p} • {s}"),
        (None, Some(d), Some(s)) => format!("{d} • {s}"),
        (Some(p), None, None) => p,
        (None, Some(d), None) => d,
        (None, None, Some(s)) => s,
        _ => "Audio Track".to_string(),
    };

    view! {
        <div class="op-chat-audio-row">
            <button
                type="button"
                class="op-chat-audio-btn"
                on:click=move |_| is_playing.set(!is_playing.get())
                aria-label=move || if is_playing.get() { "Pause audio" } else { "Play audio" }
            >
                {move || if is_playing.get() { "⏸" } else { "▶" }}
            </button>
            <div class="op-chat-audio-info">
                <span class="op-chat-audio-title">{audio.title}</span>
                <span class="op-chat-audio-meta">{meta_line}</span>
            </div>
        </div>
    }
}

/// Standalone circular video note view.
#[component]
pub fn VideoNoteView(
    /// Video note payload.
    video_note: VideoNoteAttachment,
    /// Formatted time string.
    #[prop(into)]
    timestamp: String,
    /// Message direction flag.
    #[prop(optional)]
    is_outgoing: bool,
    /// Delivery status state.
    #[prop(optional, into)]
    delivery_status: MaybeProp<DeliveryStatus>,
    /// Optional click callback.
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
) -> impl IntoView {
    let dur_text = format!(
        "{}:{:02}",
        video_note.duration_seconds / 60,
        video_note.duration_seconds % 60
    );

    view! {
        <div class="op-chat-video-note-container" data-sent=is_outgoing.then_some("")>
            <div
                class="op-chat-video-note-circle"
                on:click=move |_| {
                    if let Some(cb) = &on_click {
                        cb.run(());
                    }
                }
            >
                {video_note.thumbnail_url.map(|thumb| view! {
                    <img class="op-chat-video-note-thumb" src=thumb alt="Video note poster" />
                })}
                <span class="op-chat-video-note-play">"▶"</span>
                <span class="op-chat-video-note-duration">{dur_text}</span>
            </div>
            <div class="op-chat-video-note-badge">
                <span class="op-chat-message-time">{timestamp}</span>
                {is_outgoing.then(|| {
                    delivery_status.get().map(|st| view! {
                        <DeliveryStatusView status=st />
                    })
                })}
            </div>
        </div>
    }
}

/// Standalone or embedded sticker graphic view.
#[component]
pub fn StickerView(
    /// Sticker attachment payload.
    sticker: StickerAttachment,
    /// Formatted timestamp.
    #[prop(optional, into)]
    timestamp: MaybeProp<String>,
    /// Outgoing flag.
    #[prop(optional)]
    is_outgoing: bool,
    /// Delivery status.
    #[prop(optional, into)]
    delivery_status: MaybeProp<DeliveryStatus>,
) -> impl IntoView {
    view! {
        <div class="op-chat-sticker-container" data-sent=is_outgoing.then_some("")>
            <div class="op-chat-sticker-wrapper">
                {match sticker.format {
                    StickerFormat::Raster | StickerFormat::Animated => view! {
                        <img
                            class="op-chat-sticker-img"
                            src=sticker.url
                            alt=sticker.emoji.clone().unwrap_or_else(|| "Sticker".to_string())
                        />
                    }.into_any(),
                    StickerFormat::VectorSvg => view! {
                        <div
                            class="op-chat-sticker-svg"
                            aria-label=sticker.emoji.clone().unwrap_or_else(|| "Sticker".to_string())
                        >
                            <img class="op-chat-sticker-img" src=sticker.url alt="Sticker" />
                        </div>
                    }.into_any(),
                }}
            </div>
            {timestamp.get().map(|time| view! {
                <div class="op-chat-sticker-badge">
                    <span class="op-chat-message-time">{time}</span>
                    {is_outgoing.then(|| {
                        delivery_status.get().map(|st| view! {
                            <DeliveryStatusView status=st />
                        })
                    })}
                </div>
            })}
        </div>
    }
}
