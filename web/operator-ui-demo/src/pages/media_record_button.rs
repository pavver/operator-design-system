//! Live examples for MediaRecordButton (hold-to-record voice & video notes).

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn media_record_button() -> AnyView {
    let mode_1 = RwSignal::new(MediaRecordMode::Voice);
    let is_recording_1 = RwSignal::new(false);
    let is_locked_1 = RwSignal::new(false);
    let status_text = RwSignal::new(String::from("Idle (click to switch mode, hold to record)"));

    let mode_2 = RwSignal::new(MediaRecordMode::Video);
    let is_recording_2 = RwSignal::new(false);
    let is_locked_2 = RwSignal::new(false);

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 120);">
            <Example
                title="Hold To Record & Slide Gestures"
                note="Tap to toggle Voice/Video mode. Hold to record, slide left to cancel, slide up to lock hands-free."
                block=true
            >
                <div style="display:flex;flex-direction:column;align-items:center;gap:calc(var(--op-du) * 3);padding:calc(var(--op-du) * 4);width:100%;">
                    <div style="font-family:monospace;font-size:calc(var(--op-tu) * 12);color:var(--op-text-secondary);">
                        {move || status_text.get()}
                    </div>

                    <MediaRecordButton
                        mode=mode_1
                        recording=is_recording_1
                        locked=is_locked_1
                        on_mode_change=Callback::new(move |m| {
                            mode_1.set(m);
                            status_text.set(format!("Switched mode to {:?}", m));
                        })
                        on_record_start=Callback::new(move |m| {
                            is_recording_1.set(true);
                            is_locked_1.set(false);
                            status_text.set(format!("Recording {:?} started...", m));
                        })
                        on_record_stop=Callback::new(move |()| {
                            is_recording_1.set(false);
                            is_locked_1.set(false);
                            status_text.set("Recording completed & saved".to_string());
                        })
                        on_record_cancel=Callback::new(move |()| {
                            is_recording_1.set(false);
                            is_locked_1.set(false);
                            status_text.set("Recording cancelled".to_string());
                        })
                        on_record_locked=Callback::new(move |()| {
                            is_recording_1.set(true);
                            is_locked_1.set(true);
                            status_text.set("Recording locked (hands-free)".to_string());
                        })
                    />
                </div>
            </Example>

            <Example
                title="Circular Exception & Video Note Capture"
                note="Circular round styling option for messaging applications."
                block=true
            >
                <div style="display:flex;gap:calc(var(--op-du) * 4);align-items:center;justify-content:center;padding:calc(var(--op-du) * 4);flex-wrap:wrap;">
                    <MediaRecordButton
                        circular=true
                        mode=mode_2
                        recording=is_recording_2
                        locked=is_locked_2
                        on_mode_change=Callback::new(move |m| mode_2.set(m))
                        on_record_start=Callback::new(move |_| is_recording_2.set(true))
                        on_record_stop=Callback::new(move |()| {
                            is_recording_2.set(false);
                            is_locked_2.set(false);
                        })
                        on_record_cancel=Callback::new(move |()| {
                            is_recording_2.set(false);
                            is_locked_2.set(false);
                        })
                        on_record_locked=Callback::new(move |()| {
                            is_recording_2.set(true);
                            is_locked_2.set(true);
                        })
                    />

                    <MediaRecordButton
                        locked=true
                        mode=MediaRecordMode::Voice
                    />

                    <MediaRecordButton
                        disabled=true
                        mode=MediaRecordMode::Voice
                    />
                </div>
            </Example>
        </div>
    }.into_any()
}
