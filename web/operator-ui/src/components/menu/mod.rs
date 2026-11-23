use crate::components::icon_button::IconButton;
use crate::components::popover::{Placement, Popover, PopoverAnchor, new_layer_id};
use crate::components::primitives::Icon;
use crate::components::scrollbar::ScrollView;
use crate::core::{
    ControlState, Metric, ScopedSlot, Slot, root_class, root_style, spawn_ripple, ui_icon, ui_text,
};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};
use wasm_bindgen::JsCast;

pub(crate) const CSS: &str = include_str!("menu.css");

/// One command in a [`Menu`].
#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub icon: Option<IconData>,
    /// Keyboard shortcut hint, for example `"Ctrl+C"`.
    pub shortcut: Option<String>,
    /// Destructive command shown in the error color.
    pub danger: bool,
    pub enabled: bool,
    /// Draws a divider above the item.
    pub separator: bool,
    /// Commands of a submenu; an item with children opens it instead of being selected.
    pub children: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            danger: false,
            enabled: true,
            separator: false,
            children: Vec::new(),
        }
    }

    /// Makes the item open a submenu with `items`.
    pub fn submenu(mut self, items: Vec<MenuItem>) -> Self {
        self.children = items;
        self
    }

    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn shortcut(mut self, keys: impl Into<String>) -> Self {
        self.shortcut = Some(keys.into());
        self
    }

    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn separated(mut self) -> Self {
        self.separator = true;
        self
    }
}

/// Where a [`Menu`] opens.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MenuAnchor {
    /// Under an element, aligned to its end edge.
    Below(f64, f64, f64, f64),
    /// At a pointer position.
    Point(f64, f64),
    /// Beside an element (a submenu next to its item).
    Side(f64, f64, f64, f64),
}

/// Floating list of commands.
///
/// It closes on selection, Escape, a press outside, scroll, or resize.
#[component]
pub fn Menu(
    open: RwSignal<bool>,
    anchor: Signal<MenuAnchor>,
    items: Vec<MenuItem>,
    on_select: Callback<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] min_width: MaybeProp<f32>,
    #[prop(optional, into)] max_width: MaybeProp<f32>,
    #[prop(optional, into)] viewport_margin: MaybeProp<f32>,
    #[prop(optional, into)] shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] shadow_blur: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] item_height: MaybeProp<f32>,
    #[prop(optional, into)] padding_end: MaybeProp<f32>,
    #[prop(optional, into)] padding_start: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] shortcut_font_size: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional_no_strip, into)] option: Option<ScopedSlot<MenuItem>>,
    #[prop(optional, into)] empty: Slot,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional_no_strip, into)] submenu_icon: Option<IconData>,
    /// Layer of the menu this one is a submenu of.
    #[prop(optional)]
    parent_layer: Option<u64>,
) -> impl IntoView {
    let label = ui_text(label, "menu.label", "Menu");
    let submenu_icon_prop = submenu_icon;
    let submenu_icon = ui_icon(submenu_icon, "menu.submenu", icons::CHEVRON_RIGHT);
    let layer_class = Signal::derive(root_class("op-menu", class));
    let layer_style = Signal::derive(root_style(
        vec![
            Metric::du("--op-menu-max-height", max_height),
            Metric::du("--op-menu-min-width", min_width),
            Metric::du("--op-menu-max-width", max_width),
            Metric::du("--op-menu-viewport-margin", viewport_margin),
            Metric::du("--op-menu-shadow-y", shadow_y),
            Metric::du("--op-menu-shadow-blur", shadow_blur),
            Metric::du("--op-menu-gap", gap),
            Metric::du("--op-menu-item-height", item_height),
            Metric::du("--op-menu-padding-end", padding_end),
            Metric::du("--op-menu-padding-start", padding_start),
            Metric::du("--op-menu-icon-size", icon_size),
            Metric::tu("--op-menu-shortcut-font-size", shortcut_font_size),
            Metric::du("--op-menu-padding-y", padding_y),
            Metric::tu("--op-menu-font-size", font_size),
            Metric::tu("--op-menu-line-height", line_height),
        ],
        style,
    ));
    let customization = StoredValue::new(MenuCustomization {
        class,
        style: layer_style.into(),
        option,
        empty,
        before_options,
        after_options,
        header,
        footer,
        submenu_icon,
        submenu_icon_prop,
    });
    let menu = NodeRef::<leptos::html::Div>::new();
    let layer_id = new_layer_id();
    // Id of the item whose submenu is open.
    let open_sub = RwSignal::new(None::<String>);
    Effect::new(move |_| {
        if !open.get() {
            open_sub.set(None);
        }
    });
    let items = StoredValue::new(items);
    let placement = Signal::derive(move || match anchor.get() {
        MenuAnchor::Point(..) => Placement::AtPoint,
        MenuAnchor::Below(..) => Placement::BelowEnd,
        MenuAnchor::Side(..) => Placement::SideStart,
    });
    let popover_anchor = Signal::derive(move || match anchor.get() {
        MenuAnchor::Point(x, y) => PopoverAnchor::Point { x, y },
        MenuAnchor::Below(left, top, width, height)
        | MenuAnchor::Side(left, top, width, height) => PopoverAnchor::Rect {
            left,
            top,
            width,
            height,
        },
    });
    // The first enabled item takes focus once the menu is placed.
    let focus_first = Callback::new(move |()| {
        if let Some(first) = menu
            .try_get_untracked()
            .flatten()
            .and_then(|el| {
                el.query_selector("[role=menuitem]:not([aria-disabled])")
                    .ok()
                    .flatten()
            })
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = first.focus();
        }
    });
    let close = move || open.set(false);

    let focus_step = move |delta: i32| {
        let Some(el) = menu.try_get_untracked().flatten() else {
            return;
        };
        let Ok(list) = el.query_selector_all("[role=menuitem]:not([aria-disabled])") else {
            return;
        };
        let n = list.length();
        if n == 0 {
            return;
        }
        let active = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element());
        let mut current = None;
        for i in 0..n {
            if let (Some(node), Some(a)) = (list.item(i), active.as_ref())
                && node.is_same_node(Some(a.as_ref()))
            {
                current = Some(i);
            }
        }
        let n = i64::from(n);
        let next = match current {
            Some(i) => (i64::from(i) + i64::from(delta)).rem_euclid(n),
            None if delta > 0 => 0,
            None => n - 1,
        };
        if let Some(item) = u32::try_from(next)
            .ok()
            .and_then(|i| list.item(i))
            .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = item.focus();
        }
    };

    view! {
                <Popover dismiss_on_scroll=true
                    open=open
                    anchor=popover_anchor
                    placement=placement
                    dismiss_on_resize=true
                    class=layer_class
                    style=layer_style
                    role="menu"
                    aria_label=label
                    node_ref=menu
                    on_placed=focus_first
                    layer_id=Some(layer_id)
                    parent_layer=parent_layer
                >
                    <div
                        class="op-menu-body"
                        on:keydown=move |e| match e.key().as_str() {
                            "Escape" | "Tab" => {
                                e.prevent_default();
                                close();
                            }
                            "ArrowDown" => {
                                e.prevent_default();
                                focus_step(1);
                            }
                            "ArrowUp" => {
                                e.prevent_default();
                                focus_step(-1);
                            }
                            "ArrowLeft" if parent_layer.is_some() => {
                                e.prevent_default();
                                close();
                            }
                            _ => {}
                        }
                        on:contextmenu=|e| e.prevent_default()
                    >
                        {customization.with_value(|c| c.header.run()).map(|h| view! { <div class="op-menu-header">{h}</div> })}
                        {customization.with_value(|c| c.before_options.run())}
                        <ScrollView viewport_class="op-menu-list op-scroll-contain">
                        {items.with_value(Vec::is_empty).then(|| customization.with_value(|c| c.empty.run()))}
                        {items
                            .get_value()
                            .into_iter()
                            .map(|item| menu_row(item, open, open_sub, layer_id, on_select, customization))
                            .collect_view()}
                        </ScrollView>
                        {customization.with_value(|c| c.after_options.run())}
                        {customization.with_value(|c| c.footer.run()).map(|f| view! { <div class="op-menu-footer">{f}</div> })}
                    </div>
                </Popover>
    }
}

#[derive(Clone)]
struct MenuCustomization {
    class: MaybeProp<String>,
    style: MaybeProp<String>,
    option: Option<ScopedSlot<MenuItem>>,
    empty: Slot,
    before_options: Slot,
    after_options: Slot,
    header: Slot,
    footer: Slot,
    submenu_icon: Signal<IconData>,
    submenu_icon_prop: Option<IconData>,
}

mod rows;
use rows::menu_row;
mod triggers;
pub use triggers::{ContextMenu, ContextMenuProps, MenuButton, MenuButtonProps};
