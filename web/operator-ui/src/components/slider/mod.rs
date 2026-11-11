//! Track-and-thumb slider: Slider.

use crate::components::primitives::{Icon, PendingIndicator};
use crate::core::{
    AsyncAction, ControlState, JobToken, PendingRun, sleep_ms, use_pending_indicator,
    use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{icons, motion};
use std::time::Duration;
use wasm_bindgen::JsCast;

pub(crate) const CSS: &str = include_str!("slider.css");

fn snap(value: f32, min: f32, max: f32, step: f32) -> f32 {
    if max <= min {
        return min;
    }
    let v = value.clamp(min, max);
    if step <= 0.0 {
        return v;
    }
    (min + ((v - min) / step).round() * step).clamp(min, max)
}

fn format_value(value: f32, min: f32, max: f32) -> String {
    if max - min > 1.0 {
        if value.fract() == 0.0 {
            format!("{value:.0}")
        } else {
            format!("{value:.1}")
        }
    } else {
        format!("{value:.2}")
    }
}

/// Track-and-thumb slider with discrete steps, tick marks, a value tooltip and status inside the thumb.
#[component]
pub fn Slider(
    value: RwSignal<f32>,
    #[prop(default = 0.0)] min: f32,
    #[prop(default = 1.0)] max: f32,
    /// Step size; 0 means continuous.
    #[prop(default = 0.0)]
    step: f32,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] on_change_finished: Option<Callback<f32>>,
    /// Async commit after release; on error the value rolls back to the last committed one.
    #[prop(optional, into)]
    on_commit: Option<AsyncAction<f32>>,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let dragging = RwSignal::new(false);
    let local = RwSignal::new(ControlState::Idle);
    let committed = StoredValue::new(value.get_untracked());
    let commit_job = JobToken::new();
    let on_commit = StoredValue::new(on_commit);
    let tooltip = RwSignal::new(false);
    let hide_job = JobToken::new();
    let track_ref = NodeRef::<leptos::html::Div>::new();

    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else {
            state.get().unwrap_or_else(|| local.get())
        }
    });
    let interactive = move || {
        !matches!(
            current.get_untracked(),
            ControlState::Disabled | ControlState::Pending
        )
    };
    let track_visual = use_state_visual(current);
    let thumb_state = Signal::derive(move || {
        let s = current.get();
        if s == ControlState::Pending {
            ControlState::Idle
        } else {
            s
        }
    });
    let thumb_visual = use_state_visual(thumb_state);
    let tone = use_state_visual(current);
    let show_spinner = use_pending_indicator(move || current.get() == ControlState::Pending);

    let fraction = Signal::derive(move || {
        if max > min {
            ((value.get() - min) / (max - min)).clamp(0.0, 1.0)
        } else {
            0.0
        }
    });

    Effect::new(move |_| {
        if dragging.get() {
            tooltip.set(true);
        } else {
            let id = hide_job.next();
            set_timeout(
                move || {
                    if hide_job.is_current(id) {
                        tooltip.set(false);
                    }
                },
                Duration::from_millis(u64::from(motion::TOOLTIP_HIDE_DELAY_MS)),
            );
        }
    });

    let set_from_x = move |client_x: f64| {
        let Some(track) = track_ref.get_untracked() else {
            return;
        };
        let rect = track.get_bounding_client_rect();
        if rect.width() <= 0.0 {
            return;
        }
        let f = ((client_x - rect.left()) / rect.width()).clamp(0.0, 1.0) as f32;
        let next = snap(min + f * (max - min), min, max, step);
        if next != value.get_untracked() {
            value.set(next);
        }
    };

    // Mirrors external value changes into the committed baseline while idle.
    Effect::new(move |_| {
        let v = value.get();
        if !dragging.get_untracked() && local.get_untracked() != ControlState::Pending {
            committed.set_value(v);
        }
    });

    let commit = move |next: f32| {
        if let Some(cb) = on_change_finished {
            cb.run(next);
        }
        let Some(action) = on_commit.get_value() else {
            return;
        };
        let previous = committed.get_value();
        if next == previous {
            return;
        }
        let id = commit_job.next();
        local.set(ControlState::Pending);
        spawn_local(async move {
            let result = action.call(next).await;
            if !commit_job.is_current(id) {
                return;
            }
            if result.is_success() {
                committed.set_value(next);
                local.set(ControlState::Success);
            } else {
                value.set(previous);
                committed.set_value(previous);
                local.set(ControlState::Error);
            }
            sleep_ms(motion::RESULT_HOLD_MS).await;
            if commit_job.is_current(id) {
                local.set(ControlState::Idle);
            }
        });
    };

    let ticks = move || {
        if step <= 0.0 || max <= min {
            return Vec::new();
        }
        let count = ((max - min) / step) as usize;
        (1..count)
            .map(|i| (i as f32 * step) / (max - min))
            .collect::<Vec<_>>()
    };

    let key_step = if step > 0.0 {
        step
    } else {
        (max - min) / 100.0
    };

    view! {
        <div
            class="op-slider"
            role="slider"
            tabindex=move || if interactive() { "0" } else { "-1" }
            aria-label=label
            aria-valuemin=min
            aria-valuemax=max
            aria-valuenow=move || value.get()
            aria-disabled=move || (current.get() == ControlState::Disabled).then_some("true")
            data-disabled=move || (current.get() == ControlState::Disabled).then_some("")
            data-dragging=move || dragging.get().then_some("")
            on:pointerdown=move |ev| {
                if !interactive() {
                    return;
                }
                // Blocks text selection and native drag, which would fire pointercancel mid-drag.
                ev.prevent_default();
                if let Some(target) = ev.current_target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
                    let _ = target.focus();
                }
                if let Some(target) = ev.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                    let _ = target.set_pointer_capture(ev.pointer_id());
                }
                dragging.set(true);
                set_from_x(f64::from(ev.client_x()));
            }
            on:pointermove=move |ev| {
                if dragging.get_untracked() {
                    set_from_x(f64::from(ev.client_x()));
                }
            }
            on:pointerup=move |_| {
                if dragging.get_untracked() {
                    dragging.set(false);
                    commit(value.get_untracked());
                }
            }
            on:dragstart=|ev| ev.prevent_default()
            on:pointercancel=move |_| {
                if dragging.get_untracked() {
                    dragging.set(false);
                    value.set(committed.get_value());
                }
            }
            on:keydown=move |ev| {
                if !interactive() {
                    return;
                }
                let delta = match ev.key().as_str() {
                    "ArrowRight" | "ArrowUp" => key_step,
                    "ArrowLeft" | "ArrowDown" => -key_step,
                    _ => return,
                };
                ev.prevent_default();
                let next = snap(value.get_untracked() + delta, min, max, step);
                if next != value.get_untracked() {
                    value.set(next);
                    commit(next);
                }
            }
        >
            <div class="op-slider-rail">
                <div
                    node_ref=track_ref
                    class="op-sb op-slider-track"
                    data-state=move || track_visual.state.get()
                    data-result=move || track_visual.result.get()
                    data-fading=move || track_visual.fading.get().then_some("")
                >
                    <div class="op-slider-fill" style:width=move || format!("{}%", fraction.get() * 100.0)></div>
                    {move || {
                        let filled = fraction.get();
                        ticks()
                            .into_iter()
                            .map(|t| {
                                view! {
                                    <span
                                        class="op-slider-tick"
                                        data-filled=(t <= filled + 0.0001).then_some("")
                                        style:left=format!("{}%", t * 100.0)
                                    ></span>
                                }
                            })
                            .collect_view()
                    }}
                    <PendingRun/>
                </div>
                <div class="op-slider-thumb-slot" style:left=move || format!("{}%", fraction.get() * 100.0)>
                    <div
                        class="op-sb op-slider-thumb"
                        data-state=move || thumb_visual.state.get()
                        data-result=move || thumb_visual.result.get()
                        data-fading=move || thumb_visual.fading.get().then_some("")
                    >
                        <span
                            class="op-tone op-slider-thumb-content"
                            data-state=move || tone.state.get()
                            data-result=move || tone.result.get()
                            data-fading=move || tone.fading.get().then_some("")
                        >
                            {move || match current.get() {
                                ControlState::Pending if show_spinner.get() => {
                                    view! { <PendingIndicator size=3.0/> }
                                        .into_any()
                                }
                                ControlState::Success => view! { <Icon icon=icons::CHECK size=3.5/> }.into_any(),
                                ControlState::Error => view! { <Icon icon=icons::CLOSE size=3.5/> }.into_any(),
                                _ => view! { <span class="op-slider-dot"></span> }.into_any(),
                            }}
                        </span>
                    </div>
                    <Show when=move || tooltip.get()>
                        <div class="op-slider-tooltip">{move || format_value(value.get(), min, max)}</div>
                    </Show>
                </div>
            </div>
        </div>
    }
}
