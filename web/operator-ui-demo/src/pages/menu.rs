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
    let custom_anchor = Signal::derive(move || {
        custom_host
            .get()
            .map(|el| {
                let r = el.get_bounding_client_rect();
                MenuAnchor::Below(r.left(), r.top(), r.width(), r.height())
            })
            .unwrap_or(MenuAnchor::Point(0.0, 0.0))
    });
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
        <Example title="Customization" note="Metric and scoped content overrides apply to submenus too.">
            <div node_ref=custom_host>
                <ActionButton text=l("Open commands", "Відкрити команди") on_click=crate::kit::instant(move || custom_open.update(|v| *v = !*v))/>
            </div>
            <Menu open=custom_open anchor=custom_anchor items=camera_menu() on_select=on_select
                item_height=10.0 min_width=60.0 gap=3.0 label=l("Commands", "Команди")
                header=move || view! { <strong>{l("Camera commands", "Команди камери")}</strong> }
                footer=move || view! { <small>{l("Use the arrow keys", "Використовуйте клавіші зі стрілками")}</small> }
                option=Callback::new(|item: MenuItem| view! {
                    <span class="op-t-body">{item.label}</span>
                    {(!item.children.is_empty()).then(|| view! { <Icon icon=icons::CHEVRON_RIGHT/> })}
                }.into_any())
                style="--op-menu-background: var(--op-surface-alt);"
            />
        </Example>
    }
    .into_any()
}
