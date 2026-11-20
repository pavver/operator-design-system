//! Floating layer: Popover, the shared base of menus, lists, tooltips and hover cards.

use crate::core::overlay::{
    FloatPos, OverlayHost, place_above, place_at, place_below_end, place_below_start,
    place_side_start,
};
use leptos::portal::Portal;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::sync::atomic::{AtomicU64, Ordering};
use wasm_bindgen::JsCast;

pub(crate) const CSS: &str = include_str!("popover.css");

/// What a [`Popover`] is attached to, in viewport (CSS pixel) coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PopoverAnchor {
    /// An element's bounding box.
    Rect {
        left: f64,
        top: f64,
        width: f64,
        height: f64,
    },
    /// A pointer position.
    Point { x: f64, y: f64 },
}

impl PopoverAnchor {
    /// The current bounding box of `el`.
    pub fn of(el: &web_sys::Element) -> Self {
        let r = el.get_bounding_client_rect();
        Self::Rect {
            left: r.left(),
            top: r.top(),
            width: r.width(),
            height: r.height(),
        }
    }

    fn rect(self) -> Option<web_sys::DomRect> {
        let (x, y, w, h) = match self {
            Self::Rect {
                left,
                top,
                width,
                height,
            } => (left, top, width, height),
            Self::Point { x, y } => (x, y, 0.0, 0.0),
        };
        web_sys::DomRect::new_with_x_and_y_and_width_and_height(x, y, w, h).ok()
    }

    /// True when the viewport point lies inside the anchor box.
    pub fn contains(self, x: f64, y: f64) -> bool {
        match self {
            Self::Rect {
                left,
                top,
                width,
                height,
            } => x >= left && x <= left + width && y >= top && y <= top + height,
            Self::Point { .. } => false,
        }
    }
}

/// Where a [`Popover`] opens relative to its anchor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Placement {
    /// Below, aligned to the start edge; flips above without room.
    #[default]
    BelowStart,
    /// Below, aligned to the end edge; flips above without room.
    BelowEnd,
    /// Below with the anchor's width (lists of a field); flips above without room.
    BelowStretch,
    /// Centered above; flips below without room.
    Above,
    /// At the anchor point (context menus); shifted inside the viewport.
    AtPoint,
    /// Beside the anchor, top edges aligned, to the right or else to the left (submenus).
    SideStart,
}

/// A new unique layer id, for linking a layer to the layer it opens from (`parent_layer`).
pub fn new_layer_id() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// The overlay host of the nearest theme root, or the document body outside one.
fn mount_point(host: Option<OverlayHost>) -> web_sys::Element {
    host.and_then(|h| h.0.get_untracked())
        .map(|el| el.unchecked_into::<web_sys::Element>())
        .or_else(|| document().body().map(Into::into))
        .unwrap_or_else(|| {
            document()
                .create_element("div")
                .unwrap_or_else(|_| unreachable_host())
        })
}

#[cold]
fn unreachable_host() -> web_sys::Element {
    // Without a document there is nothing to render into; a detached JS object keeps types happy.
    wasm_bindgen::JsValue::NULL.unchecked_into()
}

/// Computes the layer position for a measured size.
pub fn place(anchor: PopoverAnchor, placement: Placement, w: f64, h: f64, gap: f32) -> FloatPos {
    let Some(rect) = anchor.rect() else {
        return FloatPos {
            left: 0.0,
            top: 0.0,
        };
    };
    match placement {
        Placement::BelowStart | Placement::BelowStretch => place_below_start(&rect, w, h, gap),
        Placement::BelowEnd => place_below_end(&rect, w, h),
        Placement::Above => place_above(&rect, w, h, gap),
        Placement::AtPoint => place_at(rect.left(), rect.top(), w, h),
        Placement::SideStart => place_side_start(&rect, w, h),
    }
}

/// Floating layer mounted in the [`ThemeRoot`](crate::core::ThemeRoot) overlay host.
///
/// It measures itself, places itself next to `anchor`, follows the anchor while the page
/// scrolls or resizes, and closes on a press outside or Escape. Focus returns to the element
/// that had it before opening when the focus was inside the layer.
#[component]
pub fn Popover(
    open: RwSignal<bool>,
    #[prop(into)] anchor: Signal<PopoverAnchor>,
    #[prop(into, default = Placement::BelowStart.into())] placement: Signal<Placement>,
    /// Gap to the anchor in DU.
    #[prop(default = 0.0)]
    gap: f32,
    /// Closes on a press outside the layer and the anchor.
    #[prop(default = true)]
    dismiss_on_outside: bool,
    /// Closes on Escape.
    #[prop(default = true)]
    dismiss_on_escape: bool,
    /// Closes when the window is resized instead of following the anchor.
    #[prop(optional)]
    dismiss_on_resize: bool,
    /// Returns focus to the previously focused element when the layer closes.
    #[prop(default = true)]
    restore_focus: bool,
    /// Lets pointer events pass through the layer (tooltips).
    #[prop(optional)]
    passive: bool,
    /// Classes of the layer element.
    #[prop(optional, into)]
    class: String,
    #[prop(optional, into)] role: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    /// Reference to the layer element.
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Id of this layer (from [`new_layer_id`]); a fresh one when not given.
    #[prop(default = None)]
    layer_id: Option<u64>,
    /// The layer this one opens from: presses inside this layer do not close the parent.
    #[prop(default = None)]
    parent_layer: Option<u64>,
    /// Called once per opening, after the first placement, with the layer visible.
    #[prop(optional)]
    on_placed: Option<Callback<()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let layer = node_ref.unwrap_or_default();
    let layer_id = layer_id.unwrap_or_else(new_layer_id);
    let pos = RwSignal::new(None::<FloatPos>);
    let width = RwSignal::new(None::<f64>);
    let previous_focus = StoredValue::new(None::<SendWrapper<web_sys::HtmlElement>>);
    let host = use_context::<OverlayHost>();

    let measure = move || {
        let Some(el) = layer.try_get_untracked().flatten() else {
            return;
        };
        let a = anchor.get_untracked();
        let placement = placement.get_untracked();
        if placement == Placement::BelowStretch
            && let PopoverAnchor::Rect { width: w, .. } = a
        {
            width.set(Some(w));
        }
        let (w, h) = (f64::from(el.offset_width()), f64::from(el.offset_height()));
        let first = pos.get_untracked().is_none();
        pos.set(Some(place(a, placement, w, h, gap)));
        // The layer becomes visible on the next render; focusing a hidden element fails.
        if first && let Some(cb) = on_placed {
            request_animation_frame(move || cb.run(()));
        }
    };

    Effect::new(move |was_open: Option<bool>| {
        let is_open = open.get();
        if is_open && was_open != Some(true) {
            previous_focus.set_value(
                document()
                    .active_element()
                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                    .map(SendWrapper::new),
            );
            pos.set(None);
            width.set(None);
            request_animation_frame(measure);
        } else if !is_open && was_open == Some(true) && restore_focus {
            // Focus was lost with the removed layer (now on body) or is still inside it.
            let active = document().active_element();
            let on_body = match (&active, document().body()) {
                (None, _) => true,
                (Some(a), Some(body)) => a.is_same_node(Some(body.as_ref())),
                (Some(_), None) => false,
            };
            let in_layer = layer
                .try_get_untracked()
                .flatten()
                .zip(active.as_ref())
                .is_some_and(|(el, a)| el.contains(Some(a.as_ref())));
            if (on_body || in_layer)
                && let Some(prev) = previous_focus.get_value()
            {
                let _ = prev.focus();
            }
        }
        is_open
    });
    // A reactive anchor (a new pointer position) re-places an open layer.
    Effect::new(move |_| {
        let _ = (anchor.get(), placement.get());
        if open.get_untracked() {
            request_animation_frame(measure);
        }
    });

    // Window listeners exist only in a browser: rendering on a server has no window.
    #[cfg(target_arch = "wasm32")]
    {
        use crate::core::overlay::event_in_layer_tree;
        use leptos::ev;
        let down = window_event_listener(ev::pointerdown, move |e| {
            if !dismiss_on_outside || !open.get_untracked() {
                return;
            }
            let in_layer = event_in_layer_tree(&e, layer_id);
            let in_anchor = anchor
                .get_untracked()
                .contains(f64::from(e.client_x()), f64::from(e.client_y()));
            if !in_layer && !in_anchor {
                open.set(false);
            }
        });
        let key = window_event_listener(ev::keydown, move |e| {
            if dismiss_on_escape && open.get_untracked() && e.key() == "Escape" {
                open.set(false);
            }
        });
        let resize = window_event_listener(ev::resize, move |_| {
            if !open.get_untracked() {
                return;
            }
            if dismiss_on_resize {
                open.set(false);
            } else {
                measure();
            }
        });
        // Scrolling of any ancestor moves the anchor; capture catches scrolls of inner containers.
        let on_scroll = wasm_bindgen::prelude::Closure::<dyn FnMut()>::new(move || {
            if open.get_untracked() {
                measure();
            }
        });
        if let Some(w) = web_sys::window() {
            let _ = w.add_event_listener_with_callback_and_bool(
                "scroll",
                on_scroll.as_ref().unchecked_ref(),
                true,
            );
        }
        let scroll = SendWrapper::new(on_scroll);
        on_cleanup(move || {
            down.remove();
            key.remove();
            resize.remove();
            if let Some(w) = web_sys::window() {
                let _ = w.remove_event_listener_with_callback_and_bool(
                    "scroll",
                    scroll.as_ref().unchecked_ref(),
                    true,
                );
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (
        measure,
        dismiss_on_outside,
        dismiss_on_escape,
        dismiss_on_resize,
        layer_id,
    );

    let children = StoredValue::new(children);
    let class = StoredValue::new(class);
    let role = StoredValue::new(role);
    let label = StoredValue::new(label);
    view! {
        <Show when=move || open.get()>
            <Portal mount=mount_point(host)>
                <div
                    node_ref=layer
                    class=format!("op-popover {}", class.get_value())
                    data-passive=passive.then_some("")
                    data-op-layer=layer_id.to_string()
                    data-op-parent=parent_layer.map(|p| p.to_string())
                    role=role.get_value()
                    aria-label=label.get_value()
                    style:visibility=move || if pos.get().is_some() { "visible" } else { "hidden" }
                    style:left=move || pos.get().map(|p| format!("{}px", p.left))
                    style:top=move || pos.get().map(|p| format!("{}px", p.top))
                    style:width=move || width.get().map(|w| format!("{w}px"))
                >
                    {children.with_value(|c| c())}
                </div>
            </Portal>
        </Show>
    }
}
