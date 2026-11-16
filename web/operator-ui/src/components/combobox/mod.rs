//! Text input with suggestions: Combobox.

use crate::components::dropdown_menu::MENU_MAX_HEIGHT;
use crate::components::primitives::Icon;
use crate::components::virtual_list::{VirtualList, VirtualListHandle};
use crate::core::{PendingRun, field_state, use_state_visual};
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};

pub(crate) const CSS: &str = include_str!("combobox.css");

/// Text input with a filtered list of suggestions.
#[component]
pub fn Combobox(
    options: Vec<String>,
    /// Committed value.
    selected: RwSignal<String>,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
    /// Accepts text that is not in `options`.
    #[prop(optional)]
    allow_custom: bool,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
) -> impl IntoView {
    let options = StoredValue::new(options);
    let query = RwSignal::new(selected.get_untracked());
    let open = RwSignal::new(false);
    let active = RwSignal::new(0usize);
    let focused = RwSignal::new(false);
    let visual = use_state_visual(field_state(enabled, Signal::stored(None), focused));

    Effect::new(move |_| {
        let v = selected.get();
        if !focused.get_untracked() {
            query.set(v);
        }
    });

    // The list shows every option when it opens; typing switches it to filtering.
    let filtering = RwSignal::new(false);
    let list = VirtualListHandle::new(dim::ROW_CONTROL);

    let filtered = Memo::new(move |_| {
        let q = if filtering.get() {
            query.get().to_lowercase()
        } else {
            String::new()
        };
        options.with_value(|o| {
            o.iter()
                .filter(|s| q.is_empty() || s.to_lowercase().contains(&q))
                .cloned()
                .collect::<Vec<_>>()
        })
    });

    // Scrolls the list so the active option is visible; `center` places it in the middle.
    let reveal_active = move |center: bool| list.reveal(active.get_untracked(), center);

    // Opens the full list with the selected option active, or else the first option matching the text.
    let open_list = move || {
        if open.get_untracked() {
            return;
        }
        filtering.set(false);
        let current = selected.get_untracked();
        let q = query.get_untracked().to_lowercase();
        let index = options.with_value(|o| {
            o.iter()
                .position(|s| !current.is_empty() && *s == current)
                .or_else(|| {
                    (!q.is_empty())
                        .then(|| o.iter().position(|s| s.to_lowercase().contains(&q)))
                        .flatten()
                })
                .unwrap_or(0)
        });
        active.set(index);
        open.set(true);
        reveal_active(true);
    };

    let commit = move |v: String| {
        query.set(v.clone());
        selected.set(v.clone());
        open.set(false);
        filtering.set(false);
        if let Some(cb) = on_select {
            cb.run(v);
        }
    };
    let settle = move || {
        open.set(false);
        filtering.set(false);
        let q = query.get_untracked();
        let known = options.with_value(|o| o.contains(&q));
        if q == selected.get_untracked() {
            return;
        }
        if known || (allow_custom && !q.is_empty()) {
            commit(q);
        } else {
            query.set(selected.get_untracked());
        }
    };

    view! {
        <div class="op-dropdown op-combobox">
            {label.clone().map(|l| view! { <div class="op-dropdown-label">{l}</div> })}
            <div class="op-dropdown-anchor">
                <div
                    class="op-sb op-field"
                    data-state=move || visual.state.get()
                    data-result=move || visual.result.get()
                    data-fading=move || visual.fading.get().then_some("")
                    data-trailing=""
                >
                    <input
                        class="op-field-input"
                        role="combobox"
                        aria-label=label
                        aria-expanded=move || open.get().to_string()
                        aria-autocomplete="list"
                        placeholder=move || placeholder.get()
                        prop:value=move || query.get()
                        disabled=move || !enabled.get()
                        on:input=move |ev| {
                            query.set(event_target_value(&ev));
                            filtering.set(true);
                            active.set(0);
                            open.set(true);
                            reveal_active(false);
                        }
                        on:focus=move |_| {
                            focused.set(true);
                            open_list();
                        }
                        on:click=move |_| open_list()
                        on:blur=move |_| {
                            focused.set(false);
                            settle();
                        }
                        on:keydown=move |ev| {
                            let n = filtered.with_untracked(Vec::len);
                            match ev.key().as_str() {
                                "ArrowDown" => {
                                    ev.prevent_default();
                                    if !open.get_untracked() {
                                        open_list();
                                    } else if n > 0 {
                                        active.update(|a| *a = (*a + 1) % n);
                                        reveal_active(false);
                                    }
                                }
                                "ArrowUp" => {
                                    ev.prevent_default();
                                    if n > 0 && open.get_untracked() {
                                        active.update(|a| *a = (*a + n - 1) % n);
                                        reveal_active(false);
                                    }
                                }
                                "Enter" => {
                                    ev.prevent_default();
                                    let pick = filtered.with_untracked(|f| f.get(active.get_untracked()).cloned());
                                    match pick {
                                        Some(v) if open.get_untracked() => commit(v),
                                        _ => settle(),
                                    }
                                }
                                "Escape" => {
                                    open.set(false);
                                    filtering.set(false);
                                    query.set(selected.get_untracked());
                                }
                                _ => {}
                            }
                        }
                    />
                    <span class="op-field-reveal op-combobox-arrow" aria-hidden="true">
                        <Icon icon=icons::ARROW_DROP_DOWN size=5.0/>
                    </span>
                    <PendingRun/>
                </div>
                <Show when=move || open.get() && enabled.get()>
                    <div class="op-dropdown-menu" role="listbox" on:mousedown=|ev| ev.prevent_default()>
                        {move || filtered.with(Vec::is_empty).then(|| {
                            let text = if allow_custom { "Press Enter to use this value" } else { "No matches" };
                            view! { <div class="op-dropdown-item op-combobox-none">{text}</div> }
                        })}
                        <VirtualList
                            items=Signal::derive(move || filtered.get())
                            handle=list
                            max_height=Some(MENU_MAX_HEIGHT)
                            class="op-scroll-contain"
                            key=|option: &String| option.clone()
                            let:row
                        >
                            {
                                let (i, option) = row;
                                let value = option.clone();
                                let is_selected = {
                                    let option = option.clone();
                                    move || selected.with(|s| *s == option)
                                };
                                let label = option.clone();
                                view! {
                                    <button
                                        type="button"
                                        role="option"
                                        tabindex="-1"
                                        class="op-dropdown-item"
                                        aria-selected=move || is_selected().to_string()
                                        data-active=move || (active.get() == i).then_some("")
                                        on:pointerenter=move |_| active.set(i)
                                        on:click=move |_| commit(value.clone())
                                    >
                                        {move || {
                                            let q = if filtering.get() { query.get() } else { String::new() };
                                            highlight(&label, &q)
                                        }}
                                    </button>
                                }
                            }
                        </VirtualList>
                    </div>
                </Show>
            </div>
        </div>
    }
}

/// Splits `text` around the first case-insensitive match of `query` and marks it.
fn highlight(text: &str, query: &str) -> AnyView {
    let lower = text.to_lowercase();
    let q = query.to_lowercase();
    match (!q.is_empty())
        .then(|| lower.find(&q))
        .flatten()
        .filter(|_| lower.len() == text.len())
    {
        Some(start) => {
            let end = start + q.len();
            let (a, rest) = text.split_at(start);
            let (m, b) = rest.split_at(end - start);
            view! { <span>{a.to_string()}<mark class="op-match">{m.to_string()}</mark>{b.to_string()}</span> }
                .into_any()
        }
        None => view! { <span>{text.to_string()}</span> }.into_any(),
    }
}
