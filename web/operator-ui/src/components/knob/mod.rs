//! Circular value control with pointer capture and keyboard stepping.
use crate::core::{AsyncAction, ControlState, JobToken, PendingRun, sleep_ms, use_state_visual};
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::motion;

pub(crate) const CSS: &str = include_str!("knob.css");
fn snap(v: f32, min: f32, max: f32, step: f32) -> f32 {
    (min + ((v.clamp(min, max) - min) / step).round() * step).clamp(min, max)
}
#[component]
pub fn Knob(
    value: RwSignal<f32>,
    #[prop(default = 0.0)] min: f32,
    #[prop(default = 100.0)] max: f32,
    #[prop(default = 1.0)] step: f32,
    #[prop(optional)] readonly: bool,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional, into)] on_change: Option<Callback<f32>>,
    #[prop(optional, into)] on_commit: Option<AsyncAction<f32>>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<f32>,
    #[prop(optional, into)] thickness: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let min = if min.is_finite() { min } else { 0.0 };
    let max = if max.is_finite() { max.max(min) } else { min };
    let step = if step.is_finite() && step > 0.0 {
        step
    } else {
        1.0
    };
    let root = NodeRef::<leptos::html::Div>::new();
    let start = StoredValue::new(value.get_untracked());
    let label = ui_text(label, "knob.label", "Value");
    let local = RwSignal::new(ControlState::Idle);
    let focused = RwSignal::new(false);
    let pressed = RwSignal::new(false);
    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else {
            state.get().unwrap_or_else(|| {
                let s = local.get();
                if s != ControlState::Idle {
                    s
                } else if pressed.get() {
                    ControlState::Pressed
                } else if focused.get() {
                    ControlState::Focused
                } else {
                    ControlState::Idle
                }
            })
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
    let commit = Callback::new(move |(next, previous): (f32, f32)| {
        if !available() || next == previous {
            return;
        }
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

    let fraction = move || {
        if max > min {
            ((value.get() - min) / (max - min)).clamp(0.0, 1.0)
        } else {
            0.0
        }
    };
    let set_pointer = move |x: f64, y: f64| {
        if let Some(el) = root.get_untracked() {
            let r = el.get_bounding_client_rect();
            let angle = (x - r.left() - r.width() / 2.0)
                .atan2(-(y - r.top() - r.height() / 2.0))
                .rem_euclid(std::f64::consts::TAU);
            value.set(snap(
                min + (angle / std::f64::consts::TAU) as f32 * (max - min),
                min,
                max,
                step,
            ));
        }
    };
    view! {
        <div node_ref=root class=root_class("op-knob op-sb",class) style=root_style(vec![Metric::du("--op-knob-size",size),Metric::du("--op-knob-thickness",thickness),Metric::tu("--op-knob-font-size",font_size)],style)
            role="slider" tabindex=move || if enabled.get() {0} else {-1} aria-label=move || label.get() aria-valuemin=min aria-valuemax=max aria-valuenow=move || value.get() aria-readonly=readonly.to_string() aria-disabled=move || (!enabled.get()).to_string() aria-busy=move || (current.get()==ControlState::Pending).to_string()
            data-state=move || visual.state.get() data-result=move || visual.result.get() data-fading=move || visual.fading.get().then_some("") data-dragging=move || pressed.get().then_some("")
            on:focus=move |_| focused.set(true) on:blur=move |_| focused.set(false)
            on:pointerdown=move |ev| { if available() { ev.prevent_default(); if let Some(el)=root.get_untracked() { let _=el.focus(); let _=el.set_pointer_capture(ev.pointer_id()); } start.set_value(value.get_untracked()); pressed.set(true); set_pointer(f64::from(ev.client_x()),f64::from(ev.client_y())); } }
            on:pointermove=move |ev| { if pressed.get_untracked() && available() { set_pointer(f64::from(ev.client_x()),f64::from(ev.client_y())); } }
            on:pointerup=move |_| { if pressed.get_untracked() { pressed.set(false); let next=value.get_untracked(); value.set(start.get_value()); commit.run((next,start.get_value())); } }
            on:pointercancel=move |_| { pressed.set(false); value.set(start.get_value()); }
            on:keydown=move |ev| { if !available() { return; } let old=value.get_untracked(); let next=match ev.key().as_str() { "ArrowRight"|"ArrowUp"=>old+step, "ArrowLeft"|"ArrowDown"=>old-step,"PageUp"=>old+step*10.0,"PageDown"=>old-step*10.0,"Home"=>min,"End"=>max,_=>return }; ev.prevent_default(); commit.run((snap(next,min,max,step),old)); }>
            <svg viewBox="0 0 100 100" aria-hidden="true"><circle class="op-knob-track" cx="50" cy="50" r="40"/><circle class="op-knob-fill" cx="50" cy="50" r="40" pathLength="1" stroke-dasharray=move || format!("{} 1",fraction()) transform="rotate(-90 50 50)"/></svg>
            <span class="op-knob-content">{if content.is_set() {content.run()} else {Some(view! { {move || format!("{:.0}",value.get())} }.into_any())}}</span>
            <PendingRun/>
        </div>
    }
}
