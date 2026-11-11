//! Radio group: RadioGroup.

use crate::components::checkbox_group::GroupItem;
use crate::components::primitives::PendingIndicator;
use crate::core::{
    AsyncAction, ControlState, PendingRun, Press, sleep_ms, spawn_ripple, use_pending_indicator,
    use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::motion;
use wasm_bindgen::JsCast;

pub(crate) const CSS: &str = include_str!("radio_group.css");

/// Single choice from a short list; with `on_change` the selection commits only after success.
#[component]
pub fn RadioGroup(
    items: Vec<GroupItem>,
    selected: RwSignal<String>,
    /// Async commit; without it the selection changes immediately.
    #[prop(optional, into)]
    on_change: Option<AsyncAction<String>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] label: Option<String>,
    /// Lays the options out in a row.
    #[prop(optional)]
    horizontal: bool,
) -> impl IntoView {
    let target = RwSignal::new(None::<String>);
    let result = RwSignal::new(ControlState::Idle);
    let busy = move || target.with(Option::is_some);
    let on_change = StoredValue::new(on_change);
    let ids: Vec<String> = items.iter().map(|i| i.id.clone()).collect();
    let ids = StoredValue::new(ids);

    let choose = move |id: String| {
        if busy() || !enabled.get_untracked() || selected.with_untracked(|s| *s == id) {
            return;
        }
        let Some(action) = on_change.get_value() else {
            selected.set(id);
            return;
        };
        target.set(Some(id.clone()));
        result.set(ControlState::Pending);
        spawn_local(async move {
            let r = action.call(id.clone()).await;
            let ok = r.is_success();
            if ok {
                selected.set(id);
            }
            result.set(r.into());
            sleep_ms(motion::RESULT_HOLD_MS).await;
            result.set(ControlState::Idle);
            target.set(None);
        });
    };

    let move_by = move |from: &str, delta: i32| {
        let list = ids.get_value();
        let Some(i) = list.iter().position(|x| x == from) else {
            return;
        };
        let n = i32::try_from(list.len()).unwrap_or(1);
        let next = (i32::try_from(i).unwrap_or(0) + delta).rem_euclid(n.max(1));
        if let Some(id) = usize::try_from(next).ok().and_then(|n| list.get(n)) {
            choose(id.clone());
            focus_radio(id);
        }
    };

    let rows = items
        .into_iter()
        .map(|item| {
            let id = item.id.clone();
            let press = Press::new();
            let is_selected = {
                let id = id.clone();
                Signal::derive(move || selected.with(|s| *s == id))
            };
            let is_target = {
                let id = id.clone();
                Signal::derive(move || target.with(|t| t.as_deref() == Some(id.as_str())))
            };
            let show_spinner = use_pending_indicator(move || {
                is_target.get() && result.get() == ControlState::Pending
            });
            let mark_state = Signal::derive(move || {
                if !enabled.get() {
                    ControlState::Disabled
                } else if is_target.get() {
                    result.get()
                } else if press.pressed.get() {
                    ControlState::Pressed
                } else if is_selected.get() {
                    ControlState::Selected
                } else {
                    ControlState::Idle
                }
            });
            let visual = use_state_visual(mark_state);
            let click_id = id.clone();
            let key_id = id.clone();
            let is_first = ids.with_value(|l| l.first() == Some(&id));
            let tab_stop = move || {
                is_selected.get()
                    || (is_first && !selected.with(|s| ids.with_value(|l| l.contains(s))))
            };
            view! {
                <button
                    type="button"
                    role="radio"
                    class="op-rippled op-radio"
                    data-radio-id=id.clone()
                    aria-checked=move || is_selected.get().to_string()
                    aria-disabled=move || (!enabled.get()).then_some("true")
                    tabindex=move || if tab_stop() { "0" } else { "-1" }
                    data-checked=move || is_selected.get().then_some("")
                    on:pointerdown=move |ev| {
                        if enabled.get_untracked() && !busy() {
                            press.down();
                            spawn_ripple(&ev);
                        }
                    }
                    on:pointerup=move |_| press.up()
                    on:pointerleave=move |_| press.up()
                    on:pointercancel=move |_| press.up()
                    on:click=move |_| choose(click_id.clone())
                    on:keydown=move |ev| match ev.key().as_str() {
                        "ArrowDown" | "ArrowRight" => {
                            ev.prevent_default();
                            move_by(&key_id, 1);
                        }
                        "ArrowUp" | "ArrowLeft" => {
                            ev.prevent_default();
                            move_by(&key_id, -1);
                        }
                        _ => {}
                    }
                >
                    <span
                        class="op-sb op-radio-box"
                        data-state=move || visual.state.get()
                        data-result=move || visual.result.get()
                        data-fading=move || visual.fading.get().then_some("")
                    >
                        {move || {
                            if show_spinner.get() {
                                view! { <PendingIndicator size=3.0/> }.into_any()
                            } else {
                                view! { <span class="op-radio-dot"></span> }.into_any()
                            }
                        }}
                        <PendingRun/>
                    </span>
                    <span class="op-radio-label">{item.name}</span>
                </button>
            }
        })
        .collect_view();

    view! {
        <div
            class="op-radio-group"
            role="radiogroup"
            aria-label=label
            data-horizontal=horizontal.then_some("")
        >
            {rows}
        </div>
    }
}

fn focus_radio(id: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let selector = format!("[data-radio-id=\"{}\"]", id.replace('"', "\\\""));
    if let Some(el) = doc
        .query_selector(&selector)
        .ok()
        .flatten()
        .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    {
        let _ = el.focus();
    }
}
