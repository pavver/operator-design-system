//! Layout: Toolbar, SplitPane, ScrollArea.

use crate::core::du;
use crate::scroll::{ScrollAxis, ScrollView, Scrollbar};
use leptos::prelude::*;
use operator_ui_tokens::motion;
use std::time::Duration;
use wasm_bindgen::JsCast;

// ---------------------------------------------------------------------------
// Toolbar
// ---------------------------------------------------------------------------

/// Horizontal bar of commands for the view below it.
///
/// Arrow keys move focus between its controls.
#[component]
pub fn Toolbar(
    #[prop(optional, into)] label: Option<String>,
    /// 8 DU tall instead of 10 DU.
    #[prop(optional)]
    dense: bool,
    children: Children,
) -> impl IntoView {
    let bar = NodeRef::<leptos::html::Div>::new();
    let step = move |delta: i32| {
        let Some(el) = bar.try_get_untracked().flatten() else {
            return;
        };
        let Ok(list) = el.query_selector_all(
            "button:not([disabled]):not([aria-disabled=true]), input:not([disabled]), [tabindex='0']",
        ) else {
            return;
        };
        let n = list.length();
        if n == 0 {
            return;
        }
        let active = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element());
        let mut current = 0i64;
        for i in 0..n {
            if let (Some(node), Some(a)) = (list.item(i), active.as_ref())
                && node.contains(Some(a.as_ref()))
            {
                current = i64::from(i);
            }
        }
        let next = (current + i64::from(delta)).rem_euclid(i64::from(n));
        if let Some(item) = u32::try_from(next)
            .ok()
            .and_then(|i| list.item(i))
            .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = item.focus();
        }
    };
    view! {
        <div
            node_ref=bar
            class="op-toolbar"
            role="toolbar"
            aria-label=label
            data-dense=dense.then_some("")
            on:keydown=move |ev| {
                let in_input = ev
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    .is_some();
                if in_input {
                    return;
                }
                match ev.key().as_str() {
                    "ArrowRight" => {
                        ev.prevent_default();
                        step(1);
                    }
                    "ArrowLeft" => {
                        ev.prevent_default();
                        step(-1);
                    }
                    _ => {}
                }
            }
        >
            {children()}
        </div>
    }
}

/// Title text inside a [`Toolbar`].
#[component]
pub fn ToolbarTitle(#[prop(into)] text: Signal<String>) -> impl IntoView {
    view! { <span class="op-toolbar-title">{move || text.get()}</span> }
}

/// Vertical divider between groups of toolbar controls.
#[component]
pub fn ToolbarSeparator() -> impl IntoView {
    view! { <span class="op-toolbar-sep" role="separator" aria-orientation="vertical"></span> }
}

/// Flexible gap that pushes the following controls to the end.
#[component]
pub fn ToolbarSpacer() -> impl IntoView {
    view! { <span class="op-toolbar-spacer"></span> }
}

// ---------------------------------------------------------------------------
// SplitPane
// ---------------------------------------------------------------------------

/// Two panes with a draggable divider.
#[component]
pub fn SplitPane(
    #[prop(into)] start: ViewFn,
    #[prop(into)] end: ViewFn,
    /// Share of the first pane, `0.0..1.0`.
    #[prop(optional, into)]
    ratio: Option<RwSignal<f32>>,
    /// Stacks the panes top and bottom.
    #[prop(optional)]
    vertical: bool,
    /// Minimum size of each pane in DU.
    #[prop(default = 20.0)]
    min_size: f32,
) -> impl IntoView {
    let ratio = ratio.unwrap_or_else(|| RwSignal::new(0.5));
    let initial = ratio.get_untracked();
    let host = NodeRef::<leptos::html::Div>::new();
    let dragging = RwSignal::new(false);

    let limits = move || -> (f32, f32) {
        let Some(el) = host.try_get_untracked().flatten() else {
            return (0.0, 1.0);
        };
        let total = if vertical {
            el.client_height()
        } else {
            el.client_width()
        };
        let total = f64::from(total).max(1.0);
        let min = (f64::from(min_size * operator_ui_tokens::DU_POINTS) / total).min(0.5);
        #[allow(clippy::cast_possible_truncation)]
        let min = min as f32;
        (min, 1.0 - min)
    };
    let set_clamped = move |v: f32| {
        let (lo, hi) = limits();
        ratio.set(v.clamp(lo, hi));
    };
    let from_pointer = move |ev: &web_sys::PointerEvent| {
        let Some(el) = host.try_get_untracked().flatten() else {
            return;
        };
        let r = el.get_bounding_client_rect();
        let v = if vertical {
            (f64::from(ev.client_y()) - r.top()) / r.height().max(1.0)
        } else {
            (f64::from(ev.client_x()) - r.left()) / r.width().max(1.0)
        };
        #[allow(clippy::cast_possible_truncation)]
        set_clamped(v as f32);
    };

    let first_basis = move || format!("{}%", ratio.get() * 100.0);
    view! {
        <div
            node_ref=host
            class="op-split"
            data-vertical=vertical.then_some("")
            data-dragging=move || dragging.get().then_some("")
        >
            <div class="op-split-pane" style:flex-basis=first_basis><ScrollView axis=ScrollAxis::Both>{start.run()}</ScrollView></div>
            <div
                class="op-split-handle"
                role="separator"
                tabindex="0"
                aria-orientation=if vertical { "horizontal" } else { "vertical" }
                aria-valuemin="0"
                aria-valuemax="100"
                aria-valuenow=move || format!("{:.0}", ratio.get() * 100.0)
                on:pointerdown=move |ev| {
                    ev.prevent_default();
                    if let Some(t) = ev.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                        let _ = t.set_pointer_capture(ev.pointer_id());
                    }
                    dragging.set(true);
                }
                on:pointermove=move |ev| {
                    if dragging.get_untracked() {
                        from_pointer(&ev);
                    }
                }
                on:pointerup=move |_| dragging.set(false)
                on:pointercancel=move |_| dragging.set(false)
                on:dblclick=move |_| set_clamped(initial)
                on:keydown=move |ev| {
                    let step = if ev.shift_key() { 0.1 } else { 0.02 };
                    let v = ratio.get_untracked();
                    let next = match (ev.key().as_str(), vertical) {
                        ("ArrowLeft", false) | ("ArrowUp", true) => Some(v - step),
                        ("ArrowRight", false) | ("ArrowDown", true) => Some(v + step),
                        ("Home", _) => Some(0.0),
                        ("End", _) => Some(1.0),
                        ("Enter", _) => Some(initial),
                        _ => None,
                    };
                    if let Some(n) = next {
                        ev.prevent_default();
                        set_clamped(n);
                    }
                }
            >
                <span class="op-split-grip" aria-hidden="true"></span>
            </div>
            <div class="op-split-pane op-split-pane-end"><ScrollView axis=ScrollAxis::Both>{end.run()}</ScrollView></div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// ScrollArea
// ---------------------------------------------------------------------------

/// Largest stretch as a share of the viewport height.
const MAX_STRETCH: f64 = 0.08;
/// Pull distance, as a share of the viewport height, that reaches the largest stretch.
const FULL_PULL: f64 = 0.6;
/// Idle time after the last wheel event before a wheel stretch springs back, in milliseconds.
const WHEEL_IDLE_MS: u64 = 120;

fn stretch_for(pull: f64, height: f64) -> f64 {
    let t = (pull.abs() / (height * FULL_PULL).max(1.0)).min(1.0);
    // Ease-out: strong response at first, saturating toward the limit.
    let eased = 1.0 - (1.0 - t) * (1.0 - t);
    MAX_STRETCH * eased * pull.signum()
}

/// Vertical scroll container with a thin scrollbar and stretch overscroll at the edges.
#[component]
pub fn ScrollArea(
    /// Maximum height in DU; fills the parent when omitted.
    #[prop(optional)]
    max_height: Option<f32>,
    /// Fixed height in DU.
    #[prop(optional)]
    height: Option<f32>,
    #[prop(default = true)] stretch: bool,
    #[prop(optional, into)] label: Option<String>,
    children: Children,
) -> impl IntoView {
    let host = NodeRef::<leptos::html::Div>::new();
    // Signed pull in px: positive at the top edge, negative at the bottom edge.
    let pull = RwSignal::new(0.0f64);
    let held = RwSignal::new(false);
    let touch_y = StoredValue::new(None::<f64>);
    let wheel_job = crate::core::JobToken::new();

    let reduced = web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|m| m.matches());
    let enabled = stretch && !reduced;

    let edges = move || {
        host.try_get_untracked().flatten().map(|el| {
            let top = el.scroll_top() <= 0;
            let bottom = el.scroll_top() + el.client_height() >= el.scroll_height() - 1;
            (top, bottom, f64::from(el.client_height()))
        })
    };
    let release = move || {
        held.set(false);
        pull.set(0.0);
    };

    let scale = move || {
        let p = pull.get();
        if p == 0.0 {
            return None;
        }
        let h = edges().map_or(1.0, |(_, _, h)| h);
        let s = stretch_for(p, h);
        Some(format!("scaleY({})", 1.0 + s.abs()))
    };
    let origin = move || if pull.get() >= 0.0 { "top" } else { "bottom" };

    view! {
        <div class="op-scrollview" style:max-height=max_height.map(du) style:height=height.map(du)>
        <div
            node_ref=host
            class="op-scroll op-scrollview-viewport op-scroll-native-off"
            data-axis="y"
            tabindex="0"
            role=label.is_some().then_some("region")
            aria-label=label
            on:touchstart=move |ev| {
                if enabled {
                    touch_y.set_value(ev.touches().get(0).map(|t| f64::from(t.client_y())));
                }
            }
            on:touchmove=move |ev| {
                if !enabled {
                    return;
                }
                let (Some(start), Some(t)) = (touch_y.get_value(), ev.touches().get(0)) else {
                    return;
                };
                let y = f64::from(t.client_y());
                let Some((top, bottom, _)) = edges() else {
                    return;
                };
                let dy = y - start;
                let current = pull.get_untracked();
                if (top && dy > 0.0) || (bottom && dy < 0.0) || current != 0.0 {
                    let next = if current == 0.0 { dy } else { current + (y - start) };
                    // Pulling back past the rest position hands control to native scrolling.
                    if current != 0.0 && next.signum() != current.signum() {
                        release();
                    } else {
                        ev.prevent_default();
                        held.set(true);
                        pull.set(next);
                    }
                }
                touch_y.set_value(Some(y));
            }
            on:touchend=move |_| {
                touch_y.set_value(None);
                release();
            }
            on:touchcancel=move |_| {
                touch_y.set_value(None);
                release();
            }
            on:wheel=move |ev| {
                if !enabled || ev.ctrl_key() {
                    return;
                }
                let Some((top, bottom, h)) = edges() else {
                    return;
                };
                let dy = -ev.delta_y();
                if (top && dy > 0.0) || (bottom && dy < 0.0) {
                    held.set(true);
                    pull.update(|p| *p = (*p + dy * 0.5).clamp(-h, h));
                    let id = wheel_job.next();
                    set_timeout(
                        move || {
                            if wheel_job.is_current(id) {
                                release();
                            }
                        },
                        Duration::from_millis(WHEEL_IDLE_MS),
                    );
                }
            }
        >
            <div
                class="op-scroll-content"
                data-held=move || held.get().then_some("")
                style:transform=scale
                style:transform-origin=origin
                style:transition-duration=move || {
                    if held.get() { "0ms".to_string() } else { format!("{}ms", motion::OVERSCROLL_RELEASE_MS) }
                }
            >
                {children()}
            </div>
        </div>
        <Scrollbar target=host axis=ScrollAxis::Vertical/>
        </div>
    }
}
