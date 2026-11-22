//! Radio group: RadioGroup.

use crate::components::checkbox_group::GroupItem;
use crate::core::{
    AsyncAction, ControlState, JobToken, Metric, PendingRun, Press, ScopedSlot, Slot, root_class,
    root_style, sleep_ms, spawn_ripple, ui_text, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::motion;
use wasm_bindgen::JsCast;

use crate::core::glyph::{pending_spinner};

const SPINNER_SIZE: &str = "var(--op-radio-group-spinner-size, calc(var(--op-du) * 3))";

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
    #[prop(optional, into)] label: MaybeProp<String>,
    /// Lays the options out in a row.
    #[prop(optional)]
    horizontal: bool,
    #[prop(optional, into)] column_gap: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] size: MaybeProp<f32>,
    #[prop(optional, into)] dot_size: MaybeProp<f32>,
    #[prop(optional, into)] spinner_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] spinner_stroke: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] option: Option<ScopedSlot<GroupItem>>,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
) -> impl IntoView {
    let loading_label = ui_text(loading_label, "radio-group.loading", "Loading");
    let label = ui_text(label, "radio-group.label", "Options");

    let root_metrics = vec![
        Metric::du("--op-radio-group-spinner-stroke", spinner_stroke),
        Metric::du("--op-radio-group-column-gap", column_gap),
        Metric::du("--op-radio-group-gap", gap),
        Metric::du("--op-radio-group-height", height),
        Metric::du("--op-radio-group-padding-x", padding_x),
        Metric::du("--op-radio-group-size", size),
        Metric::du("--op-radio-group-dot-size", dot_size),
        Metric::du("--op-radio-group-spinner-size", spinner_size),
        Metric::tu("--op-radio-group-font-size", font_size),
        Metric::tu("--op-radio-group-line-height", line_height),
    ];

    let target = RwSignal::new(None::<String>);
    let result = RwSignal::new(ControlState::Idle);
    let busy = move || result.get() == ControlState::Pending;
    let job = JobToken::new();
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
        let request = job.next();
        target.set(Some(id.clone()));
        result.set(ControlState::Pending);
        spawn_local(async move {
            let r = action.call(id.clone()).await;
            if !job.is_current(request) {
                return;
            }
            let ok = r.is_success();
            if ok {
                selected.set(id);
            }
            result.set(r.into());
            sleep_ms(motion::RESULT_HOLD_MS).await;
            if !job.is_current(request) {
                return;
            }
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
                                pending_spinner("radio-group", SPINNER_SIZE, loading_label)
                            } else {
                                view! { <span class="op-radio-dot"></span> }.into_any()
                            }
                        }}
                        <PendingRun/>
                    </span>
                    <span class="op-radio-label">{option.map(|slot| slot.run(item.clone())).unwrap_or_else(|| item.name.clone().into_any())}</span>
                </button>
            }
        })
        .collect_view();

    view! {
        <div
            class=root_class("op-radio-group", class)
            style=root_style(root_metrics, style)
            role="radiogroup"
            aria-label=move || label.get()
            data-horizontal=horizontal.then_some("")
        >
            {before_options.run()}
            {rows}
            {after_options.run()}
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
