use super::MediaRecordMode;
use leptos::ev;
use leptos::prelude::*;

/// Drag distances in pixels.
pub const DRAG_LOCK_THRESHOLD_PX: f64 = 60.0;
pub const DRAG_CANCEL_THRESHOLD_PX: f64 = 60.0;
pub const HOLD_TIMEOUT_MS: i32 = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragAction {
    Cancel,
    Lock,
}

pub fn check_drag(delta_x: f64, delta_y: f64) -> Option<DragAction> {
    if delta_x > DRAG_CANCEL_THRESHOLD_PX {
        Some(DragAction::Cancel)
    } else if delta_y > DRAG_LOCK_THRESHOLD_PX {
        Some(DragAction::Lock)
    } else {
        None
    }
}

pub fn handle_pointer_up_release(
    elapsed: f64,
    is_recording: bool,
    is_locked: bool,
    current_mode: MediaRecordMode,
    on_mode_change: Option<Callback<MediaRecordMode>>,
    on_record_stop: Option<Callback<()>>,
) {
    if elapsed < f64::from(HOLD_TIMEOUT_MS) && !is_recording {
        let next_mode = match current_mode {
            MediaRecordMode::Voice => MediaRecordMode::Video,
            MediaRecordMode::Video => MediaRecordMode::Voice,
        };
        if let Some(cb) = on_mode_change {
            cb.run(next_mode);
        }
    } else if !is_locked {
        if let Some(cb) = on_record_stop {
            cb.run(());
        }
    }
}

#[derive(Clone, Copy)]
pub struct RecordGestureState {
    pub start_x: RwSignal<f64>,
    pub start_y: RwSignal<f64>,
    pub pointer_down_time: RwSignal<f64>,
    pub is_holding: RwSignal<bool>,
    pub is_canceling: RwSignal<bool>,
}

impl RecordGestureState {
    pub fn new() -> Self {
        Self {
            start_x: RwSignal::new(0.0),
            start_y: RwSignal::new(0.0),
            pointer_down_time: RwSignal::new(0.0),
            is_holding: RwSignal::new(false),
            is_canceling: RwSignal::new(false),
        }
    }

    pub fn on_down(
        self,
        ev: ev::PointerEvent,
        disabled: bool,
        locked: bool,
        current_mode: MediaRecordMode,
        on_stop: Option<Callback<()>>,
        on_start: Option<Callback<MediaRecordMode>>,
    ) {
        if disabled {
            return;
        }
        if locked {
            if let Some(cb) = on_stop {
                cb.run(());
            }
            return;
        }
        self.start_x.set(f64::from(ev.client_x()));
        self.start_y.set(f64::from(ev.client_y()));
        self.pointer_down_time.set(js_sys::Date::now());
        self.is_holding.set(true);
        self.is_canceling.set(false);

        set_timeout(
            move || {
                if self.is_holding.get() && !locked {
                    if let Some(cb) = on_start {
                        cb.run(current_mode);
                    }
                }
            },
            std::time::Duration::from_millis(HOLD_TIMEOUT_MS as u64),
        );
    }

    pub fn on_move(
        self,
        ev: ev::PointerEvent,
        disabled: bool,
        locked: bool,
        on_cancel: Option<Callback<()>>,
        on_lock: Option<Callback<()>>,
    ) {
        if !self.is_holding.get() || locked || disabled {
            return;
        }
        let cur_x = f64::from(ev.client_x());
        let cur_y = f64::from(ev.client_y());
        let delta_x = self.start_x.get() - cur_x;
        let delta_y = self.start_y.get() - cur_y;

        if let Some(action) = check_drag(delta_x, delta_y) {
            self.is_holding.set(false);
            match action {
                DragAction::Cancel => {
                    self.is_canceling.set(true);
                    if let Some(cb) = on_cancel {
                        cb.run(());
                    }
                }
                DragAction::Lock => {
                    if let Some(cb) = on_lock {
                        cb.run(());
                    }
                }
            }
        }
    }

    pub fn on_up(
        self,
        disabled: bool,
        recording: bool,
        locked: bool,
        current_mode: MediaRecordMode,
        on_mode_change: Option<Callback<MediaRecordMode>>,
        on_stop: Option<Callback<()>>,
    ) {
        if disabled {
            return;
        }
        let elapsed = js_sys::Date::now() - self.pointer_down_time.get();
        if self.is_canceling.get() {
            self.is_canceling.set(false);
            self.is_holding.set(false);
            return;
        }
        if self.is_holding.get() {
            self.is_holding.set(false);
            handle_pointer_up_release(
                elapsed,
                recording,
                locked,
                current_mode,
                on_mode_change,
                on_stop,
            );
        }
    }

    pub fn on_cancel(self, locked: bool, on_cancel: Option<Callback<()>>) {
        if self.is_holding.get() && !locked {
            self.is_holding.set(false);
            if let Some(cb) = on_cancel {
                cb.run(());
            }
        }
    }
}
