//! Observe resolved CSS sizes so subtree variables, reactive props and root styles resize the scene.
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

pub(super) fn observe(
    root: NodeRef<leptos::html::Button>,
    probe: NodeRef<leptos::html::Span>,
) -> RwSignal<(f32, f32, f32, f32)> {
    let geometry = RwSignal::new((68.0, 30.0, 2.5, 4.0));
    Effect::new(move |_| {
        let (Some(root), Some(probe)) = (root.get(), probe.get()) else {
            return;
        };
        let refresh = {
            let root = root.clone();
            let probe = probe.clone();
            move || {
                let track = root.get_bounding_client_rect();
                let sizes = probe.get_bounding_client_rect();
                if track.width() > 0.0 && track.height() > 0.0 {
                    let next = (
                        track.width() as f32,
                        track.height() as f32,
                        sizes.width() as f32,
                        sizes.height() as f32,
                    );
                    if geometry.get_untracked() != next {
                        geometry.set(next);
                    }
                }
            }
        };
        refresh();
        let callback = Closure::<dyn FnMut()>::new(refresh);
        let observer = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()).ok();
        if let Some(observer) = &observer {
            observer.observe(&root);
            observer.observe(&probe);
        }
        let keep = SendWrapper::new((observer, callback));
        on_cleanup(move || {
            if let Some(observer) = &keep.0 {
                observer.disconnect();
            }
        });
    });
    geometry
}
