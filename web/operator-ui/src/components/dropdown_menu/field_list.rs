//! Keeps virtual-list geometry in sync with the public field CSS metrics.

use crate::components::virtual_list::{VirtualList, VirtualListHandle};
use leptos::prelude::*;
use operator_ui_tokens::{DU_POINTS, dim};
use send_wrapper::SendWrapper;
use std::hash::Hash;
use wasm_bindgen::{JsCast, closure::Closure};

#[derive(Clone, Copy)]
pub(crate) struct FieldListHandle {
    current: StoredValue<VirtualListHandle>,
    requested: StoredValue<Option<(usize, bool)>>,
}

impl FieldListHandle {
    pub(crate) fn new() -> Self {
        Self {
            current: StoredValue::new(VirtualListHandle::new(dim::ROW_CONTROL)),
            requested: StoredValue::new(None),
        }
    }

    pub(crate) fn reveal(self, index: usize, center: bool) {
        self.requested.set_value(Some((index, center)));
        request_animation_frame(move || self.current.get_value().reveal(index, center));
    }
}

/// The probes resolve inherited CSS variables, including calc expressions and DU scaling.
#[component]
pub(crate) fn FieldList<T, K, KF, V, VF>(
    #[prop(into)] items: Signal<Vec<T>>,
    handle: FieldListHandle,
    key: KF,
    children: VF,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    K: Eq + Hash + Send + Sync + 'static,
    KF: Fn(&T) -> K + Clone + Send + Sync + 'static,
    V: IntoView + 'static,
    VF: Fn((usize, T)) -> V + Clone + Send + Sync + 'static,
{
    let row_probe = NodeRef::<leptos::html::Span>::new();
    let max_probe = NodeRef::<leptos::html::Span>::new();
    let geometry = RwSignal::new((dim::ROW_CONTROL, super::MENU_MAX_HEIGHT));
    Effect::new(move |_| {
        let (Some(row), Some(max)) = (row_probe.get(), max_probe.get()) else {
            return;
        };
        let row_measure = row.clone();
        let max_measure = max.clone();
        let measure = move || {
            // Runtime geometry is measured in browser points, then converted back to DU.
            #[allow(clippy::cast_possible_truncation)]
            let next = (
                (row_measure.get_bounding_client_rect().height() / f64::from(DU_POINTS)) as f32,
                (max_measure.get_bounding_client_rect().height() / f64::from(DU_POINTS)) as f32,
            );
            if next.0 > 0.0 && next.1 > 0.0 && geometry.get_untracked() != next {
                geometry.set(next);
            }
        };
        let refresh = Closure::<dyn FnMut()>::new(measure);
        let observer = web_sys::ResizeObserver::new(refresh.as_ref().unchecked_ref()).ok();
        if let Some(observer) = &observer {
            observer.observe(&row);
            observer.observe(&max);
        }
        let keep = SendWrapper::new((observer, refresh));
        on_cleanup(move || {
            if let Some(observer) = &keep.0 {
                observer.disconnect();
            }
        });
    });
    view! {
        <span node_ref=row_probe class="op-field-list-probe" aria-hidden="true" style:height="var(--op-field-row-height, var(--op-dim-row-control))"></span>
        <span node_ref=max_probe class="op-field-list-probe" aria-hidden="true" style:height="var(--op-field-max-height, calc(var(--op-du) * 50))"></span>
        {move || {
            let (row_height, max_height) = geometry.get();
            let current = VirtualListHandle::new(row_height);
            handle.current.set_value(current);
            if let Some((index, center)) = handle.requested.get_value() {
                current.reveal(index, center);
            }
            view! {
                <VirtualList items=items handle=current max_height=Some(max_height)
                    class="op-scroll-contain" key=key.clone() children=children.clone()/>
            }
        }}
    }
}
