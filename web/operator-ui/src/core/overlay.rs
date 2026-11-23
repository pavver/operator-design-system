//! Viewport-aware placement helpers for floating layers.

use operator_ui_tokens::DU_POINTS;
use wasm_bindgen::JsCast;

/// Position of a fixed floating layer in CSS pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloatPos {
    pub left: f64,
    pub top: f64,
}

/// Current viewport size in CSS pixels, without the page scrollbars.
pub fn viewport() -> (f64, f64) {
    let Some(window) = web_sys::window() else {
        return (0.0, 0.0);
    };
    if let Some(root) = window.document().and_then(|d| d.document_element()) {
        let (w, h) = (root.client_width(), root.client_height());
        if w > 0 && h > 0 {
            return (f64::from(w), f64::from(h));
        }
    }
    let read = |v: Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>| {
        v.ok().and_then(|v| v.as_f64()).unwrap_or(0.0)
    };
    (read(window.inner_width()), read(window.inner_height()))
}

/// Centered above the anchor, flipped below when there is no room, kept inside the viewport.
///
/// Use [`place_centered`] to also get the size limits for a layer that fits on neither side.
pub fn place_above(anchor: &web_sys::DomRect, w: f64, h: f64, gap_du: f32) -> FloatPos {
    place_centered(anchor, w, h, gap_du, true).0
}

/// Size limits a placement applies when the layer does not fit the viewport, in CSS pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FloatLimit {
    /// Set when the layer fits neither above nor below; the layer scrolls inside it.
    pub max_height: Option<f64>,
    /// Set when the layer is wider than the viewport.
    pub max_width: Option<f64>,
}

/// Preferred side of the anchor for a layer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayerSide {
    Above,
    #[default]
    Below,
    Left,
    Right,
}

impl LayerSide {
    fn vertical(self) -> bool {
        matches!(self, Self::Above | Self::Below)
    }
}

/// Preferred alignment of a layer across its side of the anchor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayerAlign {
    /// Start edges aligned: the layer sticks out to the right (above or below the anchor) or
    /// downwards (left or right of it).
    Start,
    /// Centered on the anchor.
    #[default]
    Center,
    /// End edges aligned: the layer sticks out to the left (above or below the anchor) or
    /// upwards (left or right of it).
    End,
}

/// A box in viewport coordinates, in CSS pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl Bounds {
    fn right(self) -> f64 {
        self.left + self.width
    }
    fn bottom(self) -> f64 {
        self.top + self.height
    }
}

/// Position on the cross axis for an alignment, kept inside `0..extent`.
fn cross(start: f64, len: f64, size: f64, extent: f64, align: LayerAlign) -> f64 {
    let pos = match align {
        LayerAlign::Start => start,
        LayerAlign::Center => start + (len - size) / 2.0,
        LayerAlign::End => start + len - size,
    };
    pos.clamp(0.0, (extent - size).max(0.0))
}

/// Places a `w`×`h` layer next to `anchor` inside a `vw`×`vh` viewport, `gap` pixels away.
///
/// `side` and `align` are preferences: the layer takes the first side it fits on (the
/// preferred one, its opposite, then below and above for a side placement) and shifts on the
/// cross axis to stay inside the viewport. When it fits nowhere, it takes the roomier of
/// above and below and gets a `max_height`, so its content scrolls. It is never wider or
/// taller than the viewport.
#[allow(clippy::too_many_arguments)]
pub fn fit_layer(
    anchor: Bounds,
    w: f64,
    h: f64,
    vw: f64,
    vh: f64,
    gap: f64,
    side: LayerSide,
    align: LayerAlign,
) -> (FloatPos, FloatLimit) {
    use LayerSide::{Above, Below, Left, Right};
    let max_width = (w > vw).then_some(vw);
    let w = w.min(vw);
    let room = |s: LayerSide| {
        match s {
            Above => anchor.top - gap,
            Below => vh - anchor.bottom() - gap,
            Left => anchor.left - gap,
            Right => vw - anchor.right() - gap,
        }
        .max(0.0)
    };
    let fits = |s: LayerSide| {
        if s.vertical() {
            h <= room(s)
        } else {
            w <= room(s)
        }
    };
    let order: &[LayerSide] = match side {
        Above => &[Above, Below],
        Below => &[Below, Above],
        Left => &[Left, Right, Below, Above],
        Right => &[Right, Left, Below, Above],
    };
    let chosen = order.iter().copied().find(|&s| fits(s));
    let s = chosen.unwrap_or(if room(Above) > room(Below) {
        Above
    } else {
        Below
    });
    let limit_h = if s.vertical() { room(s) } else { vh };
    let max_height = (h > limit_h).then_some(limit_h);
    let shown = h.min(limit_h);
    let (left, top) = match s {
        Above => (
            cross(anchor.left, anchor.width, w, vw, align),
            anchor.top - gap - shown,
        ),
        Below => (
            cross(anchor.left, anchor.width, w, vw, align),
            anchor.bottom() + gap,
        ),
        Left => (
            anchor.left - gap - w,
            cross(anchor.top, anchor.height, shown, vh, align),
        ),
        Right => (
            anchor.right() + gap,
            cross(anchor.top, anchor.height, shown, vh, align),
        ),
    };
    (
        FloatPos { left, top },
        FloatLimit {
            max_height,
            max_width,
        },
    )
}

/// [`fit_layer`] in the current viewport, with the gap in DU.
pub fn place_on_side(
    anchor: &web_sys::DomRect,
    w: f64,
    h: f64,
    gap_du: f32,
    side: LayerSide,
    align: LayerAlign,
) -> (FloatPos, FloatLimit) {
    let (vw, vh) = viewport();
    let bounds = Bounds {
        left: anchor.left(),
        top: anchor.top(),
        width: anchor.width(),
        height: anchor.height(),
    };
    fit_layer(
        bounds,
        w,
        h,
        vw,
        vh,
        f64::from(gap_du * DU_POINTS),
        side,
        align,
    )
}

/// Centered on the anchor, above or below it; see [`place_on_side`].
pub fn place_centered(
    anchor: &web_sys::DomRect,
    w: f64,
    h: f64,
    gap_du: f32,
    prefer_above: bool,
) -> (FloatPos, FloatLimit) {
    let side = if prefer_above {
        LayerSide::Above
    } else {
        LayerSide::Below
    };
    place_on_side(anchor, w, h, gap_du, side, LayerAlign::Center)
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

/// Below the anchor aligned to its start edge, flipped above when there is no room, clamped to the viewport.
pub fn place_below_start(anchor: &web_sys::DomRect, w: f64, h: f64, gap_du: f32) -> FloatPos {
    let (vw, vh) = viewport();
    let gap = f64::from(gap_du * DU_POINTS);
    let left = anchor.left().clamp(0.0, (vw - w).max(0.0));
    let below = anchor.bottom() + gap;
    let top = if below + h <= vh || anchor.top() - gap < h {
        below.min((vh - h).max(0.0))
    } else {
        anchor.top() - gap - h
    };
    FloatPos { left, top }
}

/// Root host of floating layers inside a [`ThemeRoot`](crate::core::ThemeRoot).
///
/// Layers are mounted here instead of next to their anchor, so ancestors with transforms,
/// clipping or their own stacking context never move or cut them, and the theme still applies.
#[derive(Clone, Copy)]
pub struct OverlayHost(pub leptos::prelude::NodeRef<leptos::html::Div>);

/// Beside the anchor, top edges aligned (submenus): to the right, or to the left without room.
pub fn place_side_start(anchor: &web_sys::DomRect, w: f64, h: f64) -> FloatPos {
    let (vw, vh) = viewport();
    let left = if anchor.right() + w <= vw {
        anchor.right()
    } else {
        (anchor.left() - w).max(0.0)
    };
    let top = anchor.top().clamp(0.0, (vh - h).max(0.0));
    FloatPos { left, top }
}

/// True when the event target is inside the layer `own` or inside a layer opened from it.
///
/// Layers carry `data-op-layer` (their id) and `data-op-parent` (the id of the layer they belong to).
pub fn event_in_layer_tree(e: &web_sys::Event, own: u64) -> bool {
    let mut el = e
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .and_then(|el| el.closest("[data-op-layer]").ok().flatten());
    let doc = web_sys::window().and_then(|w| w.document());
    // Bounded walk: layer chains are short, and a cycle must never hang the page.
    for _ in 0..16 {
        let Some(layer) = el else { return false };
        if layer.get_attribute("data-op-layer").as_deref() == Some(own.to_string().as_str()) {
            return true;
        }
        let Some(parent) = layer.get_attribute("data-op-parent") else {
            return false;
        };
        el = doc.as_ref().and_then(|d| {
            d.query_selector(&format!("[data-op-layer=\"{parent}\"]"))
                .ok()
                .flatten()
        });
    }
    false
}

#[cfg(test)]
#[path = "overlay_tests.rs"]
mod tests;
