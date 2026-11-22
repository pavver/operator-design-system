//! MarqueeText: single-line text that scrolls when it overflows and shows the full text in a tooltip.

mod tooltip;

use crate::components::popover::PopoverAnchor;
use crate::core::{JobToken, Metric, Slot, root_class, root_style};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::DU_POINTS;
use send_wrapper::SendWrapper;
use std::time::Duration;
use tooltip::{MarqueeTooltip, PointerKind, forward_tooltip_style};
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

/// Single-line text for bounded containers.
///
/// When the text overflows, it stays still for `initial_delay_ms`, scrolls right to left at
/// `velocity` DU/s, and pauses `delay_ms` between loops. Hover or a touch long-press shows the full text.
#[component]
pub fn MarqueeText(
    #[prop(into)] text: Signal<String>,
    /// Extra classes, typically a typography class such as `op-t-row-title`.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    #[prop(default = marquee::INITIAL_DELAY_MS)] initial_delay_ms: u32,
    #[prop(default = marquee::LOOP_DELAY_MS)] delay_ms: u32,
    /// Scroll speed in DU per second.
    #[prop(default = marquee::VELOCITY_DU)]
    velocity: f32,
    /// Loop gap in DU.
    #[prop(optional, into)]
    spacing: MaybeProp<f32>,
    /// Number of loops; `None` repeats forever.
    #[prop(optional)]
    iterations: Option<u32>,
    #[prop(default = true)] tooltip: bool,
    /// Tooltip text; defaults to `text`.
    #[prop(optional, into)]
    tooltip_text: Option<Signal<String>>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] tooltip_slot: Slot,
    #[prop(optional, into)] tooltip_shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] tooltip_shadow_blur: MaybeProp<f32>,
    #[prop(optional, into)] tooltip_gap: MaybeProp<f32>,
    #[prop(optional, into)] tooltip_max_width: MaybeProp<f32>,
    #[prop(optional, into)] tooltip_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] tooltip_padding_y: MaybeProp<f32>,
    #[prop(optional, into)] tooltip_font_size: MaybeProp<f32>,
    #[prop(optional, into)] tooltip_line_height: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let host = NodeRef::<leptos::html::Span>::new();
    let probe = NodeRef::<leptos::html::Span>::new();
    let tip_layer = NodeRef::<leptos::html::Div>::new();
    let tip_probe = NodeRef::<leptos::html::Span>::new();
    let tip_gap = RwSignal::new(marquee::TOOLTIP_GAP_DU);
    let gap_probe = NodeRef::<leptos::html::Span>::new();
    let overflow = RwSignal::new(false);
    let distance = RwSignal::new(0.0f64);
    let run = RwSignal::new(0u32);
    let loops = StoredValue::new(0u32);
    let job = JobToken::new();

    let tip_open = RwSignal::new(false);
    let tip_anchor = RwSignal::new(PopoverAnchor::Point { x: 0.0, y: 0.0 });
    let hovered = RwSignal::new(false);
    let pressed = RwSignal::new(false);
    let dismiss_job = JobToken::new();
    let press_job = JobToken::new();
    let long_pressed = StoredValue::new(false);
    let tooltip_text = tooltip_text.unwrap_or(text);

    // Measures the text against the host and (re)starts the marquee cycle.
    let measure = move || {
        measure_marquee(
            host,
            probe,
            tip_probe,
            gap_probe,
            tip_gap,
            overflow,
            distance,
            run,
            loops,
            job,
            spacing,
            initial_delay_ms,
        );
    };

    Effect::new(move |_| {
        text.track();
        request_animation_frame(measure);
    });

    // Re-measure when the host is resized by its container.
    setup_resize_observer(
        host,
        tip_probe,
        probe,
        gap_probe,
        Callback::new(move |()| measure()),
    );

    let on_animation_end = move |_| {
        handle_animation_end(loops, iterations, run, job, delay_ms);
    };

    let open_tip = move || {
        if !tooltip {
            return;
        }
        dismiss_job.next();
        if let Some(h) = host.try_get_untracked().flatten() {
            tip_anchor.set(PopoverAnchor::of(&h));
        }
        tip_open.set(true);
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
        setup_outside_pointer_listener(host, tip_open, Callback::new(move |()| close_tip()));
    }

    Effect::new(move |_| {
        tip_open.track();
        style.track();
        tooltip_gap.track();
        tooltip_shadow_y.track();
        tooltip_shadow_blur.track();
        tooltip_max_width.track();
        tooltip_padding_x.track();
        tooltip_padding_y.track();
        tooltip_font_size.track();
        tooltip_line_height.track();
        let layer = tip_layer.get();
        request_animation_frame(move || {
            if let Some(layer) = layer {
                forward_tooltip_style(host, &layer);
            }
        });
    });

    let tooltip_slot = StoredValue::new(tooltip_slot);
    let content = Callback::new(move |()| {
        if content.is_set() {
            content.run().into_any()
        } else {
            text.get().into_any()
        }
    });
    let metrics = vec![
        Metric::du("--op-marquee-text-tooltip-shadow-y", tooltip_shadow_y),
        Metric::du("--op-marquee-text-tooltip-shadow-blur", tooltip_shadow_blur),
        Metric::du("--op-marquee-text-tooltip-gap", tooltip_gap),
        Metric::du("--op-marquee-text-tooltip-max-width", tooltip_max_width),
        Metric::du("--op-marquee-text-tooltip-padding-x", tooltip_padding_x),
        Metric::du("--op-marquee-text-tooltip-padding-y", tooltip_padding_y),
        Metric::tu("--op-marquee-text-tooltip-font-size", tooltip_font_size),
        Metric::tu("--op-marquee-text-tooltip-line-height", tooltip_line_height),
        Metric::tu("--op-marquee-text-font-size", font_size),
        Metric::tu("--op-marquee-text-line-height", line_height),
        Metric::du("--op-marquee-text-spacing", spacing),
    ];
    view! {
        <span
            node_ref=host
            class=root_class("op-marquee", class) style=root_style(metrics, style)
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
                handle_marquee_click(long_pressed, tip_open, hovered, close_tip, on_click);
            }
        >
            <MarqueeTrack
                run=run.into()
                distance=distance.into()
                velocity=velocity
                overflow=overflow.into()
                probe=probe
                tip_probe=tip_probe
                gap_probe=gap_probe
                content=content
                on_animation_end=Callback::new(on_animation_end)
            />
            {move || view! {
                <MarqueeTooltip
                    tip_layer=tip_layer
                    tip_open=tip_open
                    tip_anchor=tip_anchor
                    tip_gap=tip_gap.into()
                    tooltip_slot=tooltip_slot
                    tooltip_text=tooltip_text
                />
            }}
        </span>
    }
}

fn animation_duration_ms(distance: f64, velocity: f32) -> String {
    let px_per_s = f64::from(velocity * DU_POINTS).max(1.0);
    format!("{}ms", (distance / px_per_s * 1000.0).round())
}

fn track_run_attr(run: u32) -> Option<&'static str> {
    match run {
        0 => None,
        n if n % 2 == 1 => Some("a"),
        _ => Some("b"),
    }
}

fn handle_marquee_click(
    long_pressed: StoredValue<bool>,
    tip_open: RwSignal<bool>,
    hovered: RwSignal<bool>,
    close_tip: impl Fn(),
    on_click: Option<Callback<()>>,
) {
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

#[component]
fn MarqueeTrack(
    run: Signal<u32>,
    distance: Signal<f64>,
    velocity: f32,
    overflow: Signal<bool>,
    probe: NodeRef<leptos::html::Span>,
    tip_probe: NodeRef<leptos::html::Span>,
    gap_probe: NodeRef<leptos::html::Span>,
    content: Callback<(), AnyView>,
    on_animation_end: Callback<ev::AnimationEvent>,
) -> impl IntoView {
    view! {
        <span
            class="op-marquee-track"
            data-run=move || track_run_attr(run.get())
            style:--op-marquee-distance=move || format!("{}px", distance.get())
            style:animation-duration=move || animation_duration_ms(distance.get(), velocity)
            on:animationend=move |e| on_animation_end.run(e)
        >
            <span node_ref=probe class="op-marquee-text">{move || content.run(())}</span>
            <span node_ref=tip_probe class="op-marquee-tip-measure" aria-hidden="true"></span>
            <span node_ref=gap_probe class="op-marquee-measure" aria-hidden="true"></span>
            <Show when=move || overflow.get()>
                <span class="op-marquee-gap"></span>
                <span class="op-marquee-text" aria-hidden="true" inert="">{move || content.run(())}</span>
            </Show>
        </span>
    }
}

#[allow(clippy::too_many_arguments)]
fn measure_marquee(
    host: NodeRef<leptos::html::Span>,
    probe: NodeRef<leptos::html::Span>,
    tip_probe: NodeRef<leptos::html::Span>,
    gap_probe: NodeRef<leptos::html::Span>,
    tip_gap: RwSignal<f32>,
    overflow: RwSignal<bool>,
    distance: RwSignal<f64>,
    run: RwSignal<u32>,
    loops: StoredValue<u32>,
    job: JobToken,
    spacing: MaybeProp<f32>,
    initial_delay_ms: u32,
) {
    let (Some(h), Some(p)) = (
        host.try_get_untracked().flatten(),
        probe.try_get_untracked().flatten(),
    ) else {
        return;
    };
    if let Some(marker) = tip_probe.get_untracked() {
        #[allow(clippy::cast_possible_truncation)]
        let gap = marker.get_bounding_client_rect().width() as f32 / DU_POINTS;
        if (gap - tip_gap.get_untracked()).abs() > f32::EPSILON {
            tip_gap.set(gap);
        }
    }
    let width = f64::from(p.offset_width());
    let available = f64::from(h.client_width());
    let over = width > available + 0.5;
    let gap = gap_probe.get_untracked().map_or(
        f64::from(spacing.get_untracked().unwrap_or(marquee::SPACING_DU) * DU_POINTS),
        |g| g.get_bounding_client_rect().width(),
    );
    let next = width + gap;
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
}

fn setup_resize_observer(
    host: NodeRef<leptos::html::Span>,
    tip_probe: NodeRef<leptos::html::Span>,
    probe: NodeRef<leptos::html::Span>,
    gap_probe: NodeRef<leptos::html::Span>,
    measure: Callback<()>,
) {
    Effect::new(move |_| {
        let Some(el) = host.get() else {
            return;
        };
        let callback = Closure::<dyn FnMut()>::new(move || measure.run(()));
        let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) else {
            return;
        };
        observer.observe(&el);
        if let Some(marker) = tip_probe.get_untracked() {
            observer.observe(&marker);
        }
        if let Some(probe) = probe.get_untracked() {
            observer.observe(&probe);
        }
        if let Some(gap) = gap_probe.get_untracked() {
            observer.observe(&gap);
        }
        let keep = SendWrapper::new((observer, callback));
        on_cleanup(move || keep.0.disconnect());
    });
}

fn handle_animation_end(
    loops: StoredValue<u32>,
    iterations: Option<u32>,
    run: RwSignal<u32>,
    job: JobToken,
    delay_ms: u32,
) {
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
}

fn setup_outside_pointer_listener(
    host: NodeRef<leptos::html::Span>,
    tip_open: RwSignal<bool>,
    close_tip: Callback<()>,
) {
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
            close_tip.run(());
        }
    });
    on_cleanup(move || handle.remove());
}

#[cfg(test)]
mod tests;
