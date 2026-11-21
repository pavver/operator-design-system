//! Live examples for VideoStream.

use crate::kit::{Example, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn video_stream() -> AnyView {
    // High-contrast offline placeholder video URL (transparent or data URL)
    let src = Signal::stored("data:video/mp4;base64,".to_string());
    let state_override = RwSignal::new(VideoStreamState::Playing);

    view! {
        <Example
            title="Live Video Stream Shell"
            note="Displays video frames with a pulsing live badge, mute control, and fullscreen."
            block=true
        >
            <div style="max-width:calc(var(--op-du)*80);">
                <VideoStream
                    src=src
                    title="Perimeter Security Cam 01"
                    state=state_override
                    live=true
                />
            </div>
        </Example>

        <Example
            title="Stream Playback States"
            note="Connecting spinner, stalled buffer recovery, and error fallback."
        >
            <div style="display:flex;gap:calc(var(--op-du)*2);margin-bottom:calc(var(--op-du)*2);">
                <ActionButton
                    text="Set Connecting"
                    on_click=instant(move || state_override.set(VideoStreamState::Connecting))
                />
                <ActionButton
                    text="Set Playing"
                    on_click=instant(move || state_override.set(VideoStreamState::Playing))
                />
                <ActionButton
                    text="Set Stalled"
                    on_click=instant(move || state_override.set(VideoStreamState::Stalled))
                />
                <ActionButton
                    text="Set Error"
                    dangerous=true
                    on_click=instant(move || state_override.set(VideoStreamState::Error("RTSP connection dropped by peer".to_string())))
                />
            </div>
        </Example>
    }
    .into_any()
}
