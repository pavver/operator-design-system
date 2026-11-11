//! Examples for navigation and overlay components.

use crate::kit::{Example, backend, instant};
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

pub fn dialog() -> AnyView {
    let b = backend();
    let open = RwSignal::new(false);
    let name = RwSignal::new("Gate North".to_string());
    let host = RwSignal::new("192.168.1.40".to_string());
    let save = AsyncAction::new(move |()| async move {
        b.call(format!("save camera {}", name.get_untracked()))
            .await
    });
    view! {
        <Example title="Form dialog" note="Save runs inside the dialog: success closes it, error keeps it open. Width follows clamp(75 DU, 90% of the viewport, 140 DU).">
            <ActionButton text="Edit camera…" icon=icons::EDIT on_click=instant(move || open.set(true))/>
            <Dialog
                open=open
                title="Edit camera"
                confirm_text="Save"
                on_confirm=save
            >
                <TextField value=name placeholder="Name" label="Name"/>
                <TextField value=host placeholder="Host" label="Host"/>
                <Banner tone=Tone::Info text="The camera reconnects after the change."/>
            </Dialog>
        </Example>
    }
    .into_any()
}

pub fn tabs() -> AnyView {
    let main = RwSignal::new(0usize);
    let line = RwSignal::new(1usize);
    let panel = move |i: usize| match i {
        0 => "Chats: conversations with people and groups.",
        1 => "Functions: cameras, sensors and automations.",
        _ => "Logs: events of the last 24 hours.",
    };
    view! {
        <Example title="Folder tabs" note="The variant used on the Android main screen: the active tab opens into the content through the accent baseline." block=true>
            <Tabs items=vec![TabItem::new("Chats").count(3), TabItem::new("Functions")] selected=main label="Main sections"/>
            <div class="demo-tab-panel">{move || panel(main.get())}</div>
        </Example>
        <Example title="Line tabs" block=true>
            <Tabs
                variant=TabsVariant::Line
                items=vec![TabItem::new("Live").icon(icons::VIDEOCAM), TabItem::new("Recordings"), TabItem::new("Settings").icon(icons::SETTINGS)]
                selected=line
            />
        </Example>
        <Example title="Filled" block=true>
            <Tabs variant=TabsVariant::Line fill=true items=vec![TabItem::new("Day"), TabItem::new("Week"), TabItem::new("Month")] selected=RwSignal::new(0usize)/>
        </Example>
    }
    .into_any()
}

pub fn sheet() -> AnyView {
    let left = RwSignal::new(false);
    let right = RwSignal::new(false);
    let bottom = RwSignal::new(false);
    let log = RwSignal::new(false);
    let events = || {
        (1..=40)
            .map(|i| {
                let title = format!("Motion · zone {}", i % 4 + 1);
                let subtitle = format!("14:{:02}:{:02} · Gate North", 59 - i, (i * 17) % 60);
                view! { <ListItem title=title subtitle=subtitle/> }
            })
            .collect_view()
    };
    view! {
        <Example title="Sides" note="Escape, the scrim, or the close button dismiss the sheet.">
            <ActionButton text="Navigation drawer" icon=icons::MENU on_click=instant(move || left.set(true))/>
            <ActionButton text="Details panel" on_click=instant(move || right.set(true))/>
            <ActionButton text="Bottom sheet" on_click=instant(move || bottom.set(true))/>
        </Example>
        <Example title="Long content" note="The sheet body scrolls; the page behind it stays still, also when the wheel is over the scrim.">
            <ActionButton text="Event log" on_click=instant(move || log.set(true))/>
        </Example>
        <Sheet open=log side=SheetSide::Right title="Event log">
            {events()}
        </Sheet>
        <Sheet open=left title="Operator">
            <ListItem title="Chats" leading=ViewFn::from(|| view! { <Icon icon=icons::INBOX size=dim::ICON_SMALL/> })/>
            <ListItem title="Cameras" leading=ViewFn::from(|| view! { <Icon icon=icons::VIDEOCAM size=dim::ICON_SMALL/> })/>
            <ListItem title="Settings" leading=ViewFn::from(|| view! { <Icon icon=icons::SETTINGS size=dim::ICON_SMALL/> })/>
        </Sheet>
        <Sheet open=right side=SheetSide::Right title="Gate North" size=90.0>
            <div class="demo-sheet-pad">
                <PropertyList>
                    <Property label="Model" value="DS-2CD2143"/>
                    <Property label="Firmware" value="5.7.3"/>
                    <Property label="Address" value="192.168.1.40" copyable=true/>
                </PropertyList>
            </div>
        </Sheet>
        <Sheet open=bottom side=SheetSide::Bottom title="Share clip" size=100.0>
            <ListItem title="Copy link"/>
            <ListItem title="Send to chat"/>
            <ListItem title="Download"/>
        </Sheet>
    }
    .into_any()
}

pub fn pagination() -> AnyView {
    let page = RwSignal::new(6usize);
    let small = RwSignal::new(1usize);
    let big = RwSignal::new(37usize);
    view! {
        <Example title="Many pages" block=true>
            <Pagination page=page total=24usize/>
            <span class="op-t-caption demo-muted">{move || format!("Page {} of 24", page.get())}</span>
        </Example>
        <Example title="Few pages and compact" block=true>
            <Pagination page=small total=4usize/>
            <Pagination page=page total=24usize compact=true/>
        </Example>
        <Example title="Type a page" note="Click the current page, type a number, and press Enter; values past the end go to the last page. Escape restores the page." block=true>
            <Pagination page=big total=480usize/>
            <span class="op-t-caption demo-muted">{move || format!("Page {} of 480", big.get())}</span>
            <Pagination page=big total=480usize compact=true/>
        </Example>
    }
    .into_any()
}
