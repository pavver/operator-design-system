//! Reactive browser observer hooks: intersection, resize, and scroll.

use leptos::ev;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Reactive state of an observed intersection.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IntersectionState {
    /// True when the element intersects the viewport or root.
    pub is_intersecting: bool,
    /// Ratio of the intersection area to the target area (0.0 to 1.0).
    pub ratio: f64,
}

/// Reactive state of an observed element's dimensions.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ResizeState {
    /// Element bounding or content box width in CSS pixels.
    pub width: f64,
    /// Element bounding or content box height in CSS pixels.
    pub height: f64,
}

/// Reactive state of a scroll container or the window.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScrollState {
    /// Vertical scroll offset from the top.
    pub scroll_top: f64,
    /// Horizontal scroll offset from the left.
    pub scroll_left: f64,
    /// Total scrollable height.
    pub scroll_height: f64,
    /// Total scrollable width.
    pub scroll_width: f64,
    /// Visible client viewport height.
    pub client_height: f64,
    /// Visible client viewport width.
    pub client_width: f64,
}

/// Configuration options for [`use_intersection`].
#[derive(Clone, Debug, Default)]
pub struct IntersectionOptions {
    /// Disconnect after the first intersection trigger.
    pub once: bool,
    /// Root margin string (e.g. "100px" or "0px 0px 200px 0px").
    pub root_margin: Option<String>,
    /// Intersection thresholds from 0.0 to 1.0.
    pub threshold: Option<f64>,
}

/// Observes a DOM element via `IntersectionObserver` and returns an [`IntersectionState`] signal.
pub fn use_intersection<E>(
    target: NodeRef<E>,
    options: IntersectionOptions,
) -> RwSignal<IntersectionState>
where
    E: leptos::html::ElementType + 'static,
    E::Output: JsCast + Clone + 'static,
{
    let state = RwSignal::new(IntersectionState::default());

    Effect::new(move |_| {
        let Some(el) = target.get() else {
            return;
        };

        let Some(window) = web_sys::window() else {
            state.set(IntersectionState {
                is_intersecting: true,
                ratio: 1.0,
            });
            return;
        };

        let has_io = js_sys::Reflect::has(
            &window,
            &wasm_bindgen::JsValue::from_str("IntersectionObserver"),
        )
        .unwrap_or(false);
        if !has_io {
            state.set(IntersectionState {
                is_intersecting: true,
                ratio: 1.0,
            });
            return;
        }

        let element: web_sys::Element = el.unchecked_ref::<web_sys::Element>().clone();
        let target_el = element.clone();
        let state_writer = state;
        let once = options.once;

        let callback = Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::new(
            move |entries: js_sys::Array, observer: web_sys::IntersectionObserver| {
                for i in 0..entries.length() {
                    let entry_val = entries.get(i);
                    if let Ok(entry) = entry_val.dyn_into::<web_sys::IntersectionObserverEntry>() {
                        let intersecting = entry.is_intersecting();
                        let ratio = entry.intersection_ratio();
                        state_writer.set(IntersectionState {
                            is_intersecting: intersecting,
                            ratio,
                        });
                        if once && intersecting {
                            observer.unobserve(&target_el);
                        }
                    }
                }
            },
        );

        let init = web_sys::IntersectionObserverInit::new();
        if let Some(ref margin) = options.root_margin {
            init.set_root_margin(margin);
        }
        if let Some(th) = options.threshold {
            init.set_threshold(&wasm_bindgen::JsValue::from_f64(th));
        }

        if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
            callback.as_ref().unchecked_ref(),
            &init,
        ) {
            observer.observe(&element);
            let keep = SendWrapper::new((observer, callback));
            on_cleanup(move || {
                keep.0.disconnect();
            });
        } else {
            state.set(IntersectionState {
                is_intersecting: true,
                ratio: 1.0,
            });
        }
    });

    state
}

/// Observes an element's size via `ResizeObserver` and returns a [`ResizeState`] signal.
pub fn use_resize<E>(target: NodeRef<E>) -> RwSignal<ResizeState>
where
    E: leptos::html::ElementType + 'static,
    E::Output: JsCast + Clone + 'static,
{
    let state = RwSignal::new(ResizeState::default());

    Effect::new(move |_| {
        let Some(el) = target.get() else {
            return;
        };

        let Some(window) = web_sys::window() else {
            return;
        };

        let has_ro =
            js_sys::Reflect::has(&window, &wasm_bindgen::JsValue::from_str("ResizeObserver"))
                .unwrap_or(false);
        if !has_ro {
            let element: web_sys::Element = el.unchecked_ref::<web_sys::Element>().clone();
            state.set(ResizeState {
                width: element.client_width() as f64,
                height: element.client_height() as f64,
            });
            return;
        }

        let element: web_sys::Element = el.unchecked_ref::<web_sys::Element>().clone();
        let target_el = element.clone();
        let state_writer = state;

        state_writer.set(ResizeState {
            width: target_el.client_width() as f64,
            height: target_el.client_height() as f64,
        });

        let callback = Closure::<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>::new(
            move |entries: js_sys::Array, _obs: web_sys::ResizeObserver| {
                for i in 0..entries.length() {
                    let entry_val = entries.get(i);
                    if let Ok(entry) = entry_val.dyn_into::<web_sys::ResizeObserverEntry>() {
                        let rect = entry.content_rect();
                        state_writer.set(ResizeState {
                            width: rect.width(),
                            height: rect.height(),
                        });
                    }
                }
            },
        );

        if let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            observer.observe(&element);
            let keep = SendWrapper::new((observer, callback));
            on_cleanup(move || {
                keep.0.disconnect();
            });
        }
    });

    state
}

/// Observes scroll position and dimensions of the window or a target element.
pub fn use_scroll<E>(target: Option<NodeRef<E>>) -> RwSignal<ScrollState>
where
    E: leptos::html::ElementType + 'static,
    E::Output: JsCast + Clone + 'static,
{
    let state = RwSignal::new(ScrollState::default());

    Effect::new(move |_| {
        let state_writer = state;

        if let Some(target_ref) = target
            && let Some(el) = target_ref.get()
        {
            let element: web_sys::Element = el.unchecked_ref::<web_sys::Element>().clone();
            let update_el = element.clone();

            let update = move || {
                state_writer.set(ScrollState {
                    scroll_top: update_el.scroll_top() as f64,
                    scroll_left: update_el.scroll_left() as f64,
                    scroll_height: update_el.scroll_height() as f64,
                    scroll_width: update_el.scroll_width() as f64,
                    client_height: update_el.client_height() as f64,
                    client_width: update_el.client_width() as f64,
                });
            };

            update();

            let handle = window_event_listener(ev::scroll, move |_| {
                update();
            });
            on_cleanup(move || handle.remove());
            return;
        }

        // Window scroll fallback
        if web_sys::window().is_none() {
            return;
        }

        let update_window = move || {
            let Some(win) = web_sys::window() else {
                return;
            };
            let doc_el = win.document().and_then(|d| d.document_element());
            let scroll_top = win.scroll_y().unwrap_or(0.0);
            let scroll_left = win.scroll_x().unwrap_or(0.0);
            let (scroll_height, scroll_width, client_height, client_width) = match doc_el {
                Some(de) => (
                    de.scroll_height() as f64,
                    de.scroll_width() as f64,
                    de.client_height() as f64,
                    de.client_width() as f64,
                ),
                None => (0.0, 0.0, 0.0, 0.0),
            };

            state_writer.set(ScrollState {
                scroll_top,
                scroll_left,
                scroll_height,
                scroll_width,
                client_height,
                client_width,
            });
        };

        update_window();

        let handle = window_event_listener(ev::scroll, move |_| {
            update_window();
        });
        on_cleanup(move || handle.remove());
    });

    state
}
