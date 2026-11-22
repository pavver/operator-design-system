//! Lazy loading via web_sys::IntersectionObserver.

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Observes a DOM element and triggers the `is_visible` signal when it enters the viewport.
pub fn use_intersection_observer(target_ref: NodeRef<leptos::html::Div>) -> RwSignal<bool> {
    let is_visible = RwSignal::new(false);

    Effect::new(move |_| {
        let Some(el) = target_ref.get() else {
            return;
        };

        let window = match web_sys::window() {
            Some(w) => w,
            None => {
                is_visible.set(true);
                return;
            }
        };

        let has_io = js_sys::Reflect::has(
            &window,
            &wasm_bindgen::JsValue::from_str("IntersectionObserver"),
        )
        .unwrap_or(false);
        if !has_io {
            is_visible.set(true);
            return;
        }

        let is_visible_writer = is_visible;
        let element: web_sys::Element = (*el).clone().into();
        let target_el = element.clone();

        let callback = Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::new(
            move |entries: js_sys::Array, observer: web_sys::IntersectionObserver| {
                for i in 0..entries.length() {
                    let entry_val = entries.get(i);
                    if let Ok(entry) = entry_val.dyn_into::<web_sys::IntersectionObserverEntry>()
                        && entry.is_intersecting()
                    {
                        is_visible_writer.set(true);
                        observer.unobserve(&target_el);
                    }
                }
            },
        );

        if let Ok(observer) = web_sys::IntersectionObserver::new(callback.as_ref().unchecked_ref())
        {
            observer.observe(&element);
            let keep = SendWrapper::new((observer, callback));
            on_cleanup(move || {
                keep.0.disconnect();
            });
        } else {
            is_visible.set(true);
        }
    });

    is_visible
}
