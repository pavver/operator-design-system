//! Stepper control: Stepper.

use crate::core::{
    AsyncAction, ControlState, JobToken, Metric, PendingRun, Press, Slot, root_class, root_style,
    sleep_ms, spawn_ripple, ui_icon, ui_text, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, icons, motion};

mod visuals;
use visuals::{pending as pending_indicator, symbol};

const ICON_SIZE: &str = "var(--op-stepper-icon-size, var(--op-dim-icon-small))";
const SPINNER_SIZE: &str = "var(--op-stepper-spinner-size, calc(var(--op-du) * 3))";

pub(crate) const CSS: &str = include_str!("stepper.css");

/// `- value +` stepper; stays interactive while pending to allow continuous input.
#[component]
pub fn Stepper(
    value: RwSignal<i32>,
    #[prop(optional)] min: Option<i32>,
    #[prop(optional)] max: Option<i32>,
    #[prop(default = 1)] step: i32,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Async commit; the value is updated only on success.
    #[prop(optional, into)]
    on_async_change: Option<AsyncAction<i32>>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] button_size: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] spinner_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] spinner_stroke: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
    #[prop(optional, into)] success_label: MaybeProp<String>,
    #[prop(optional)] success_icon: Option<IconData>,
    #[prop(optional, into)] error_label: MaybeProp<String>,
    #[prop(optional)] error_icon: Option<IconData>,
    #[prop(optional, into)] increase_label: MaybeProp<String>,
    #[prop(optional)] increase_icon: Option<IconData>,
    #[prop(optional, into)] decrease_label: MaybeProp<String>,
    #[prop(optional)] decrease_icon: Option<IconData>,
) -> impl IntoView {
    let loading_label = ui_text(loading_label, "stepper.loading", "Loading");
    let success_label = ui_text(success_label, "stepper.success", "Success");
    let success_icon = ui_icon(success_icon, "stepper.success", icons::CHECK);
    let error_label = ui_text(error_label, "stepper.error", "Error");
    let error_icon = ui_icon(error_icon, "stepper.error", icons::CLOSE);
    let increase_label = ui_text(increase_label, "stepper.increase", "Increase");
    let increase_icon = ui_icon(increase_icon, "stepper.increase", icons::ADD);
    let decrease_label = ui_text(decrease_label, "stepper.decrease", "Decrease");
    let decrease_icon = ui_icon(decrease_icon, "stepper.decrease", icons::REMOVE);

    let root_metrics = vec![
        Metric::du("--op-stepper-spinner-stroke", spinner_stroke),
        Metric::du("--op-stepper-width", width),
        Metric::du("--op-stepper-height", height),
        Metric::du("--op-stepper-button-size", button_size),
        Metric::du("--op-stepper-icon-size", icon_size),
        Metric::du("--op-stepper-spinner-size", spinner_size),
        Metric::tu("--op-stepper-font-size", font_size),
        Metric::tu("--op-stepper-line-height", line_height),
    ];

    let local = RwSignal::new(ControlState::Idle);
    let pending = RwSignal::new(false);
    let direction = RwSignal::new(None::<bool>);
    let requested = RwSignal::new(value.get_untracked());
    let job = JobToken::new();
    let on_async_change = StoredValue::new(on_async_change);
    Effect::new(move |_| requested.set(value.get()));

    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if let Some(s) = state.get() {
            s
        } else {
            local.get()
        }
    });
    let visual = use_state_visual(current);
    let can_dec = Signal::derive(move || enabled.get() && min.is_none_or(|m| requested.get() > m));
    let can_inc = Signal::derive(move || enabled.get() && max.is_none_or(|m| requested.get() < m));

    let change = move |increment: bool| {
        let target = requested.get_untracked() + if increment { step } else { -step };
        requested.set(target);
        let Some(action) = on_async_change.get_value() else {
            value.set(target);
            return;
        };
        let id = job.next();
        spawn_local(async move {
            pending.set(true);
            local.set(ControlState::Pending);
            direction.set(Some(increment));
            let result = action.call(target).await;
            if !job.is_current(id) {
                return;
            }
            pending.set(false);
            if result.is_success() {
                value.set(target);
                local.set(ControlState::Success);
            } else {
                requested.set(value.get_untracked());
                local.set(ControlState::Error);
            }
            sleep_ms(motion::RESULT_HOLD_MS).await;
            if job.is_current(id) {
                local.set(ControlState::Idle);
                direction.set(None);
            }
        });
    };

    let button = move |increment: bool| {
        let press = Press::new();
        let allowed = if increment { can_inc } else { can_dec };
        let feedback = Signal::derive(move || {
            if direction.get() == Some(increment) {
                current.get()
            } else if !allowed.get() {
                ControlState::Disabled
            } else {
                ControlState::Idle
            }
        });
        let tone = use_state_visual(feedback);
        let disabled_visual =
            move || !allowed.get() || (direction.get() == Some(!increment) && pending.get());
        let mine = move || direction.get() == Some(increment);
        let show_spinner = use_pending_indicator(move || mine() && pending.get());
        let (icon, label) = if increment {
            (increase_icon, increase_label)
        } else {
            (decrease_icon, decrease_label)
        };
        view! {
            <button
                type="button"
                class="op-rippled op-stepper-btn"
                aria-label=move || label.get()
                aria-disabled=move || (!allowed.get()).then_some("true")
                data-pressed=move || press.pressed.get().then_some("")
                data-muted=move || disabled_visual().then_some("")
                on:pointerdown=move |ev| {
                    if allowed.get_untracked() {
                        press.down();
                        spawn_ripple(&ev);
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
                on:click=move |_| {
                    if allowed.get_untracked() {
                        change(increment);
                    }
                }
            >
                <span
                    class="op-tone"
                    data-state=move || tone.state.get()
                    data-result=move || tone.result.get()
                    data-fading=move || tone.fading.get().then_some("")
                >
                    {move || {
                        if show_spinner.get() {
                            pending_indicator(SPINNER_SIZE, loading_label)
                        } else if mine() && current.get() == ControlState::Success {
                            symbol(success_icon, ICON_SIZE, Some(success_label))
                        } else if mine() && current.get() == ControlState::Error {
                            symbol(error_icon, ICON_SIZE, Some(error_label))
                        } else {
                            symbol(icon, ICON_SIZE, None)
                        }
                    }}
                </span>
            </button>
        }
    };

    view! {
        <div
            class=root_class("op-sb op-stepper", class)
            style=root_style(root_metrics, style)
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
        >
            {button(false)}
            <div class="op-stepper-value" data-disabled=move || (!enabled.get()).then_some("")>
                {move || if content.is_set() { content.run().into_any() } else { value.get().into_any() }}
            </div>
            {button(true)}
            <PendingRun/>
        </div>
    }
}
