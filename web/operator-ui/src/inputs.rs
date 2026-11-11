//! TextField, Slider, Stepper, SegmentedControl and DropdownMenu.

use crate::basic::{Icon, PendingIndicator};
use crate::buttons::ActionButton;
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, PendingRun, Press, du, sleep_ms,
    spawn_ripple, use_pending_indicator, use_state_visual,
};
use crate::scroll::ScrollView;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};
use std::time::Duration;
use wasm_bindgen::JsCast;

// ---------------------------------------------------------------------------
// TextField
// ---------------------------------------------------------------------------

/// Single-line text field; shows an async SAVE button once the value differs from `initial_value`.
#[component]
pub fn TextField(
    value: RwSignal<String>,
    #[prop(into)] placeholder: Signal<String>,
    /// Baseline used to detect modifications; defaults to the value at creation.
    #[prop(optional, into)]
    initial_value: Option<Signal<String>>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(default = "SAVE".into(), into)] save_text: Signal<String>,
    #[prop(optional)] save_icon: Option<IconData>,
    #[prop(optional, into)] on_save: Option<AsyncAction<String>>,
    #[prop(optional)] password: bool,
    /// Shows the reveal toggle for password fields.
    #[prop(default = true)]
    show_reveal: bool,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let revealed = RwSignal::new(false);
    let has_reveal = password && show_reveal;
    let initial = initial_value.unwrap_or_else(|| Signal::stored(value.get_untracked()));
    let focused = RwSignal::new(false);
    let local = RwSignal::new(ControlState::Idle);
    let save_visible = RwSignal::new(false);
    let job = JobToken::new();
    let has_save = on_save.is_some();

    Effect::new(move |_| {
        let modified = value.get() != initial.get();
        if modified && has_save {
            save_visible.set(true);
        } else if !modified && local.get_untracked() == ControlState::Idle {
            save_visible.set(false);
        }
    });

    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if let Some(s) = state.get() {
            s
        } else if local.get() != ControlState::Idle {
            local.get()
        } else if focused.get() {
            ControlState::Focused
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(current);

    let save_action = on_save.map(|action| {
        AsyncAction::new(move |()| {
            let action = action.clone();
            async move {
                let id = job.next();
                local.set(ControlState::Pending);
                let result: ActionResult = action.call(value.get_untracked()).await;
                if !job.is_current(id) {
                    return result;
                }
                local.set(result.clone().into());
                let success = result.is_success();
                spawn_local(async move {
                    if success {
                        sleep_ms(700).await;
                        if !job.is_current(id) {
                            return;
                        }
                        save_visible.set(false);
                        sleep_ms(100).await;
                    } else {
                        sleep_ms(1000).await;
                    }
                    if job.is_current(id) {
                        local.set(ControlState::Idle);
                    }
                });
                result
            }
        })
    });

    view! {
        <div
            class="op-sb op-field"
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
            data-trailing=move || (has_reveal || save_visible.get()).then_some("")
        >
            <input
                class="op-field-input"
                type=move || if password && !revealed.get() { "password" } else { "text" }
                aria-label=label
                placeholder=move || placeholder.get()
                prop:value=move || value.get()
                disabled=move || !enabled.get()
                on:input=move |ev| value.set(event_target_value(&ev))
                on:focus=move |_| focused.set(true)
                on:blur=move |_| focused.set(false)
            />
            {has_reveal
                .then(|| {
                    view! {
                        <button
                            type="button"
                            class="op-rippled op-field-reveal"
                            aria-label=move || if revealed.get() { "Hide password" } else { "Show password" }
                            aria-pressed=move || revealed.get().to_string()
                            disabled=move || !enabled.get()
                            on:mousedown=|ev| ev.prevent_default()
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| revealed.update(|r| *r = !*r)
                        >
                            {move || {
                                let icon = if revealed.get() { icons::VISIBILITY_OFF } else { icons::VISIBILITY };
                                view! { <Icon icon=icon size=dim::ICON_SMALL/> }
                            }}
                        </button>
                    }
                })}
            {move || {
                (save_visible.get())
                    .then(|| {
                        save_action
                            .clone()
                            .map(|action| {
                                view! {
                                    // Keeps input focus (and the focused border) while pressing SAVE.
                                    <span class="op-field-save-slot" on:mousedown=|ev| ev.prevent_default()>
                                    <ActionButton
                                        text=save_text
                                        icon=save_icon.unwrap_or(icons::CHECK)
                                        show_pending_border=false
                                        keep_flash_icon_on_success=true
                                        enabled=enabled
                                        on_click=action
                                        class="op-field-save"
                                    />
                                    </span>
                                }
                            })
                    })
            }}
            <PendingRun/>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Slider
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Stepper
// ---------------------------------------------------------------------------

/// `- value +` stepper; stays interactive while pending to allow continuous input.
#[component]
pub fn Stepper(
    value: RwSignal<i32>,
    #[prop(optional)] min: Option<i32>,
    #[prop(optional)] max: Option<i32>,
    #[prop(default = 1)] step: i32,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Async commit; the value is updated only on success.
    #[prop(optional, into)]
    on_async_change: Option<AsyncAction<i32>>,
    /// Width in DU.
    #[prop(default = 35.0)]
    width: f32,
) -> impl IntoView {
    let local = RwSignal::new(ControlState::Idle);
    let pending = RwSignal::new(false);
    let direction = RwSignal::new(None::<bool>);
    let requested = RwSignal::new(value.get_untracked());
    let job = JobToken::new();
    let on_async_change = StoredValue::new(on_async_change);
    Effect::new(move |_| requested.set(value.get()));

    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if let Some(s) = state.get() {
            s
        } else {
            local.get()
        }
    });
    let visual = use_state_visual(current);
    let can_dec = Signal::derive(move || enabled.get() && min.is_none_or(|m| requested.get() > m));
    let can_inc = Signal::derive(move || enabled.get() && max.is_none_or(|m| requested.get() < m));

    let change = move |increment: bool| {
        let target = requested.get_untracked() + if increment { step } else { -step };
        requested.set(target);
        let Some(action) = on_async_change.get_value() else {
            value.set(target);
            return;
        };
        let id = job.next();
        spawn_local(async move {
            pending.set(true);
            local.set(ControlState::Pending);
            direction.set(Some(increment));
            let result = action.call(target).await;
            if !job.is_current(id) {
                return;
            }
            pending.set(false);
            if result.is_success() {
                value.set(target);
                local.set(ControlState::Success);
            } else {
                requested.set(value.get_untracked());
                local.set(ControlState::Error);
            }
            sleep_ms(motion::RESULT_HOLD_MS).await;
            if job.is_current(id) {
                local.set(ControlState::Idle);
                direction.set(None);
            }
        });
    };

    let button = move |increment: bool| {
        let press = Press::new();
        let allowed = if increment { can_inc } else { can_dec };
        let feedback = Signal::derive(move || {
            if direction.get() == Some(increment) {
                current.get()
            } else if !allowed.get() {
                ControlState::Disabled
            } else {
                ControlState::Idle
            }
        });
        let tone = use_state_visual(feedback);
        let disabled_visual =
            move || !allowed.get() || (direction.get() == Some(!increment) && pending.get());
        let mine = move || direction.get() == Some(increment);
        let show_spinner = use_pending_indicator(move || mine() && pending.get());
        let (icon, label) = if increment {
            (icons::ADD, "Increase")
        } else {
            (icons::REMOVE, "Decrease")
        };
        view! {
            <button
                type="button"
                class="op-rippled op-stepper-btn"
                aria-label=label
                aria-disabled=move || (!allowed.get()).then_some("true")
                data-pressed=move || press.pressed.get().then_some("")
                data-muted=move || disabled_visual().then_some("")
                on:pointerdown=move |ev| {
                    if allowed.get_untracked() {
                        press.down();
                        spawn_ripple(&ev);
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
                on:click=move |_| {
                    if allowed.get_untracked() {
                        change(increment);
                    }
                }
            >
                <span
                    class="op-tone"
                    data-state=move || tone.state.get()
                    data-result=move || tone.result.get()
                    data-fading=move || tone.fading.get().then_some("")
                >
                    {move || {
                        if show_spinner.get() {
                            view! { <PendingIndicator size=3.0/> }.into_any()
                        } else if mine() && current.get() == ControlState::Success {
                            view! { <Icon icon=icons::CHECK size=dim::ICON_SMALL label="Success"/> }.into_any()
                        } else if mine() && current.get() == ControlState::Error {
                            view! { <Icon icon=icons::CLOSE size=dim::ICON_SMALL label="Error"/> }.into_any()
                        } else {
                            view! { <Icon icon=icon size=dim::ICON_SMALL/> }.into_any()
                        }
                    }}
                </span>
            </button>
        }
    };

    view! {
        <div
            class="op-sb op-stepper"
            style:width=du(width)
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
        >
            {button(false)}
            <div class="op-stepper-value" data-disabled=move || (!enabled.get()).then_some("")>
                {move || value.get()}
            </div>
            {button(true)}
            <PendingRun/>
        </div>
    }
}

// ---------------------------------------------------------------------------
// SegmentedControl
// ---------------------------------------------------------------------------

/// Segment descriptor supporting text, icon, or icon + text.
#[derive(Clone, Debug, PartialEq)]
pub struct SegmentItem {
    pub text: Option<String>,
    pub icon: Option<IconData>,
}

impl SegmentItem {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            icon: None,
        }
    }

    pub fn icon(icon: IconData) -> Self {
        Self {
            text: None,
            icon: Some(icon),
        }
    }

    pub fn item(text: impl Into<String>, icon: IconData) -> Self {
        Self {
            text: Some(text.into()),
            icon: Some(icon),
        }
    }
}

/// Segment switcher with a sliding indicator; async mode adds pending, success flash and error rollback.
#[component]
pub fn SegmentedControl(
    items: Vec<SegmentItem>,
    selected: RwSignal<usize>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// When set, the selection commits only after the request succeeds.
    #[prop(optional, into)]
    on_select: Option<AsyncAction<usize>>,
    /// Height in DU.
    #[prop(default = 9.0)]
    height: f32,
) -> impl IntoView {
    let count = items.len().max(1);
    let pending_index = RwSignal::new(None::<usize>);
    let feedback = RwSignal::new(ControlState::Idle);
    let is_async = on_select.is_some();
    let on_select = StoredValue::new(on_select);

    let target = Signal::derive(move || {
        pending_index
            .get()
            .unwrap_or_else(|| selected.get().min(count - 1))
    });
    let indicator_state = Signal::derive(move || {
        if pending_index.get().is_some() {
            ControlState::Pending
        } else {
            feedback.get()
        }
    });
    let visual = use_state_visual(indicator_state);

    let pick = move |index: usize| {
        let Some(action) = on_select.get_value() else {
            selected.set(index);
            return;
        };
        if index == selected.get_untracked() || pending_index.get_untracked().is_some() {
            return;
        }
        pending_index.set(Some(index));
        spawn_local(async move {
            let result = action.call(index).await;
            pending_index.set(None);
            if result.is_success() {
                selected.set(index);
                feedback.set(ControlState::Success);
                sleep_ms(500).await;
            } else {
                feedback.set(ControlState::Error);
                sleep_ms(600).await;
            }
            feedback.set(ControlState::Idle);
        });
    };

    let buttons = items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            let press = Press::new();
            let is_selected = move || target.get() == index;
            let busy = move || pending_index.get().is_some();
            let show_spinner = use_pending_indicator(move || pending_index.get() == Some(index));
            let tone = move || {
                if !enabled.get() {
                    "disabled"
                } else if pending_index.get() == Some(index) {
                    "pending"
                } else if is_selected() {
                    match feedback.get() {
                        ControlState::Success => "success",
                        ControlState::Error => "error",
                        _ => "selected",
                    }
                } else {
                    "idle"
                }
            };
            view! {
                <button
                    type="button"
                    role="tab"
                    class="op-rippled op-seg-item"
                    aria-selected=move || is_selected().to_string()
                    data-tone=tone
                    data-pressed=move || press.pressed.get().then_some("")
                    on:pointerdown=move |ev| {
                        if enabled.get_untracked() && !busy() {
                            press.down();
                            spawn_ripple(&ev);
                        }
                    }
                    on:pointerup=move |_| press.up()
                    on:pointerleave=move |_| press.up()
                    on:pointercancel=move |_| press.up()
                    on:click=move |_| {
                        if enabled.get_untracked() && !busy() {
                            pick(index);
                        }
                    }
                >
                    {move || {
                        if show_spinner.get() {
                            view! { <PendingIndicator size=dim::ICON_SMALL/> }.into_any()
                        } else {
                            let item = item.clone();
                            view! {
                                <span class="op-seg-content">
                                    {item.icon.map(|i| view! { <Icon icon=i size=dim::ICON_SMALL/> })}
                                    {item.text.map(|t| view! { <span>{t}</span> })}
                                </span>
                            }
                                .into_any()
                        }
                    }}
                </button>
            }
        })
        .collect_view();

    view! {
        <div
            class="op-seg"
            role="tablist"
            style:height=du(height)
            data-disabled=move || (!enabled.get()).then_some("")
        >
            <div class="op-seg-track">
                <Show when=move || enabled.get()>
                    <div
                        class="op-seg-slide"
                        style:width=format!("{}%", 100.0 / count as f32)
                        style:transform=move || format!("translateX({}%)", target.get() * 100)
                    >
                        <div
                            class="op-sb op-seg-indicator"
                            class:op-seg-indicator-async=is_async
                            data-state=move || visual.state.get()
                            data-result=move || visual.result.get()
                            data-fading=move || visual.fading.get().then_some("")
                        >
                            <PendingRun/>
                        </div>
                    </div>
                </Show>
                {buttons}
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// DropdownMenu
// ---------------------------------------------------------------------------

/// Rectangular dropdown with a scrollable option list (max 50 DU).
#[component]
pub fn DropdownMenu(
    options: Vec<String>,
    selected: RwSignal<String>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    let expanded = RwSignal::new(false);
    let press = Press::new();
    let external = Signal::derive(move || state.get().unwrap_or_default());

    let border_state = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if external.get() != ControlState::Idle {
            external.get()
        } else if press.pressed.get() {
            ControlState::Pressed
        } else if expanded.get() {
            ControlState::Selected
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(border_state);
    let show_spinner = use_pending_indicator(move || external.get() == ControlState::Pending);
    let clickable = move || {
        enabled.get()
            && !matches!(
                external.get(),
                ControlState::Disabled | ControlState::Pending
            )
    };
    Effect::new(move |_| {
        if !clickable() {
            expanded.set(false);
        }
    });

    let options = StoredValue::new(options);

    view! {
            <div class="op-dropdown">
                {label.map(|l| view! { <div class="op-dropdown-label">{l}</div> })}
                <div class="op-dropdown-anchor">
                    <button
                        type="button"
                        class="op-sb op-rippled op-dropdown-trigger"
                        aria-haspopup="listbox"
                        aria-expanded=move || expanded.get().to_string()
                        aria-disabled=move || (!clickable()).then_some("true")
                        data-state=move || visual.state.get()
                        data-result=move || visual.result.get()
                        data-fading=move || visual.fading.get().then_some("")
                        on:pointerdown=move |ev| {
                            if clickable() {
                                press.down();
                                spawn_ripple(&ev);
                            }
                        }
                        on:pointerup=move |_| press.up()
                        on:pointerleave=move |_| press.up()
                        on:pointercancel=move |_| press.up()
                        on:click=move |_| {
                            if clickable() {
                                expanded.update(|e| *e = !*e);
                            }
                        }
                        on:keydown=move |ev| {
                            if ev.key() == "Escape" {
                                expanded.set(false);
                            }
                        }
                    >
                        <span class="op-dropdown-value">{move || selected.get()}</span>
                        {move || {
                            if show_spinner.get() {
                                view! { <PendingIndicator size=dim::ICON_SMALL/> }
                                    .into_any()
                            } else {
                                view! { <span class="op-dropdown-arrow"><Icon icon=icons::ARROW_DROP_DOWN size=5.0/></span> }
                                    .into_any()
                            }
                        }}
                        <PendingRun/>
                    </button>
                    <Show when=move || expanded.get()>
                        <div class="op-dropdown-scrim" on:click=move |_| expanded.set(false)></div>
                        <div class="op-dropdown-menu" role="listbox">
                            <ScrollView>
                            {options
                                .get_value()
                                .into_iter()
                                .map(|option| {
                                    let value = option.clone();
                                    let is_selected = {
                                        let option = option.clone();
                                        move || selected.with(|s| *s == option)
                                    };
                                    view! {
                                        <button
                                            type="button"
                                            role="option"
                                            class="op-rippled op-dropdown-item"
                                            aria-selected=move || is_selected().to_string()
                                            on:pointerdown=move |ev| spawn_ripple(&ev)
                                            on:click=move |_| {
                                                selected.set(value.clone());
                                                if let Some(cb) = on_select {
                                                    cb.run(value.clone());
                                                }
                                                expanded.set(false);
                                            }
                                        >
                                            {option}
                                        </button>
                                    }
                                })
                                .collect_view()}
    </ScrollView>
                        </div>
                    </Show>
                </div>
            </div>
        }
}
