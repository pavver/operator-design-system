//! Discrete rating with pointer preview and roving keyboard focus.
use crate::components::primitives::Icon;
use crate::core::{AsyncAction, ControlState, JobToken, PendingRun, sleep_ms, use_state_visual};
use crate::core::{Metric, Slot, root_class, root_style, spawn_ripple, ui_icon, ui_text};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::motion;
use operator_ui_tokens::{Icon as IconData, icons};

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
    #[prop(optional)] selected_icon: Option<IconData>,
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
    let icon = ui_icon(icon, "rating.empty", icons::CHECK_CIRCLE);
    let selected_icon = ui_icon(selected_icon, "rating.selected", icons::CHECK_CIRCLE);
    let preview = RwSignal::new(None::<u32>);
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
    let commit = Callback::new(move |(next, previous): (u32, u32)| {
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

    view! {
        <div class=root_class("op-rating op-sb",class) style=root_style(vec![Metric::du("--op-rating-size",size),Metric::du("--op-rating-gap",gap)],style)
            role="radiogroup" aria-label=move || label.get() aria-readonly=readonly.to_string() aria-disabled=move || (!enabled.get()).to_string() aria-busy=move || (current.get()==ControlState::Pending).to_string()
            data-state=move || visual.state.get() data-result=move || visual.result.get() data-fading=move || visual.fading.get().then_some("") on:pointerleave=move |_| { preview.set(None); pressed.set(false); }>
            {leading.run()}
            {content.run()}
            {(1..=max).map(|n| {
                let button=NodeRef::<leptos::html::Button>::new();
                view! {
                    <button type="button" node_ref=button class="op-rating-item op-rippled" role="radio" aria-checked=move || (value.get().min(max)==n).to_string()
                        aria-label=move || item_label.get().replace("{value}",&n.to_string()).replace("{max}",&max.to_string())
                        tabindex=move || if n==value.get().clamp(1,max) {0} else {-1} disabled=move || !enabled.get()
                        data-selected=move || (n<=preview.get().unwrap_or_else(|| value.get())).then_some("")
                        on:focus=move |_| focused.set(true) on:blur=move |_| focused.set(false)
                        on:pointerenter=move |_| { if available() { preview.set(Some(n)); } }
                        on:pointerdown=move |ev| { if available() { pressed.set(true); spawn_ripple(&ev); } }
                        on:pointerup=move |_| pressed.set(false) on:pointercancel=move |_| pressed.set(false)
                        on:click=move |_| { if available() { let old=value.get_untracked(); commit.run((if old==n && !no_reset {0} else {n},old)); preview.set(None); } }
                        on:keydown=move |ev| {
                            if !available() { return; }
                            let old=value.get_untracked();
                            let next=match ev.key().as_str() { "ArrowRight"|"ArrowUp" => (old+1).min(max), "ArrowLeft"|"ArrowDown"=>old.saturating_sub(1).max(u32::from(no_reset)), "Home"=>u32::from(no_reset), "End"=>max, _=>return };
                            ev.prevent_default(); commit.run((next,old));
                            if let Some(el)=button.get_untracked().and_then(|b| b.parent_element()) && let Ok(Some(target))=el.query_selector(&format!("button:nth-of-type({})",next.max(1))) { use wasm_bindgen::JsCast; if let Ok(el)=target.dyn_into::<web_sys::HtmlElement>() { let _=el.focus(); } }
                        }>
                        {move || view! { <Icon icon=if n<=preview.get().unwrap_or_else(|| value.get()) {selected_icon.get()} else {icon.get()} size=5.0/> }}
                    </button>
                }
            }).collect_view()}
            {trailing.run()}
            <PendingRun/>
        </div>
    }
}
