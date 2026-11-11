use crate::components::icon_button::IconButton;
use crate::core::{ControlState, spawn_ripple, use_state_visual};
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

/// Page count from which [`Pagination`] shows the jump field when `jump` is not set.
pub const PAGINATION_JUMP_MIN_PAGES: usize = 20;

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

/// Navigation between numbered pages; `page` is 1-based.
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
    /// Shows a field for typing a page number; by default shown from
    /// [`PAGINATION_JUMP_MIN_PAGES`] pages.
    #[prop(optional)]
    jump: Option<bool>,
) -> impl IntoView {
    let go = move |p: usize| {
        let t = total.get_untracked();
        if t > 0 {
            page.set(p.clamp(1, t));
        }
    };
    let typed = RwSignal::new(String::new());
    let focused = RwSignal::new(false);
    let commit = move || {
        if let Some(p) = parse_page(&typed.get_untracked(), total.get_untracked()) {
            page.set(p);
        }
        typed.set(String::new());
    };
    let show_jump = move || jump.unwrap_or_else(|| total.get() >= PAGINATION_JUMP_MIN_PAGES);
    let numbers = move || {
        page_window(page.get(), total.get(), siblings)
            .into_iter()
            .map(|p| match p {
                Some(n) => {
                    let current = move || page.get() == n;
                    let visual = use_state_visual(Signal::derive(move || {
                        if current() {
                            ControlState::Selected
                        } else {
                            ControlState::Idle
                        }
                    }));
                    view! {
                        <button
                            type="button"
                            class="op-sb op-rippled op-page"
                            aria-label=format!("Page {n}")
                            aria-current=move || current().then_some("page")
                            data-state=move || visual.state.get()
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| go(n)
                        >
                            {n.to_string()}
                        </button>
                    }
                    .into_any()
                }
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
                        <span class="op-page-compact">{format!("{} / {}", page.get(), total.get())}</span>
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
            <Show when=show_jump>
                <label class="op-page-jump">
                    <span class="op-page-jump-label">"Go to"</span>
                    <input
                        class="op-sb op-page-input"
                        type="text"
                        inputmode="numeric"
                        autocomplete="off"
                        aria-label="Go to page"
                        data-state=move || if focused.get() { "focused" } else { "idle" }
                        placeholder=move || page.get().to_string()
                        style:width=move || format!("calc({}ch + var(--op-dim-pad-content) * 2)", total.get().to_string().len().max(2))
                        prop:value=move || typed.get()
                        on:input=move |ev| {
                            let digits: String = event_target_value(&ev).chars().filter(char::is_ascii_digit).collect();
                            typed.set(digits);
                        }
                        on:keydown=move |ev| match ev.key().as_str() {
                            "Enter" => {
                                ev.prevent_default();
                                commit();
                            }
                            "Escape" => typed.set(String::new()),
                            _ => {}
                        }
                        on:focus=move |_| focused.set(true)
                        on:blur=move |_| {
                            focused.set(false);
                            typed.set(String::new());
                        }
                    />
                </label>
            </Show>
        </nav>
    }
}
