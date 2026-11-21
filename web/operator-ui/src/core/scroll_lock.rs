//! Counted page scroll locks for modal overlays.

use leptos::prelude::*;

#[cfg(any(feature = "dialog", feature = "sheet", feature = "gallery-viewer"))]
thread_local! {
    static SCROLL_LOCKS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// Sets or clears the page scroll lock attribute on the document element.
#[cfg(any(feature = "dialog", feature = "sheet", feature = "gallery-viewer"))]
fn set_page_scroll_lock(locked: bool) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(root) = window.document().and_then(|d| d.document_element()) else {
        return;
    };
    let _ = if locked {
        // Keep the gutter only when a classic scrollbar takes layout space, so nothing shifts.
        let inner = window
            .inner_width()
            .ok()
            .and_then(|w| w.as_f64())
            .unwrap_or_default();
        let gutter = inner - f64::from(root.client_width()) > 0.0;
        root.set_attribute("data-op-scroll-lock", if gutter { "gutter" } else { "" })
    } else {
        root.remove_attribute("data-op-scroll-lock")
    };
}

/// Stops the page behind a modal overlay from scrolling while `open` is true.
/// Locks are counted, so stacked overlays release the page only when the last one closes.
#[cfg(any(feature = "dialog", feature = "sheet", feature = "gallery-viewer"))]
pub(crate) fn use_page_scroll_lock(open: RwSignal<bool>) {
    let held = StoredValue::new(false);
    let apply = move |want: bool| {
        if held.get_value() == want {
            return;
        }
        held.set_value(want);
        let count = SCROLL_LOCKS.with(|c| {
            let next = if want {
                c.get() + 1
            } else {
                c.get().saturating_sub(1)
            };
            c.set(next);
            next
        });
        set_page_scroll_lock(count > 0);
    };
    Effect::new(move |_| apply(open.get()));
    on_cleanup(move || apply(false));
}
