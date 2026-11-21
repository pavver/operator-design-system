//! Preserve field-family variables when a popup moves to the overlay host.

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

pub(crate) fn field_popup_style(
    host: NodeRef<leptos::html::Div>,
    open: RwSignal<bool>,
    instance: Signal<Option<String>>,
) -> Signal<Option<String>> {
    let inherited = RwSignal::new(String::new());
    Effect::new(move |_| {
        if !open.get() {
            return;
        }
        instance.track();
        let Some(host) = host.get() else { return };
        let observed = host.clone();
        let refresh = move || {
            let Some(computed) = web_sys::window()
                .and_then(|window| window.get_computed_style(&observed).ok().flatten())
            else {
                return;
            };
            let mut declarations = String::new();
            for i in 0..computed.length() {
                let name = computed.item(i);
                if name.starts_with("--op-field-")
                    && let Ok(value) = computed.get_property_value(&name)
                {
                    declarations.push_str(&format!("{name}:{value};"));
                }
            }
            inherited.set(declarations);
        };
        refresh();
        let callback = Closure::<dyn FnMut()>::new(refresh);
        let observer = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()).ok();
        if let Some(observer) = &observer {
            let options = web_sys::MutationObserverInit::new();
            options.set_attributes(true);
            let mut ancestor = host.parent_element();
            while let Some(element) = ancestor {
                let _ = observer.observe_with_options(&element, &options);
                ancestor = element.parent_element();
            }
            let _ = observer.observe_with_options(&host, &options);
        }
        let keep = SendWrapper::new((observer, callback));
        on_cleanup(move || {
            if let Some(observer) = &keep.0 {
                observer.disconnect();
            }
        });
    });
    Signal::derive(move || {
        let declarations = inherited.get();
        (!declarations.is_empty()).then_some(declarations)
    })
}

/// Popover's label prop is static; keep its DOM label reactive for language changes.
pub(crate) fn field_popup_label(node: NodeRef<leptos::html::Div>, text: Signal<String>) {
    Effect::new(move |_| {
        let text = text.get();
        if let Some(element) = node.get() {
            let _ = element.set_attribute("aria-label", &text);
        }
    });
}
