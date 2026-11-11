//! Floating layers: viewport-aware placement helpers and the Tooltip component.

use crate::core::{JobToken, du};
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::DU_POINTS;
use std::time::Duration;
use wasm_bindgen::JsCast;

/// Tooltip timings and geometry shared by [`Tooltip`] and [`crate::text::MarqueeText`].
pub mod tooltip {
    /// Hover time before a tooltip opens, in milliseconds.
    pub const SHOW_DELAY_MS: u32 = operator_ui_tokens::motion::TOOLTIP_SHOW_DELAY_MS;
    /// Auto-dismiss delay after a touch tooltip is released, in milliseconds.
    pub const DISMISS_MS: u32 = 3500;
    /// Touch hold time that opens a tooltip, in milliseconds.
    pub const LONG_PRESS_MS: u32 = operator_ui_tokens::motion::LONG_PRESS_MS;
    /// Distance from the anchor, in DU.
    pub const GAP_DU: f32 = 1.5;
    /// Maximum width, in DU.
    pub const MAX_WIDTH_DU: f32 = 75.0;
}

/// Context marker provided by [`Tooltip`] to its children.
///
/// Components that show their own tooltip (such as [`crate::buttons::IconButton`]) skip it
/// when they are already inside a [`Tooltip`], so only one tooltip ever opens.
#[derive(Clone, Copy, Debug)]
pub struct InsideTooltip;

/// Position of a fixed floating layer in CSS pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloatPos {
    pub left: f64,
    pub top: f64,
}

/// Current viewport size in CSS pixels.
pub fn viewport() -> (f64, f64) {
    let Some(window) = web_sys::window() else {
        return (0.0, 0.0);
    };
    let read = |v: Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>| {
        v.ok().and_then(|v| v.as_f64()).unwrap_or(0.0)
    };
    (read(window.inner_width()), read(window.inner_height()))
}

/// Centered above the anchor, flipped below when there is no room, clamped to the viewport.
pub fn place_above(anchor: &web_sys::DomRect, w: f64, h: f64, gap_du: f32) -> FloatPos {
    let (vw, vh) = viewport();
    let gap = f64::from(gap_du * DU_POINTS);
    let left = (anchor.left() + (anchor.width() - w) / 2.0).clamp(0.0, (vw - w).max(0.0));
    let top = if anchor.top() >= h + gap {
        anchor.top() - h - gap
    } else {
        (anchor.bottom() + gap).min((vh - h).max(0.0))
    };
    FloatPos { left, top }
}

/// Below the anchor aligned to its end edge, flipped above when there is no room, clamped to the viewport.
pub fn place_below_end(anchor: &web_sys::DomRect, w: f64, h: f64) -> FloatPos {
    let (vw, vh) = viewport();
    let left = (anchor.right() - w).clamp(0.0, (vw - w).max(0.0));
    let top = if anchor.bottom() + h <= vh || anchor.top() < h {
        anchor.bottom().min((vh - h).max(0.0))
    } else {
        anchor.top() - h
    };
    FloatPos { left, top }
}

/// At a pointer position, shifted inside the viewport.
pub fn place_at(x: f64, y: f64, w: f64, h: f64) -> FloatPos {
    let (vw, vh) = viewport();
    let left = if x + w <= vw { x } else { (x - w).max(0.0) };
    let top = if y + h <= vh { y } else { (y - h).max(0.0) };
    FloatPos { left, top }
}

/// True when the event target is inside `el`.
pub fn event_inside(e: &web_sys::Event, el: &web_sys::Element) -> bool {
    e.target()
        .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
        .is_some_and(|n| el.contains(Some(&n)))
}

/// Hint for an element: opens on hover after a delay, on keyboard focus, or on a touch long press.
///
/// The tooltip is informational only; it never contains interactive content.
#[component]
pub fn Tooltip(
    #[prop(into)] text: Signal<String>,
    /// Hover delay before opening, in milliseconds.
    #[prop(default = tooltip::SHOW_DELAY_MS)]
    delay_ms: u32,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let host = NodeRef::<leptos::html::Span>::new();
    let tip = NodeRef::<leptos::html::Div>::new();
    let open = RwSignal::new(false);
    let pos = RwSignal::new(None::<FloatPos>);
    let show_job = JobToken::new();
    let pressed = RwSignal::new(false);
    let long_pressed = StoredValue::new(false);

    let place = move || {
        let (Some(h), Some(t)) = (
            host.try_get_untracked().flatten(),
            tip.try_get_untracked().flatten(),
        ) else {
            return;
        };
        let rect = h.get_bounding_client_rect();
        pos.set(Some(place_above(
            &rect,
            f64::from(t.offset_width()),
            f64::from(t.offset_height()),
            tooltip::GAP_DU,
        )));
    };
    let show = move || {
        if !enabled.get_untracked() || text.with_untracked(String::is_empty) {
            return;
        }
        show_job.next();
        pos.set(None);
        open.set(true);
        request_animation_frame(place);
    };
    let hide = move || {
        show_job.next();
        open.set(false);
    };
    let show_later = move |ms: u32| {
        let id = show_job.next();
        set_timeout(
            move || {
                if show_job.is_current(id) {
                    show();
                }
            },
            Duration::from_millis(u64::from(ms)),
        );
    };

    provide_context(InsideTooltip);

    let handle = window_event_listener(ev::pointerdown, move |e| {
        if open.get_untracked() && !host.get_untracked().is_some_and(|h| event_inside(&e, &h)) {
            hide();
        }
    });
    on_cleanup(move || handle.remove());

    view! {
        <span
            node_ref=host
            class="op-tooltip-anchor"
            on:pointerenter=move |e| {
                if e.pointer_type() == "mouse" {
                    show_later(delay_ms);
                }
            }
            on:pointerleave=move |e| {
                if e.pointer_type() == "mouse" {
                    hide();
                }
            }
            on:pointerdown=move |e| {
                if e.pointer_type() == "mouse" {
                    hide();
                    return;
                }
                pressed.set(true);
                long_pressed.set_value(false);
                let id = show_job.next();
                set_timeout(
                    move || {
                        if show_job.is_current(id) && pressed.get_untracked() {
                            long_pressed.set_value(true);
                            show();
                        }
                    },
                    Duration::from_millis(u64::from(tooltip::LONG_PRESS_MS)),
                );
            }
            on:pointerup=move |e| {
                if e.pointer_type() == "mouse" {
                    return;
                }
                pressed.set(false);
                if open.get_untracked() {
                    let id = show_job.next();
                    set_timeout(
                        move || {
                            if show_job.is_current(id) {
                                open.set(false);
                            }
                        },
                        Duration::from_millis(u64::from(tooltip::DISMISS_MS)),
                    );
                } else {
                    show_job.next();
                }
            }
            on:click:capture=move |e| {
                // A long press that opened the tooltip must not also activate the child.
                if long_pressed.get_value() {
                    long_pressed.set_value(false);
                    e.prevent_default();
                    e.stop_propagation();
                }
            }
            on:contextmenu=move |e| {
                if pressed.get_untracked() {
                    e.prevent_default();
                }
            }
            on:focusin=move |_| show()
            on:focusout=move |_| hide()
            on:keydown=move |e| {
                if e.key() == "Escape" {
                    hide();
                }
            }
        >
            {children()}
            <Show when=move || open.get()>
                <div
                    node_ref=tip
                    class="op-tooltip op-t-caption"
                    role="tooltip"
                    style:max-width=du(tooltip::MAX_WIDTH_DU)
                    style:visibility=move || if pos.get().is_some() { "visible" } else { "hidden" }
                    style:left=move || pos.get().map(|p| format!("{}px", p.left))
                    style:top=move || pos.get().map(|p| format!("{}px", p.top))
                >
                    {move || text.get()}
                </div>
            </Show>
        </span>
    }
}
