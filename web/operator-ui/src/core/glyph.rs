//! Inline SVG glyphs shared by the controls.
//!
//! Icons follow the inherited family metrics and localized accessible names.
//! The pending spinner reads the family's own variables first
//! (`--op-<family>-pending`, `--op-<family>-spinner-stroke`) and falls back to the shared tokens.
use leptos::prelude::*;
use operator_ui_tokens::Icon as IconData;

/// Draws an icon; with a `label` it is announced as an image, otherwise it is hidden from screen readers.
pub fn symbol(icon: Signal<IconData>, size: &'static str, label: Option<Signal<String>>) -> AnyView {
    view! {
        <svg class="op-icon" viewBox=move || icon.get().view_box
            role=label.map(|_| "img") aria-hidden=label.is_none().then_some("true")
            aria-label=move || label.map(|text| text.get()) style:width=size style:height=size>
            {move || icon.get().paths.iter().map(|path| view! { <path d=*path></path> }).collect_view()}
        </svg>
    }
    .into_any()
}

/// Draws the pending spinner of a control `family` such as `"button"` or `"checkbox"`.
pub fn pending_spinner(family: &'static str, size: &'static str, label: Signal<String>) -> AnyView {
    let stroke = format!("var(--op-{family}-spinner-stroke, var(--op-dim-spinner-stroke))");
    let radius = format!("calc(50% - {stroke} / 2)");
    let colour = format!("var(--op-{family}-pending, var(--op-pending))");
    view! {
        <span class="op-pending-indicator">
            <svg class="op-spinner" role="progressbar" aria-label=move || label.get()
                style:width=size style:height=size>
                <circle cx="50%" cy="50%" r=radius pathLength="100" stroke=colour stroke-width=stroke></circle>
            </svg>
        </span>
    }
    .into_any()
}
