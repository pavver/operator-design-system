use crate::components::icon_button::IconButton;
use crate::components::primitives::Icon;
use crate::components::scrollbar::ScrollView;
use crate::core::overlay::{FloatPos, event_inside, place_at, place_below_end};
use crate::core::{ControlState, spawn_ripple};
use leptos::ev;
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
        }
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
) -> impl IntoView {
    let menu = NodeRef::<leptos::html::Div>::new();
    let pos = RwSignal::new(None::<FloatPos>);
    let items = StoredValue::new(items);

    let place = move || {
        let Some(el) = menu.try_get_untracked().flatten() else {
            return;
        };
        let (w, h) = (f64::from(el.offset_width()), f64::from(el.offset_height()));
        let p = match anchor.get_untracked() {
            MenuAnchor::Point(x, y) => place_at(x, y, w, h),
            MenuAnchor::Below(left, top, width, height) => {
                let rect = web_sys::DomRect::new_with_x_and_y_and_width_and_height(
                    left, top, width, height,
                );
                match rect {
                    Ok(r) => place_below_end(&r, w, h),
                    Err(_) => place_at(left, top + height, w, h),
                }
            }
        };
        pos.set(Some(p));
        if let Some(first) = el
            .query_selector("[role=menuitem]:not([aria-disabled])")
            .ok()
            .flatten()
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = first.focus();
        }
    };
    Effect::new(move |_| {
        if open.get() {
            pos.set(None);
            request_animation_frame(place);
        }
    });

    let close = move || open.set(false);
    let down = window_event_listener(ev::pointerdown, move |e| {
        if open.get_untracked()
            && !menu
                .try_get_untracked()
                .flatten()
                .is_some_and(|m| event_inside(&e, &m))
        {
            close();
        }
    });
    let resize = window_event_listener(ev::resize, move |_| close());
    on_cleanup(move || {
        down.remove();
        resize.remove();
    });

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
        <Show when=move || open.get()>
            <div
                node_ref=menu
                class="op-menu"
                role="menu"
                aria-label=label.clone()
                style:visibility=move || if pos.get().is_some() { "visible" } else { "hidden" }
                style:left=move || pos.get().map(|p| format!("{}px", p.left))
                style:top=move || pos.get().map(|p| format!("{}px", p.top))
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
                    _ => {}
                }
                on:contextmenu=|e| e.prevent_default()
            >
                <ScrollView viewport_class="op-menu-list">
                {items
                    .get_value()
                    .into_iter()
                    .map(|item| {
                        let id = item.id.clone();
                        let enabled = item.enabled;
                        view! {
                            {item.separator.then(|| view! { <div class="op-menu-sep" role="separator"></div> })}
                            <button
                                type="button"
                                role="menuitem"
                                class="op-rippled op-menu-item"
                                data-danger=item.danger.then_some("")
                                aria-disabled=(!enabled).then_some("true")
                                on:pointerdown=move |ev| {
                                    if enabled {
                                        spawn_ripple(&ev);
                                    }
                                }
                                on:click=move |_| {
                                    if enabled {
                                        close();
                                        on_select.run(id.clone());
                                    }
                                }
                            >
                                <span class="op-menu-icon">
                                    {item.icon.map(|i| view! { <Icon icon=i size=dim::ICON_SMALL/> })}
                                </span>
                                <span class="op-menu-label">{item.label}</span>
                                {item.shortcut.map(|s| view! { <span class="op-menu-shortcut">{s}</span> })}
                            </button>
                        }
                    })
                    .collect_view()}
                </ScrollView>
            </div>
        </Show>
    }
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
