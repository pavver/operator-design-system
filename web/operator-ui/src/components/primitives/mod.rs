//! Basic building blocks: Icon, ProgressIndicator, PendingIndicator, Divider, Card.

use crate::core::{Metric, Slot, du, root_class, root_style, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim};

pub(crate) const CSS: &str = include_str!("primitives.css");

/// Vector icon tinted with `currentColor` unless `color` is given.
#[component]
pub fn Icon(
    icon: IconData,
    /// Size in DU (`--op-icon-size`).
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    /// Optional CSS color; defaults to the inherited text color.
    #[prop(optional, into)]
    color: Option<Signal<String>>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let hidden = label.is_none().then_some("true");
    let base_style = root_style(vec![Metric::du("--op-icon-size", size)], style);
    // One style attribute: mixing `style=` with `style:*` lets the attribute overwrite properties.
    let full_style = move || {
        let color = color
            .map(|c| format!("color:{};", c.get()))
            .unwrap_or_default();
        let rest = base_style().unwrap_or_default();
        let out = format!("{color}{rest}");
        (!out.is_empty()).then_some(out)
    };
    view! {
        <svg
            class=root_class("op-icon", class)
            viewBox=icon.view_box
            role=label.as_ref().map(|_| "img")
            aria-label=label
            aria-hidden=hidden
            style=full_style
        >
            {icon.paths.iter().map(|d| view! { <path d=*d></path> }).collect_view()}
        </svg>
    }
}

/// Circular 270° spinner rotating every `spinner_lap` milliseconds.
#[component]
pub fn ProgressIndicator(
    /// Size in DU (`--op-progress-indicator-size`).
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    /// Stroke width in DU (`--op-progress-indicator-stroke`).
    #[prop(optional, into)]
    stroke: MaybeProp<f32>,
    /// CSS color.
    #[prop(optional, into)]
    color: MaybeProp<String>,
    /// Accessible name; defaults to `progress-indicator.loading` ("Loading").
    #[prop(optional, into)]
    label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_label = ui_text(label, "progress-indicator.loading", "Loading");
    let size_val = move || size.get().unwrap_or(6.0);
    let stroke_val = move || stroke.get().unwrap_or(0.5);
    let px = move || size_val() * operator_ui_tokens::DU_POINTS;
    let stroke_px = move || stroke_val() * operator_ui_tokens::DU_POINTS;
    let r = move || ((px() - stroke_px()) / 2.0).max(0.5);
    let spinner_style = root_style(
        vec![
            Metric::du("--op-progress-indicator-size", size),
            Metric::du("--op-progress-indicator-stroke", stroke),
        ],
        style,
    );
    let color_val = move || {
        color
            .get()
            .unwrap_or_else(|| "var(--op-progress-indicator-color, var(--op-accent))".to_string())
    };
    view! {
        <svg
            class=root_class("op-spinner", class)
            role="progressbar"
            aria-label=move || resolved_label.get()
            viewBox=move || format!("0 0 {} {}", px(), px())
            style=move || {
                let rest = spinner_style().unwrap_or_default();
                format!("width:{0};height:{0};{rest}", du(size_val()))
            }
        >
            <circle
                cx=move || px() / 2.0
                cy=move || px() / 2.0
                r=r
                pathLength="100"
                stroke=color_val
                stroke-width=stroke_px
            ></circle>
        </svg>
    }
}

/// Spinner used by every async control in the `Pending` state.
///
/// Gate it with [`crate::core::use_pending_indicator`] so it appears only for slow requests.
/// It fades in over `motion.pending_indicator_fade`; replacing the visual here changes every control at once.
#[component]
pub fn PendingIndicator(
    /// Size in DU (`--op-pending-indicator-size`).
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    /// Stroke width in DU (`--op-pending-indicator-stroke`).
    #[prop(optional, into)]
    stroke: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let size_val = move || size.get().unwrap_or(dim::ICON_SMALL);
    let stroke_val = move || stroke.get().unwrap_or(dim::SPINNER_STROKE);
    view! {
        <span
            class=root_class("op-pending-indicator", class)
            style=root_style(
                vec![
                    Metric::du("--op-pending-indicator-size", size),
                    Metric::du("--op-pending-indicator-stroke", stroke),
                ],
                style,
            )
        >
            <ProgressIndicator
                size=size_val()
                stroke=stroke_val()
                color="var(--op-pending-indicator-color, var(--op-pending))"
            />
        </span>
    }
}

/// Horizontal divider line.
#[component]
pub fn Divider(
    /// Thickness in DU (`--op-divider-thickness`).
    #[prop(optional, into)]
    thickness: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            class=root_class("op-divider", class)
            role="separator"
            style=root_style(vec![Metric::du("--op-divider-thickness", thickness)], style)
        ></div>
    }
}

/// Rectangular container with surface background and a standard border.
#[component]
pub fn Card(
    #[prop(optional)] alt: bool,
    /// Header slot at the top of the card.
    #[prop(optional, into)]
    header: Slot,
    /// Actions slot below the body.
    #[prop(optional, into)]
    actions: Slot,
    /// Footer slot at the bottom of the card.
    #[prop(optional, into)]
    footer: Slot,
    /// Padding in DU (`--op-card-padding`).
    #[prop(optional, into)]
    padding: MaybeProp<f32>,
    /// Gap between sections in DU (`--op-card-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Border outline width in DU (`--op-card-border-width`).
    #[prop(optional, into)]
    border_width: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let has_sections = header.is_set() || actions.is_set() || footer.is_set();
    let base_class = if alt {
        if has_sections {
            "op-card op-card-alt op-card-sections"
        } else {
            "op-card op-card-alt"
        }
    } else if has_sections {
        "op-card op-card-sections"
    } else {
        "op-card"
    };

    let content = if has_sections {
        view! {
            {header.run().map(|h| view! { <div class="op-card-header">{h}</div> })}
            <div class="op-card-body">{children()}</div>
            {actions.run().map(|a| view! { <div class="op-card-actions">{a}</div> })}
            {footer.run().map(|f| view! { <div class="op-card-footer">{f}</div> })}
        }
        .into_any()
    } else {
        children().into_any()
    };

    view! {
        <div
            class=root_class(base_class, class)
            style=root_style(
                vec![
                    Metric::du("--op-card-padding", padding),
                    Metric::du("--op-card-gap", gap),
                    Metric::du("--op-card-border-width", border_width),
                ],
                style,
            )
        >
            {content}
        </div>
    }
}
