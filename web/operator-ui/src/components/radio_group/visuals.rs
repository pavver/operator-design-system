//! Private SVG rendering that follows inherited family metrics and localized accessible names.
use leptos::prelude::*;

pub(super) fn pending(size: &'static str, label: Signal<String>) -> AnyView {
    view! {
        <span class="op-pending-indicator">
            <svg class="op-spinner" role="progressbar" aria-label=move || label.get()
                style:width=size style:height=size>
                <circle cx="50%" cy="50%" r="calc(50% - var(--op-radio-group-spinner-stroke, var(--op-dim-spinner-stroke)) / 2)" pathLength="100"
                    stroke="var(--op-radio-group-pending, var(--op-pending))"
                    stroke-width="var(--op-radio-group-spinner-stroke, var(--op-dim-spinner-stroke))"></circle>
            </svg>
        </span>
    }
    .into_any()
}
