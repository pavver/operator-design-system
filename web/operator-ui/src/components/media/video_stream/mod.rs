//! Generic live video and camera stream shell around a `<video>` element: VideoStream.

mod controls;
mod types;

pub use types::*;

use crate::components::primitives::{Icon, ProgressIndicator};
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use controls::VideoStreamChrome;
use leptos::prelude::*;
use operator_ui_tokens::dim;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("video_stream.css");

/// Generic VideoStream shell around an HTML `<video>` element.
///
/// Features connecting, playing, stalled, and error state handling,
/// an interactive controls overlay with a live indicator, mute toggle,
/// and fullscreen support. Does not include WebRTC signalling; the application
/// provides the stream through a `src` URL or a `web_sys::MediaStream`.
#[component]
pub fn VideoStream(
    /// Video source URL (e.g. streaming endpoint, blob, or file URL).
    #[prop(optional, into)]
    src: Option<Signal<String>>,
    /// Browser `MediaStream` (from WebRTC `ontrack`, `getUserMedia`, or canvas capture).
    #[prop(optional, into)]
    stream: Option<Signal<Option<web_sys::MediaStream>>>,
    /// Explicit stream state override. If omitted, state is driven automatically by DOM media events.
    #[prop(optional, into)]
    state: Option<Signal<VideoStreamState>>,
    /// Stream title for display and accessibility.
    #[prop(optional, into)]
    title: MaybeProp<String>,
    /// Aspect ratio (width / height); defaults to 16:9 (`1.7778`).
    #[prop(optional, into)]
    ratio: MaybeProp<f32>,
    /// Maximum width in DU (`--op-video-stream-max-width`).
    #[prop(optional, into)]
    max_width: MaybeProp<f32>,
    /// Maximum height in DU (`--op-video-stream-max-height`).
    #[prop(optional, into)]
    max_height: MaybeProp<f32>,
    /// Whether audio is muted (default `true` to ensure browser autoplay policies pass).
    #[prop(default = true.into(), into)]
    muted: Signal<bool>,
    /// Autoplay flag (default `true`).
    #[prop(default = true)]
    autoplay: bool,
    /// Shows a pulsing "LIVE" badge in the controls chrome.
    #[prop(default = true)]
    live: bool,
    /// Callback invoked when a media error occurs.
    #[prop(optional)]
    on_error: Option<Callback<String>>,
    /// Custom header overlay slot (camera name, status badges, timestamps).
    #[prop(optional, into)]
    header_slot: Slot,
    /// Custom footer controls slot (extra buttons, PTZ toggle, snapshot action).
    #[prop(optional, into)]
    controls_slot: Slot,
    /// Custom full-frame overlay slot (e.g. bounding boxes, annotations).
    #[prop(optional, into)]
    overlay_slot: Slot,
    /// Custom connecting slot replacing the default spinner.
    #[prop(optional, into)]
    connecting_slot: Slot,
    /// Custom error slot replacing the default error box.
    #[prop(optional, into)]
    error_slot: Slot,
    /// Translatable connecting status message.
    #[prop(optional, into)]
    connecting_label: MaybeProp<String>,
    /// Translatable stalled status message.
    #[prop(optional, into)]
    stalled_label: MaybeProp<String>,
    /// Translatable error title.
    #[prop(optional, into)]
    error_label: MaybeProp<String>,
    /// Translatable load failure error message.
    #[prop(optional, into)]
    load_error_label: MaybeProp<String>,
    /// Translatable empty stream status message.
    #[prop(optional, into)]
    empty_label: MaybeProp<String>,
    /// Translatable live badge label.
    #[prop(optional, into)]
    live_label: MaybeProp<String>,
    /// Translatable mute button label.
    #[prop(optional, into)]
    mute_label: MaybeProp<String>,
    /// Translatable fullscreen button label.
    #[prop(optional, into)]
    fullscreen_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_title = ui_text(title, "video-stream.title", "Live stream");
    let resolved_connecting = ui_text(
        connecting_label,
        "video-stream.connecting",
        "Connecting stream...",
    );
    let resolved_stalled = ui_text(stalled_label, "video-stream.stalled", "Stream stalled...");
    let resolved_error = ui_text(error_label, "video-stream.error", "Video stream error");
    let resolved_load_error = ui_text(
        load_error_label,
        "video-stream.load-error",
        "Failed to load video stream",
    );
    let resolved_empty = ui_text(empty_label, "video-stream.empty", "No active stream");
    let resolved_live = ui_text(live_label, "video-stream.live", "LIVE");
    let resolved_mute = ui_text(mute_label, "video-stream.mute", "Mute");
    let resolved_fullscreen = ui_text(fullscreen_label, "video-stream.fullscreen", "Fullscreen");

    let video_ref = NodeRef::<leptos::html::Video>::new();
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let internal_state = RwSignal::new(VideoStreamState::Connecting);
    let is_muted = RwSignal::new(muted.get_untracked());

    // Keep internal mute state in sync if parent signal changes
    Effect::new(move |_| {
        is_muted.set(muted.get());
    });

    let current_state = Signal::derive(move || {
        if let Some(explicit) = state {
            explicit.get()
        } else {
            internal_state.get()
        }
    });

    // Attach web_sys::MediaStream directly to HTMLVideoElement when provided
    Effect::new(move |_| {
        if let Some(stream_sig) = stream {
            let stream_opt = stream_sig.get();
            if let Some(video) = video_ref.get() {
                let html_video: &web_sys::HtmlVideoElement = &video;
                if let Some(ms) = stream_opt {
                    html_video.set_src_object(Some(&ms));
                    // A clip that cannot play rejects the promise; the poster stays visible.
                    if let Ok(promise) = html_video.play() {
                        leptos::task::spawn_local(async move {
                            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                        });
                    }
                    internal_state.set(VideoStreamState::Playing);
                } else {
                    html_video.set_src_object(None);
                    internal_state.set(VideoStreamState::Idle);
                }
            }
        }
    });

    let base_style = root_style(
        vec![
            Metric::du("--op-video-stream-max-width", max_width),
            Metric::du("--op-video-stream-max-height", max_height),
        ],
        style,
    );
    let full_style = move || {
        let mut out = String::new();
        let r = ratio.get().unwrap_or(16.0 / 9.0);
        out.push_str(&format!("aspect-ratio:{r};"));
        if let Some(rest) = base_style() {
            out.push_str(&rest);
        }
        Some(out)
    };

    let toggle_fullscreen = Callback::new(move |()| {
        if let Some(container) = container_ref.get() {
            let element: &web_sys::Element = &container;
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if doc.fullscreen_element().is_some() {
                    doc.exit_fullscreen();
                } else {
                    let _ = element.request_fullscreen();
                }
            }
        }
    });

    let header_stored = StoredValue::new(header_slot);
    let controls_stored = StoredValue::new(controls_slot);

    view! {
        <div
            node_ref=container_ref
            class=root_class("op-video-stream", class)
            style=full_style
            tabindex="0"
            data-state=move || current_state.get().as_str()
            role="region"
            aria-label=move || resolved_title.get()
        >
            <video
                node_ref=video_ref
                class="op-video-stream-element"
                src=move || src.as_ref().map(|s| s.get())
                autoplay=autoplay
                playsinline=true
                muted=move || is_muted.get()
                on:loadstart=move |_| {
                    if state.is_none() {
                        internal_state.set(VideoStreamState::Connecting);
                    }
                }
                on:playing=move |_| {
                    if state.is_none() {
                        internal_state.set(VideoStreamState::Playing);
                    }
                }
                on:waiting=move |_| {
                    if state.is_none() {
                        internal_state.set(VideoStreamState::Stalled);
                    }
                }
                on:stalled=move |_| {
                    if state.is_none() {
                        internal_state.set(VideoStreamState::Stalled);
                    }
                }
                on:error=move |_| {
                    let err = resolved_load_error.get_untracked();
                    if let Some(cb) = on_error {
                        cb.run(err.clone());
                    }
                    if state.is_none() {
                        internal_state.set(VideoStreamState::Error(err));
                    }
                }
            ></video>

            // Overlays per state
            {move || match current_state.get() {
                VideoStreamState::Idle => {
                    view! {
                        <div class="op-video-stream-overlay">
                            <Icon icon=icons::VIDEOCAM size=dim::ICON_XLARGE />
                            <span class="op-video-stream-status-text" style="margin-top:calc(var(--op-du)*1.5);">
                                {move || resolved_empty.get()}
                            </span>
                        </div>
                    }.into_any()
                }
                VideoStreamState::Connecting => {
                    if connecting_slot.is_set() {
                        connecting_slot.run().into_any()
                    } else {
                        view! {
                            <div class="op-video-stream-overlay">
                                <ProgressIndicator size=8.0 />
                                <span class="op-video-stream-status-text" style="margin-top:calc(var(--op-du)*2);">{resolved_connecting.get()}</span>
                            </div>
                        }.into_any()
                    }
                }
                VideoStreamState::Stalled => {
                    view! {
                        <div class="op-video-stream-overlay">
                            <ProgressIndicator size=8.0 />
                            <span class="op-video-stream-status-text" style="margin-top:calc(var(--op-du)*2);">{resolved_stalled.get()}</span>
                        </div>
                    }.into_any()
                }
                VideoStreamState::Error(err) => {
                    if error_slot.is_set() {
                        error_slot.run().into_any()
                    } else {
                        view! {
                            <div class="op-video-stream-overlay">
                                <div class="op-video-stream-error-box">
                                    <Icon icon=icons::ERROR size=dim::ICON_XLARGE />
                                    <span class="op-video-stream-status-text">{resolved_error.get()}</span>
                                    <span class="op-video-stream-error-msg">{err}</span>
                                </div>
                            </div>
                        }.into_any()
                    }
                }
                VideoStreamState::Playing => ().into_any(),
            }}

            // Full-frame overlay slot (e.g. detections, annotations)
            {overlay_slot.is_set().then(|| view! {
                <div style="position: absolute; inset: 0; pointer-events: none; z-index: 2;">
                    {overlay_slot.run()}
                </div>
            })}

            // Controls chrome overlay
            <VideoStreamChrome
                title=resolved_title
                live=live
                live_label=resolved_live
                is_muted=is_muted
                mute_label=resolved_mute
                fullscreen_label=resolved_fullscreen
                on_toggle_fullscreen=toggle_fullscreen
                header_slot=header_stored
                controls_slot=controls_stored
            />
        </div>
    }
}
