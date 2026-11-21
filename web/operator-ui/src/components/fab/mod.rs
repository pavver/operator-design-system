//! Floating action group, using the shared anchored overlay.
use crate::components::{
    action_button::ActionButton,
    popover::{Placement, Popover, PopoverAnchor},
};
use crate::core::{
    ActionResult, AsyncAction, Metric, Slot, root_class, root_style, ui_icon, ui_text,
};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

pub(crate) const CSS: &str = include_str!("fab.css");
#[derive(Clone)]
pub struct FabAction {
    pub label: String,
    pub icon: Option<IconData>,
    pub action: AsyncAction,
}
impl FabAction {
    pub fn new(label: impl Into<String>, action: impl Into<AsyncAction>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            action: action.into(),
        }
    }
    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }
}
#[component]
pub fn Fab(
    actions: Vec<FabAction>,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional)] below: bool,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] leading: Slot,
    #[prop(optional, into)] trailing: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let open = open.unwrap_or_else(|| RwSignal::new(false));
    let root = NodeRef::<leptos::html::Div>::new();
    let layer = NodeRef::<leptos::html::Div>::new();
    let anchor = RwSignal::new(PopoverAnchor::Point { x: 0.0, y: 0.0 });
    let pending = RwSignal::new(false);
    let available = Signal::derive(move || enabled.get() && !pending.get());
    let label = ui_text(label, "fab.label", "Actions");
    let icon = ui_icon(icon, "fab.open", icons::ADD);
    Effect::new(move |_| {
        if !enabled.get() {
            open.set(false);
        }
    });
    let toggle = AsyncAction::new(move |()| async move {
        if let Some(el) = root.get_untracked() {
            anchor.set(PopoverAnchor::of(&el));
            open.update(|v| *v = !*v);
        }
        ActionResult::Success
    });
    let actions = StoredValue::new(actions);
    view! {
        <div node_ref=root class=root_class("op-fab",class) style=root_style(vec![Metric::du("--op-fab-size",size),Metric::du("--op-fab-gap",gap)],style)>
            <div aria-expanded=move || open.get().to_string() aria-haspopup="dialog">
                <ActionButton text=label icon=icon.get_untracked() enabled=available on_click=toggle content=content leading=leading trailing=trailing/>
            </div>
            <Popover open=open anchor=anchor placement=if below { Placement::BelowStart } else { Placement::Above } aria_label=label role="dialog" node_ref=layer
                dismiss_on_outside=true
                on_placed=Callback::new(move |()| { if let Some(el)=layer.get_untracked() && let Ok(Some(first))=el.query_selector("button") { use wasm_bindgen::JsCast; if let Ok(button)=first.dyn_into::<web_sys::HtmlElement>() { let _=button.focus(); } } })>
                <div class="op-fab-actions" style=root_style(vec![Metric::du("--op-fab-gap",gap)],None::<String>.into())>
                    {actions.get_value().into_iter().map(|item| {
                        let action=item.action;
                        view! { <ActionButton text=item.label icon=item.icon.unwrap_or(icons::CHEVRON_RIGHT) enabled=available on_click=move |()| {
                            let action=action.clone();
                            async move { pending.set(true); let result=action.call(()).await; if !pending.is_disposed() { pending.set(false); if result.is_success() { open.set(false); } } result }
                        }/> }
                    }).collect_view()}
                </div>
            </Popover>
        </div>
    }
}
