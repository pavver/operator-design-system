//! Floating action group, using the shared anchored overlay.
use crate::components::{
    action_button::ActionButton,
    popover::{LayerAlign, LayerSide, Placement, Popover, PopoverAnchor},
    scrollbar::ScrollView,
};
use crate::core::{
    ActionResult, AsyncAction, ControlState, Metric, Slot, root_class, root_style, sleep_ms,
    ui_icon, ui_text,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, icons, motion};

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
    /// Preferred side for the action stack; it flips when it does not fit there.
    #[prop(into, default = LayerSide::Below.into())]
    side: Signal<LayerSide>,
    /// Preferred alignment across that side; it shifts when the stack would leave the screen.
    #[prop(into, default = LayerAlign::Center.into())]
    align: Signal<LayerAlign>,
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
    // Read live on every placement, so the stack follows the trigger while the page scrolls.
    let anchor = Signal::derive(move || {
        root.get()
            .map(|el| PopoverAnchor::of(&el))
            .unwrap_or(PopoverAnchor::Point { x: 0.0, y: 0.0 })
    });
    // Index of the action that is running: it shows its own pending state, the trigger shows
    // it too, and the other actions are blocked until it settles.
    let running = RwSignal::new(None::<usize>);
    let trigger_state = Signal::derive(move || running.get().map(|_| ControlState::Pending));
    let label = ui_text(label, "fab.label", "Actions");
    let icon = ui_icon(icon, "fab.open", icons::ADD);
    Effect::new(move |_| {
        if !enabled.get() {
            open.set(false);
        }
    });
    let toggle = AsyncAction::new(move |()| async move {
        open.update(|v| *v = !*v);
        ActionResult::Success
    });
    let actions = StoredValue::new(actions);
    // The stack grows with its content but is never narrower than the trigger.
    let layer_style = Signal::derive(move || match anchor.get() {
        PopoverAnchor::Rect { width, .. } => {
            Some(format!("width:max-content;min-width:{width}px;"))
        }
        PopoverAnchor::Point { .. } => None,
    });
    view! {
        <div node_ref=root class=root_class("op-fab",class) style=root_style(vec![Metric::du("--op-fab-size",size),Metric::du("--op-fab-gap",gap)],style)>
            <div aria-expanded=move || open.get().to_string() aria-haspopup="dialog">
                <ActionButton text=label icon=icon.get_untracked() enabled=enabled state=trigger_state on_click=toggle content=content leading=leading trailing=trailing/>
            </div>
            <Popover open=open anchor=anchor placement=Signal::derive(move || Placement::Side(side.get(), align.get())) class="op-fab-layer" aria_label=label role="dialog" node_ref=layer style=layer_style
                dismiss_on_outside=true
                dismiss_on_scroll=true
                hold_open=Signal::derive(move || running.get().is_some())
                on_placed=Callback::new(move |()| { if let Some(el)=layer.get_untracked() && let Ok(Some(first))=el.query_selector("button") { use wasm_bindgen::JsCast; if let Ok(button)=first.dyn_into::<web_sys::HtmlElement>() { let _=button.focus(); } } })>
                <ScrollView viewport_class="op-fab-actions op-scroll-contain" style=Signal::derive(root_style(vec![Metric::du("--op-fab-gap",gap)],None::<String>.into()))>
                    {actions.get_value().into_iter().enumerate().map(|(index, item)| {
                        let action = item.action;
                        let available = Signal::derive(move || {
                            enabled.get() && running.get().is_none_or(|r| r == index)
                        });
                        view! { <ActionButton text=item.label icon=item.icon.unwrap_or(icons::CHEVRON_RIGHT) enabled=available on_click=move |()| {
                            let action = action.clone();
                            async move {
                                running.set(Some(index));
                                let result = action.call(()).await;
                                if !running.is_disposed() {
                                    running.set(None);
                                    // The stack closes once the success flash has been seen;
                                    // an error keeps it open with the error flash.
                                    if result.is_success() {
                                        spawn_local(async move {
                                            sleep_ms(motion::LOCAL_FLASH_ICON_MS).await;
                                            if !open.is_disposed() {
                                                open.set(false);
                                            }
                                        });
                                    }
                                }
                                result
                            }
                        }/> }
                    }).collect_view()}
                </ScrollView>
            </Popover>
        </div>
    }
}
