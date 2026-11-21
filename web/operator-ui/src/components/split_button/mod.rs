//! Primary action with an attached menu toggle.
use crate::components::{
    action_button::ActionButton,
    menu::{Menu, MenuAnchor, MenuItem},
    primitives::Icon,
};
use crate::core::{
    AsyncAction, ControlState, Metric, Slot, root_class, root_style, spawn_ripple, ui_icon, ui_text,
};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};
pub(crate) const CSS: &str = include_str!("split_button.css");

#[component]
pub fn SplitButton(
    items: Vec<MenuItem>,
    on_select: Callback<String>,
    #[prop(optional, into)] on_click: Option<AsyncAction>,
    #[prop(optional, into)] text: MaybeProp<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional, into)] toggle_label: MaybeProp<String>,
    #[prop(optional)] toggle_icon: Option<IconData>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] toggle_size: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] leading: Slot,
    #[prop(optional, into)] trailing: Slot,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let pending = RwSignal::new(false);
    let anchor = RwSignal::new(MenuAnchor::Point(0.0, 0.0));
    let trigger = NodeRef::<leptos::html::Button>::new();
    let text = ui_text(text, "split-button.text", "Action");
    let label = ui_text(toggle_label, "split-button.toggle", "More actions");
    let icon = ui_icon(toggle_icon, "split-button.toggle", icons::ARROW_DROP_DOWN);
    let available = Signal::derive(move || {
        enabled.get()
            && !pending.get()
            && !matches!(
                state.get(),
                Some(ControlState::Pending | ControlState::Disabled)
            )
    });
    Effect::new(move |_| {
        if !available.get() {
            open.set(false);
        }
    });
    let action = on_click.map(|action| {
        AsyncAction::new(move |()| {
            let action = action.clone();
            async move {
                pending.set(true);
                let result = action.call(()).await;
                if !pending.is_disposed() {
                    pending.set(false);
                }
                result
            }
        })
    });
    let action = StoredValue::new(action);
    view! {
        <div class=root_class("op-split-button", class) style=root_style(vec![Metric::du("--op-split-button-height", height), Metric::du("--op-split-button-toggle-size", toggle_size), Metric::du("--op-split-button-icon-size", icon_size)], style)>
            <ActionButton text=text state=state enabled=enabled leading=leading trailing=trailing content=content on_click=action.get_value().unwrap_or_else(|| AsyncAction::new(|()| async { crate::core::ActionResult::Success })) class="op-split-button-main"/>
            <button node_ref=trigger type="button" class="op-split-button-toggle op-rippled" disabled=move || !available.get() aria-label=move || label.get() aria-haspopup="menu" aria-expanded=move || open.get().to_string()
                on:pointerdown=move |ev| { if available.get_untracked() { spawn_ripple(&ev); } }
                on:click=move |_| { if available.get_untracked() && let Some(el) = trigger.get_untracked() { let r=el.get_bounding_client_rect(); anchor.set(MenuAnchor::Below(r.left(),r.top(),r.width(),r.height())); open.update(|v| *v = !*v); } }
                on:keydown=move |ev| { if ev.key()=="ArrowDown" && available.get_untracked() { ev.prevent_default(); if let Some(el)=trigger.get_untracked() { let r=el.get_bounding_client_rect(); anchor.set(MenuAnchor::Below(r.left(),r.top(),r.width(),r.height())); open.set(true); } } }>
                {move || view! { <Icon icon=icon.get() size=4.0/> }}
            </button>
            <Menu open=open anchor=anchor.into() items=items on_select=Callback::new(move |id| { if available.get_untracked() { on_select.run(id); } })/>
        </div>
    }
}
