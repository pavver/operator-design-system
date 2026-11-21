//! Media record button supporting hold-to-record, drag-left-to-cancel, drag-up-to-lock, and mode toggle.

use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("media_record_button.css");

/// Drag distances in pixels.
const DRAG_LOCK_THRESHOLD_PX: f64 = 60.0;
const DRAG_CANCEL_THRESHOLD_PX: f64 = 60.0;
const HOLD_TIMEOUT_MS: i32 = 200;

/// Media recording mode: voice audio note or round video note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaRecordMode {
    #[default]
    Voice,
    Video,
}

impl MediaRecordMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Voice => "voice",
            Self::Video => "video",
        }
    }
}

/// Tactical media record button that captures audio/video notes.
///
/// Reports user recording intents (start, lock, cancel, stop, mode switch) to the host app.
/// The host app is responsible for the actual media capture hardware.
#[component]
pub fn MediaRecordButton(
    /// Current recording mode (Voice audio or Video note).
    #[prop(optional, into)]
    mode: MaybeProp<MediaRecordMode>,
    /// Whether recording is actively in progress.
    #[prop(optional, into)]
    recording: MaybeProp<bool>,
    /// Whether recording is locked for hands-free capture.
    #[prop(optional, into)]
    locked: MaybeProp<bool>,
    /// Disables interaction.
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// Render in circular shape (round exception); defaults to false (square geometry).
    #[prop(optional, into)]
    circular: MaybeProp<bool>,
    /// Size in Design Units (`--op-media-record-button-size`). Default is 12 DU.
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline CSS styles.
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// Callback when user toggles mode by quick click.
    #[prop(optional, into)]
    on_mode_change: Option<Callback<MediaRecordMode>>,
    /// Callback when hold duration is reached and recording should start.
    #[prop(optional, into)]
    on_record_start: Option<Callback<MediaRecordMode>>,
    /// Callback when recording is finalized / completed.
    #[prop(optional, into)]
    on_record_stop: Option<Callback<()>>,
    /// Callback when recording is cancelled by sliding left.
    #[prop(optional, into)]
    on_record_cancel: Option<Callback<()>>,
    /// Callback when recording is locked into hands-free mode by sliding up.
    #[prop(optional, into)]
    on_record_locked: Option<Callback<()>>,
) -> impl IntoView {
    let start_x = RwSignal::new(0.0_f64);
    let start_y = RwSignal::new(0.0_f64);
    let pointer_down_time = RwSignal::new(0.0_f64);
    let is_holding = RwSignal::new(false);
    let is_canceling = RwSignal::new(false);
    let timer_seconds = RwSignal::new(0_u32);

    let current_mode = Signal::derive(move || mode.get().unwrap_or_default());
    let is_recording = Signal::derive(move || recording.get().unwrap_or(false));
    let is_locked = Signal::derive(move || locked.get().unwrap_or(false));
    let is_disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let is_circular = Signal::derive(move || circular.get().unwrap_or(false));

    // Timer interval during recording
    Effect::new(move |_| {
        if is_recording.get() {
            timer_seconds.set(0);
            let timer = set_interval_with_handle(
                move || {
                    timer_seconds.update(|s| *s += 1);
                },
                std::time::Duration::from_secs(1),
            );
            on_cleanup(move || {
                if let Ok(t) = timer {
                    t.clear();
                }
            });
        } else {
            timer_seconds.set(0);
        }
    });

    let voice_label = ui_text(None::<String>.into(), "media-record.audio", "Voice note");
    let video_label = ui_text(None::<String>.into(), "media-record.video", "Video note");
    let locked_label = ui_text(
        None::<String>.into(),
        "media-record.locked-label",
        "Locked recording (click to stop)",
    );
    let cancel_hint = ui_text(
        None::<String>.into(),
        "media-record.cancel-hint",
        "< Slide to cancel",
    );
    let lock_hint = ui_text(
        None::<String>.into(),
        "media-record.lock-hint",
        "^ Slide to lock",
    );

    let icon_voice = ui_icon(None, "media-record.voice", icons::ARROW_UPWARD);
    let icon_video = ui_icon(None, "media-record.video", icons::VIDEOCAM);
    let icon_lock = ui_icon(None, "media-record.lock", icons::LOCK);

    let on_pointerdown = move |ev: ev::PointerEvent| {
        if is_disabled.get() {
            return;
        }
        if is_locked.get() {
            // Tap while locked concludes recording
            if let Some(cb) = on_record_stop {
                cb.run(());
            }
            return;
        }
        start_x.set(f64::from(ev.client_x()));
        start_y.set(f64::from(ev.client_y()));
        pointer_down_time.set(js_sys::Date::now());
        is_holding.set(true);
        is_canceling.set(false);

        // Schedule transition to recording if held
        let mode_val = current_mode.get();
        set_timeout(
            move || {
                if let (true, false, Some(cb)) =
                    (is_holding.get(), is_locked.get(), on_record_start)
                {
                    cb.run(mode_val);
                }
            },
            std::time::Duration::from_millis(HOLD_TIMEOUT_MS as u64),
        );
    };

    let on_pointermove = move |ev: ev::PointerEvent| {
        if !is_holding.get() || is_locked.get() || is_disabled.get() {
            return;
        }
        let cur_x = f64::from(ev.client_x());
        let cur_y = f64::from(ev.client_y());
        let delta_x = start_x.get() - cur_x;
        let delta_y = start_y.get() - cur_y;

        // Slide left to cancel
        if delta_x > DRAG_CANCEL_THRESHOLD_PX {
            is_canceling.set(true);
            is_holding.set(false);
            if let Some(cb) = on_record_cancel {
                cb.run(());
            }
            return;
        }

        // Slide up to lock hands-free
        if delta_y > DRAG_LOCK_THRESHOLD_PX {
            is_holding.set(false);
            if let Some(cb) = on_record_locked {
                cb.run(());
            }
        }
    };

    let on_pointerup = move |_ev: ev::PointerEvent| {
        if is_disabled.get() {
            return;
        }
        let elapsed = js_sys::Date::now() - pointer_down_time.get();
        if is_canceling.get() {
            is_canceling.set(false);
            is_holding.set(false);
            return;
        }
        if is_holding.get() {
            is_holding.set(false);
            if elapsed < f64::from(HOLD_TIMEOUT_MS) && !is_recording.get() {
                // Short tap toggles mode
                let next_mode = match current_mode.get() {
                    MediaRecordMode::Voice => MediaRecordMode::Video,
                    MediaRecordMode::Video => MediaRecordMode::Voice,
                };
                if let Some(cb) = on_mode_change {
                    cb.run(next_mode);
                }
            } else if !is_locked.get() {
                // Released without locking: stop recording
                if let Some(cb) = on_record_stop {
                    cb.run(());
                }
            }
        }
    };

    let on_pointercancel = move |_ev: ev::PointerEvent| {
        if is_holding.get() && !is_locked.get() {
            is_holding.set(false);
            if let Some(cb) = on_record_cancel {
                cb.run(());
            }
        }
    };

    let metrics = vec![Metric::du("--op-media-record-button-size", size)];
    let root_cls = root_class("op-media-record-button", class);
    let root_stl = root_style(metrics, style);

    let format_time = move |secs: u32| {
        let m = secs / 60;
        let s = secs % 60;
        format!("{m}:{s:02}")
    };

    view! {
        <div class="op-media-record-container">
            {move || {
                if is_recording.get() || is_locked.get() {
                    view! {
                        <div class="op-media-record-timer">
                            <span class="op-media-record-timer__dot"></span>
                            <span>{move || format_time(timer_seconds.get())}</span>
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}

            {move || {
                if is_recording.get() && !is_locked.get() {
                    view! {
                        <span class="op-media-record-hint">{cancel_hint.get()}</span>
                        <span class="op-media-record-hint">{lock_hint.get()}</span>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}

            <button
                type="button"
                class=root_cls
                style=root_stl
                data-circular=move || is_circular.get().then_some("true")
                data-mode=move || current_mode.get().as_str()
                data-recording=move || is_recording.get().then_some("true")
                data-locked=move || is_locked.get().then_some("true")
                data-canceling=move || is_canceling.get().then_some("true")
                data-disabled=move || is_disabled.get().then_some("true")
                disabled=move || is_disabled.get()
                on:pointerdown=on_pointerdown
                on:pointermove=on_pointermove
                on:pointerup=on_pointerup
                on:pointercancel=on_pointercancel
                aria-label=move || {
                    if is_locked.get() {
                        locked_label.get()
                    } else if current_mode.get() == MediaRecordMode::Video {
                        video_label.get()
                    } else {
                        voice_label.get()
                    }
                }
            >
                <span class="op-media-record-button__icon">
                    {move || {
                        let icon_data = if is_locked.get() {
                            icon_lock.get()
                        } else if current_mode.get() == MediaRecordMode::Video {
                            icon_video.get()
                        } else {
                            icon_voice.get()
                        };
                        view! { <Icon icon=icon_data /> }
                    }}
                </span>
            </button>
        </div>
    }
}
