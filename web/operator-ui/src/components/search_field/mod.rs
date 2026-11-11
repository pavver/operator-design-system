//! Search input with clear button: SearchField.

use crate::components::primitives::{Icon, ProgressIndicator};
use crate::core::{JobToken, PendingRun, field_state, spawn_ripple, use_state_visual};
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons, motion};
use std::time::Duration;

pub(crate) const CSS: &str = include_str!("search_field.css");

/// Search input with a clear button; reports the query after a short pause in typing.
#[component]
pub fn SearchField(
    value: RwSignal<String>,
    #[prop(default = "Search".into(), into)] placeholder: Signal<String>,
    /// Called with the query after `debounce_ms` without typing, on Enter, and on clear.
    #[prop(optional, into)]
    on_search: Option<Callback<String>>,
    #[prop(default = motion::SEARCH_DEBOUNCE_MS)] debounce_ms: u32,
    /// Shows a spinner in place of the search icon while results load.
    #[prop(optional, into)]
    pending: Signal<bool>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(default = "Search".into(), into)] label: String,
) -> impl IntoView {
    let input = NodeRef::<leptos::html::Input>::new();
    let focused = RwSignal::new(false);
    let visual = use_state_visual(field_state(enabled, Signal::stored(None), focused));
    let job = JobToken::new();

    let emit_now = move || {
        job.next();
        if let Some(cb) = on_search {
            cb.run(value.get_untracked());
        }
    };
    let emit_later = move || {
        let id = job.next();
        set_timeout(
            move || {
                if let Some(cb) = on_search.filter(|_| job.is_current(id)) {
                    cb.run(value.get_untracked());
                }
            },
            Duration::from_millis(u64::from(debounce_ms)),
        );
    };
    let clear = move || {
        value.set(String::new());
        emit_now();
        if let Some(el) = input.try_get_untracked().flatten() {
            let _ = el.focus();
        }
    };

    view! {
        <div
            class="op-sb op-field op-search"
            role="search"
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
            data-trailing=move || value.with(|v| !v.is_empty()).then_some("")
        >
            <span class="op-search-lead">
                {move || {
                    if pending.get() {
                        view! { <ProgressIndicator size=dim::ICON_MEDIUM stroke=dim::SPINNER_STROKE color="var(--op-pending)"/> }
                            .into_any()
                    } else {
                        view! { <Icon icon=icons::SEARCH size=dim::ICON_LARGE/> }.into_any()
                    }
                }}
            </span>
            <input
                node_ref=input
                class="op-field-input"
                type="search"
                aria-label=label
                placeholder=move || placeholder.get()
                prop:value=move || value.get()
                disabled=move || !enabled.get()
                on:input=move |ev| {
                    value.set(event_target_value(&ev));
                    emit_later();
                }
                on:keydown=move |ev| match ev.key().as_str() {
                    "Enter" => emit_now(),
                    "Escape" if !value.with_untracked(String::is_empty) => {
                        ev.prevent_default();
                        clear();
                    }
                    _ => {}
                }
                on:focus=move |_| focused.set(true)
                on:blur=move |_| focused.set(false)
            />
            <Show when=move || value.with(|v| !v.is_empty())>
                <button
                    type="button"
                    class="op-rippled op-field-reveal"
                    aria-label="Clear search"
                    disabled=move || !enabled.get()
                    on:mousedown=|ev| ev.prevent_default()
                    on:pointerdown=move |ev| spawn_ripple(&ev)
                    on:click=move |_| clear()
                >
                    <Icon icon=icons::CLOSE size=dim::ICON_SMALL/>
                </button>
            </Show>
            <PendingRun/>
        </div>
    }
}
