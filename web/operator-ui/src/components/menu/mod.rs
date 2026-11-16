use crate::components::icon_button::IconButton;
use crate::components::popover::{Placement, Popover, PopoverAnchor, new_layer_id};
use crate::components::primitives::Icon;
use crate::components::scrollbar::ScrollView;
use crate::core::{ControlState, spawn_ripple};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};
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
    #[prop(default = "Menu".into(), into)] label: String,
    /// Layer of the menu this one is a submenu of.
    #[prop(optional)]
    parent_layer: Option<u64>,
) -> impl IntoView {
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

    let label = StoredValue::new(label);
    view! {
                <Popover
                    open=open
                    anchor=popover_anchor
                    placement=placement
                    dismiss_on_resize=true
                    class="op-menu"
                    role="menu"
                    label=label.get_value()
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
                        <ScrollView viewport_class="op-menu-list op-scroll-contain">
                        {items
                            .get_value()
                            .into_iter()
                            .map(|item| menu_row(item, open, open_sub, layer_id, on_select))
                            .collect_view()}
                        </ScrollView>
                    </div>
                </Popover>
    }
}

/// One row of a [`Menu`]: a command, or an item that opens a submenu beside itself.
fn menu_row(
    item: MenuItem,
    open: RwSignal<bool>,
    open_sub: RwSignal<Option<String>>,
    layer_id: u64,
    on_select: Callback<String>,
) -> AnyView {
    let id = item.id.clone();
    let enabled = item.enabled;
    let has_sub = !item.children.is_empty();
    let row = NodeRef::<leptos::html::Button>::new();
    let sub_anchor = RwSignal::new(MenuAnchor::Point(0.0, 0.0));
    let sub_open = RwSignal::new(false);
    let show_sub = {
        let id = id.clone();
        move || {
            if let Some(r) = row.get_untracked() {
                let b = r.get_bounding_client_rect();
                sub_anchor.set(MenuAnchor::Side(b.left(), b.top(), b.width(), b.height()));
            }
            open_sub.set(Some(id.clone()));
        }
    };
    {
        let id = id.clone();
        Effect::new(move |_| sub_open.set(open_sub.with(|s| s.as_deref() == Some(id.as_str()))));
    }
    // Closing a submenu by itself (ArrowLeft) clears the open entry.
    {
        let id = id.clone();
        Effect::new(move |_| {
            if !sub_open.get() && open_sub.with_untracked(|s| s.as_deref() == Some(id.as_str())) {
                open_sub.set(None);
            }
        });
    }
    // Picking in the submenu closes this menu too.
    let pick_sub = Callback::new(move |picked: String| {
        open.set(false);
        on_select.run(picked);
    });
    let children = item.children;
    let label = item.label.clone();
    view! {
        {item.separator.then(|| view! { <div class="op-menu-sep" role="separator"></div> })}
        <button
            node_ref=row
            type="button"
            role="menuitem"
            class="op-rippled op-menu-item"
            data-danger=item.danger.then_some("")
            data-open=move || sub_open.get().then_some("")
            aria-disabled=(!enabled).then_some("true")
            aria-haspopup=has_sub.then_some("menu")
            aria-expanded=move || has_sub.then(|| sub_open.get().to_string())
            on:pointerdown=move |ev| {
                if enabled {
                    spawn_ripple(&ev);
                }
            }
            on:pointerenter={
                let show_sub = show_sub.clone();
                move |_| {
                    if has_sub && enabled {
                        show_sub();
                    } else {
                        open_sub.set(None);
                    }
                }
            }
            on:keydown={
                let show_sub = show_sub.clone();
                move |e| {
                    if has_sub && enabled && e.key() == "ArrowRight" {
                        e.prevent_default();
                        e.stop_propagation();
                        show_sub();
                    }
                }
            }
            on:click=move |_| {
                if !enabled {
                    return;
                }
                if has_sub {
                    show_sub();
                } else {
                    open.set(false);
                    on_select.run(id.clone());
                }
            }
        >
            <span class="op-menu-icon">
                {item.icon.map(|i| view! { <Icon icon=i size=dim::ICON_SMALL/> })}
            </span>
            <span class="op-menu-label">{item.label}</span>
            {item.shortcut.map(|s| view! { <span class="op-menu-shortcut">{s}</span> })}
            {has_sub.then(|| view! { <span class="op-menu-chevron"><Icon icon=icons::CHEVRON_RIGHT size=dim::ICON_SMALL/></span> })}
        </button>
        {has_sub.then(|| submenu(sub_open, sub_anchor, children, pick_sub, label, layer_id))}
    }
    .into_any()
}

/// A nested [`Menu`]; boxed so the component can contain itself.
fn submenu(
    open: RwSignal<bool>,
    anchor: RwSignal<MenuAnchor>,
    items: Vec<MenuItem>,
    on_select: Callback<String>,
    label: String,
    parent: u64,
) -> AnyView {
    view! { <Menu open=open anchor=anchor.into() items=items on_select=on_select label=label parent_layer=parent/> }
        .into_any()
}

/// Icon button that opens a [`Menu`] under itself; the overflow (⋮) menu.
#[component]
pub fn MenuButton(
    items: Vec<MenuItem>,
    on_select: Callback<String>,
    #[prop(default = icons::MORE_VERT)] icon: IconData,
    #[prop(default = "More actions".into(), into)] label: String,
    /// Button size in DU.
    #[prop(default = dim::ROW_COMPACT)]
    size: f32,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let host = NodeRef::<leptos::html::Span>::new();
    let anchor = RwSignal::new(MenuAnchor::Point(0.0, 0.0));
    // The window listener closes an open menu on the same press, so remember the state at press time.
    let open_at_press = StoredValue::new(false);
    let toggle = Callback::new(move |()| {
        if let Some(h) = host.try_get_untracked().flatten() {
            let r = h.get_bounding_client_rect();
            anchor.set(MenuAnchor::Below(r.left(), r.top(), r.width(), r.height()));
        }
        open.set(!open_at_press.get_value());
        open_at_press.set_value(false);
    });
    let state = Signal::derive(move || open.get().then_some(ControlState::Selected));
    view! {
        <span
            node_ref=host
            class="op-menu-anchor"
            aria-haspopup="menu"
            aria-expanded=move || open.get().to_string()
            on:pointerdown=move |_| open_at_press.set_value(open.get_untracked())
        >
            <IconButton icon=icon label=label.clone() size=size state=state on_click=toggle/>
        </span>
        <Menu open=open anchor=anchor.into() items=items on_select=on_select label=label/>
    }
}

/// Area that opens a [`Menu`] on right click or touch long press.
#[component]
pub fn ContextMenu(
    items: Vec<MenuItem>,
    on_select: Callback<String>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let anchor = RwSignal::new(MenuAnchor::Point(0.0, 0.0));
    view! {
        <div
            class="op-context-area"
            on:contextmenu=move |e| {
                e.prevent_default();
                anchor.set(MenuAnchor::Point(f64::from(e.client_x()), f64::from(e.client_y())));
                open.set(true);
            }
        >
            {children()}
        </div>
        <Menu open=open anchor=anchor.into() items=items on_select=on_select label="Context menu"/>
    }
}
