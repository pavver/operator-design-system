//! Parallax component: background image moving at a fraction of scroll speed.

use crate::core::observers::use_scroll;
use crate::core::{Metric, root_class, root_style};
use leptos::html::Div;
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("parallax.css");

/// Visual banner where the background image scrolls slower than the rest of the page.
#[component]
pub fn Parallax(
    #[prop(into)] src: Signal<String>,
    #[prop(optional, into)] alt: MaybeProp<String>,
    /// Height in DU (`--op-parallax-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Speed ratio between 0.0 and 1.0; 0.5 scrolls at half speed.
    #[prop(default = 0.5.into(), into)]
    speed: Signal<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let scroll = use_scroll::<Div>(None);

    let prefers_reduced_motion = move || {
        let Some(win) = web_sys::window() else {
            return false;
        };
        win.match_media("(prefers-reduced-motion: reduce)")
            .ok()
            .flatten()
            .map(|mql| mql.matches())
            .unwrap_or(false)
    };

    let transform_style = move || {
        let _ = scroll.get();
        if prefers_reduced_motion() {
            return "transform: translateY(0px);".to_string();
        }
        let Some(el) = container_ref.get() else {
            return "transform: translateY(0px);".to_string();
        };
        let rect = el.get_bounding_client_rect();
        let win_height = web_sys::window()
            .and_then(|w| w.inner_height().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(800.0);

        let center = rect.top() + rect.height() / 2.0;
        let win_center = win_height / 2.0;
        let delta = (center - win_center) * (speed.get() as f64);

        format!("transform: translateY({}px);", delta as i32)
    };

    let metrics = vec![Metric::du("--op-parallax-height", height)];
    let class_fn = root_class("op-parallax", class);
    let style_fn = root_style(metrics, style);

    view! {
        <div class=class_fn style=style_fn node_ref=container_ref>
            <img
                class="op-parallax-image"
                src=move || src.get()
                alt=move || alt.get().unwrap_or_default()
                style=transform_style
            />
            {children.map(|c| view! {
                <div class="op-parallax-content">
                    {c()}
                </div>
            })}
        </div>
    }
}
