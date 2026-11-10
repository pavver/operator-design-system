//! AsyncSwitch, AsyncCheckbox and CheckboxGroup.

use crate::basic::{Icon, ProgressIndicator};
use crate::core::{
    AsyncAction, ControlState, JobToken, PendingRun, Press, sleep_ms, spawn_ripple,
    use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{dim, icons, motion};
use std::collections::HashMap;

/// Shared async toggle flow: Pending -> result -> hold -> Idle, with rollback on error.
fn run_toggle(
    action: AsyncAction<bool>,
    checked: RwSignal<bool>,
    local: RwSignal<ControlState>,
    job: JobToken,
) {
    let id = job.next();
    spawn_local(async move {
        let previous = checked.get_untracked();
        let target = !previous;
        local.set(ControlState::Pending);
        let result = action.call(target).await;
        if !job.is_current(id) {
            return;
        }
        if result.is_success() {
            checked.set(target);
            local.set(ControlState::Success);
        } else {
            checked.set(previous);
            local.set(ControlState::Error);
        }
        sleep_ms(motion::RESULT_HOLD_MS).await;
        if job.is_current(id) {
            local.set(ControlState::Idle);
        }
    });
}

/// Local copy of an external boolean that resets whenever the external value changes.
fn mirrored(source: Signal<bool>) -> RwSignal<bool> {
    let local = RwSignal::new(source.get_untracked());
    Effect::new(move |_| local.set(source.get()));
    local
}

/// Capsule switch; the thumb parks in the center with a spinner while the request is pending.
#[component]
pub fn AsyncSwitch(
    #[prop(into)] checked: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] on_toggle: Option<AsyncAction<bool>>,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let local = RwSignal::new(ControlState::Idle);
    let value = mirrored(checked);
    let job = JobToken::new();
    let press = Press::new();
    let has_action = on_toggle.is_some();

    let current = Signal::derive(move || {
        let managed = state.get().unwrap_or_else(|| local.get());
        if !enabled.get() {
            ControlState::Disabled
        } else if managed != ControlState::Idle {
            managed
        } else if press.pressed.get() {
            ControlState::Pressed
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(current);
    let clickable = move || {
        has_action
            && !matches!(
                current.get(),
                ControlState::Disabled | ControlState::Pending
            )
    };
    let toggle = move || {
        if let Some(action) = on_toggle.clone() {
            run_toggle(action, value, local, job);
        }
    };

    view! {
        <button
            type="button"
            role="switch"
            class="op-sb op-rippled op-switch"
            aria-label=label
            aria-checked=move || value.get().to_string()
            aria-busy=move || (current.get() == ControlState::Pending).then_some("true")
            aria-disabled=move || (!clickable()).then_some("true")
            data-checked=move || value.get().then_some("")
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
                    toggle();
                }
            }
        >
            <span class="op-switch-thumb">
                {move || {
                    if current.get() == ControlState::Pending {
                        view! { <ProgressIndicator size=3.5 stroke=dim::SPINNER_STROKE color="var(--op-pending)"/> }
                            .into_any()
                    } else {
                        let icon = if value.get() { icons::CHECK } else { icons::CLOSE };
                        view! { <Icon icon=icon size=3.0/> }.into_any()
                    }
                }}
            </span>
            <PendingRun/>
        </button>
    }
}

/// Rectangular checkbox with optional label; commits the value only after a successful request.
#[component]
pub fn AsyncCheckbox(
    #[prop(into)] checked: Signal<bool>,
    #[prop(optional, into)] text: Option<Signal<String>>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] on_change: Option<AsyncAction<bool>>,
) -> impl IntoView {
    let local = RwSignal::new(ControlState::Idle);
    let value = mirrored(checked);
    let job = JobToken::new();
    let press = Press::new();
    let has_action = on_change.is_some();

    let current = Signal::derive(move || {
        let managed = state.get().unwrap_or_else(|| local.get());
        if !enabled.get() {
            ControlState::Disabled
        } else if managed != ControlState::Idle {
            managed
        } else if press.pressed.get() {
            ControlState::Pressed
        } else {
            ControlState::Idle
        }
    });
    let border_state = Signal::derive(move || {
        let s = current.get();
        if s == ControlState::Idle && value.get() {
            ControlState::Selected
        } else {
            s
        }
    });
    let visual = use_state_visual(border_state);
    let tone = use_state_visual(current);
    let clickable = move || {
        has_action
            && !matches!(
                current.get(),
                ControlState::Disabled | ControlState::Pending
            )
    };
    let toggle = move || {
        if let Some(action) = on_change.clone() {
            run_toggle(action, value, local, job);
        }
    };

    view! {
        <button
            type="button"
            role="checkbox"
            class="op-rippled op-checkbox"
            aria-checked=move || value.get().to_string()
            aria-busy=move || (current.get() == ControlState::Pending).then_some("true")
            aria-disabled=move || (!clickable()).then_some("true")
            data-disabled=move || (current.get() == ControlState::Disabled).then_some("")
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
                    toggle();
                }
            }
        >
            <span
                class="op-sb op-checkbox-box"
                data-checked=move || value.get().then_some("")
                data-state=move || visual.state.get()
                data-result=move || visual.result.get()
                data-fading=move || visual.fading.get().then_some("")
            >
                <span
                    class="op-tone op-checkbox-mark"
                    data-state=move || tone.state.get()
                    data-result=move || tone.result.get()
                    data-fading=move || tone.fading.get().then_some("")
                >
                    {move || {
                        if current.get() == ControlState::Pending {
                            view! { <ProgressIndicator size=3.0 stroke=dim::SPINNER_STROKE color="var(--op-pending)"/> }
                                .into_any()
                        } else if value.get() {
                            view! { <Icon icon=icons::CHECK size=3.5/> }.into_any()
                        } else {
                            ().into_any()
                        }
                    }}
                </span>
                <PendingRun/>
            </span>
            {text.map(|t| view! { <span class="op-checkbox-text">{move || t.get()}</span> })}
        </button>
    }
}

/// One option of a [`CheckboxGroup`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupItem {
    pub id: String,
    pub name: String,
}

impl GroupItem {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
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
                    <span class="op-group-name">{item.name}</span>
                    <span class="op-group-box" data-checked=move || is_selected.get().then_some("")>
                        <span
                            class="op-tone op-group-mark"
                            data-state=move || tone.state.get()
                            data-result=move || tone.result.get()
                            data-fading=move || tone.fading.get().then_some("")
                        >
                            {move || {
                                if item_state.get() == ControlState::Pending {
                                    view! {
                                        <ProgressIndicator size=3.0 stroke=dim::SPINNER_STROKE color="var(--op-pending)"/>
                                    }
                                        .into_any()
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
