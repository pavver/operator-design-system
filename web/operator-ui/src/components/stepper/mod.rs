//! Stepper control: Stepper.

use crate::components::primitives::{Icon, PendingIndicator};
use crate::core::{
    AsyncAction, ControlState, JobToken, PendingRun, Press, du, sleep_ms, spawn_ripple,
    use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{dim, icons, motion};

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
    /// Width in DU.
    #[prop(default = 35.0)]
    width: f32,
) -> impl IntoView {
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
            (icons::ADD, "Increase")
        } else {
            (icons::REMOVE, "Decrease")
        };
        view! {
            <button
                type="button"
                class="op-rippled op-stepper-btn"
                aria-label=label
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
                            view! { <PendingIndicator size=3.0/> }.into_any()
                        } else if mine() && current.get() == ControlState::Success {
                            view! { <Icon icon=icons::CHECK size=dim::ICON_SMALL label="Success"/> }.into_any()
                        } else if mine() && current.get() == ControlState::Error {
                            view! { <Icon icon=icons::CLOSE size=dim::ICON_SMALL label="Error"/> }.into_any()
                        } else {
                            view! { <Icon icon=icon size=dim::ICON_SMALL/> }.into_any()
                        }
                    }}
                </span>
            </button>
        }
    };

    view! {
        <div
            class="op-sb op-stepper"
            style:width=du(width)
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
        >
            {button(false)}
            <div class="op-stepper-value" data-disabled=move || (!enabled.get()).then_some("")>
                {move || value.get()}
            </div>
            {button(true)}
            <PendingRun/>
        </div>
    }
}
