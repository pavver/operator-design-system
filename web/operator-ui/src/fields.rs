//! Choice inputs: Chip.

use crate::basic::Icon;
use crate::core::{ControlState, Press, du, spawn_ripple, use_state_visual};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};

// ---------------------------------------------------------------------------
// Chip
// ---------------------------------------------------------------------------

/// Compact token: a toggleable filter, a removable value, or a static tag.
#[component]
pub fn Chip(
    #[prop(into)] text: Signal<String>,
    #[prop(optional)] icon: Option<IconData>,
    /// Makes the chip a toggle (filter chip).
    #[prop(optional, into)]
    selected: Option<RwSignal<bool>>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    /// Shows a remove button (input chip).
    #[prop(optional, into)]
    on_remove: Option<Callback<()>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
) -> impl IntoView {
    let press = Press::new();
    let interactive = selected.is_some() || on_click.is_some();
    let is_selected = move || selected.is_some_and(|s| s.get());
    let state = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if press.pressed.get() {
            ControlState::Pressed
        } else if is_selected() {
            ControlState::Selected
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(state);
    let content = move || {
        view! {
            {(selected.is_some())
                .then(|| {
                    view! {
                        <Show when=is_selected>
                            <span class="op-chip-check"><Icon icon=icons::CHECK size=3.5/></span>
                        </Show>
                    }
                })}
            {icon.map(|i| view! { <Icon icon=i size=3.5/> })}
            <span class="op-chip-text">{move || text.get()}</span>
        }
    };
    let main = if interactive {
        view! {
            <button
                type="button"
                class="op-rippled op-chip-main"
                role=selected.is_some().then_some("checkbox")
                aria-checked=move || selected.map(|s| s.get().to_string())
                aria-disabled=move || (!enabled.get()).then_some("true")
                on:pointerdown=move |ev| {
                    if enabled.get_untracked() {
                        press.down();
                        spawn_ripple(&ev);
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
                on:click=move |_| {
                    if !enabled.get_untracked() {
                        return;
                    }
                    if let Some(s) = selected {
                        s.update(|v| *v = !*v);
                    }
                    if let Some(cb) = on_click {
                        cb.run(());
                    }
                }
            >
                {content()}
            </button>
        }
        .into_any()
    } else {
        view! { <span class="op-chip-main">{content()}</span> }.into_any()
    };
    view! {
        <span
            class="op-sb op-chip"
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
            data-selected=move || is_selected().then_some("")
        >
            {main}
            {on_remove
                .map(|cb| {
                    view! {
                        <button
                            type="button"
                            class="op-rippled op-chip-remove"
                            aria-label=move || format!("Remove {}", text.get())
                            disabled=move || !enabled.get()
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| cb.run(())
                        >
                            <Icon icon=icons::CLOSE size=3.5/>
                        </button>
                    }
                })}
        </span>
    }
}

/// Row of chips that wraps onto new lines.
#[component]
pub fn ChipSet(children: Children) -> impl IntoView {
    view! { <div class="op-chip-set" style:gap=du(dim::PAD_TIGHT)>{children()}</div> }
}
