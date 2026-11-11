//! Dropdown menu: DropdownMenu.

use crate::components::primitives::{Icon, PendingIndicator};
use crate::components::scrollbar::ScrollView;
use crate::core::{
    ControlState, PendingRun, Press, spawn_ripple, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};

pub(crate) const CSS: &str = include_str!("dropdown_menu.css");

/// Rectangular dropdown with a scrollable option list (max 50 DU).
#[component]
pub fn DropdownMenu(
    options: Vec<String>,
    selected: RwSignal<String>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    let expanded = RwSignal::new(false);
    let press = Press::new();
    let external = Signal::derive(move || state.get().unwrap_or_default());

    let border_state = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if external.get() != ControlState::Idle {
            external.get()
        } else if press.pressed.get() {
            ControlState::Pressed
        } else if expanded.get() {
            ControlState::Selected
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(border_state);
    let show_spinner = use_pending_indicator(move || external.get() == ControlState::Pending);
    let clickable = move || {
        enabled.get()
            && !matches!(
                external.get(),
                ControlState::Disabled | ControlState::Pending
            )
    };
    Effect::new(move |_| {
        if !clickable() {
            expanded.set(false);
        }
    });

    let options = StoredValue::new(options);

    view! {
            <div class="op-dropdown">
                {label.map(|l| view! { <div class="op-dropdown-label">{l}</div> })}
                <div class="op-dropdown-anchor">
                    <button
                        type="button"
                        class="op-sb op-rippled op-dropdown-trigger"
                        aria-haspopup="listbox"
                        aria-expanded=move || expanded.get().to_string()
                        aria-disabled=move || (!clickable()).then_some("true")
                        data-state=move || visual.state.get()
                        data-result=move || visual.result.get()
                        data-fading=move || visual.fading.get().then_some("")
                        on:pointerdown=move |ev| {
                            if clickable() {
                                press.down();
                                spawn_ripple(&ev);
                            }
                        }
                        on:pointerup=move |_| press.up()
                        on:pointerleave=move |_| press.up()
                        on:pointercancel=move |_| press.up()
                        on:click=move |_| {
                            if clickable() {
                                expanded.update(|e| *e = !*e);
                            }
                        }
                        on:keydown=move |ev| {
                            if ev.key() == "Escape" {
                                expanded.set(false);
                            }
                        }
                    >
                        <span class="op-dropdown-value">{move || selected.get()}</span>
                        {move || {
                            if show_spinner.get() {
                                view! { <PendingIndicator size=dim::ICON_SMALL/> }
                                    .into_any()
                            } else {
                                view! { <span class="op-dropdown-arrow"><Icon icon=icons::ARROW_DROP_DOWN size=5.0/></span> }
                                    .into_any()
                            }
                        }}
                        <PendingRun/>
                    </button>
                    <Show when=move || expanded.get()>
                        <div class="op-dropdown-scrim" on:click=move |_| expanded.set(false)></div>
                        <div class="op-dropdown-menu" role="listbox">
                            <ScrollView viewport_class="op-scroll-contain">
                            {options
                                .get_value()
                                .into_iter()
                                .map(|option| {
                                    let value = option.clone();
                                    let is_selected = {
                                        let option = option.clone();
                                        move || selected.with(|s| *s == option)
                                    };
                                    view! {
                                        <button
                                            type="button"
                                            role="option"
                                            class="op-rippled op-dropdown-item"
                                            aria-selected=move || is_selected().to_string()
                                            on:pointerdown=move |ev| spawn_ripple(&ev)
                                            on:click=move |_| {
                                                selected.set(value.clone());
                                                if let Some(cb) = on_select {
                                                    cb.run(value.clone());
                                                }
                                                expanded.set(false);
                                            }
                                        >
                                            {option}
                                        </button>
                                    }
                                })
                                .collect_view()}
    </ScrollView>
                        </div>
                    </Show>
                </div>
            </div>
        }
}
