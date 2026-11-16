//! ComboboxPopup: a search field and an option list opened at a point (quick add, wire menus).

use super::list::ComboList;
use super::options::{
    ComboOption, ComboOptions, filter_rows, option_at, option_count, row_of, step,
};
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::virtual_list::VirtualListHandle;
use crate::core::du;
use leptos::prelude::*;
use operator_ui_tokens::dim;

/// Search field with a filtered option list in a floating layer, without a field on the page.
///
/// Typing filters, arrows move the active option, Enter or a click picks it and closes the layer.
/// Enter with no matching option passes the typed text to `on_custom` when it is set.
#[component]
pub fn ComboboxPopup(
    open: RwSignal<bool>,
    #[prop(into)] anchor: Signal<PopoverAnchor>,
    #[prop(into)] options: ComboOptions,
    on_select: Callback<ComboOption>,
    #[prop(optional)] on_custom: Option<Callback<String>>,
    #[prop(default = Placement::AtPoint)] placement: Placement,
    /// Layer width in DU.
    #[prop(default = 80.0)]
    width: f32,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(default = "Search".into(), into)] label: String,
    #[prop(default = "No matches".into(), into)] empty_text: Signal<String>,
    #[prop(default = "Press Enter to use this value".into(), into)] custom_hint: Signal<String>,
) -> impl IntoView {
    let options = options.0;
    let query = RwSignal::new(String::new());
    let active = RwSignal::new(0usize);
    let input = NodeRef::<leptos::html::Input>::new();
    let list = VirtualListHandle::new(dim::ROW_CONTROL);

    // Every opening starts with an empty search.
    Effect::new(move |_| {
        if open.get() {
            query.set(String::new());
            active.set(0);
        }
    });
    let rows = Memo::new(move |_| options.with(|o| filter_rows(o, &query.get())));
    let reveal_active = move || {
        if let Some(row) = rows.with_untracked(|r| row_of(r, active.get_untracked())) {
            list.reveal(row, false);
        }
    };
    let pick = Callback::new(move |o: ComboOption| {
        open.set(false);
        on_select.run(o);
    });
    let focus_input = Callback::new(move |()| {
        if let Some(el) = input.get_untracked() {
            let _ = el.focus();
        }
    });
    let empty = Signal::derive(move || {
        Some(if on_custom.is_some() && !query.with(String::is_empty) {
            custom_hint.get()
        } else {
            empty_text.get()
        })
    });

    view! {
        <Popover
            open=open
            anchor=anchor
            placement=placement
            class="op-dropdown-menu op-combo-popup"
            role="dialog"
            label=label.clone()
            on_placed=focus_input
        >
            <div class="op-combo-popup-body" style:width=du(width)>
                <input
                    node_ref=input
                    class="op-combo-popup-input"
                    role="combobox"
                    aria-label=label.clone()
                    aria-expanded="true"
                    aria-autocomplete="list"
                    placeholder=move || placeholder.get()
                    prop:value=move || query.get()
                    on:input=move |ev| {
                        query.set(event_target_value(&ev));
                        active.set(0);
                        reveal_active();
                    }
                    on:keydown=move |ev| {
                        let n = rows.with_untracked(|r| option_count(r));
                        match ev.key().as_str() {
                            "ArrowDown" => {
                                ev.prevent_default();
                                active.update(|a| *a = step(*a, n, 1));
                                reveal_active();
                            }
                            "ArrowUp" => {
                                ev.prevent_default();
                                active.update(|a| *a = step(*a, n, -1));
                                reveal_active();
                            }
                            "Enter" => {
                                ev.prevent_default();
                                match rows.with_untracked(|r| option_at(r, active.get_untracked())) {
                                    Some(o) => pick.run(o),
                                    None => {
                                        let q = query.get_untracked();
                                        if let Some(cb) = on_custom.filter(|_| !q.is_empty()) {
                                            open.set(false);
                                            cb.run(q);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                />
                <ComboList
                    rows=rows
                    handle=list
                    active=active
                    selected=Signal::stored(String::new())
                    highlight=query
                    empty=empty
                    on_pick=pick
                />
            </div>
        </Popover>
    }
}
