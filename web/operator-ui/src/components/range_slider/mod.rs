//! Two ordered thumbs on the shared Slider track and thumb visuals.
use crate::core::{AsyncAction, ControlState, JobToken, PendingRun, sleep_ms, use_state_visual};
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::motion;

pub(crate) const CSS: &str = include_str!("range_slider.css");
#[component]
pub fn RangeSlider(
    value: RwSignal<(f32, f32)>,
    #[prop(default = 0.0)] min: f32,
    #[prop(default = 100.0)] max: f32,
    #[prop(default = 1.0)] step: f32,
    #[prop(optional)] readonly: bool,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional, into)] on_change: Option<Callback<(f32, f32)>>,
    #[prop(optional, into)] on_commit: Option<AsyncAction<(f32, f32)>>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] lower_label: MaybeProp<String>,
    #[prop(optional, into)] upper_label: MaybeProp<String>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] rail_inset: MaybeProp<f32>,
    #[prop(optional, into)] track_height: MaybeProp<f32>,
    #[prop(optional, into)] thumb_size: MaybeProp<f32>,
    #[prop(optional, into)] dot_size: MaybeProp<f32>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] thumb: Slot,
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
    let label = ui_text(label, "range-slider.label", "Range");
    let lower_label = ui_text(lower_label, "range-slider.lower", "Lower value");
    let upper_label = ui_text(upper_label, "range-slider.upper", "Upper value");
    let rail = NodeRef::<leptos::html::Div>::new();
    let lower = NodeRef::<leptos::html::Div>::new();
    let upper = NodeRef::<leptos::html::Div>::new();
    let active = RwSignal::new(0usize);
    let start = StoredValue::new(value.get_untracked());
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
    let commit = Callback::new(move |(next, previous): ((f32, f32), (f32, f32))| {
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

    let normalized = move || {
        let (a, b) = value.get();
        (a.clamp(min, max), b.clamp(a.clamp(min, max), max))
    };
    let fraction = move |n: usize| {
        let (a, b) = normalized();
        if max > min {
            ((if n == 0 { a } else { b }) - min) / (max - min)
        } else {
            0.0
        }
    };
    let next_value = move |n: usize, v: f32| {
        let (a, b) = normalized();
        let v = (min + ((v - min) / step).round() * step).clamp(min, max);
        if n == 0 { (v.min(b), b) } else { (a, v.max(a)) }
    };
    let from_x = move |x: f64| {
        rail.get_untracked().map(|el| {
            let r = el.get_bounding_client_rect();
            min + ((x - r.left()) / r.width().max(1.0)).clamp(0.0, 1.0) as f32 * (max - min)
        })
    };
    let thumb = StoredValue::new(thumb);
    view! {
            <div class=root_class("op-slider op-range-slider",class) style=root_style(vec![Metric::du("--op-slider-height",height),Metric::du("--op-slider-rail-inset",rail_inset),Metric::du("--op-slider-track-height",track_height),Metric::du("--op-slider-thumb-size",thumb_size),Metric::du("--op-slider-dot-size",dot_size),Metric::du("--op-slider-width",width)],style)
                role="group" aria-label=move || label.get() aria-busy=move || (current.get()==ControlState::Pending).to_string() data-disabled=move || (!enabled.get()).then_some("") data-dragging=move || pressed.get().then_some("")
                on:pointerdown=move |ev| { if !available() {return;}
    if let Some(v)=from_x(f64::from(ev.client_x())) { ev.prevent_default(); let (a,b)=normalized(); let n=if (v-a).abs()<=(v-b).abs() {0} else {1}; active.set(n); start.set_value(value.get_untracked()); pressed.set(true); let target=if n==0 {lower} else {upper}; if let Some(el)=target.get_untracked() {let _=el.focus();} use wasm_bindgen::JsCast; if let Some(el)=ev.current_target().and_then(|t|t.dyn_into::<web_sys::Element>().ok()) {let _=el.set_pointer_capture(ev.pointer_id());} value.set(next_value(n,v)); } }
                on:pointermove=move |ev| {if pressed.get_untracked() && available() && let Some(v)=from_x(f64::from(ev.client_x())) {value.set(next_value(active.get_untracked(),v));}}
                on:pointerup=move |_| {if pressed.get_untracked() {pressed.set(false);let next=value.get_untracked();value.set(start.get_value());commit.run((next,start.get_value()));}}
                on:pointercancel=move |_| {pressed.set(false);value.set(start.get_value());}>
                <div class="op-slider-rail" node_ref=rail>
                    <div class="op-slider-track op-sb" data-state=move || visual.state.get() data-result=move || visual.result.get() data-fading=move || visual.fading.get().then_some("")>
                        <div class="op-slider-fill" style=move || format!("left:{}%;width:{}%", fraction(0)*100.0, (fraction(1)-fraction(0))*100.0)></div><PendingRun/>
                    </div>
                    {(0..2).map(move |n| view! {
                        <div class="op-slider-thumb-slot" style=move || format!("left:{}%", fraction(n)*100.0)>
                            <div class="op-slider-thumb op-sb" node_ref=if n==0 {lower} else {upper} role="slider" tabindex=move || if enabled.get() {0} else {-1}
                                aria-label=move || if n==0 {lower_label.get()} else {upper_label.get()} aria-valuemin=move || if n==0 {min} else {normalized().0} aria-valuemax=move || if n==0 {normalized().1} else {max} aria-valuenow=move || if n==0 {normalized().0} else {normalized().1} aria-readonly=readonly.to_string() aria-disabled=move || (!enabled.get()).to_string()
                                data-state=move || visual.state.get() data-result=move || visual.result.get() data-fading=move || visual.fading.get().then_some("")
                                on:focus=move |_| {focused.set(true);active.set(n);} on:blur=move |_| focused.set(false)
                                on:keydown=move |ev| {if !available() {return;} let old=value.get_untracked(); let v=if n==0 {old.0} else {old.1}; let v=match ev.key().as_str() {"ArrowRight"|"ArrowUp"=>v+step,"ArrowLeft"|"ArrowDown"=>v-step,"PageUp"=>v+10.0*step,"PageDown"=>v-10.0*step,"Home"=>min,"End"=>max,_=>return};ev.prevent_default();commit.run((next_value(n,v),old));}>
                                {if thumb.get_value().is_set() {thumb.get_value().run()} else {Some(view! {<span class="op-slider-dot"></span>}.into_any())}}
                            </div>
                        </div>
                    }).collect_view()}
                </div>
            </div>
        }
}
