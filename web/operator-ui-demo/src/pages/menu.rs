//! Live examples for menu.

use crate::i18n::l;
use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

fn camera_menu() -> Vec<MenuItem> {
    vec![
        MenuItem::new("live", "Open live view")
            .icon(icons::VIDEOCAM)
            .shortcut("Enter"),
        MenuItem::new("rename", "Rename")
            .icon(icons::EDIT)
            .shortcut("F2"),
        MenuItem::new("export", "Export clip").icon(icons::DOWNLOAD),
        MenuItem::new("move", "Move to zone").submenu(vec![
            MenuItem::new("zone-north", "North"),
            MenuItem::new("zone-south", "South"),
            MenuItem::new("zone-more", "Parking").submenu(vec![
                MenuItem::new("zone-p1", "Level 1"),
                MenuItem::new("zone-p2", "Level 2"),
            ]),
        ]),
        MenuItem::new("reboot", "Reboot")
            .icon(icons::REFRESH)
            .disabled(),
        MenuItem::new("delete", "Delete camera")
            .icon(icons::DELETE)
            .danger()
            .separated(),
    ]
}

pub fn menu() -> AnyView {
    let b = backend();
    let picked = RwSignal::new(String::from("none"));
    let on_select = Callback::new(move |id: String| {
        b.log(format!("[UI] menu: {id}"));
        picked.set(id);
    });
    let custom_open = RwSignal::new(false);
    let custom_host = NodeRef::<leptos::html::Div>::new();
    // Measured on every press, so the menu opens under the button wherever the page is scrolled.
    let custom_anchor = RwSignal::new(MenuAnchor::Point(0.0, 0.0));
    let toggle_custom = move || {
        if let Some(el) = custom_host.get_untracked() {
            let r = el.get_bounding_client_rect();
            custom_anchor.set(MenuAnchor::Below(r.left(), r.top(), r.width(), r.height()));
        }
        custom_open.update(|v| *v = !*v);
    };
    view! {
        <Example title="Overflow menu" note="Opens under the button, aligned to its end edge. Arrow keys move, Escape closes.">
            <MenuButton items=camera_menu() on_select=on_select/>
            <span class="op-t-caption demo-muted">{move || format!("Last command: {}", picked.get())}</span>
        </Example>
        <Example title="Context menu" note="Right-click inside the area." block=true>
            <ContextMenu items=camera_menu() on_select=on_select>
                <div class="demo-context">"Right-click here"</div>
            </ContextMenu>
        </Example>
        <Example title="Customization" note="Metric and scoped content overrides apply to submenus too; the header and footer frame only the root menu.">
            <div node_ref=custom_host>
                <ActionButton text=l("Open commands", "Відкрити команди") on_click=crate::kit::instant(toggle_custom)/>
            </div>
            <Menu open=custom_open anchor=custom_anchor.into() items=camera_menu() on_select=on_select
                item_height=10.0 min_width=60.0 gap=3.0 label=l("Commands", "Команди")
                header=move || view! { <strong>{l("Camera commands", "Команди камери")}</strong> }
                footer=move || view! { <small>{l("Use the arrow keys", "Використовуйте клавіші зі стрілками")}</small> }
                option=Callback::new(|item: MenuItem| {
                    // A custom row keeps the menu parts and shows the command id in place of the shortcut.
                    let id = (item.children.is_empty() && item.enabled).then(|| item.id.clone());
                    view! {
                        <span class="op-menu-icon">{item.icon.map(|i| view! { <Icon icon=i/> })}</span>
                        <span class="op-menu-label">{item.label}</span>
                        {id.map(|id| view! { <span class="op-menu-shortcut">{id}</span> })}
                        {(!item.children.is_empty()).then(|| view! { <span class="op-menu-chevron"><Icon icon=icons::CHEVRON_RIGHT/></span> })}
                    }.into_any()
                })
            />
        </Example>
    }
    .into_any()
}
