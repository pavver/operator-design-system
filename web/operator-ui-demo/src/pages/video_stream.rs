//! Live examples for VideoStream.

use super::demo_video;
use crate::kit::{Example, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn video_stream() -> AnyView {
    // Live animated stream drawn from a canvas via canvas.captureStream() with requestAnimationFrame
    let stream_signal = Signal::stored(demo_video::create_animated_canvas_stream(640, 360));
    let state_override = RwSignal::new(VideoStreamState::Connecting);

    let failed_src = Signal::stored("https://invalid.stream.internal/offline-feed".to_string());

    view! {
        <Example
            title="Live Video Stream Shell"
            note="Displays live animated stream from a canvas (canvas.captureStream() as MediaStream) with playing state and pulsing live badge."
            block=true
        >
            <div style="max-width:calc(var(--op-du)*80);">
                <VideoStream
                    stream=stream_signal
                    title="Perimeter Security Cam 01"
                    live=true
                />
            </div>
        </Example>

        <Example
            title="Failed Stream Error Handling"
            note="When a stream fails to load or the connection is refused, VideoStream automatically enters its error state."
            block=true
        >
            <div style="max-width:calc(var(--op-du)*80);">
                <VideoStream
                    src=failed_src
                    title="Offline Perimeter Sensor 09"
                    live=false
                />
            </div>
        </Example>

        <Example
            title="Manual Stream Playback States"
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

            <div style="max-width:calc(var(--op-du)*80);">
                <VideoStream
                    state=state_override
                    title="Diagnostic Test Stream"
                    live=true
                />
            </div>
        </Example>
    }
    .into_any()
}
