//! Inline value editing in an anchored popover with async save.
use crate::components::{
    action_button::ActionButton,
    popover::{Popover, PopoverAnchor},
    text_field::TextField,
};
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, Metric, Slot, root_class, root_style,
    sleep_ms, ui_text,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::motion;
use wasm_bindgen::JsCast;
pub(crate) const CSS: &str = include_str!("popup_edit.css");
#[component]
pub fn PopupEdit(
    value: RwSignal<String>,
    #[prop(optional, into)] on_save: Option<AsyncAction<String>>,
    #[prop(optional, into)] on_change: Option<Callback<String>>,
    #[prop(optional)] validate: Option<Callback<String, Option<String>>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] save_label: MaybeProp<String>,
    #[prop(optional, into)] cancel_label: MaybeProp<String>,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let draft = RwSignal::new(value.get_untracked());
    let error = RwSignal::new(None::<String>);
    let local = RwSignal::new(ControlState::Idle);
    let pending = move || local.get() == ControlState::Pending;
    let available = Signal::derive(move || enabled.get() && !pending());
    let anchor = RwSignal::new(PopoverAnchor::Point { x: 0.0, y: 0.0 });
    let trigger = NodeRef::<leptos::html::Button>::new();
    let layer = NodeRef::<leptos::html::Div>::new();
    let label = ui_text(label, "popup-edit.label", "Edit value");
    let save_label = ui_text(save_label, "popup-edit.save", "Save");
    let cancel_label = ui_text(cancel_label, "popup-edit.cancel", "Cancel");
    let job = JobToken::new();
    let save = AsyncAction::new(move |()| {
        let action = on_save.clone();
        async move {
            if !available.get_untracked() {
                return ActionResult::Error(None);
            }
            let next = draft.get_untracked();
            if let Some(message) = validate.and_then(|cb| cb.run(next.clone())) {
                error.set(Some(message.clone()));
                return ActionResult::Error(Some(message));
            }
            let id = job.next();
            local.set(ControlState::Pending);
            error.set(None);
            let result = if let Some(action) = action {
                action.call(next.clone()).await
            } else {
                ActionResult::Success
            };
            if !job.is_current(id) {
                return result;
            }
            local.set(result.clone().into());
            if result.is_success() {
                value.set(next.clone());
                if let Some(cb) = on_change {
                    cb.run(next);
                }
                open.set(false);
            } else {
                draft.set(value.get_untracked());
                if let ActionResult::Error(message) = &result {
                    error.set(message.clone());
                }
            }
            spawn_local(async move {
                sleep_ms(motion::RESULT_HOLD_MS).await;
                if job.is_current(id) {
                    local.set(ControlState::Idle);
                }
            });
            result
        }
    });
    let save = StoredValue::new(save);
    let cancel = move || {
        if !pending() {
            open.set(false);
            draft.set(value.get_untracked());
            error.set(None);
        }
    };
    #[cfg(target_arch = "wasm32")]
    {
        let down = window_event_listener(leptos::ev::pointerdown, move |ev| {
            if !open.get_untracked() || pending() {
                return;
            }
            let inside = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                .is_some_and(|node| {
                    layer
                        .get_untracked()
                        .is_some_and(|el| el.contains(Some(&node)))
                        || trigger
                            .get_untracked()
                            .is_some_and(|el| el.contains(Some(&node)))
                });
            if !inside {
                cancel();
            }
        });
        let key = window_event_listener(leptos::ev::keydown, move |ev| {
            if open.get_untracked() && ev.key() == "Escape" {
                ev.prevent_default();
                cancel();
            }
        });
        on_cleanup(move || {
            down.remove();
            key.remove();
        });
    }
    Effect::new(move |_| {
        if !enabled.get() && !pending() {
            open.set(false);
        }
    });
    view! {
        <div class=root_class("op-popup-edit",class) style=root_style(Vec::new(),style)>
            <button node_ref=trigger type="button" class="op-popup-edit-trigger" disabled=move || !available.get() aria-label=move || label.get() aria-haspopup="dialog" aria-expanded=move || open.get().to_string()
                on:click=move |_| {if available.get_untracked() && let Some(el)=trigger.get_untracked() {draft.set(value.get_untracked());error.set(None);anchor.set(PopoverAnchor::of(&el));open.update(|v|*v= !*v);}}>
                {if content.is_set() {content.run()} else {Some(view! {{move || value.get()}}.into_any())}}
            </button>
            <Popover open=open anchor=anchor aria_label=label role="dialog" node_ref=layer dismiss_on_outside=false dismiss_on_escape=false
                on_placed=Callback::new(move |()| {if let Some(el)=layer.get_untracked() && let Ok(Some(el))=el.query_selector("input") && let Ok(el)=el.dyn_into::<web_sys::HtmlElement>() {let _=el.focus();}})>
                <div class="op-popup-edit-panel" style=root_style(vec![Metric::du("--op-popup-edit-width",width),Metric::du("--op-popup-edit-padding",padding),Metric::du("--op-popup-edit-gap",gap)],style)>
                    {header.run()}
                    <form on:submit=move |ev| {ev.prevent_default();if available.get_untracked() {spawn_local(async move {save.get_value().call(()).await;});}}>
                        <TextField value=draft label=label error=Signal::derive(move || error.get()) enabled=available state=Signal::derive(move || Some(local.get()))/>
                        <div class="op-popup-edit-actions">
                            <ActionButton text=save_label enabled=available on_click=save.get_value()/>
                            <ActionButton text=cancel_label enabled=available on_click=move |()| async move {cancel();ActionResult::Success}/>
                        </div>
                    </form>
                    {footer.run()}
                </div>
            </Popover>
        </div>
    }
}
