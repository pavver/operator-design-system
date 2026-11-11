//! Floating layers: viewport-aware placement helpers.

use operator_ui_tokens::DU_POINTS;
use wasm_bindgen::JsCast;

/// Tooltip timings and geometry used by [`crate::text::MarqueeText`].
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
