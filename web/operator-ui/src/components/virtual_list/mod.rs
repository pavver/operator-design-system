//! Windowed list: VirtualList renders only the rows inside the viewport.

use crate::components::scrollbar::ScrollView;
use crate::core::{Metric, Slot, root_class, root_style};
use leptos::prelude::*;
use operator_ui_tokens::DU_POINTS;
use send_wrapper::SendWrapper;
use std::hash::Hash;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

pub(crate) const CSS: &str = include_str!("virtual_list.css");

/// Rows drawn above and below the visible window by default.
pub const DEFAULT_OVERSCAN: usize = 6;

/// Half-open range of row indices to render for a scroll position.
///
/// `row` and `viewport` are in the same unit (CSS pixels); a zero row height renders nothing.
pub fn visible_window(
    len: usize,
    row: f64,
    scroll_top: f64,
    viewport: f64,
    overscan: usize,
) -> (usize, usize) {
    if len == 0 || row <= 0.0 {
        return (0, 0);
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let first = (scroll_top.max(0.0) / row).floor() as usize;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let visible = (viewport.max(0.0) / row).ceil() as usize + 1;
    let start = first.saturating_sub(overscan).min(len);
    let end = first
        .saturating_add(visible)
        .saturating_add(overscan)
        .min(len);
    (start, end.max(start))
}

/// Scroll offset that brings row `index` into view, or `None` when it is already visible.
///
/// With `center` the row is placed in the middle of the viewport.
pub fn reveal_offset(
    index: usize,
    row: f64,
    scroll_top: f64,
    viewport: f64,
    center: bool,
) -> Option<f64> {
    #[allow(clippy::cast_precision_loss)]
    let top = index as f64 * row;
    let bottom = top + row;
    if center {
        return Some((top - (viewport - row) / 2.0).max(0.0));
    }
    if top < scroll_top {
        Some(top)
    } else if bottom > scroll_top + viewport {
        Some((bottom - viewport).max(0.0))
    } else {
        None
    }
}

/// Handle for scrolling a [`VirtualList`] from the outside (keyboard navigation).
#[derive(Clone, Copy)]
pub struct VirtualListHandle {
    viewport: NodeRef<leptos::html::Div>,
    row_px: f64,
}

impl VirtualListHandle {
    /// Creates a handle for rows of `row_height` DU.
    pub fn new(row_height: f32) -> Self {
        Self {
            viewport: NodeRef::new(),
            row_px: f64::from(row_height * DU_POINTS),
        }
    }

    /// Scrolls row `index` into view; `center` places it in the middle.
    pub fn reveal(&self, index: usize, center: bool) {
        let viewport = self.viewport;
        let row = self.row_px;
        request_animation_frame(move || {
            let Some(el) = viewport.get_untracked() else {
                return;
            };
            let top = f64::from(el.scroll_top());
            let height = f64::from(el.client_height());
            if let Some(next) = reveal_offset(index, row, top, height, center) {
                #[allow(clippy::cast_possible_truncation)]
                el.set_scroll_top(next.round() as i32);
            }
        });
    }
}

/// Vertical list with fixed-height rows that renders only the visible window.
///
/// Rows are `row_height` DU tall; their content must not grow taller.
/// `children` receives `(index in items, item)`.
#[component]
pub fn VirtualList<T, K, KF, V, VF>(
    #[prop(into)] items: Signal<Vec<T>>,
    /// Handle with the row height in DU; also scrolls the list from the outside.
    handle: VirtualListHandle,
    /// Fixed height in DU (`--op-virtual-list-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Maximum height in DU (`--op-virtual-list-max-height`).
    #[prop(optional, into)]
    max_height: MaybeProp<f32>,
    /// Row height in DU (`--op-virtual-list-row-height`).
    #[prop(optional, into)]
    row_height: MaybeProp<f32>,
    /// Extra rows rendered above and below the visible window.
    #[prop(default = DEFAULT_OVERSCAN)]
    overscan: usize,
    /// Classes for the outer box.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline styles for the outer box.
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// Makes the scrolled element focusable so keyboard users can scroll it.
    #[prop(optional)]
    focusable: bool,
    /// Slot rendered before the scrolled viewport.
    #[prop(optional, into)]
    header: Slot,
    /// Slot rendered after the scrolled viewport.
    #[prop(optional, into)]
    footer: Slot,
    /// Slot rendered when items is empty.
    #[prop(optional, into)]
    empty: Slot,
    /// Stable key of an item.
    key: KF,
    /// Renders one row.
    children: VF,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    K: Eq + Hash + Send + Sync + 'static,
    KF: Fn(&T) -> K + Clone + Send + Sync + 'static,
    V: IntoView + 'static,
    VF: Fn((usize, T)) -> V + Clone + Send + Sync + 'static,
{
    let viewport = handle.viewport;
    let row = handle.row_px;
    let scroll = RwSignal::new(0.0_f64);
    let size = RwSignal::new(0.0_f64);

    Effect::new(move |_| {
        let Some(el) = viewport.get() else {
            return;
        };
        let el: web_sys::HtmlElement = el.unchecked_ref::<web_sys::HtmlElement>().clone();
        let refresh = {
            let el = el.clone();
            move || {
                scroll.set(f64::from(el.scroll_top()));
                size.set(f64::from(el.client_height()));
            }
        };
        refresh();
        let on_scroll = Closure::<dyn FnMut()>::new(refresh.clone());
        let _ = el.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());
        let on_resize = Closure::<dyn FnMut()>::new(refresh);
        let resize = web_sys::ResizeObserver::new(on_resize.as_ref().unchecked_ref()).ok();
        if let Some(ro) = &resize {
            ro.observe(&el);
        }
        let keep = SendWrapper::new((el, on_scroll, resize, on_resize));
        on_cleanup(move || {
            let (el, on_scroll, resize, _) = &*keep;
            let _ = el
                .remove_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());
            if let Some(ro) = resize {
                ro.disconnect();
            }
        });
    });

    let range = Memo::new(move |_| {
        let len = items.with(Vec::len);
        visible_window(len, row, scroll.get(), size.get(), overscan)
    });
    let visible = Signal::derive(move || {
        let (start, end) = range.get();
        items.with(|all| {
            all.get(start..end)
                .map(|slice| {
                    slice
                        .iter()
                        .cloned()
                        .enumerate()
                        .map(|(i, item)| (start + i, item))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        })
    });
    #[allow(clippy::cast_precision_loss)]
    let total = move || format!("{}px", items.with(Vec::len) as f64 * row);
    #[allow(clippy::cast_precision_loss)]
    let offset = move || format!("translateY({}px)", range.get().0 as f64 * row);
    let row_css = format!("{row}px");

    let root_metrics = vec![
        Metric::du("--op-virtual-list-height", height),
        Metric::du("--op-virtual-list-max-height", max_height),
        Metric::du("--op-virtual-list-row-height", row_height),
    ];
    let class_str = Signal::derive(root_class("op-vlist", class));
    let custom_style = Signal::derive(root_style(root_metrics, style));

    let has_empty = empty.is_set();

    view! {
        <ScrollView
            class=class_str
            style=custom_style
            node_ref=viewport
            height=height
            max_height=max_height
            focusable=focusable
            header=header
            footer=footer
        >
            <Show
                when=move || has_empty && items.with(Vec::is_empty)
                fallback={
                    let row_css = row_css.clone();
                    move || {
                        let row_css = row_css.clone();
                        view! {
                            <div class="op-vlist-space" style=move || format!("height:{};", total())>
                                <div
                                    class="op-vlist-window"
                                    style=move || format!("transform:{};--op-vlist-row:{row_css};", offset())
                                >
                                    <For
                                        each=move || visible.get()
                                        key={
                                            let key = key.clone();
                                            move |(i, item): &(usize, T)| (*i, key(item))
                                        }
                                        children={
                                            let children = children.clone();
                                            move |row: (usize, T)| children(row)
                                        }
                                    />
                                </div>
                            </div>
                        }
                    }
                }
            >
                {empty.run()}
            </Show>
        </ScrollView>
    }
}

#[cfg(test)]
mod tests;
