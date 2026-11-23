use super::*;

/// One row of a [`Menu`]: a command, or an item that opens a submenu beside itself.
pub(super) fn menu_row(
    item: MenuItem,
    open: RwSignal<bool>,
    open_sub: RwSignal<Option<String>>,
    layer_id: u64,
    on_select: Callback<String>,
    customization: StoredValue<MenuCustomization>,
) -> AnyView {
    let custom_content = customization
        .with_value(|c| c.option)
        .map(|slot| slot.run(item.clone()));
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
            {custom_content.unwrap_or_else(|| view! {
            <span class="op-menu-icon">
                {item.icon.map(|i| view! { <Icon icon=i/> })}
            </span>
            <span class="op-menu-label">{item.label}</span>
            {item.shortcut.map(|s| view! { <span class="op-menu-shortcut">{s}</span> })}
            {has_sub.then(|| view! { <span class="op-menu-chevron">{move || view! { <Icon icon=customization.with_value(|c| c.submenu_icon.get())/> }}</span> })}
            }.into_any())}
        </button>
        {has_sub.then(|| submenu(sub_open, sub_anchor, children, pick_sub, label, layer_id, customization))}
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
    customization: StoredValue<MenuCustomization>,
) -> AnyView {
    let c = customization.get_value();
    view! { <Menu open=open anchor=anchor.into() items=items on_select=on_select label=label parent_layer=parent
        class=c.class style=c.style option=c.option empty=c.empty before_options=c.before_options after_options=c.after_options submenu_icon=c.submenu_icon_prop/> }
        .into_any()
}
