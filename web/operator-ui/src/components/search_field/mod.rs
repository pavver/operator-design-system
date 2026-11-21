//! Search input with clear button: SearchField.

use crate::components::field::{Field, FieldMetrics, FieldVariant};
use crate::components::primitives::{Icon, ProgressIndicator};
use crate::core::{JobToken, Slot, field_state, spawn_ripple, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};
use std::time::Duration;

pub(crate) const CSS: &str = include_str!("search_field.css");

/// Search input with a clear button; reports the query after a short pause in typing.
#[component]
pub fn SearchField(
    value: RwSignal<String>,
    /// Hint inside the empty field; defaults to the `search-field.placeholder` string.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Called with the query after `debounce_ms` without typing, on Enter, and on clear.
    #[prop(optional, into)]
    on_search: Option<Callback<String>>,
    #[prop(default = motion::SEARCH_DEBOUNCE_MS)] debounce_ms: u32,
    /// Shows a spinner in place of the search icon while results load.
    #[prop(optional, into)]
    pending: Signal<bool>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Visible label above the field; also the accessible name.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Accessible name when there is no visible label; defaults to the `search-field.label` string.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// Accessible label of the clear button; defaults to the `search-field.clear` string.
    #[prop(optional, into)]
    clear_label: MaybeProp<String>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] variant: Signal<FieldVariant>,
    /// Search icon; defaults to the `search-field.search` icon.
    #[prop(optional)]
    icon: Option<IconData>,
    /// Clear icon; defaults to the `field.clear` icon.
    #[prop(optional)]
    clear_icon: Option<IconData>,
    /// Replaces the search icon slot.
    #[prop(optional, into)]
    prepend: Slot,
    /// Content inside the box before the clear button.
    #[prop(optional, into)]
    append: Slot,
    #[prop(optional, into)] before: Slot,
    #[prop(optional, into)] after: Slot,
    /// Box height in DU (`--op-field-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Horizontal box padding in DU (`--op-field-padding-x`).
    #[prop(optional, into)]
    padding_x: MaybeProp<f32>,
    /// Size of the icon and clear-button slots in DU (`--op-field-slot-size`).
    #[prop(optional, into)]
    slot_size: MaybeProp<f32>,
    /// Gap between label, box, hint and the outer slots in DU (`--op-field-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Text size in TU (`--op-field-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let input = NodeRef::<leptos::html::Input>::new();
    let focused = RwSignal::new(false);
    let state = field_state(enabled, Signal::stored(None), focused);
    let job = JobToken::new();
    let placeholder = ui_text(placeholder, "search-field.placeholder", "Search");
    let fallback_name = ui_text(aria_label, "search-field.label", "Search");
    let accessible_name =
        Signal::derive(move || label.get().unwrap_or_else(|| fallback_name.get()));
    let clear_label = ui_text(clear_label, "search-field.clear", "Clear search");
    let icon = ui_icon(icon, "search-field.search", icons::SEARCH);
    let clear_icon = ui_icon(clear_icon, "field.clear", icons::CLOSE);

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

    let lead = prepend.or(move || {
        {
            view! {
                <span class="op-search-lead">
                    {move || {
                        if pending.get() {
                            view! { <ProgressIndicator size=dim::ICON_MEDIUM stroke=dim::SPINNER_STROKE color="var(--op-pending)"/> }
                                .into_any()
                        } else {
                            view! { <Icon icon=icon.get() size=dim::ICON_LARGE/> }.into_any()
                        }
                    }}
                </span>
            }
        }
    });
    let trailing = Slot::from(move || {
        view! {
            {append.run()}
            <Show when=move || value.with(|v| !v.is_empty())>
                <button
                    type="button"
                    class="op-rippled op-field-reveal"
                    aria-label=move || clear_label.get()
                    disabled=move || !enabled.get()
                    on:mousedown=|ev| ev.prevent_default()
                    on:pointerdown=move |ev| spawn_ripple(&ev)
                    on:click=move |_| clear()
                >
                    {move || view! { <Icon icon=clear_icon.get() size=dim::ICON_SMALL/> }}
                </button>
            </Show>
        }
    });

    view! {
        <Field
            state=state
            label=label
            hint=hint
            error=error
            variant=variant
            prepend=lead
            append=trailing
            before=before
            after=after
            box_class="op-search"
            role="search"
            trailing=Signal::derive(move || value.with(|v| !v.is_empty()))
            pending_run=true
            metrics=FieldMetrics { height, padding_x, slot_size, gap, font_size }
            class=class
            style=style
        >
            <input
                node_ref=input
                class="op-field-input"
                type="search"
                aria-label=move || accessible_name.get()
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
        </Field>
    }
}
