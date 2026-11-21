//! Live examples for sheet.

use crate::kit::{Example, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

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
