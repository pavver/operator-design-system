//! Checkbox group: GroupItem and CheckboxGroup.

use crate::components::primitives::{Icon, PendingIndicator};
use crate::core::{
    AsyncAction, ControlState, PendingRun, Press, sleep_ms, spawn_ripple, use_pending_indicator,
    use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};
use std::collections::HashMap;

pub(crate) const CSS: &str = include_str!("checkbox_group.css");

/// One option of a [`CheckboxGroup`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupItem {
    pub id: String,
    pub name: String,
    /// Optional leading icon, shown by [`CheckboxGroup`].
    pub icon: Option<IconData>,
}

impl GroupItem {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            icon: None,
        }
    }

    /// Adds a leading icon.
    #[must_use]
    pub fn with_icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Vertical group of full-width checkbox tiles; each tile runs its own async request.
#[component]
pub fn CheckboxGroup(
    items: Vec<GroupItem>,
    #[prop(into)] selected: Signal<Vec<String>>,
    /// Called with `(id, target_checked)`; the parent updates `selected` on success.
    #[prop(into)]
    on_change: AsyncAction<(String, bool)>,
) -> impl IntoView {
    let states = RwSignal::new(HashMap::<String, ControlState>::new());

    let tiles = items
        .into_iter()
        .map(|item| {
            let id = item.id.clone();
            let press = Press::new();
            let action = on_change.clone();
            let is_selected = {
                let id = id.clone();
                Signal::derive(move || selected.with(|s| s.contains(&id)))
            };
            let item_state = {
                let id = id.clone();
                Signal::derive(move || states.with(|m| m.get(&id).copied().unwrap_or_default()))
            };
            let border_state = Signal::derive(move || {
                let s = item_state.get();
                if s != ControlState::Idle {
                    s
                } else if press.pressed.get() {
                    ControlState::Pressed
                } else if is_selected.get() {
                    ControlState::Selected
                } else {
                    ControlState::Idle
                }
            });
            let visual = use_state_visual(border_state);
            let tone = use_state_visual(item_state);
            let show_spinner =
                use_pending_indicator(move || item_state.get() == ControlState::Pending);
            let clickable = move || item_state.get() == ControlState::Idle;
            let click = move || {
                let id = id.clone();
                let action = action.clone();
                states.update(|m| {
                    m.insert(id.clone(), ControlState::Pending);
                });
                spawn_local(async move {
                    let target = !is_selected.get_untracked();
                    let result = action.call((id.clone(), target)).await;
                    states.update(|m| {
                        m.insert(id.clone(), result.into());
                    });
                    sleep_ms(motion::RESULT_HOLD_MS).await;
                    states.update(|m| {
                        m.insert(id, ControlState::Idle);
                    });
                });
            };
            view! {
                <button
                    type="button"
                    role="checkbox"
                    class="op-sb op-rippled op-group-tile"
                    aria-checked=move || is_selected.get().to_string()
                    aria-busy=move || (item_state.get() == ControlState::Pending).then_some("true")
                    data-checked=move || is_selected.get().then_some("")
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
                            click();
                        }
                    }
                >
                    {item.icon.map(|i| view! { <span class="op-group-icon"><Icon icon=i size=dim::ICON_SMALL/></span> })}
                    <span class="op-group-name">{item.name}</span>
                    <span class="op-group-box" data-checked=move || is_selected.get().then_some("")>
                        <span
                            class="op-tone op-group-mark"
                            data-state=move || tone.state.get()
                            data-result=move || tone.result.get()
                            data-fading=move || tone.fading.get().then_some("")
                        >
                            {move || {
                                if show_spinner.get() {
                                    view! { <PendingIndicator size=3.0/> }.into_any()
                                } else if is_selected.get() {
                                    view! { <Icon icon=icons::CHECK size=3.0/> }.into_any()
                                } else {
                                    ().into_any()
                                }
                            }}
                        </span>
                    </span>
                    <PendingRun/>
                </button>
            }
        })
        .collect_view();

    view! { <div class="op-group">{tiles}</div> }
}
