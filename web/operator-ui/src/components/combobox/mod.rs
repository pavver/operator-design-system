//! Text input with suggestions: Combobox, and the pointer-anchored ComboboxPopup.

mod list;
mod options;
mod popup;

pub use options::{
    ComboOption, ComboOptions, ComboRow, display_text, filter_rows, find_exact, option_at,
    option_count, row_of, step,
};
pub use popup::ComboboxPopup;

use crate::components::dropdown_menu::anchor_signal;
use crate::components::popover::{Placement, Popover};
use crate::components::primitives::Icon;
use crate::components::virtual_list::VirtualListHandle;
use crate::core::{PendingRun, field_state, use_state_visual};
use leptos::prelude::*;
pub(crate) use list::ComboList;
use operator_ui_tokens::{dim, icons};

pub(crate) const CSS: &str = include_str!("combobox.css");

/// Text input with a filtered list of suggestions.
#[component]
pub fn Combobox(
    /// Choices: plain strings or [`ComboOption`]s, fixed or as a signal.
    #[prop(into)]
    options: ComboOptions,
    /// Committed value.
    selected: RwSignal<String>,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
    /// Accepts text that is not an option.
    #[prop(optional)]
    allow_custom: bool,
    /// Turns typed text into a value, or rejects it with `None`; implies `allow_custom`.
    #[prop(optional)]
    parse_custom: Option<Callback<String, Option<String>>>,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Row shown when nothing matches.
    #[prop(default = "No matches".into(), into)]
    empty_text: Signal<String>,
    /// Row shown when nothing matches but the typed text can be used.
    #[prop(default = "Press Enter to use this value".into(), into)]
    custom_hint: Signal<String>,
) -> impl IntoView {
    let options = options.0;
    let custom = allow_custom || parse_custom.is_some();
    let shown = move || options.with_untracked(|o| display_text(o, &selected.get_untracked()));
    let query = RwSignal::new(shown());
    let open = RwSignal::new(false);
    let active = RwSignal::new(0usize);
    let focused = RwSignal::new(false);
    let visual = use_state_visual(field_state(enabled, Signal::stored(None), focused));

    // An unfocused field shows the committed value, also when the options arrive later.
    Effect::new(move |_| {
        let v = selected.get();
        let text = options.with(|o| display_text(o, &v));
        if !focused.get_untracked() {
            query.set(text);
        }
    });

    // The list shows every option when it opens; typing switches it to filtering.
    let filtering = RwSignal::new(false);
    let list = VirtualListHandle::new(dim::ROW_CONTROL);
    let anchor_ref = NodeRef::<leptos::html::Div>::new();
    let anchor = anchor_signal(anchor_ref);
    // A disabled field never keeps its list open.
    Effect::new(move |_| {
        if !enabled.get() {
            open.set(false);
        }
    });

    let rows = Memo::new(move |_| {
        let q = if filtering.get() {
            query.get()
        } else {
            String::new()
        };
        options.with(|o| filter_rows(o, &q))
    });
    let highlight = Signal::derive(move || {
        if filtering.get() {
            query.get()
        } else {
            String::new()
        }
    });

    // Scrolls the list so the active option is visible; `center` places it in the middle.
    let reveal_active = move |center: bool| {
        if let Some(row) = rows.with_untracked(|r| row_of(r, active.get_untracked())) {
            list.reveal(row, center);
        }
    };

    // Opens the full list with the selected option active, or else the first option matching the text.
    let open_list = move || {
        if open.get_untracked() {
            return;
        }
        filtering.set(false);
        let current = selected.get_untracked();
        let q = query.get_untracked().to_lowercase();
        let pos = options.with_untracked(|o| {
            o.iter()
                .position(|s| !current.is_empty() && s.value == current)
                .or_else(|| {
                    (!q.is_empty())
                        .then(|| o.iter().position(|s| s.matches(&q)))
                        .flatten()
                })
                .unwrap_or(0)
        });
        active.set(pos);
        open.set(true);
        reveal_active(true);
    };

    let commit_value = move |value: String| {
        query.set(options.with_untracked(|o| display_text(o, &value)));
        selected.set(value.clone());
        open.set(false);
        filtering.set(false);
        if let Some(cb) = on_select {
            cb.run(value);
        }
    };
    let pick = Callback::new(move |o: ComboOption| commit_value(o.value));
    let settle = move || {
        open.set(false);
        filtering.set(false);
        let q = query.get_untracked();
        if q == shown() {
            return;
        }
        if let Some(o) = options.with_untracked(|o| find_exact(o, &q)) {
            commit_value(o.value);
            return;
        }
        let parsed = if !custom || q.is_empty() {
            None
        } else if let Some(parse) = parse_custom {
            parse.run(q)
        } else {
            Some(q)
        };
        match parsed {
            Some(v) => commit_value(v),
            None => query.set(shown()),
        }
    };
    let empty = Signal::derive(move || {
        Some(if custom && !query.with(String::is_empty) {
            custom_hint.get()
        } else {
            empty_text.get()
        })
    });

    view! {
        <div class="op-dropdown op-combobox">
            {label.clone().map(|l| view! { <div class="op-dropdown-label">{l}</div> })}
            <div class="op-dropdown-anchor" node_ref=anchor_ref>
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
                            let n = rows.with_untracked(|r| option_count(r));
                            match ev.key().as_str() {
                                "ArrowDown" => {
                                    ev.prevent_default();
                                    if !open.get_untracked() {
                                        open_list();
                                    } else {
                                        active.update(|a| *a = step(*a, n, 1));
                                        reveal_active(false);
                                    }
                                }
                                "ArrowUp" => {
                                    ev.prevent_default();
                                    if open.get_untracked() {
                                        active.update(|a| *a = step(*a, n, -1));
                                        reveal_active(false);
                                    }
                                }
                                "Enter" => {
                                    ev.prevent_default();
                                    let option = rows.with_untracked(|r| option_at(r, active.get_untracked()));
                                    match option {
                                        Some(o) if open.get_untracked() => commit_value(o.value),
                                        _ => settle(),
                                    }
                                }
                                "Escape" => {
                                    open.set(false);
                                    filtering.set(false);
                                    query.set(shown());
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
                <Popover open=open anchor=anchor placement=Placement::BelowStretch class="op-dropdown-menu" role="listbox" restore_focus=false>
                    <ComboList
                        rows=rows
                        handle=list
                        active=active
                        selected=selected
                        highlight=highlight
                        empty=empty
                        on_pick=pick
                    />
                </Popover>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests;
