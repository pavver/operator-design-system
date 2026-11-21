//! Pending overlay blocking input: LoadingOverlay.

use crate::components::primitives::ProgressIndicator;
use crate::core::{Metric, root_class, root_style, ui_text};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("loading_overlay.css");

/// Full-cover overlay with spinner and label that obscures its parent and blocks interaction while work is pending.
#[component]
pub fn LoadingOverlay(
    /// Whether the loading overlay is currently active.
    #[prop(into)]
    showing: Signal<bool>,
    /// Visible text label and accessible description; defaults to `"Loading..."`.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Spinner size in DU (`--op-loading-overlay-spinner-size`); defaults to `8 DU`.
    #[prop(optional, into)]
    spinner_size: MaybeProp<f32>,
    /// Spinner stroke thickness in DU (`--op-loading-overlay-spinner-stroke`); defaults to `0.5 DU`.
    #[prop(optional, into)]
    spinner_stroke: MaybeProp<f32>,
    /// Gap between spinner and label in DU (`--op-loading-overlay-gap`); defaults to `2 DU`.
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Font size for the label in TU (`--op-loading-overlay-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let resolved_label = ui_text(label, "loading-overlay.loading", "Loading...");
    let spinner_size_val = move || spinner_size.get().unwrap_or(8.0);
    let spinner_stroke_val = move || spinner_stroke.get().unwrap_or(0.5);

    let overlay_style = root_style(
        vec![
            Metric::du("--op-loading-overlay-spinner-size", spinner_size),
            Metric::du("--op-loading-overlay-spinner-stroke", spinner_stroke),
            Metric::du("--op-loading-overlay-gap", gap),
            Metric::tu("--op-loading-overlay-font-size", font_size),
        ],
        style,
    );

    let overlay = view! {
        <div
            class=root_class("op-loading-overlay", class)
            style=overlay_style
            role="status"
            aria-live="polite"
            aria-busy=move || showing.get().then_some("true")
            data-showing=move || showing.get().then_some("")
        >
            {move || showing.get().then(|| view! {
                <div class="op-loading-overlay-content">
                    <ProgressIndicator
                        size=spinner_size_val()
                        stroke=spinner_stroke_val()
                        color="var(--op-loading-overlay-spinner, var(--op-accent))"
                    />
                    <span class="op-loading-overlay-label">{resolved_label.get()}</span>
                </div>
            })}
        </div>
    };

    if let Some(children) = children {
        view! {
            <div class="op-loading-overlay-container">
                {children()}
                {overlay}
            </div>
        }
        .into_any()
    } else {
        overlay.into_any()
    }
}
