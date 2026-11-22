use crate::components::combobox::options::{ComboRow, option_at, option_count, step};
use crate::core::ControlState;
use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboKeyAction {
    Down,
    Up,
    Enter,
    Escape,
}

pub fn handle_combobox_key(key: &str) -> Option<ComboKeyAction> {
    match key {
        "ArrowDown" => Some(ComboKeyAction::Down),
        "ArrowUp" => Some(ComboKeyAction::Up),
        "Enter" => Some(ComboKeyAction::Enter),
        "Escape" => Some(ComboKeyAction::Escape),
        _ => None,
    }
}

#[component]
pub fn ComboboxInput(
    query: RwSignal<String>,
    open: RwSignal<bool>,
    active: RwSignal<usize>,
    filtering: RwSignal<bool>,
    focused: RwSignal<bool>,
    current: Signal<ControlState>,
    error: MaybeProp<String>,
    placeholder: Signal<String>,
    accessible_name: Signal<String>,
    rows: Memo<Vec<ComboRow>>,
    on_open_list: Callback<()>,
    on_reveal_active: Callback<bool>,
    on_commit_value: Callback<String>,
    on_settle: Callback<()>,
    shown_text: Callback<(), String>,
    blocked: Signal<bool>,
) -> impl IntoView {
    let on_keydown = move |ev: ev::KeyboardEvent| {
        if blocked.get() {
            return;
        }
        let Some(action) = handle_combobox_key(ev.key().as_str()) else {
            return;
        };
        let n = rows.with_untracked(|r| option_count(r));
        match action {
            ComboKeyAction::Down => {
                ev.prevent_default();
                if !open.get_untracked() {
                    on_open_list.run(());
                } else {
                    active.update(|a| *a = step(*a, n, 1));
                    on_reveal_active.run(false);
                }
            }
            ComboKeyAction::Up => {
                ev.prevent_default();
                if open.get_untracked() {
                    active.update(|a| *a = step(*a, n, -1));
                    on_reveal_active.run(false);
                }
            }
            ComboKeyAction::Enter => {
                ev.prevent_default();
                let option = rows.with_untracked(|r| option_at(r, active.get_untracked()));
                match option {
                    Some(o) if open.get_untracked() => on_commit_value.run(o.value),
                    _ => on_settle.run(()),
                }
            }
            ComboKeyAction::Escape => {
                open.set(false);
                filtering.set(false);
                query.set(shown_text.run(()));
            }
        }
    };

    view! {
        <input
            class="op-field-input"
            role="combobox"
            aria-label=move || accessible_name.get()
            aria-expanded=move || open.get().to_string()
            aria-autocomplete="list"
            aria-invalid=move || error.get().is_some().then_some("true")
            placeholder=move || placeholder.get()
            prop:value=move || query.get()
            disabled=move || current.get() == ControlState::Disabled
            readonly=move || current.get() == ControlState::Pending
            on:input=move |ev| {
                query.set(event_target_value(&ev));
                filtering.set(true);
                active.set(0);
                open.set(true);
                on_reveal_active.run(false);
            }
            on:focus=move |_| {
                focused.set(true);
                on_open_list.run(());
            }
            on:click=move |_| on_open_list.run(())
            on:blur=move |_| {
                focused.set(false);
                on_settle.run(());
            }
            on:keydown=on_keydown
        />
    }
}
