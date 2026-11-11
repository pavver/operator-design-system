//! MarqueeText: single-line text that scrolls when it overflows and shows the full text in a tooltip.

use crate::core::overlay::place_above;
use crate::core::{JobToken, du};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::DU_POINTS;
use send_wrapper::SendWrapper;
use std::time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(crate) const CSS: &str = include_str!("marquee_text.css");

/// Default marquee timings and geometry.
pub mod marquee {
    /// Stationary reading time before the first scroll, in milliseconds.
    pub const INITIAL_DELAY_MS: u32 = 2000;
    /// Pause between loops, in milliseconds.
    pub const LOOP_DELAY_MS: u32 = 1200;
    /// Scroll speed in DU per second.
    pub const VELOCITY_DU: f32 = 7.5;
    /// Gap between the end of the text and its repeated start, in DU.
    pub const SPACING_DU: f32 = 6.0;
    /// Tooltip auto-dismiss delay after the finger lifts, in milliseconds.
    pub const TOOLTIP_DISMISS_MS: u32 = crate::components::tooltip::tooltip::DISMISS_MS;
    /// Touch hold time that opens the tooltip, in milliseconds.
    pub const LONG_PRESS_MS: u32 = crate::components::tooltip::tooltip::LONG_PRESS_MS;
    /// Tooltip distance from the anchor, in DU.
    pub const TOOLTIP_GAP_DU: f32 = crate::components::tooltip::tooltip::GAP_DU;
    /// Tooltip maximum width, in DU.
    pub const TOOLTIP_MAX_WIDTH_DU: f32 = crate::components::tooltip::tooltip::MAX_WIDTH_DU;
}

#[derive(Clone, Copy)]
struct TooltipPos {
    left: f64,
    top: f64,
}

/// Single-line text for bounded containers.
///
/// When the text overflows, it stays still for `initial_delay_ms`, scrolls right to left at
/// `velocity` DU/s, and pauses `delay_ms` between loops. Hover or a touch long-press shows the full text.
#[component]
pub fn MarqueeText(
    #[prop(into)] text: Signal<String>,
    /// Extra classes, typically a typography class such as `op-t-row-title`.
    #[prop(optional, into)]
    class: String,
    #[prop(default = marquee::INITIAL_DELAY_MS)] initial_delay_ms: u32,
    #[prop(default = marquee::LOOP_DELAY_MS)] delay_ms: u32,
    /// Scroll speed in DU per second.
    #[prop(default = marquee::VELOCITY_DU)]
    velocity: f32,
    /// Loop gap in DU.
    #[prop(default = marquee::SPACING_DU)]
    spacing: f32,
    /// Number of loops; `None` repeats forever.
    #[prop(optional)]
    iterations: Option<u32>,
    #[prop(default = true)] tooltip: bool,
    /// Tooltip text; defaults to `text`.
    #[prop(optional, into)]
    tooltip_text: Option<Signal<String>>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let host = NodeRef::<leptos::html::Span>::new();
    let probe = NodeRef::<leptos::html::Span>::new();
    let tip = NodeRef::<leptos::html::Div>::new();
    let overflow = RwSignal::new(false);
    let distance = RwSignal::new(0.0f64);
    let run = RwSignal::new(0u32);
    let loops = StoredValue::new(0u32);
    let job = JobToken::new();

    let tip_open = RwSignal::new(false);
    let tip_pos = RwSignal::new(None::<TooltipPos>);
    let hovered = RwSignal::new(false);
    let pressed = RwSignal::new(false);
    let dismiss_job = JobToken::new();
    let press_job = JobToken::new();
    let long_pressed = StoredValue::new(false);
    let tooltip_text = tooltip_text.unwrap_or(text);

    // Measures the text against the host and (re)starts the marquee cycle.
    let measure = move || {
        let (Some(h), Some(p)) = (
            host.try_get_untracked().flatten(),
            probe.try_get_untracked().flatten(),
        ) else {
            return;
        };
        let width = f64::from(p.offset_width());
        let available = f64::from(h.client_width());
        let over = width > available + 0.5;
        let next = width + f64::from(spacing * DU_POINTS);
        if over == overflow.get_untracked() && (next - distance.get_untracked()).abs() < 0.5 {
            return;
        }
        overflow.set(over);
        distance.set(next);
        run.set(0);
        loops.set_value(0);
        let id = job.next();
        if over {
            set_timeout(
                move || {
                    if job.is_current(id) {
                        run.set(1);
                    }
                },
                Duration::from_millis(u64::from(initial_delay_ms)),
            );
        }
    };

    Effect::new(move |_| {
        text.track();
        request_animation_frame(measure);
    });

    // Re-measure when the host is resized by its container.
    Effect::new(move |_| {
        let Some(el) = host.get() else {
            return;
        };
        let callback = Closure::<dyn FnMut()>::new(measure);
        let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) else {
            return;
        };
        observer.observe(&el);
        let keep = SendWrapper::new((observer, callback));
        on_cleanup(move || keep.0.disconnect());
    });

    let on_animation_end = move |_| {
        let id = job.next();
        loops.update_value(|n| *n += 1);
        if iterations.is_some_and(|max| loops.get_value() >= max) {
            run.set(0);
            return;
        }
        let next = run.get_untracked() + 1;
        run.set(0);
        set_timeout(
            move || {
                if job.is_current(id) {
                    run.set(next);
                }
            },
            Duration::from_millis(u64::from(delay_ms)),
        );
    };

    // Tooltip placement: above the anchor, flipped below near the top, clamped to the viewport.
    let place_tooltip = move || {
        let (Some(h), Some(t)) = (
            host.try_get_untracked().flatten(),
            tip.try_get_untracked().flatten(),
        ) else {
            return;
        };
        let p = place_above(
            &h.get_bounding_client_rect(),
            f64::from(t.offset_width()),
            f64::from(t.offset_height()),
            marquee::TOOLTIP_GAP_DU,
        );
        tip_pos.set(Some(TooltipPos {
            left: p.left,
            top: p.top,
        }));
    };

    let open_tip = move || {
        if !tooltip {
            return;
        }
        dismiss_job.next();
        tip_pos.set(None);
        tip_open.set(true);
        request_animation_frame(place_tooltip);
    };
    let close_tip = move || {
        dismiss_job.next();
        tip_open.set(false);
    };
    let schedule_dismiss = move || {
        let id = dismiss_job.next();
        set_timeout(
            move || {
                if dismiss_job.is_current(id)
                    && !hovered.get_untracked()
                    && !pressed.get_untracked()
                {
                    tip_open.set(false);
                }
            },
            Duration::from_millis(u64::from(marquee::TOOLTIP_DISMISS_MS)),
        );
    };

    // Any press outside the anchor closes the tooltip.
    if tooltip {
        let handle = window_event_listener(ev::pointerdown, move |e| {
            if !tip_open.get_untracked() {
                return;
            }
            let inside = host.try_get_untracked().flatten().is_some_and(|h| {
                e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                    .is_some_and(|n| h.contains(Some(&n)))
            });
            if !inside {
                close_tip();
            }
        });
        on_cleanup(move || handle.remove());
    }

    let content = move || text.get();

    view! {
        <span
            node_ref=host
            class=format!("op-marquee {class}")
            data-overflow=move || overflow.get().then_some("")
            data-clickable=on_click.is_some().then_some("")
            on:pointerenter=move |e| {
                if e.pointer_type() == "mouse" {
                    hovered.set(true);
                    open_tip();
                }
            }
            on:pointerleave=move |e| {
                if e.pointer_type() == "mouse" {
                    hovered.set(false);
                    close_tip();
                }
            }
            on:pointerdown=move |e| {
                if e.pointer_type() == "mouse" {
                    return;
                }
                pressed.set(true);
                long_pressed.set_value(false);
                let id = press_job.next();
                set_timeout(
                    move || {
                        if press_job.is_current(id) && pressed.get_untracked() {
                            long_pressed.set_value(true);
                            open_tip();
                        }
                    },
                    Duration::from_millis(u64::from(marquee::LONG_PRESS_MS)),
                );
            }
            on:pointerup=move |e| {
                if e.pointer_type() == "mouse" {
                    return;
                }
                press_job.next();
                pressed.set(false);
                if tip_open.get_untracked() {
                    schedule_dismiss();
                }
            }
            on:pointercancel=move |_| {
                press_job.next();
                pressed.set(false);
                if tip_open.get_untracked() {
                    schedule_dismiss();
                }
            }
            on:contextmenu=move |e| {
                if tooltip && e.pointer_type_is_touch() {
                    e.prevent_default();
                }
            }
            on:click=move |_| {
                if long_pressed.get_value() {
                    long_pressed.set_value(false);
                    return;
                }
                if tip_open.get_untracked() && !hovered.get_untracked() {
                    close_tip();
                    return;
                }
                if let Some(cb) = on_click {
                    cb.run(());
                }
            }
        >
            <span
                class="op-marquee-track"
                data-run=move || match run.get() {
                    0 => None,
                    n if n % 2 == 1 => Some("a"),
                    _ => Some("b"),
                }
                style:--op-marquee-distance=move || format!("{}px", distance.get())
                style:animation-duration=move || {
                    let px_per_s = f64::from(velocity * DU_POINTS).max(1.0);
                    format!("{}ms", (distance.get() / px_per_s * 1000.0).round())
                }
                on:animationend=on_animation_end
            >
                <span node_ref=probe class="op-marquee-text">{content}</span>
                <Show when=move || overflow.get()>
                    <span class="op-marquee-gap" style:width=du(spacing)></span>
                    <span class="op-marquee-text" aria-hidden="true">{content}</span>
                </Show>
            </span>
            <Show when=move || tip_open.get()>
                <div
                    node_ref=tip
                    class="op-tooltip op-t-caption"
                    role="tooltip"
                    style:max-width=du(marquee::TOOLTIP_MAX_WIDTH_DU)
                    style:visibility=move || if tip_pos.get().is_some() { "visible" } else { "hidden" }
                    style:left=move || tip_pos.get().map(|p| format!("{}px", p.left))
                    style:top=move || tip_pos.get().map(|p| format!("{}px", p.top))
                >
                    {move || tooltip_text.get()}
                </div>
            </Show>
        </span>
    }
}

trait PointerKind {
    fn pointer_type_is_touch(&self) -> bool;
}

impl PointerKind for web_sys::MouseEvent {
    fn pointer_type_is_touch(&self) -> bool {
        self.dyn_ref::<web_sys::PointerEvent>()
            .is_some_and(|p| p.pointer_type() == "touch")
    }
}
