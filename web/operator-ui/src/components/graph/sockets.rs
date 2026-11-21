//! Observe rendered socket geometry, including custom rows and dynamically inserted slots.
use super::{SocketPosition, SocketSide};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

pub(super) fn observe_sockets(
    card: NodeRef<leptos::html::Div>,
    report: Callback<Vec<SocketPosition>>,
) {
    Effect::new(move |_| {
        let Some(el) = card.get() else { return };
        let el: web_sys::HtmlElement = el.unchecked_ref::<web_sys::HtmlElement>().clone();
        let previous = StoredValue::new(Vec::<SocketPosition>::new());
        let measure = {
            let el = el.clone();
            move || {
                let next = measure_sockets(&el);
                if previous.get_value() != next {
                    previous.set_value(next.clone());
                    report.run(next);
                }
            }
        };
        measure();
        let resized = Closure::<dyn FnMut()>::new(measure.clone());
        let resize = web_sys::ResizeObserver::new(resized.as_ref().unchecked_ref()).ok();
        let observe = {
            let el = el.clone();
            let resize = resize.clone();
            move || {
                if let Some(observer) = &resize {
                    observer.disconnect();
                    observer.observe(&el);
                    if let Ok(nodes) = el.query_selector_all(
                        ".op-node-head, .op-node-body, .op-node-row, [data-socket]",
                    ) {
                        for i in 0..nodes.length() {
                            if let Some(node) = nodes
                                .item(i)
                                .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
                            {
                                observer.observe(&node);
                            }
                        }
                    }
                }
            }
        };
        observe();
        let changed = Closure::<dyn FnMut()>::new(move || {
            observe();
            let measure = measure.clone();
            request_animation_frame(measure);
        });
        let mutation = web_sys::MutationObserver::new(changed.as_ref().unchecked_ref()).ok();
        if let Some(observer) = &mutation {
            let options = web_sys::MutationObserverInit::new();
            options.set_subtree(true);
            // Only structure changes: size changes of rows and sockets reach the
            // ResizeObserver, and live values must not re-measure every tick.
            options.set_child_list(true);
            let _ = observer.observe_with_options(&el, &options);
        }
        let keep = SendWrapper::new((resize, resized, mutation, changed));
        on_cleanup(move || {
            if let Some(observer) = &keep.0 {
                observer.disconnect();
            }
            if let Some(observer) = &keep.2 {
                observer.disconnect();
            }
        });
    });
}

/// Socket centres of a card in world units, independent of the camera zoom.
pub(super) fn measure_sockets(card: &web_sys::HtmlElement) -> Vec<SocketPosition> {
    let rect = card.get_bounding_client_rect();
    let width = f64::from(card.offset_width());
    let scale = if width > 0.0 {
        rect.width() / width
    } else {
        1.0
    };
    let Ok(list) = card.query_selector_all("[data-socket]") else {
        return Vec::new();
    };
    (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|n| n.dyn_into::<web_sys::Element>().ok())
        .filter_map(|el| {
            let id = el.get_attribute("data-socket")?;
            let side = if el.get_attribute("data-side").as_deref() == Some("out") {
                SocketSide::Out
            } else {
                SocketSide::In
            };
            let r = el.get_bounding_client_rect();
            let y = (r.top() + r.height() / 2.0 - rect.top()) / scale;
            Some((id, side, y))
        })
        .collect()
}
