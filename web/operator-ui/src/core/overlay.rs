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

/// Centered on the anchor, on the preferred side when it fits, else on the other side.
///
/// When neither side fits, the layer takes the roomier side and gets a `max_height`, so it
/// scrolls; horizontally it shifts inside the viewport and never gets wider than it.
pub fn place_centered(
    anchor: &web_sys::DomRect,
    w: f64,
    h: f64,
    gap_du: f32,
    prefer_above: bool,
) -> (FloatPos, FloatLimit) {
    let (vw, vh) = viewport();
    let gap = f64::from(gap_du * DU_POINTS);
    let max_width = (w > vw).then_some(vw);
    let w = w.min(vw);
    let left = (anchor.left() + (anchor.width() - w) / 2.0).clamp(0.0, (vw - w).max(0.0));
    let room_above = (anchor.top() - gap).max(0.0);
    let room_below = (vh - anchor.bottom() - gap).max(0.0);
    let above = match (h <= room_above, h <= room_below) {
        (true, true) => prefer_above,
        (fits_above, fits_below) if fits_above != fits_below => fits_above,
        _ => room_above > room_below,
    };
    let room = if above { room_above } else { room_below };
    let max_height = (h > room).then_some(room);
    let shown = h.min(room);
    let top = if above {
        anchor.top() - gap - shown
    } else {
        anchor.bottom() + gap
    };
    (
        FloatPos { left, top },
        FloatLimit {
            max_height,
            max_width,
        },
    )
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
