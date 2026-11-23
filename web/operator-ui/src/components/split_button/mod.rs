//! Primary action with an attached toggle that opens an ActionStack of secondary actions.
use crate::components::{
    action_button::ActionButton,
    action_stack::{ActionStack, ActionStackItem},
    popover::{LayerAlign, LayerSide},
};
use crate::core::{
    AsyncAction, ControlState, Metric, Slot, root_class, root_style, ui_icon, ui_text,
};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};
pub(crate) const CSS: &str = include_str!("split_button.css");

#[component]
pub fn SplitButton(
    /// Secondary actions in the stack opened by the toggle; each runs with its own feedback.
    actions: Vec<ActionStackItem>,
    #[prop(optional, into)] on_click: Option<AsyncAction>,
    #[prop(optional, into)] text: MaybeProp<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional, into)] toggle_label: MaybeProp<String>,
    #[prop(optional)] toggle_icon: Option<IconData>,
    /// Preferred side for the stack; it flips when it does not fit there.
    #[prop(into, default = LayerSide::Below.into())]
    side: Signal<LayerSide>,
    /// Preferred alignment of the stack across the whole control.
    #[prop(into, default = LayerAlign::Start.into())]
    align: Signal<LayerAlign>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] toggle_size: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] leading: Slot,
    #[prop(optional, into)] trailing: Slot,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let root = NodeRef::<leptos::html::Div>::new();
    // Pending on one part blocks the other: the primary action and a secondary one never run
    // together.
    let main_busy = RwSignal::new(false);
    let stack_busy = RwSignal::new(false);
    let text = ui_text(text, "split-button.text", "Action");
    let toggle_label = ui_text(toggle_label, "split-button.toggle", "More actions");
    let toggle_icon = ui_icon(toggle_icon, "split-button.toggle", icons::ARROW_DROP_DOWN);
    let blocked_by_state = move || {
        matches!(
            state.get(),
            Some(ControlState::Pending | ControlState::Disabled)
        )
    };
    let main_enabled = Signal::derive(move || enabled.get() && !stack_busy.get());
    let stack_enabled =
        Signal::derive(move || enabled.get() && !main_busy.get() && !blocked_by_state());
    let action = on_click.map(|action| {
        AsyncAction::new(move |()| {
            let action = action.clone();
            async move {
                main_busy.set(true);
                let result = action.call(()).await;
                if !main_busy.is_disposed() {
                    main_busy.set(false);
                }
                result
            }
        })
    });
    let action = action
        .unwrap_or_else(|| AsyncAction::new(|()| async { crate::core::ActionResult::Success }));
    view! {
        <div node_ref=root class=root_class("op-split-button", class) style=root_style(vec![Metric::du("--op-split-button-height", height), Metric::du("--op-split-button-toggle-size", toggle_size), Metric::du("--op-split-button-icon-size", icon_size)], style)>
            <ActionButton text=text state=state enabled=main_enabled leading=leading trailing=trailing content=content on_click=action class="op-split-button-main"/>
            <ActionStack actions=actions icon_only=true icon=toggle_icon.get_untracked() label=toggle_label
                enabled=stack_enabled side=side align=align anchor_to=root busy=stack_busy class="op-split-button-toggle"/>
        </div>
    }
}
