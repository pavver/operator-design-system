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
