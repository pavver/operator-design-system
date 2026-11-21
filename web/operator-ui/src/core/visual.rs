//! Reactive state-border visuals and pending indicators.

use super::{ControlState, JobToken};
use leptos::prelude::*;
use std::time::Duration;

/// Reactive attributes that drive the CSS state border (`.op-sb`).
///
/// - `data-state` mirrors the current [`ControlState`].
/// - `data-result` keeps the last result (`success` / `error`) so the exit fade keeps its color.
/// - `data-fading` is present for the duration of the result exit fade.
#[derive(Clone, Copy)]
pub struct StateVisual {
    pub state: Signal<&'static str>,
    pub result: Signal<&'static str>,
    pub fading: Signal<bool>,
}

pub fn use_state_visual(state: Signal<ControlState>) -> StateVisual {
    let result = RwSignal::new(if state.get_untracked() == ControlState::Error {
        "error"
    } else {
        "success"
    });
    let fading = RwSignal::new(false);
    let previous = StoredValue::new(state.get_untracked());
    let fade_job = JobToken::new();
    let fade_ms = operator_ui_tokens::motion::RESULT_FADE_MS;

    Effect::new(move |_| {
        let current = state.get();
        let before = previous.get_value();
        if current.is_result() {
            result.set(if current == ControlState::Success {
                "success"
            } else {
                "error"
            });
            fading.set(false);
        } else if before.is_result() {
            fading.set(true);
            let id = fade_job.next();
            set_timeout(
                move || {
                    if fade_job.is_current(id) {
                        fading.set(false);
                    }
                },
                Duration::from_millis(u64::from(fade_ms)),
            );
        }
        previous.set_value(current);
    });

    StateVisual {
        state: Signal::derive(move || state.get().as_str()),
        result: result.into(),
        fading: fading.into(),
    }
}

/// Returns `true` once `pending` has stayed `true` for `motion.pending_indicator_delay`.
///
/// Fast requests therefore show only the running Pending border, never the spinner.
/// Controls render their regular content while the returned signal is `false`.
pub fn use_pending_indicator(pending: impl Fn() -> bool + Send + Sync + 'static) -> Signal<bool> {
    // A memo notifies only on real changes, so repeated `Pending` writes do not restart the delay.
    let pending = Memo::new(move |_| pending());
    let elapsed = RwSignal::new(false);
    let job = JobToken::new();
    let delay_ms = operator_ui_tokens::motion::PENDING_INDICATOR_DELAY_MS;

    Effect::new(move |_| {
        let id = job.next();
        elapsed.set(false);
        if pending.get() {
            set_timeout(
                move || {
                    if job.is_current(id) {
                        elapsed.set(true);
                    }
                },
                Duration::from_millis(u64::from(delay_ms)),
            );
        }
    });

    Signal::derive(move || pending.get() && elapsed.get())
}

/// Running clockwise pending segment, shown by CSS only while the parent `.op-sb` is pending.
#[component]
pub fn PendingRun() -> impl IntoView {
    view! {
        <svg class="op-run" aria-hidden="true">
            <rect pathLength="100"></rect>
        </svg>
    }
}
