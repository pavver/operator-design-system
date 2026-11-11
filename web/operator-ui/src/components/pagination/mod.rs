use crate::components::icon_button::IconButton;
use crate::core::spawn_ripple;
use leptos::prelude::*;
use operator_ui_tokens::icons;

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
fn PageInput(page: RwSignal<usize>, #[prop(into)] total: Signal<usize>) -> impl IntoView {
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
            aria-label=move || format!("Page {} of {}, type a page number", page.get(), total.get())
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
) -> impl IntoView {
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
                Some(n) if n == current => view! { <PageInput page=page total=total/> }.into_any(),
                Some(n) => view! {
                    <button
                        type="button"
                        class="op-sb op-rippled op-page"
                        aria-label=format!("Page {n}")
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
    view! {
        <nav class="op-pagination" aria-label="Pagination">
            <IconButton
                icon=icons::CHEVRON_LEFT
                label="Previous page"
                enabled=Signal::derive(move || page.get() > 1)
                on_click=Callback::new(move |()| go(page.get_untracked().saturating_sub(1)))
            />
            {move || {
                if compact {
                    view! {
                        <span class="op-page-compact">
                            <PageInput page=page total=total/>
                            <span class="op-page-total">{move || format!("/ {}", total.get())}</span>
                        </span>
                    }
                        .into_any()
                } else {
                    numbers().into_any()
                }
            }}
            <IconButton
                icon=icons::CHEVRON_RIGHT
                label="Next page"
                enabled=Signal::derive(move || page.get() < total.get())
                on_click=Callback::new(move |()| go(page.get_untracked() + 1))
            />
        </nav>
    }
}
