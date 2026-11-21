use super::*;

/// Icon button that opens a [`Menu`] under itself; the overflow (⋮) menu.
#[component]
pub fn MenuButton(
    items: Vec<MenuItem>,
    on_select: Callback<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] label: MaybeProp<String>,
    /// Button size in DU.
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let label = ui_text(label, "menu.more-actions", "More actions");
    let icon = ui_icon(icon, "menu.more", icons::MORE_VERT);
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
            class=root_class("op-menu-anchor", class)
            style=root_style(vec![Metric::du("--op-menu-button-size", size)], style)
            aria-haspopup="menu"
            aria-expanded=move || open.get().to_string()
            on:pointerdown=move |_| open_at_press.set_value(open.get_untracked())
        >
            {move || view! { <IconButton icon=icon.get() label=label state=state on_click=toggle/> }}
        </span>
        <Menu open=open anchor=anchor.into() items=items on_select=on_select label=label/>
    }
}

/// Area that opens a [`Menu`] on right click or touch long press.
#[component]
pub fn ContextMenu(
    items: Vec<MenuItem>,
    on_select: Callback<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let label = ui_text(label, "menu.context", "Context menu");
    let open = RwSignal::new(false);
    let anchor = RwSignal::new(MenuAnchor::Point(0.0, 0.0));
    view! {
        <div
            class=root_class("op-context-area", class)
            style=root_style(vec![], style)
            on:contextmenu=move |e| {
                e.prevent_default();
                anchor.set(MenuAnchor::Point(f64::from(e.client_x()), f64::from(e.client_y())));
                open.set(true);
            }
        >
            {children()}
        </div>
        <Menu open=open anchor=anchor.into() items=items on_select=on_select label=label/>
    }
}
