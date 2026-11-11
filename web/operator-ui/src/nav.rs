//! Navigation and overlays: Menu, MenuButton, ContextMenu, Dialog, Tabs, Sheet, Pagination.

use crate::basic::Icon;
use crate::buttons::IconButton;
use crate::core::{ControlState, du, spawn_ripple, use_state_visual};
use crate::overlay::{FloatPos, event_inside, place_at, place_below_end};
use crate::scroll::ScrollView;
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};
use wasm_bindgen::JsCast;

// ---------------------------------------------------------------------------
// Menu
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Dialog
// ---------------------------------------------------------------------------

/// Modal window for a focused task: a form, details, or a choice.
///
/// Width follows the clamp model `clamp(min_width, 90% viewport, max_width)`.
#[component]
pub fn Dialog(
    open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    /// Buttons at the bottom, right-aligned.
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    /// Closes on Escape, scrim click, and the close button.
    #[prop(default = true.into(), into)]
    dismissible: Signal<bool>,
    #[prop(default = 75.0)] min_width: f32,
    #[prop(default = 140.0)] max_width: f32,
    children: ChildrenFn,
) -> impl IntoView {
    let panel = NodeRef::<leptos::html::Div>::new();
    let children = StoredValue::new(children);
    let dismiss = move || {
        if dismissible.get_untracked() {
            open.set(false);
        }
    };
    Effect::new(move |_| {
        if open.get() {
            request_animation_frame(move || {
                if let Some(p) = panel.try_get_untracked().flatten() {
                    let _ = p.focus();
                }
            });
        }
    });
    view! {
        <Show when=move || open.get()>
            <div
                class="op-dialog-scrim"
                on:click=move |_| dismiss()
                on:keydown=move |ev| {
                    if ev.key() == "Escape" {
                        dismiss();
                    }
                }
            >
                <div
                    node_ref=panel
                    class="op-dialog op-dialog-generic"
                    role="dialog"
                    aria-modal="true"
                    aria-label=move || title.get()
                    tabindex="-1"
                    style:width=format!(
                        "clamp(min({}, 100%), 90vw, {})",
                        du(min_width),
                        du(max_width),
                    )
                    on:click=|ev| ev.stop_propagation()
                >
                    <div class="op-dialog-head">
                        <div class="op-dialog-title">{move || title.get()}</div>
                        <Show when=move || dismissible.get()>
                            <IconButton icon=icons::CLOSE label="Close" on_click=Callback::new(move |()| open.set(false))/>
                        </Show>
                    </div>
                    <ScrollView class="op-dialog-body" viewport_class="op-dialog-content">{move || children.with_value(|c| c())}</ScrollView>
                    {actions.clone().map(|a| view! { <div class="op-dialog-actions">{a.run()}</div> })}
                </div>
            </div>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Tabs
// ---------------------------------------------------------------------------

/// One tab of [`Tabs`].
#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    pub label: String,
    pub icon: Option<IconData>,
    /// Counter shown after the label; hidden at zero.
    pub count: Option<u32>,
}

impl TabItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            count: None,
        }
    }

    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }
}

/// Visual style of [`Tabs`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
    /// Folder tabs: the active tab rises above an accent baseline and opens into the content.
    #[default]
    Folder,
    /// Underline tabs: a sliding accent line under the active label.
    Line,
}

/// Switches between views of the same level.
#[component]
pub fn Tabs(
    items: Vec<TabItem>,
    selected: RwSignal<usize>,
    #[prop(optional)] variant: TabsVariant,
    /// Stretches the tabs to fill the width equally.
    #[prop(optional)]
    fill: bool,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let list = NodeRef::<leptos::html::Div>::new();
    let indicator = RwSignal::new(None::<(f64, f64)>);
    let count = items.len();

    let measure = move || {
        let Some(el) = list.try_get_untracked().flatten() else {
            return;
        };
        let Ok(tabs) = el.query_selector_all("[role=tab]") else {
            return;
        };
        let i = u32::try_from(selected.get_untracked()).unwrap_or(0);
        if let Some(tab) = tabs
            .item(i)
            .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        {
            indicator.set(Some((
                f64::from(tab.offset_left()),
                f64::from(tab.offset_width()),
            )));
        }
    };
    Effect::new(move |_| {
        selected.track();
        request_animation_frame(measure);
    });
    let resize = window_event_listener(ev::resize, move |_| measure());
    on_cleanup(move || resize.remove());

    let focus_tab = move |i: usize| {
        selected.set(i);
        if let Some(t) = list.try_get_untracked().flatten().and_then(|el| {
            el.query_selector_all("[role=tab]")
                .ok()
                .and_then(|l| l.item(u32::try_from(i).unwrap_or(0)))
                .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        }) {
            let _ = t.focus();
        }
    };

    let tabs = items
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            let active = move || selected.get() == i;
            view! {
                <button
                    type="button"
                    role="tab"
                    class="op-rippled op-tab"
                    aria-selected=move || active().to_string()
                    tabindex=move || if active() { "0" } else { "-1" }
                    data-active=move || active().then_some("")
                    on:pointerdown=move |ev| spawn_ripple(&ev)
                    on:click=move |_| selected.set(i)
                    on:keydown=move |ev| {
                        let n = count.max(1);
                        match ev.key().as_str() {
                            "ArrowRight" => focus_tab((i + 1) % n),
                            "ArrowLeft" => focus_tab((i + n - 1) % n),
                            "Home" => focus_tab(0),
                            "End" => focus_tab(n - 1),
                            _ => {}
                        }
                    }
                >
                    {item.icon.map(|ic| view! { <Icon icon=ic size=dim::ICON_SMALL/> })}
                    <span class="op-tab-label">{item.label}</span>
                    {item.count.filter(|c| *c > 0).map(|c| view! { <span class="op-tab-count">{c.to_string()}</span> })}
                </button>
            }
        })
        .collect_view();

    let variant_name = match variant {
        TabsVariant::Folder => "folder",
        TabsVariant::Line => "line",
    };
    view! {
        <div class="op-tabs" data-variant=variant_name data-fill=fill.then_some("")>
            <div node_ref=list class="op-tabs-list" role="tablist" aria-label=label>
                <span
                    class="op-tabs-indicator"
                    aria-hidden="true"
                    style:visibility=move || if indicator.get().is_some() { "visible" } else { "hidden" }
                    style:transform=move || indicator.get().map(|(l, _)| format!("translateX({l}px)"))
                    style:width=move || indicator.get().map(|(_, w)| format!("{w}px"))
                ></span>
                {tabs}
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Sheet
// ---------------------------------------------------------------------------

/// Edge a [`Sheet`] slides in from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSide {
    #[default]
    Left,
    Right,
    Bottom,
}

/// Panel that slides in from a screen edge over a scrim: navigation drawers and bottom sheets.
#[component]
pub fn Sheet(
    open: RwSignal<bool>,
    #[prop(optional)] side: SheetSide,
    #[prop(optional, into)] title: Option<String>,
    /// Width for side sheets or maximum height for bottom sheets, in DU.
    #[prop(default = 75.0)]
    size: f32,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let panel = NodeRef::<leptos::html::Div>::new();
    let side_name = match side {
        SheetSide::Left => "left",
        SheetSide::Right => "right",
        SheetSide::Bottom => "bottom",
    };
    Effect::new(move |_| {
        if open.get() {
            request_animation_frame(move || {
                if let Some(p) = panel.try_get_untracked().flatten() {
                    let _ = p.focus();
                }
            });
        }
    });
    let title_view = title.clone();
    view! {
        <Show when=move || open.get()>
            <div
                class="op-sheet-scrim"
                on:click=move |_| open.set(false)
                on:keydown=move |ev| {
                    if ev.key() == "Escape" {
                        open.set(false);
                    }
                }
            >
                <div
                    node_ref=panel
                    class="op-sheet"
                    role="dialog"
                    aria-modal="true"
                    aria-label=title.clone()
                    tabindex="-1"
                    data-side=side_name
                    style:--sheet-size=du(size)
                    on:click=|ev| ev.stop_propagation()
                >
                    {(side == SheetSide::Bottom).then(|| view! { <span class="op-sheet-handle" aria-hidden="true"></span> })}
                    {title_view
                        .clone()
                        .map(|t| {
                            view! {
                                <div class="op-sheet-head">
                                    <span class="op-sheet-title">{t}</span>
                                    <IconButton icon=icons::CLOSE label="Close" on_click=Callback::new(move |()| open.set(false))/>
                                </div>
                            }
                        })}
                    <ScrollView class="op-sheet-body">{move || children.with_value(|c| c())}</ScrollView>
                </div>
            </div>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

/// Page numbers to show: `None` is an ellipsis.
pub fn page_window(current: usize, total: usize, siblings: usize) -> Vec<Option<usize>> {
    if total == 0 {
        return Vec::new();
    }
    let current = current.clamp(1, total);
    let start = current.saturating_sub(siblings).max(2);
    let end = (current + siblings).min(total.saturating_sub(1));
    let mut pages = vec![Some(1)];
    if start > 2 {
        pages.push(if start == 3 { Some(2) } else { None });
    }
    pages.extend((start..=end).map(Some));
    if end + 1 < total {
        pages.push(if end + 2 == total {
            Some(total - 1)
        } else {
            None
        });
    }
    if total > 1 {
        pages.push(Some(total));
    }
    pages
}

/// Page count from which [`Pagination`] shows the jump field when `jump` is not set.
pub const PAGINATION_JUMP_MIN_PAGES: usize = 20;

/// Parses a typed page number and clamps it to `1..=total`; `None` for empty input or no pages.
pub fn parse_page(text: &str, total: usize) -> Option<usize> {
    if total == 0 {
        return None;
    }
    let digits: String = text.chars().filter(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return None;
    }
    // Too many digits to fit means far past the end.
    Some(digits.parse::<usize>().map_or(total, |n| n.clamp(1, total)))
}

/// Navigation between numbered pages; `page` is 1-based.
#[component]
pub fn Pagination(
    page: RwSignal<usize>,
    #[prop(into)] total: Signal<usize>,
    /// Pages shown on each side of the current one.
    #[prop(default = 1)]
    siblings: usize,
    /// Shows `3 / 12` instead of page numbers.
    #[prop(optional)]
    compact: bool,
    /// Shows a field for typing a page number; by default shown from
    /// [`PAGINATION_JUMP_MIN_PAGES`] pages.
    #[prop(optional)]
    jump: Option<bool>,
) -> impl IntoView {
    let go = move |p: usize| {
        let t = total.get_untracked();
        if t > 0 {
            page.set(p.clamp(1, t));
        }
    };
    let typed = RwSignal::new(String::new());
    let focused = RwSignal::new(false);
    let commit = move || {
        if let Some(p) = parse_page(&typed.get_untracked(), total.get_untracked()) {
            page.set(p);
        }
        typed.set(String::new());
    };
    let show_jump = move || jump.unwrap_or_else(|| total.get() >= PAGINATION_JUMP_MIN_PAGES);
    let numbers = move || {
        page_window(page.get(), total.get(), siblings)
            .into_iter()
            .map(|p| match p {
                Some(n) => {
                    let current = move || page.get() == n;
                    let visual = use_state_visual(Signal::derive(move || {
                        if current() {
                            ControlState::Selected
                        } else {
                            ControlState::Idle
                        }
                    }));
                    view! {
                        <button
                            type="button"
                            class="op-sb op-rippled op-page"
                            aria-label=format!("Page {n}")
                            aria-current=move || current().then_some("page")
                            data-state=move || visual.state.get()
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| go(n)
                        >
                            {n.to_string()}
                        </button>
                    }
                    .into_any()
                }
                None => {
                    view! { <span class="op-page-gap" aria-hidden="true">"…"</span> }.into_any()
                }
            })
            .collect_view()
    };
    view! {
        <nav class="op-pagination" aria-label="Pagination">
            <IconButton
                icon=icons::CHEVRON_LEFT
                label="Previous page"
                enabled=Signal::derive(move || page.get() > 1)
                on_click=Callback::new(move |()| go(page.get_untracked().saturating_sub(1)))
            />
            {move || {
                if compact {
                    view! {
                        <span class="op-page-compact">{format!("{} / {}", page.get(), total.get())}</span>
                    }
                        .into_any()
                } else {
                    numbers().into_any()
                }
            }}
            <IconButton
                icon=icons::CHEVRON_RIGHT
                label="Next page"
                enabled=Signal::derive(move || page.get() < total.get())
                on_click=Callback::new(move |()| go(page.get_untracked() + 1))
            />
            <Show when=show_jump>
                <label class="op-page-jump">
                    <span class="op-page-jump-label">"Go to"</span>
                    <input
                        class="op-sb op-page-input"
                        type="text"
                        inputmode="numeric"
                        autocomplete="off"
                        aria-label="Go to page"
                        data-state=move || if focused.get() { "focused" } else { "idle" }
                        placeholder=move || page.get().to_string()
                        style:width=move || format!("calc({}ch + var(--op-dim-pad-content) * 2)", total.get().to_string().len().max(2))
                        prop:value=move || typed.get()
                        on:input=move |ev| {
                            let digits: String = event_target_value(&ev).chars().filter(char::is_ascii_digit).collect();
                            typed.set(digits);
                        }
                        on:keydown=move |ev| match ev.key().as_str() {
                            "Enter" => {
                                ev.prevent_default();
                                commit();
                            }
                            "Escape" => typed.set(String::new()),
                            _ => {}
                        }
                        on:focus=move |_| focused.set(true)
                        on:blur=move |_| {
                            focused.set(false);
                            typed.set(String::new());
                        }
                    />
                </label>
            </Show>
        </nav>
    }
}
