//! Discrete rating with pointer preview and roving keyboard focus.

use crate::components::primitives::Icon;
use crate::core::{
    AsyncAction, ControlState, JobToken, Metric, Slot, root_class, root_style, sleep_ms,
    spawn_ripple, ui_icon, ui_text, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::motion;
use operator_ui_tokens::{Icon as IconData, icons};
use wasm_bindgen::JsCast;

pub(crate) const CSS: &str = include_str!("rating.css");

#[component]
pub fn Rating(
    value: RwSignal<u32>,
    #[prop(default = 5)] max: u32,
    #[prop(optional)] readonly: bool,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional, into)] on_change: Option<Callback<u32>>,
    #[prop(optional, into)] on_commit: Option<AsyncAction<u32>>,
    #[prop(optional)] no_reset: bool,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] item_label: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional)] icon_selected: Option<IconData>,
    #[prop(optional, into)] size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] leading: Slot,
    #[prop(optional, into)] trailing: Slot,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let max = max.max(1);
    let label = ui_text(label, "rating.label", "Rating");
    let item_label = ui_text(item_label, "rating.item", "Rate {value} of {max}");
    let icon = ui_icon(icon, "rating.icon", icons::CHECK_CIRCLE);
    let icon_selected = ui_icon(icon_selected, "rating.icon-selected", icons::CHECK_CIRCLE);
    let preview = RwSignal::new(None::<u32>);
    let local = RwSignal::new(ControlState::Idle);
    let chosen_star = RwSignal::new(None::<u32>);
    let pressed = RwSignal::new(false);
    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else {
            state.get().unwrap_or_else(|| local.get())
        }
    });
    let available = move || {
        !readonly
            && !matches!(
                current.get(),
                ControlState::Pending | ControlState::Disabled
            )
    };
    let visual = use_state_visual(current);
    let job = JobToken::new();
    let action = StoredValue::new(on_commit);
    let active_star = move || {
        chosen_star
            .get()
            .unwrap_or_else(|| value.get().clamp(1, max))
    };
    let commit = Callback::new(move |(next, previous, chosen): (u32, u32, u32)| {
        if !available() || next == previous {
            return;
        }
        chosen_star.set(Some(chosen));
        value.set(next);
        if let Some(cb) = on_change {
            cb.run(next);
        }
        let Some(action) = action.get_value() else {
            return;
        };
        let id = job.next();
        local.set(ControlState::Pending);
        spawn_local(async move {
            let result = action.call(next).await;
            if !job.is_current(id) {
                return;
            }
            if !result.is_success() {
                value.set(previous);
            }
            local.set(result.into());
            sleep_ms(motion::RESULT_HOLD_MS).await;
            if job.is_current(id) {
                local.set(ControlState::Idle);
            }
        });
    });

    view! {
        <div
            class=root_class("op-rating", class)
            style=root_style(
                vec![
                    Metric::du("--op-rating-size", size),
                    Metric::du("--op-rating-gap", gap),
                ],
                style,
            )
            role="radiogroup"
            aria-label=move || label.get()
            aria-readonly=readonly.to_string()
            aria-disabled=move || (!enabled.get()).to_string()
            aria-busy=move || (current.get() == ControlState::Pending).to_string()
            data-disabled=move || (!enabled.get()).then_some("")
            on:pointerleave=move |_| {
                preview.set(None);
                pressed.set(false);
            }
        >
            {leading.run()}
            {content.run()}
            {(1..=max)
                .map(|n| {
                    let button = NodeRef::<leptos::html::Button>::new();
                    let is_chosen = move || active_star() == n;
                    let is_pending = move || is_chosen() && current.get() == ControlState::Pending;
                    view! {
                        <button
                            type="button"
                            node_ref=button
                            class="op-rating-item op-rippled"
                            role="radio"
                            aria-checked=move || (value.get().min(max) == n).to_string()
                            aria-label=move || {
                                item_label
                                    .get()
                                    .replace("{value}", &n.to_string())
                                    .replace("{max}", &max.to_string())
                            }
                            tabindex=move || if n == value.get().clamp(1, max) { 0 } else { -1 }
                            disabled=move || !enabled.get()
                            data-rating-value=n
                            data-selected=move || {
                                (n <= preview.get().unwrap_or_else(|| value.get())).then_some("")
                            }
                            data-state=move || {
                                if !enabled.get() {
                                    "disabled"
                                } else if is_chosen() {
                                    visual.state.get()
                                } else {
                                    "idle"
                                }
                            }
                            data-result=move || {
                                if is_chosen() {
                                    visual.result.get()
                                } else {
                                    "none"
                                }
                            }
                            data-fading=move || (is_chosen() && visual.fading.get()).then_some("")
                            data-pending=move || is_pending().then_some("")
                            on:pointerenter=move |_| {
                                if available() {
                                    preview.set(Some(n));
                                }
                            }
                            on:pointerdown=move |ev| {
                                if available() {
                                    pressed.set(true);
                                    spawn_ripple(&ev);
                                }
                            }
                            on:pointerup=move |_| pressed.set(false)
                            on:pointercancel=move |_| pressed.set(false)
                            on:click=move |_| {
                                if available() {
                                    let old = value.get_untracked();
                                    let next = if old == n && !no_reset { 0 } else { n };
                                    commit.run((next, old, n));
                                    preview.set(None);
                                }
                            }
                            on:keydown=move |ev| {
                                if !available() {
                                    return;
                                }
                                let old = value.get_untracked();
                                let next = match ev.key().as_str() {
                                    "ArrowRight" | "ArrowUp" => (old + 1).min(max),
                                    "ArrowLeft" | "ArrowDown" => {
                                        old.saturating_sub(1).max(u32::from(no_reset))
                                    }
                                    "Home" => u32::from(no_reset),
                                    "End" => max,
                                    _ => return,
                                };
                                ev.prevent_default();
                                let chosen = if next == 0 { old.max(1) } else { next };
                                commit.run((next, old, chosen));
                                if let Some(el) = button.get_untracked().and_then(|b| b.parent_element())
                                    && let Ok(Some(target)) = el.query_selector(&format!("[data-rating-value=\"{}\"]", next.max(1)))
                                    && let Ok(el) = target.dyn_into::<web_sys::HtmlElement>()
                                {
                                    let _ = el.focus();
                                }
                            }
                        >
                            <span class="op-rating-star">
                                <Icon
                                    icon=if n <= preview.get().unwrap_or_else(|| value.get()) {
                                        icon_selected.get()
                                    } else {
                                        icon.get()
                                    }
                                />
                                {move || {
                                    is_pending().then(|| {
                                        view! {
                                            <svg
                                                class="op-spinner op-rating-spinner"
                                                viewBox="0 0 24 24"
                                                aria-hidden="true"
                                            >
                                                <circle
                                                    cx="12"
                                                    cy="12"
                                                    r="9"
                                                    pathLength="100"
                                                ></circle>
                                            </svg>
                                        }
                                    })
                                }}
                            </span>
                        </button>
                    }
                })
                .collect_view()}
            {trailing.run()}
        </div>
    }
}
