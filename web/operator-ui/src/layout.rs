//! Layout: SplitPane.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

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
            <div class="op-split-pane" style:flex-basis=first_basis>{start.run()}</div>
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
            <div class="op-split-pane op-split-pane-end">{end.run()}</div>
        </div>
    }
}
