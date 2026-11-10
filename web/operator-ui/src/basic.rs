//! Basic building blocks: Icon, ProgressIndicator, Divider, Card.

use crate::core::du;
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim};

/// Vector icon tinted with `currentColor` unless `color` is given.
#[component]
pub fn Icon(
    icon: IconData,
    /// Size in DU.
    #[prop(default = dim::ICON_LARGE)]
    size: f32,
    /// Optional CSS color; defaults to the inherited text color.
    #[prop(optional, into)]
    color: Option<Signal<String>>,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let hidden = label.is_none().then_some("true");
    view! {
        <svg
            class="op-icon"
            viewBox=icon.view_box
            role=label.as_ref().map(|_| "img")
            aria-label=label
            aria-hidden=hidden
            style:width=du(size)
            style:height=du(size)
            style:color=move || color.map(|c| c.get())
        >
            {icon.paths.iter().map(|d| view! { <path d=*d></path> }).collect_view()}
        </svg>
    }
}

/// Circular 270° spinner rotating every `spinner_lap` milliseconds.
#[component]
pub fn ProgressIndicator(
    /// Size in DU.
    #[prop(default = 6.0)]
    size: f32,
    /// Stroke width in DU.
    #[prop(default = 0.5)]
    stroke: f32,
    /// CSS color.
    #[prop(default = "var(--op-accent)".into(), into)]
    color: String,
) -> impl IntoView {
    let px = size * operator_ui_tokens::DU_POINTS;
    let stroke_px = stroke * operator_ui_tokens::DU_POINTS;
    let r = ((px - stroke_px) / 2.0).max(0.5);
    view! {
        <svg
            class="op-spinner"
            role="progressbar"
            aria-label="Loading"
            viewBox=format!("0 0 {px} {px}")
            style:width=du(size)
            style:height=du(size)
        >
            <circle
                cx=px / 2.0
                cy=px / 2.0
                r=r
                pathLength="100"
                stroke=color
                stroke-width=stroke_px
            ></circle>
        </svg>
    }
}

/// Horizontal divider line.
#[component]
pub fn Divider(
    /// Thickness in DU.
    #[prop(default = dim::BORDER)]
    thickness: f32,
) -> impl IntoView {
    view! { <div class="op-divider" role="separator" style:height=du(thickness)></div> }
}

/// Rectangular container with surface background and a standard border.
#[component]
pub fn Card(
    #[prop(optional)] alt: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("op-card {class}") class:op-card-alt=alt>
            {children()}
        </div>
    }
}
