use crate::components::icon_button::IconButton;
use crate::core::{Metric, Slot, root_class, root_style, spawn_ripple, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

pub(crate) const CSS: &str = include_str!("pagination.css");

/// Page numbers to show: `None` is an ellipsis.
pub fn page_window(current: usize, total: usize, siblings: usize) -> Vec<Option<usize>> {
    if total == 0 {
        return Vec::new();
    }
    let current = current.clamp(1, total);
    let start = current.saturating_sub(siblings).max(2);
    let end = (current + siblings).min(total.saturating_sub(1));
    let mut pages = vec![Some(1)];
    if start > 2 {
        pages.push(if start == 3 { Some(2) } else { None });
    }
    pages.extend((start..=end).map(Some));
    if end + 1 < total {
        pages.push(if end + 2 == total {
            Some(total - 1)
        } else {
            None
        });
    }
    if total > 1 {
        pages.push(Some(total));
    }
    pages
}

/// Parses a typed page number and clamps it to `1..=total`; `None` for empty input or no pages.
pub fn parse_page(text: &str, total: usize) -> Option<usize> {
    if total == 0 {
        return None;
    }
    let digits: String = text.chars().filter(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return None;
    }
    // Too many digits to fit means far past the end.
    Some(digits.parse::<usize>().map_or(total, |n| n.clamp(1, total)))
}

/// Current page as an editable field: shows the page, takes a typed number, and goes there on Enter.
#[component]
fn PageInput(
    page: RwSignal<usize>,
    #[prop(into)] total: Signal<usize>,
    aria_label: Signal<String>,
) -> impl IntoView {
    // `Some` while the operator is typing; the field shows the current page otherwise.
    let draft = RwSignal::new(None::<String>);
    let focused = RwSignal::new(false);
    let commit = move || {
        if let Some(p) = draft
            .get_untracked()
            .and_then(|text| parse_page(&text, total.get_untracked()))
        {
            page.set(p);
        }
        draft.set(None);
    };
    view! {
        <input
            class="op-sb op-page op-page-input"
            type="text"
            inputmode="numeric"
            autocomplete="off"
            aria-label=move || aria_label.get()
            aria-current="page"
            data-state=move || if focused.get() { "focused" } else { "selected" }
            style:width=move || format!("calc({}ch + var(--op-dim-pad-tight) * 2)", total.get().to_string().len())
            prop:value=move || draft.get().unwrap_or_else(|| page.get().to_string())
            on:focus=move |ev| {
                focused.set(true);
                event_target::<web_sys::HtmlInputElement>(&ev).select();
            }
            on:input=move |ev| {
                let digits: String = event_target_value(&ev).chars().filter(char::is_ascii_digit).collect();
                draft.set(Some(digits));
            }
            on:keydown=move |ev| match ev.key().as_str() {
                "Enter" => {
                    ev.prevent_default();
                    commit();
                }
                "Escape" => {
                    draft.set(None);
                    event_target::<web_sys::HtmlInputElement>(&ev).select();
                }
                _ => {}
            }
            on:blur=move |_| {
                focused.set(false);
                draft.set(None);
            }
        />
    }
}

/// Navigation between numbered pages; `page` is 1-based.
///
/// The current page is a field: the operator can type a page number into it and press Enter.
#[component]
pub fn Pagination(
    page: RwSignal<usize>,
    #[prop(into)] total: Signal<usize>,
    /// Pages shown on each side of the current one.
    #[prop(default = 1)]
    siblings: usize,
    /// Shows `3 / 12` instead of page numbers.
    #[prop(optional)]
    compact: bool,
    /// Spacing between page controls in DU (`--op-pagination-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Size of page buttons in DU (`--op-pagination-page-size`).
    #[prop(optional, into)]
    page_size: MaybeProp<f32>,
    /// Horizontal padding of page buttons in DU (`--op-pagination-page-padding-x`).
    #[prop(optional, into)]
    page_padding_x: MaybeProp<f32>,
    /// Gap in compact mode in DU (`--op-pagination-compact-gap`).
    #[prop(optional, into)]
    compact_gap: MaybeProp<f32>,
    /// Horizontal padding in compact mode in DU (`--op-pagination-compact-padding-x`).
    #[prop(optional, into)]
    compact_padding_x: MaybeProp<f32>,
    /// Accessible name of the pagination container; defaults to `pagination.label`.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Label of the previous page button; defaults to `pagination.previous`.
    #[prop(optional, into)]
    prev_label: MaybeProp<String>,
    /// Label of the next page button; defaults to `pagination.next`.
    #[prop(optional, into)]
    next_label: MaybeProp<String>,
    /// Template for page buttons; defaults to `pagination.page` ("Page {n}").
    #[prop(optional, into)]
    page_label: MaybeProp<String>,
    /// Template for the current page input; defaults to `pagination.current_page` ("Page {page} of {total}, type a page number").
    #[prop(optional, into)]
    current_page_label: MaybeProp<String>,
    /// Previous button icon; defaults to `pagination.previous`.
    #[prop(optional)]
    prev_icon: Option<IconData>,
    /// Next button icon; defaults to `pagination.next`.
    #[prop(optional)]
    next_icon: Option<IconData>,
    /// Slot before pagination elements.
    #[prop(optional, into)]
    before: Slot,
    /// Slot after pagination elements.
    #[prop(optional, into)]
    after: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let label = ui_text(label, "pagination.label", "Pagination");
    let prev_label = ui_text(prev_label, "pagination.previous", "Previous page");
    let next_label = ui_text(next_label, "pagination.next", "Next page");
    let page_label = ui_text(page_label, "pagination.page", "Page {n}");
    let current_page_label = ui_text(
        current_page_label,
        "pagination.current_page",
        "Page {page} of {total}, type a page number",
    );
    let prev_icon = ui_icon(prev_icon, "pagination.previous", icons::CHEVRON_LEFT);
    let next_icon = ui_icon(next_icon, "pagination.next", icons::CHEVRON_RIGHT);

    let format_page_label = move |n: usize| page_label.get().replace("{n}", &n.to_string());
    let format_current_label = Signal::derive(move || {
        current_page_label
            .get()
            .replace("{page}", &page.get().to_string())
            .replace("{total}", &total.get().to_string())
    });

    let go = move |p: usize| {
        let t = total.get_untracked();
        if t > 0 {
            page.set(p.clamp(1, t));
        }
    };
    let numbers = move || {
        let current = page.get();
        page_window(current, total.get(), siblings)
            .into_iter()
            .map(|p| match p {
                Some(n) if n == current => {
                    view! { <PageInput page=page total=total aria_label=format_current_label/> }
                        .into_any()
                }
                Some(n) => view! {
                    <button
                        type="button"
                        class="op-sb op-rippled op-page"
                        aria-label=format_page_label(n)
                        data-state="idle"
                        on:pointerdown=move |ev| spawn_ripple(&ev)
                        on:click=move |_| go(n)
                    >
                        {n.to_string()}
                    </button>
                }
                .into_any(),
                None => {
                    view! { <span class="op-page-gap" aria-hidden="true">"…"</span> }.into_any()
                }
            })
            .collect_view()
    };
    let root_metrics = vec![
        Metric::du("--op-pagination-gap", gap),
        Metric::du("--op-pagination-page-size", page_size),
        Metric::du("--op-pagination-page-padding-x", page_padding_x),
        Metric::du("--op-pagination-compact-gap", compact_gap),
        Metric::du("--op-pagination-compact-padding-x", compact_padding_x),
    ];
    view! {
        <nav class=root_class("op-pagination", class) style=root_style(root_metrics, style) aria-label=move || label.get()>
            {before.run()}
            <IconButton
                icon=prev_icon.get()
                label=prev_label.get()
                enabled=Signal::derive(move || page.get() > 1)
                on_click=Callback::new(move |()| go(page.get_untracked().saturating_sub(1)))
            />
            {move || {
                if compact {
                    view! {
                        <span class="op-page-compact">
                            <PageInput page=page total=total aria_label=format_current_label/>
                            <span class="op-page-total">{move || format!("/ {}", total.get())}</span>
                        </span>
                    }
                        .into_any()
                } else {
                    numbers().into_any()
                }
            }}
            <IconButton
                icon=next_icon.get()
                label=next_label.get()
                enabled=Signal::derive(move || page.get() < total.get())
                on_click=Callback::new(move |()| go(page.get_untracked() + 1))
            />
            {after.run()}
        </nav>
    }
}
