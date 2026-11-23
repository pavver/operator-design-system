//! Floating action group, using the shared anchored overlay.
use crate::components::{
    action_button::ActionButton,
    icon_button::IconButton,
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

mod focus;
use focus::{Target, focus_action};

pub(crate) const CSS: &str = include_str!("action_stack.css");
#[derive(Clone)]
pub struct ActionStackItem {
    pub label: String,
    pub icon: Option<IconData>,
    pub action: AsyncAction,
}
impl ActionStackItem {
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
pub fn ActionStack(
    actions: Vec<ActionStackItem>,
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
    /// Size of the trigger icon in DU.
    #[prop(optional, into)]
    icon_size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    /// Shows only the icon on a square trigger; `label` becomes its accessible name and tooltip.
    #[prop(optional)]
    icon_only: bool,
    /// Element the stack opens from instead of the trigger, such as a whole composite control.
    #[prop(optional)]
    anchor_to: Option<NodeRef<leptos::html::Div>>,
    /// Set to true while one of the actions runs, for a composite control that blocks its other
    /// parts meanwhile.
    #[prop(optional)]
    busy: Option<RwSignal<bool>>,
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
    let anchor_el = anchor_to.unwrap_or(root);
    let anchor = Signal::derive(move || {
        anchor_el
            .get()
            .map(|el| PopoverAnchor::of(&el))
            .unwrap_or(PopoverAnchor::Point { x: 0.0, y: 0.0 })
    });
    // Index of the action that is running: it shows its own pending state, the trigger shows
    // it too, and the other actions are blocked until it settles.
    let running = RwSignal::new(None::<usize>);
    let trigger_state = Signal::derive(move || running.get().map(|_| ControlState::Pending));
    if let Some(busy) = busy {
        Effect::new(move |_| busy.set(running.get().is_some()));
    }
    // The trigger button carries the popup state itself; in the icon-only mode it sits inside
    // its tooltip wrapper, so the attributes are set on the button element directly.
    Effect::new(move |_| {
        let expanded = open.get().to_string();
        if let Some(button) = root
            .get()
            .and_then(|el| el.query_selector("button").ok().flatten())
        {
            let _ = button.set_attribute("aria-haspopup", "menu");
            let _ = button.set_attribute("aria-expanded", &expanded);
        }
    });
    let label = ui_text(label, "action-stack.label", "Actions");
    let icon = ui_icon(icon, "action-stack.open", icons::ADD);
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
    // Which action takes focus once the stack is placed: the last one after Arrow Up.
    let focus_on_open = StoredValue::new(Target::First);
    let open_with = move |target: Target| {
        focus_on_open.set_value(target);
        open.set(true);
    };
    let on_layer_key = move |e: leptos::ev::KeyboardEvent| {
        let target = match e.key().as_str() {
            "ArrowDown" => Target::Step(1),
            "ArrowUp" => Target::Step(-1),
            "Home" => Target::First,
            "End" => Target::Last,
            "Tab" => {
                // Like a menu, Tab leaves the stack; focus returns to the trigger.
                if running.get_untracked().is_none() {
                    e.prevent_default();
                    open.set(false);
                }
                return;
            }
            _ => return,
        };
        e.prevent_default();
        focus_action(layer, target);
    };
    let on_trigger_key = move |e: leptos::ev::KeyboardEvent| {
        if open.get_untracked() || !enabled.get_untracked() || running.get_untracked().is_some() {
            return;
        }
        let target = match e.key().as_str() {
            "ArrowDown" => Target::First,
            "ArrowUp" => Target::Last,
            _ => return,
        };
        e.prevent_default();
        open_with(target);
    };
    // The stack grows with its content but is never narrower than the trigger.
    let layer_style = Signal::derive(move || match anchor.get() {
        PopoverAnchor::Rect { width, .. } => {
            Some(format!("width:max-content;min-width:{width}px;"))
        }
        PopoverAnchor::Point { .. } => None,
    });
    view! {
        <div node_ref=root class=root_class("op-action-stack",class) style=root_style(vec![Metric::du("--op-action-stack-size",size),Metric::du("--op-action-stack-icon-size",icon_size),Metric::du("--op-action-stack-gap",gap)],style)>
            {if icon_only {
                view! { <IconButton icon=icon.get_untracked() label=label enabled=enabled state=trigger_state on_async_click=toggle content=content leading=leading trailing=trailing on:keydown=on_trigger_key/> }.into_any()
            } else {
                view! { <ActionButton text=label icon=icon.get_untracked() enabled=enabled state=trigger_state on_click=toggle content=content leading=leading trailing=trailing on:keydown=on_trigger_key/> }.into_any()
            }}
            <Popover open=open anchor=anchor placement=Signal::derive(move || Placement::Side(side.get(), align.get())) class="op-action-stack-layer" aria_label=label role="menu" node_ref=layer style=layer_style
                dismiss_on_outside=true
                dismiss_on_scroll=true
                hold_open=Signal::derive(move || running.get().is_some())
                on_placed=Callback::new(move |()| {
                    focus_action(layer, focus_on_open.get_value());
                    focus_on_open.set_value(Target::First);
                })>
                <div class="op-action-stack-body" on:keydown=on_layer_key>
                <ScrollView viewport_class="op-action-stack-actions op-scroll-contain" style=Signal::derive(root_style(vec![Metric::du("--op-action-stack-gap",gap)],None::<String>.into()))>
                    {actions.get_value().into_iter().enumerate().map(|(index, item)| {
                        let action = item.action;
                        let available = Signal::derive(move || {
                            enabled.get() && running.get().is_none_or(|r| r == index)
                        });
                        view! { <ActionButton text=item.label icon=item.icon.unwrap_or(icons::CHEVRON_RIGHT) enabled=available attr:role="menuitem" on_click=move |()| {
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
                </div>
            </Popover>
        </div>
    }
}
