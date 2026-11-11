//! Examples for navigation and overlay components.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

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
