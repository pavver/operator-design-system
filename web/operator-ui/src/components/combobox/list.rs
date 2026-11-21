//! The option list shared by the Combobox field and the ComboboxPopup.

use super::options::{ComboOption, ComboRow};
use crate::components::dropdown_menu::{FieldList, FieldListHandle};
use crate::components::primitives::Icon;
use crate::components::virtual_list::{VirtualList, VirtualListHandle};
use crate::core::{ScopedSlot, Slot};
use leptos::prelude::*;
use operator_ui_tokens::dim;

/// Existing extension callers keep their fixed virtual-list handle.
#[derive(Clone, Copy)]
pub(crate) enum ComboListHandle {
    Field(FieldListHandle),
    Fixed(VirtualListHandle),
}
impl From<FieldListHandle> for ComboListHandle {
    fn from(handle: FieldListHandle) -> Self {
        Self::Field(handle)
    }
}
impl From<VirtualListHandle> for ComboListHandle {
    fn from(handle: VirtualListHandle) -> Self {
        Self::Fixed(handle)
    }
}

/// Virtualized rows with section headers, the active option and match highlighting.
#[component]
pub(crate) fn ComboList(
    rows: Memo<Vec<ComboRow>>,
    #[prop(into)] handle: ComboListHandle,
    /// Keyboard position of the active option.
    active: RwSignal<usize>,
    /// Committed value, marked as selected.
    #[prop(into)]
    selected: Signal<String>,
    /// Text to highlight in the labels.
    #[prop(into)]
    highlight: Signal<String>,
    /// Text of the row shown when no option matches; nothing when `None`.
    #[prop(into)]
    empty: Signal<Option<String>>,
    on_pick: Callback<ComboOption>,
    #[prop(default = None)] option: Option<ScopedSlot<ComboOption>>,
    #[prop(optional, into)] empty_slot: Slot,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let option_slot = option;
    let children = move |row: (usize, ComboRow)| match row.1 {
        ComboRow::Section(name) => {
            view! { <div class="op-combo-section op-t-caption" role="presentation">{name}</div> }
                .into_any()
        }
        ComboRow::Option(pos, option) => option_row(
            pos,
            option,
            active,
            selected,
            highlight,
            on_pick,
            option_slot,
        )
        .into_any(),
    };
    view! {
        <div class="op-dropdown-list" style=move || style.get() on:mousedown=|ev| ev.prevent_default()>
            {header.run()}
            {before_options.run()}
            {move || {
                rows.with(Vec::is_empty).then(|| {
                    if empty_slot.is_set() { empty_slot.run() }
                    else { empty.get().map(|text| view! { <div class="op-dropdown-item op-combobox-none">{text}</div> }.into_any()) }
                }).flatten()
            }}
            {match handle {
                ComboListHandle::Field(handle) => view! {
                    <FieldList items=Signal::derive(move || rows.get()) handle=handle
                        key=ComboRow::key children=children/>
                }.into_any(),
                ComboListHandle::Fixed(handle) => view! {
                    <VirtualList items=Signal::derive(move || rows.get()) handle=handle
                        max_height=Some(crate::components::dropdown_menu::MENU_MAX_HEIGHT)
                        class="op-scroll-contain" key=ComboRow::key children=children/>
                }.into_any(),
            }}
            {after_options.run()}
            {footer.run()}
        </div>
    }
}

fn option_row(
    pos: usize,
    option: ComboOption,
    active: RwSignal<usize>,
    selected: Signal<String>,
    highlight: Signal<String>,
    on_pick: Callback<ComboOption>,
    option_slot: Option<ScopedSlot<ComboOption>>,
) -> impl IntoView {
    let value = option.value.clone();
    let text = option.text().to_string();
    let picked = option.clone();
    view! {
        <button
            type="button"
            role="option"
            tabindex="-1"
            class="op-dropdown-item op-combo-option"
            aria-selected=move || selected.with(|s| *s == value).to_string()
            data-active=move || (active.get() == pos).then_some("")
            on:pointermove=move |_| {
                if active.get_untracked() != pos {
                    active.set(pos);
                }
            }
            on:click=move |_| on_pick.run(picked.clone())
        >
            {match option_slot {
                Some(slot) => slot.run(option),
                None => view! {
            {option.icon.map(|i| view! { <span class="op-combo-icon"><Icon icon=i size=dim::ICON_SMALL attr:style="width:var(--op-field-option-icon-size, var(--op-dim-icon-small));height:var(--op-field-option-icon-size, var(--op-dim-icon-small));"/></span> })}
            <span class="op-combo-label">{move || highlight_text(&text, &highlight.get())}</span>
            {option.detail.map(|d| view! { <span class="op-combo-detail op-t-caption">{d}</span> })}
                }.into_any(),
            }}
        </button>
    }
}

/// Splits `text` around the first case-insensitive match of `query` and marks it.
pub(crate) fn highlight_text(text: &str, query: &str) -> AnyView {
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
