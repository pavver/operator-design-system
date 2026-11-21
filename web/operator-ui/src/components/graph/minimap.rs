//! Graph overview with a viewport and pointer navigation.
use super::camera::{Camera, WorldRect};
use super::canvas::GraphCamera;
use crate::core::{Metric, root_class, root_style, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::DU_POINTS;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

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
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] minimap_width: MaybeProp<f32>,
    #[prop(optional, into)] minimap_height: MaybeProp<f32>,
    #[prop(optional, into)] minimap_offset_x: MaybeProp<f32>,
    #[prop(optional, into)] minimap_offset_y: MaybeProp<f32>,
    #[prop(optional, into)] minimap_item_min_size: MaybeProp<f32>,
    #[prop(optional, into)] border_width: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let ctx = use_context::<GraphCamera>();
    let label = ui_text(label, "graph.minimap", "Minimap");
    let host = NodeRef::<leptos::html::Div>::new();
    let minimum_probe = NodeRef::<leptos::html::Span>::new();
    let measured = RwSignal::new(None::<(f64, f64, f64)>);
    let dimensions = move || {
        if let Some((w, h, _)) = measured.get() {
            return (w, h);
        }
        (
            f64::from(minimap_width.get().unwrap_or(size.0) * DU_POINTS),
            f64::from(minimap_height.get().unwrap_or(size.1) * DU_POINTS),
        )
    };
    let custom_style = root_style(
        vec![
            Metric::du("--op-graph-minimap-width", minimap_width),
            Metric::du("--op-graph-minimap-height", minimap_height),
            Metric::du("--op-graph-minimap-offset-x", minimap_offset_x),
            Metric::du("--op-graph-minimap-offset-y", minimap_offset_y),
            Metric::du("--op-graph-minimap-item-min-size", minimap_item_min_size),
            Metric::du("--op-graph-border-width", border_width),
        ],
        style,
    );
    Effect::new(move |_| {
        let (Some(host), Some(probe)) = (host.get(), minimum_probe.get()) else {
            return;
        };
        let (measure_host, measure_probe) = (host.clone(), probe.clone());
        let refresh = Closure::<dyn FnMut()>::new(move || {
            let next = (
                f64::from(measure_host.client_width()),
                f64::from(measure_host.client_height()),
                f64::from(measure_probe.offset_width()),
            );
            if next.0 > 0.0 && next.1 > 0.0 && measured.get_untracked() != Some(next) {
                measured.set(Some(next));
            }
        });
        let observer = web_sys::ResizeObserver::new(refresh.as_ref().unchecked_ref()).ok();
        if let Some(observer) = &observer {
            observer.observe(&host);
            observer.observe(&probe);
        }
        let keep = SendWrapper::new((observer, refresh));
        on_cleanup(move || {
            if let Some(observer) = &keep.0 {
                observer.disconnect();
            }
        });
    });
    let geometry = Memo::new(move |_| {
        let view = ctx.map_or(WorldRect::default(), |c| {
            let (vw, vh) = c.viewport.get();
            c.camera.get().visible(vw, vh)
        });
        let bounds = items
            .with(|i| WorldRect::bounds(i.iter().map(|m| m.rect)))
            .map_or(view, |b| b.union(view));
        let (w, h) = dimensions();
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
        let (w, h) = dimensions();
        let wx = (f64::from(e.client_x()) - r.left()) * w / r.width().max(1.0) / scale + ox;
        let wy = (f64::from(e.client_y()) - r.top()) * h / r.height().max(1.0) / scale + oy;
        let (vw, vh) = c.viewport.get_untracked();
        c.camera
            .update(|cam: &mut Camera| *cam = cam.centered_on(wx, wy, vw, vh));
    };
    let mini_rect = move |rect: WorldRect| {
        let (_, (scale, ox, oy)) = geometry.get();
        let minimum = measured.get().map_or_else(
            || f64::from(minimap_item_min_size.get().unwrap_or(0.25) * DU_POINTS),
            |(_, _, minimum)| minimum,
        );
        (
            (rect.x - ox) * scale,
            (rect.y - oy) * scale,
            (rect.w * scale).max(minimum),
            (rect.h * scale).max(minimum),
        )
    };
    let entries = move || items.get().into_iter().enumerate().collect::<Vec<_>>();
    let entry_key = |(i, m): &(usize, MiniItem)| (*i, format!("{:?}{:?}", m.rect, m.color));
    view! {
        <div
            node_ref=host
            class=root_class("op-minimap", class)
            role="img"
            aria-label=move || label.get()
            style=move || format!("--op-graph-minimap-default-width:calc(var(--op-du) * {});--op-graph-minimap-default-height:calc(var(--op-du) * {});{}", size.0, size.1, custom_style().unwrap_or_default())
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
            on:pointercancel=move |_| dragging.set_value(false)
        >
            <span node_ref=minimum_probe class="op-minimap-probe" aria-hidden="true"></span>
            <svg class="op-minimap-map" viewBox=move || { let (w,h) = dimensions(); format!("0 0 {w} {h}") } preserveAspectRatio="none" aria-hidden="true">
                <For each=entries key=entry_key let:entry>
                    <rect class="op-minimap-item"
                        x=move || mini_rect(entry.1.rect).0 y=move || mini_rect(entry.1.rect).1
                        width=move || mini_rect(entry.1.rect).2 height=move || mini_rect(entry.1.rect).3
                        style=move || entry.1.color.clone().map(|c| format!("--op-graph-minimap-item:{c}"))
                    ></rect>
                </For>
                <foreignObject x=move || mini_rect(geometry.get().0).0 y=move || mini_rect(geometry.get().0).1
                    width=move || mini_rect(geometry.get().0).2 height=move || mini_rect(geometry.get().0).3>
                    <div class="op-minimap-view"></div>
                </foreignObject>
            </svg>
        </div>
    }
}
