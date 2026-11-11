//! Scrollbar and ScrollView: the Operator UI scrollbar that replaces the native one.
//!
//! The native scrollbar is hidden and the scroll position is still driven by the browser,
//! so wheel, touch, keyboard and assistive technology keep their native behavior.
//! [`Scrollbar`] draws a rectangular overlay thumb on top of the scrolled element.

use leptos::html::ElementType;
use leptos::prelude::*;
use operator_ui_tokens::DU_POINTS;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Shortest thumb along the scroll axis, in DU.
const THUMB_MIN_DU: f64 = 6.0;
/// Share of the viewport scrolled by a click on the track.
const PAGE_STEP: f64 = 0.9;

/// Scroll direction handled by a [`Scrollbar`] or allowed in a [`ScrollView`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollAxis {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

impl ScrollAxis {
    const fn vertical(self) -> bool {
        matches!(self, Self::Vertical | Self::Both)
    }

    const fn horizontal(self) -> bool {
        matches!(self, Self::Horizontal | Self::Both)
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "y",
            Self::Horizontal => "x",
            Self::Both => "both",
        }
    }
}

/// Scroll geometry of one axis, measured in CSS pixels at runtime.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Metrics {
    /// Visible length of the scrolled element.
    client: f64,
    /// Full content length.
    content: f64,
    /// Current scroll offset.
    offset: f64,
}

impl Metrics {
    fn overflows(&self) -> bool {
        self.content - self.client > 1.0
    }

    fn max_offset(&self) -> f64 {
        (self.content - self.client).max(0.0)
    }

    /// Thumb length and position inside a track of `track` pixels.
    fn thumb(&self, track: f64) -> (f64, f64) {
        if !self.overflows() || track <= 0.0 {
            return (0.0, 0.0);
        }
        let min = (THUMB_MIN_DU * f64::from(DU_POINTS)).min(track);
        let length = (track * self.client / self.content).clamp(min, track);
        let progress = self.offset / self.max_offset();
        (length, (track - length) * progress.clamp(0.0, 1.0))
    }
}

fn measure(el: &web_sys::HtmlElement, vertical: bool) -> Metrics {
    if vertical {
        Metrics {
            client: f64::from(el.client_height()),
            content: f64::from(el.scroll_height()),
            offset: f64::from(el.scroll_top()),
        }
    } else {
        Metrics {
            client: f64::from(el.client_width()),
            content: f64::from(el.scroll_width()),
            offset: f64::from(el.scroll_left()),
        }
    }
}

fn set_offset(el: &web_sys::HtmlElement, vertical: bool, value: f64) {
    // Scroll offsets are whole pixels in the DOM API; rounding keeps drags stable.
    #[allow(clippy::cast_possible_truncation)]
    let value = value.round() as i32;
    if vertical {
        el.set_scroll_top(value);
    } else {
        el.set_scroll_left(value);
    }
}

/// Rectangular overlay scrollbar for one axis of `target`.
///
/// Place it next to `target` inside a positioned parent; [`ScrollView`] does this for you.
/// The target must hide its native scrollbar with the `op-scroll-native-off` class.
#[component]
pub fn Scrollbar<E>(
    /// Element that scrolls.
    target: NodeRef<E>,
    /// `Vertical` or `Horizontal`; `Both` is treated as `Vertical`.
    #[prop(default = ScrollAxis::Vertical)]
    axis: ScrollAxis,
    /// Leaves room for the crossing scrollbar in the corner.
    #[prop(optional)]
    corner: Signal<bool>,
) -> impl IntoView
where
    E: ElementType + 'static,
    E::Output: JsCast + Clone + 'static,
{
    let vertical = axis != ScrollAxis::Horizontal;
    let metrics = RwSignal::new(Metrics::default());
    let track_len = RwSignal::new(0.0f64);
    let dragging = RwSignal::new(false);
    // Pointer coordinate and scroll offset at the start of a drag.
    let drag_from = StoredValue::new((0.0f64, 0.0f64));
    let track = NodeRef::<leptos::html::Div>::new();

    let element = move || {
        target
            .get_untracked()
            .map(|el| el.unchecked_ref::<web_sys::HtmlElement>().clone())
    };
    let refresh = move || {
        if let Some(el) = element() {
            metrics.set(measure(&el, vertical));
        }
        if let Some(t) = track.get_untracked() {
            track_len.set(if vertical {
                f64::from(t.client_height())
            } else {
                f64::from(t.client_width())
            });
        }
    };

    // Follow scrolling, size changes of the element and its children, and content mutations.
    Effect::new(move |_| {
        let Some(el) = target.get() else {
            return;
        };
        let el: web_sys::HtmlElement = el.unchecked_ref::<web_sys::HtmlElement>().clone();

        let on_scroll = Closure::<dyn FnMut()>::new(refresh);
        let _ = el.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());
        let _ = el.add_event_listener_with_callback("input", on_scroll.as_ref().unchecked_ref());

        let on_resize = Closure::<dyn FnMut()>::new(refresh);
        let resize = web_sys::ResizeObserver::new(on_resize.as_ref().unchecked_ref()).ok();
        let observe_children = {
            let el = el.clone();
            let resize = resize.clone();
            move || {
                if let Some(ro) = &resize {
                    ro.observe(&el);
                    let children = el.children();
                    for i in 0..children.length() {
                        if let Some(child) = children.item(i) {
                            ro.observe(&child);
                        }
                    }
                }
            }
        };
        observe_children();

        let on_mutation = {
            let observe_children = observe_children.clone();
            Closure::<dyn FnMut()>::new(move || {
                observe_children();
                refresh();
            })
        };
        let mutation = web_sys::MutationObserver::new(on_mutation.as_ref().unchecked_ref()).ok();
        if let Some(mo) = &mutation {
            let init = web_sys::MutationObserverInit::new();
            init.set_child_list(true);
            init.set_subtree(true);
            init.set_character_data(true);
            let _ = mo.observe_with_options(&el, &init);
        }
        request_animation_frame(refresh);

        let keep = SendWrapper::new((el, on_scroll, resize, on_resize, mutation, on_mutation));
        on_cleanup(move || {
            let (el, on_scroll, resize, _, mutation, _) = &*keep;
            let _ = el
                .remove_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());
            let _ =
                el.remove_event_listener_with_callback("input", on_scroll.as_ref().unchecked_ref());
            if let Some(ro) = resize {
                ro.disconnect();
            }
            if let Some(mo) = mutation {
                mo.disconnect();
            }
        });
    });

    let thumb = Memo::new(move |_| metrics.get().thumb(track_len.get()));
    let visible = move || metrics.get().overflows();

    let pointer = move |ev: &web_sys::PointerEvent| {
        if vertical {
            f64::from(ev.client_y())
        } else {
            f64::from(ev.client_x())
        }
    };

    let on_thumb_down = move |ev: web_sys::PointerEvent| {
        if ev.button() != 0 {
            return;
        }
        ev.prevent_default();
        ev.stop_propagation();
        if let Some(t) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let _ = t.set_pointer_capture(ev.pointer_id());
        }
        drag_from.set_value((pointer(&ev), metrics.get_untracked().offset));
        dragging.set(true);
    };
    let on_thumb_move = move |ev: web_sys::PointerEvent| {
        if !dragging.get_untracked() {
            return;
        }
        let Some(el) = element() else {
            return;
        };
        let m = measure(&el, vertical);
        let (length, _) = m.thumb(track_len.get_untracked());
        let room = track_len.get_untracked() - length;
        if room <= 0.0 {
            return;
        }
        let (start, offset) = drag_from.get_value();
        let delta = (pointer(&ev) - start) * m.max_offset() / room;
        set_offset(&el, vertical, (offset + delta).clamp(0.0, m.max_offset()));
    };
    let on_thumb_up = move |_| dragging.set(false);

    // A click on the empty track pages toward the pointer.
    let on_track_down = move |ev: web_sys::PointerEvent| {
        if ev.button() != 0 {
            return;
        }
        ev.prevent_default();
        let (Some(el), Some(t)) = (element(), track.get_untracked()) else {
            return;
        };
        let rect = t.get_bounding_client_rect();
        let start = if vertical { rect.top() } else { rect.left() };
        let (_, position) = thumb.get_untracked();
        let m = measure(&el, vertical);
        let step = m.client * PAGE_STEP;
        let next = if pointer(&ev) - start < position {
            m.offset - step
        } else {
            m.offset + step
        };
        set_offset(&el, vertical, next.clamp(0.0, m.max_offset()));
    };

    view! {
        <div
            node_ref=track
            class="op-scrollbar"
            aria-hidden="true"
            data-axis=if vertical { "y" } else { "x" }
            data-visible=move || visible().then_some("")
            data-dragging=move || dragging.get().then_some("")
            data-corner=move || corner.get().then_some("")
            on:pointerdown=on_track_down
        >
            <div
                class="op-scrollbar-thumb"
                style:height=move || vertical.then(|| format!("{}px", thumb.get().0))
                style:width=move || (!vertical).then(|| format!("{}px", thumb.get().0))
                style:transform=move || {
                    let offset = thumb.get().1;
                    if vertical {
                        format!("translateY({offset}px)")
                    } else {
                        format!("translateX({offset}px)")
                    }
                }
                on:pointerdown=on_thumb_down
                on:pointermove=on_thumb_move
                on:pointerup=on_thumb_up
                on:pointercancel=on_thumb_up
                on:lostpointercapture=on_thumb_up
            ></div>
        </div>
    }
}

/// Scroll container that uses the Operator UI [`Scrollbar`] instead of the native one.
///
/// `class` styles the outer box (size, flex role); `viewport_class` styles the scrolled
/// element (padding, gap, layout of the children).
#[component]
pub fn ScrollView(
    #[prop(default = ScrollAxis::Vertical)] axis: ScrollAxis,
    /// Classes for the outer box.
    #[prop(optional, into)]
    class: String,
    /// Classes for the scrolled element.
    #[prop(optional, into)]
    viewport_class: String,
    /// Fixed height in DU.
    #[prop(into, default = None)]
    height: Option<f32>,
    /// Maximum height in DU.
    #[prop(into, default = None)]
    max_height: Option<f32>,
    /// Reference to the scrolled element, for components that scroll it programmatically.
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Makes the scrolled element focusable so keyboard users can scroll it.
    #[prop(optional)]
    focusable: bool,
    children: Children,
) -> impl IntoView {
    let viewport = node_ref.unwrap_or_default();
    let both = axis == ScrollAxis::Both;
    view! {
        <div
            class=format!("op-scrollview {class}")
            style:height=height.map(crate::core::du)
            style:max-height=max_height.map(crate::core::du)
        >
            <div
                node_ref=viewport
                class=format!("op-scrollview-viewport op-scroll-native-off {viewport_class}")
                data-axis=axis.as_str()
                tabindex=focusable.then_some("0")
            >
                {children()}
            </div>
            {axis.vertical().then(|| view! { <Scrollbar target=viewport axis=ScrollAxis::Vertical corner=Signal::stored(both)/> })}
            {axis.horizontal().then(|| view! { <Scrollbar target=viewport axis=ScrollAxis::Horizontal corner=Signal::stored(both)/> })}
        </div>
    }
}
