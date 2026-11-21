//! Private SVG rendering that follows inherited family metrics and localized accessible names.
use leptos::prelude::*;
use operator_ui_tokens::Icon as IconData;

pub(super) fn symbol(
    icon: Signal<IconData>,
    size: &'static str,
    label: Option<Signal<String>>,
) -> AnyView {
    view! {
        <svg class="op-icon" viewBox=move || icon.get().view_box
            role=label.map(|_| "img") aria-hidden=label.is_none().then_some("true")
            aria-label=move || label.map(|text| text.get()) style:width=size style:height=size>
            {move || icon.get().paths.iter().map(|path| view! { <path d=*path></path> }).collect_view()}
        </svg>
    }.into_any()
}

pub(super) fn pending(size: &'static str, label: Signal<String>) -> AnyView {
    view! {
        <span class="op-pending-indicator">
            <svg class="op-spinner" role="progressbar" aria-label=move || label.get()
                style:width=size style:height=size>
                <circle cx="50%" cy="50%" r="calc(50% - var(--op-stepper-spinner-stroke, var(--op-dim-spinner-stroke)) / 2)" pathLength="100"
                    stroke="var(--op-stepper-pending, var(--op-pending))"
                    stroke-width="var(--op-stepper-spinner-stroke, var(--op-dim-spinner-stroke))"></circle>
            </svg>
        </span>
    }
    .into_any()
}
