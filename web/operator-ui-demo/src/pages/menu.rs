//! Live examples for menu.

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
    }
    .into_any()
}
