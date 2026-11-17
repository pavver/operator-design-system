//! Wires, frames and the minimap of a graph.

use super::camera::{Camera, WorldRect};
use super::canvas::GraphCamera;
use super::route::{Point, route_midpoint, route_path};
use leptos::prelude::*;

/// Look of a [`Wire`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WireState {
    #[default]
    Normal,
    Selected,
    /// Being dragged from a socket.
    Preview,
    /// Kept although an end lost its socket.
    Detached,
}

/// SVG layer for [`Wire`]s in world coordinates; place it first in the canvas world.
#[component]
pub fn WireLayer(children: Children) -> impl IntoView {
    view! { <svg class="op-wires" aria-hidden="true">{children()}</svg> }
}

/// A connection between two world points, optionally through route points.
#[component]
pub fn Wire(
    #[prop(into)] from: Signal<Point>,
    #[prop(into)] to: Signal<Point>,
    #[prop(optional, into)] points: Signal<Vec<Point>>,
    /// Wire colour, any CSS colour (a type colour).
    #[prop(optional, into)]
    color: Option<Signal<String>>,
    #[prop(optional, into)] state: Signal<WireState>,
    /// Control flow wires are drawn thicker.
    #[prop(optional)]
    exec: bool,
    /// Text in a box at the middle of the wire (a conversion, a value).
    #[prop(optional, into)]
    label: Option<Signal<String>>,
    #[prop(optional)] on_pointerdown: Option<Callback<leptos::ev::PointerEvent>>,
) -> impl IntoView {
    let d = move || points.with(|p| route_path(from.get(), p, to.get()));
    let mid = move || points.with(|p| route_midpoint(from.get(), p, to.get()));
    let state_attr = move || match state.get() {
        WireState::Normal => "normal",
        WireState::Selected => "selected",
        WireState::Preview => "preview",
        WireState::Detached => "detached",
    };
    view! {
        <g
            class="op-wire"
            data-state=state_attr
            data-exec=exec.then_some("")
            style=move || color.map(|c| format!("--op-wire-color: {}", c.get()))
        >
            <path class="op-wire-hit" d=d on:pointerdown=move |e| {
                if let Some(cb) = on_pointerdown {
                    cb.run(e);
                }
            }></path>
            <path class="op-wire-line" d=d></path>
            {label.map(|l| view! {
                <foreignObject
                    class="op-wire-label-box"
                    x=move || mid().0 - 200.0
                    y=move || mid().1 - 12.0
                    width="400"
                    height="24"
                >
                    <div class="op-wire-label-wrap"><span class="op-wire-label op-t-caption">{move || l.get()}</span></div>
                </foreignObject>
            })}
        </g>
    }
}

/// A titled region behind nodes, in world units.
#[component]
pub fn GraphFrame(
    #[prop(into)] rect: Signal<WorldRect>,
    #[prop(into)] title: Signal<String>,
    /// Frame colour, any CSS colour.
    #[prop(optional, into)]
    color: Option<Signal<String>>,
    #[prop(optional, into)] selected: Signal<bool>,
    /// Pointer down on the header, for dragging the frame.
    #[prop(optional)]
    on_header_down: Option<Callback<leptos::ev::PointerEvent>>,
    /// Views at the end of the header.
    #[prop(optional)]
    actions: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            class="op-graph-frame"
            data-selected=move || selected.get().then_some("")
            style:left=move || format!("{}px", rect.get().x)
            style:top=move || format!("{}px", rect.get().y)
            style:width=move || format!("{}px", rect.get().w)
            style:height=move || format!("{}px", rect.get().h)
            style=move || color.map(|c| format!("--op-frame-color: {}", c.get()))
        >
            <div class="op-graph-frame-head" on:pointerdown=move |e| {
                if let Some(cb) = on_header_down {
                    cb.run(e);
                }
            }>
                <span class="op-graph-frame-title">{move || title.get()}</span>
                {actions.map(|a| a())}
            </div>
        </div>
    }
}

/// One rectangle of a [`Minimap`].
#[derive(Clone, Debug, PartialEq)]
pub struct MiniItem {
    pub rect: WorldRect,
    /// Any CSS colour; the default is the neutral node colour.
    pub color: Option<String>,
}

/// Scale and offset that fit `bounds` into a `w` × `h` box: `box = (world - origin) * scale`.
pub fn minimap_scale(bounds: WorldRect, w: f64, h: f64) -> (f64, f64, f64) {
    let scale = (w / bounds.w.max(1.0)).min(h / bounds.h.max(1.0));
    let ox = bounds.x - (w / scale - bounds.w) / 2.0;
    let oy = bounds.y - (h / scale - bounds.h) / 2.0;
    (scale, ox, oy)
}

/// Overview of the graph in a [`GraphCanvas`](super::GraphCanvas) overlay: the items and the
/// visible area; a press or drag moves the camera there.
#[component]
pub fn Minimap(
    #[prop(into)] items: Signal<Vec<MiniItem>>,
    /// Size in DU.
    #[prop(default = (50.0, 35.0))]
    size: (f32, f32),
    #[prop(default = "Minimap".into(), into)] label: String,
) -> impl IntoView {
    let ctx = use_context::<GraphCamera>();
    let (w, h) = (
        f64::from(size.0 * operator_ui_tokens::DU_POINTS),
        f64::from(size.1 * operator_ui_tokens::DU_POINTS),
    );
    let geometry = Memo::new(move |_| {
        let view = ctx.map_or(WorldRect::default(), |c| {
            let (vw, vh) = c.viewport.get();
            c.camera.get().visible(vw, vh)
        });
        let bounds = items
            .with(|i| WorldRect::bounds(i.iter().map(|m| m.rect)))
            .map_or(view, |b| b.union(view));
        (view, minimap_scale(bounds, w, h))
    });
    let dragging = StoredValue::new(false);
    let move_to = move |e: &leptos::ev::PointerEvent| {
        let Some(c) = ctx else { return };
        let Some(target) = e
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        else {
            return;
        };
        let r = target.get_bounding_client_rect();
        let (_, (scale, ox, oy)) = geometry.get_untracked();
        let wx = (f64::from(e.client_x()) - r.left()) / scale + ox;
        let wy = (f64::from(e.client_y()) - r.top()) / scale + oy;
        let (vw, vh) = c.viewport.get_untracked();
        c.camera
            .update(|cam: &mut Camera| *cam = cam.centered_on(wx, wy, vw, vh));
    };
    let rect_style = move |rect: WorldRect| {
        let (_, (scale, ox, oy)) = geometry.get();
        format!(
            "left:{}px;top:{}px;width:{}px;height:{}px",
            (rect.x - ox) * scale,
            (rect.y - oy) * scale,
            (rect.w * scale).max(1.0),
            (rect.h * scale).max(1.0)
        )
    };
    let entries = move || items.get().into_iter().enumerate().collect::<Vec<_>>();
    let entry_key = |(i, m): &(usize, MiniItem)| (*i, format!("{:?}", m.rect));
    view! {
        <div
            class="op-minimap"
            role="img"
            aria-label=label
            style:width=format!("{w}px")
            style:height=format!("{h}px")
            on:pointerdown=move |e| {
                e.stop_propagation();
                dragging.set_value(true);
                if let Some(t) = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                    let _ = t.set_pointer_capture(e.pointer_id());
                }
                move_to(&e);
            }
            on:pointermove=move |e| {
                if dragging.get_value() {
                    move_to(&e);
                }
            }
            on:pointerup=move |_| dragging.set_value(false)
        >
            <For each=entries key=entry_key let:entry>
                <span
                    class="op-minimap-item"
                    style=move || {
                        let base = rect_style(entry.1.rect);
                        match &entry.1.color {
                            Some(c) => format!("{base};background-color:{c}"),
                            None => base,
                        }
                    }
                ></span>
            </For>
            <span class="op-minimap-view" style=move || rect_style(geometry.get().0)></span>
        </div>
    }
}

use wasm_bindgen::JsCast;
