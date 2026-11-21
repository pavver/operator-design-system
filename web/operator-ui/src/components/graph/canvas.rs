//! GraphCanvas: a pannable, zoomable surface with a grid.

use super::camera::Camera;
use crate::core::{Metric, Slot, root_class, root_style};
use leptos::prelude::*;

/// Camera of the nearest [`GraphCanvas`], for components inside it (minimap, overlays).
#[derive(Clone, Copy)]
pub struct GraphCamera {
    pub camera: RwSignal<Camera>,
    /// Viewport size in screen pixels.
    pub viewport: RwSignal<(f64, f64)>,
}

/// Wheel delta to zoom factor: one notch (100 px) zooms by about 10 %.
pub fn wheel_factor(delta_y: f64) -> f64 {
    (-delta_y * 0.001).exp()
}

/// Surface for node graphs: the world layer moves with the camera, the overlay stays on screen.
///
/// The wheel zooms around the pointer, the middle button pans anywhere, and with
/// `pan_on_background` a primary drag on the empty background pans too. The grid follows
/// the camera; its cell is `grid` DU in world units.
#[component]
pub fn GraphCanvas(
    camera: RwSignal<Camera>,
    #[prop(default = 0.25)] min_zoom: f64,
    #[prop(default = 2.0)] max_zoom: f64,
    /// Grid cell in DU; `0` hides the grid.
    #[prop(optional, into)]
    grid: MaybeProp<f32>,
    /// A primary-button drag on the empty background pans.
    #[prop(default = true)]
    pan_on_background: bool,
    /// Pointer down on the empty background (for selection boxes), in screen coordinates.
    #[prop(optional)]
    on_background_down: Option<Callback<leptos::ev::PointerEvent>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] grid_width: MaybeProp<f32>,
    /// Shown in screen space when `is_empty` is true.
    #[prop(optional, into)]
    empty: Slot,
    #[prop(optional, into)] is_empty: Signal<bool>,
    /// Screen-space views above the world (toolbars, minimap).
    #[prop(optional)]
    overlay: Option<ChildrenFn>,
    /// World-space content: frames, wires and nodes.
    children: Children,
) -> impl IntoView {
    let host = NodeRef::<leptos::html::Div>::new();
    let viewport = RwSignal::new((0.0, 0.0));
    provide_context(GraphCamera { camera, viewport });
    let drag = StoredValue::new(None::<(i32, f64, f64)>);

    Effect::new(move |_| {
        if let Some(el) = host.get() {
            viewport.set((f64::from(el.client_width()), f64::from(el.client_height())));
        }
    });
    let resize = window_event_listener(leptos::ev::resize, move |_| {
        if let Some(el) = host.get_untracked() {
            viewport.set((f64::from(el.client_width()), f64::from(el.client_height())));
        }
    });
    on_cleanup(move || resize.remove());

    let local = move |x: i32, y: i32| {
        host.get_untracked()
            .map_or((f64::from(x), f64::from(y)), |el| {
                let r = el.get_bounding_client_rect();
                (f64::from(x) - r.left(), f64::from(y) - r.top())
            })
    };
    let grid_style = move || {
        let c = camera.get();
        format!(
            "--op-graph-zoom:{zoom}; background-position: {x}px {y}px, {x}px {y}px",
            zoom = c.zoom,
            x = c.x,
            y = c.y
        )
    };
    let world_style = move || {
        let c = camera.get();
        format!(
            "transform: translate({}px, {}px) scale({})",
            c.x, c.y, c.zoom
        )
    };
    let custom_style = root_style(
        vec![
            Metric::du("--op-graph-grid", grid),
            Metric::du("--op-graph-grid-width", grid_width),
        ],
        style,
    );
    view! {
        <div
            node_ref=host
            class=root_class("op-graph", class)
            data-grid=move || (grid.get().unwrap_or(5.0) > 0.0).then_some("")
            style=move || format!("{};{}", grid_style(), custom_style().unwrap_or_default())
            on:wheel=move |e| {
                e.prevent_default();
                let (sx, sy) = local(e.client_x(), e.client_y());
                camera.update(|c| *c = c.zoom_at(sx, sy, wheel_factor(e.delta_y()), min_zoom, max_zoom));
            }
            on:pointerdown=move |e| {
                let background = e.target() == e.current_target();
                let middle = e.button() == 1;
                if middle || (background && e.button() == 0 && pan_on_background) {
                    e.prevent_default();
                    drag.set_value(Some((e.pointer_id(), f64::from(e.client_x()), f64::from(e.client_y()))));
                    if let Some(el) = host.get_untracked() {
                        let _ = el.set_pointer_capture(e.pointer_id());
                    }
                }
                if background && e.button() == 0 && let Some(cb) = on_background_down {
                    cb.run(e);
                }
            }
            on:pointermove=move |e| {
                if let Some((id, x, y)) = drag.get_value() && id == e.pointer_id() {
                    let (nx, ny) = (f64::from(e.client_x()), f64::from(e.client_y()));
                    camera.update(|c| *c = c.pan(nx - x, ny - y));
                    drag.set_value(Some((id, nx, ny)));
                }
            }
            on:pointerup=move |_| drag.set_value(None)
            on:pointercancel=move |_| drag.set_value(None)
        >
            <div class="op-graph-world" style=world_style>
                {children()}
            </div>
            <Show when=move || is_empty.get()><div class="op-graph-empty">{empty.run()}</div></Show>
            {overlay.map(|o| view! { <div class="op-graph-overlay">{o()}</div> })}
        </div>
    }
}
