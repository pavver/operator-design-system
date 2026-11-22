//! Media record button supporting hold-to-record, drag-left-to-cancel, drag-up-to-lock, and mode toggle.

mod gesture;

use gesture::RecordGestureState;
pub use gesture::{DRAG_CANCEL_THRESHOLD_PX, DRAG_LOCK_THRESHOLD_PX, HOLD_TIMEOUT_MS};

use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_icon, ui_text};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

pub(crate) const CSS: &str = include_str!("media_record_button.css");

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

fn format_record_time(secs: u32) -> String {
    let m = secs / 60;
    let s = secs % 60;
    format!("{m}:{s:02}")
}

#[component]
fn MediaRecordTimer(is_active: Signal<bool>, seconds: RwSignal<u32>) -> impl IntoView {
    view! {
        {move || {
            if is_active.get() {
                view! {
                    <div class="op-media-record-timer">
                        <span class="op-media-record-timer__dot"></span>
                        <span>{move || format_record_time(seconds.get())}</span>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }
        }}
    }
}

#[component]
fn MediaRecordHints(
    show: Signal<bool>,
    cancel_hint: Signal<String>,
    lock_hint: Signal<String>,
) -> impl IntoView {
    view! {
        {move || {
            if show.get() {
                view! {
                    <span class="op-media-record-hint">{cancel_hint.get()}</span>
                    <span class="op-media-record-hint">{lock_hint.get()}</span>
                }.into_any()
            } else {
                ().into_any()
            }
        }}
    }
}

fn record_button_label(
    is_locked: bool,
    mode: MediaRecordMode,
    locked_label: &str,
    video_label: &str,
    voice_label: &str,
) -> String {
    if is_locked {
        locked_label.to_string()
    } else if mode == MediaRecordMode::Video {
        video_label.to_string()
    } else {
        voice_label.to_string()
    }
}

fn record_button_icon(
    is_locked: bool,
    mode: MediaRecordMode,
    icon_lock: IconData,
    icon_video: IconData,
    icon_voice: IconData,
) -> IconData {
    if is_locked {
        icon_lock
    } else if mode == MediaRecordMode::Video {
        icon_video
    } else {
        icon_voice
    }
}

#[component]
fn MediaRecordActionButton(
    class: Signal<String>,
    style: Signal<Option<String>>,
    is_circular: Signal<bool>,
    current_mode: Signal<MediaRecordMode>,
    is_recording: Signal<bool>,
    is_locked: Signal<bool>,
    is_canceling: RwSignal<bool>,
    is_disabled: Signal<bool>,
    on_pointerdown: Callback<ev::PointerEvent>,
    on_pointermove: Callback<ev::PointerEvent>,
    on_pointerup: Callback<ev::PointerEvent>,
    on_pointercancel: Callback<ev::PointerEvent>,
    aria_label: Signal<String>,
    icon_data: Signal<IconData>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class=class
            style=style
            data-circular=move || is_circular.get().then_some("true")
            data-mode=move || current_mode.get().as_str()
            data-recording=move || is_recording.get().then_some("true")
            data-locked=move || is_locked.get().then_some("true")
            data-canceling=move || is_canceling.get().then_some("true")
            data-disabled=move || is_disabled.get().then_some("true")
            disabled=move || is_disabled.get()
            on:pointerdown=move |ev| on_pointerdown.run(ev)
            on:pointermove=move |ev| on_pointermove.run(ev)
            on:pointerup=move |ev| on_pointerup.run(ev)
            on:pointercancel=move |ev| on_pointercancel.run(ev)
            aria-label=move || aria_label.get()
        >
            <span class="op-media-record-button__icon">
                {move || view! { <Icon icon=icon_data.get() /> }}
            </span>
        </button>
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
    let gesture = RecordGestureState::new();
    let is_canceling = gesture.is_canceling;
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
        gesture.on_down(
            ev,
            is_disabled.get(),
            is_locked.get(),
            current_mode.get(),
            on_record_stop,
            on_record_start,
        );
    };

    let on_pointermove = move |ev: ev::PointerEvent| {
        gesture.on_move(
            ev,
            is_disabled.get(),
            is_locked.get(),
            on_record_cancel,
            on_record_locked,
        );
    };

    let on_pointerup = move |_ev: ev::PointerEvent| {
        gesture.on_up(
            is_disabled.get(),
            is_recording.get(),
            is_locked.get(),
            current_mode.get(),
            on_mode_change,
            on_record_stop,
        );
    };

    let on_pointercancel = move |_ev: ev::PointerEvent| {
        gesture.on_cancel(is_locked.get(), on_record_cancel);
    };

    let metrics = vec![Metric::du("--op-media-record-button-size", size)];
    let root_cls = Signal::derive(root_class("op-media-record-button", class));
    let root_stl = Signal::derive(root_style(metrics, style));

    let is_active_timer = Signal::derive(move || is_recording.get() || is_locked.get());
    let is_show_hints = Signal::derive(move || is_recording.get() && !is_locked.get());

    let aria_label = Signal::derive(move || {
        record_button_label(
            is_locked.get(),
            current_mode.get(),
            &locked_label.get(),
            &video_label.get(),
            &voice_label.get(),
        )
    });
    let icon_data = Signal::derive(move || {
        record_button_icon(
            is_locked.get(),
            current_mode.get(),
            icon_lock.get(),
            icon_video.get(),
            icon_voice.get(),
        )
    });

    view! {
        <div class="op-media-record-container">
            <MediaRecordTimer is_active=is_active_timer seconds=timer_seconds />
            <MediaRecordHints show=is_show_hints cancel_hint=cancel_hint lock_hint=lock_hint />
            <MediaRecordActionButton
                class=root_cls
                style=root_stl
                is_circular=is_circular
                current_mode=current_mode
                is_recording=is_recording
                is_locked=is_locked
                is_canceling=is_canceling
                is_disabled=is_disabled
                on_pointerdown=Callback::new(on_pointerdown)
                on_pointermove=Callback::new(on_pointermove)
                on_pointerup=Callback::new(on_pointerup)
                on_pointercancel=Callback::new(on_pointercancel)
                aria_label=aria_label
                icon_data=icon_data
            />
        </div>
    }
}
