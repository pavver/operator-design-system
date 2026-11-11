//! Scroll container with stretch overscroll: ScrollArea.

use crate::components::scrollbar::{ScrollAxis, Scrollbar};
use crate::core::{JobToken, du};
use leptos::prelude::*;
use operator_ui_tokens::motion;
use std::time::Duration;

pub(crate) const CSS: &str = include_str!("scroll_area.css");

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
    let wheel_job = JobToken::new();

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
