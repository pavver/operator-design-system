//! Bounded pointer ripples and their release lifecycle.

use leptos::prelude::set_timeout;
use std::time::Duration;
use wasm_bindgen::JsCast;

/// Spawns a bounded ripple inside the event's current target (host must be `.op-rippled`).
pub fn spawn_ripple(ev: &web_sys::PointerEvent) {
    let Some(host) = ev
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return;
    };
    let Some(document) = host.owner_document() else {
        return;
    };
    let Ok(node) = document.create_element("span") else {
        return;
    };
    let Ok(ripple) = node.dyn_into::<web_sys::HtmlElement>() else {
        return;
    };
    let rect = host.get_bounding_client_rect();
    let x = f64::from(ev.client_x()) - rect.left();
    let y = f64::from(ev.client_y()) - rect.top();
    let radius = (rect.width().max(x).hypot(rect.height().max(y))).max(1.0);
    ripple.set_class_name("op-ripple");
    let style = ripple.style();
    let _ = style.set_property("left", &format!("{}px", x - radius));
    let _ = style.set_property("top", &format!("{}px", y - radius));
    let _ = style.set_property("width", &format!("{}px", radius * 2.0));
    let _ = style.set_property("height", &format!("{}px", radius * 2.0));
    if host.append_child(&ripple).is_err() {
        return;
    }
    hold_ripple(ripple);
}

/// Time the ripple takes to grow over the control, in milliseconds.
const RIPPLE_GROW_MS: u64 = 330;
/// Time the ripple takes to fade after the pointer is released, in milliseconds.
const RIPPLE_FADE_MS: u64 = 220;

/// Keeps a grown ripple on screen while the pointer is held and fades it after release.
///
/// The fade starts once both the growth animation has ended and the pointer is up,
/// so a long press holds the last frame of the ripple.
fn hold_ripple(ripple: web_sys::HtmlElement) {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use web_sys::js_sys::Function;

    const GROWN: u8 = 1;
    const RELEASED: u8 = 2;
    let flags = Rc::new(Cell::new(0u8));
    let finish = {
        let ripple = ripple.clone();
        let flags = flags.clone();
        move |flag: u8| {
            flags.set(flags.get() | flag);
            if flags.get() == GROWN | RELEASED {
                let _ = ripple.set_attribute("data-out", "");
                let ripple = ripple.clone();
                set_timeout(
                    move || ripple.remove(),
                    Duration::from_millis(RIPPLE_FADE_MS),
                );
            }
        }
    };

    // A timer rather than `animationend`: the event never fires when animations are disabled.
    {
        let finish = finish.clone();
        set_timeout(move || finish(GROWN), Duration::from_millis(RIPPLE_GROW_MS));
    }

    let Some(window) = web_sys::window() else {
        return;
    };
    // The release listener removes itself from both events; the cell breaks the self-reference.
    let slot: Rc<RefCell<Option<Function>>> = Rc::new(RefCell::new(None));
    let on_release = {
        let slot = slot.clone();
        let window = window.clone();
        wasm_bindgen::closure::Closure::once_into_js(move || {
            if let Some(f) = slot.borrow_mut().take() {
                let _ = window.remove_event_listener_with_callback("pointerup", &f);
                let _ = window.remove_event_listener_with_callback("pointercancel", &f);
            }
            finish(RELEASED);
        })
    };
    let f: Function = on_release.unchecked_into();
    let _ = window.add_event_listener_with_callback("pointerup", &f);
    let _ = window.add_event_listener_with_callback("pointercancel", &f);
    *slot.borrow_mut() = Some(f);
}
