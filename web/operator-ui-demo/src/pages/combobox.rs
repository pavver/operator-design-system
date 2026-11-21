//! Live examples for combobox.

use super::common::SEARCH_CAMERAS as CAMERAS;
use crate::i18n::l;
use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn combobox() -> AnyView {
    let b = backend();
    let camera = RwSignal::new("Gate North".to_string());
    let tag = RwSignal::new(String::new());
    let options: Vec<String> = CAMERAS.iter().map(|s| (*s).to_string()).collect();
    let entity = RwSignal::new(String::new());
    let entities: Vec<String> = (1..=20_000)
        .map(|i| format!("sensor.probe_{i:05}"))
        .collect();
    let device = RwSignal::new("light.kitchen".to_string());
    let level = RwSignal::new("50".to_string());
    let devices = vec![
        ComboOption::new("light.kitchen")
            .label("Kitchen")
            .icon(operator_ui::tokens::icons::INFO)
            .detail("on")
            .section("Lights"),
        ComboOption::new("light.hall")
            .label("Hall")
            .detail("off")
            .section("Lights"),
        ComboOption::new("sensor.outdoor")
            .label("Outdoor temperature")
            .detail("°C")
            .section("Sensors"),
        ComboOption::new("sensor.power")
            .label("Battery power")
            .detail("W")
            .section("Sensors"),
    ];
    let popup = RwSignal::new(false);
    let at = RwSignal::new(PopoverAnchor::Point { x: 0.0, y: 0.0 });
    let added = RwSignal::new(String::from("nothing yet"));
    let nodes = vec![
        ComboOption::new("delay").label("Delay").section("Flow"),
        ComboOption::new("if").label("If").section("Flow"),
        ComboOption::new("add").label("Add").section("Math"),
        ComboOption::new("template")
            .label("Template")
            .detail("text")
            .section("Text"),
    ];
    view! {
        <Example title="Pick from a list" note="Type to filter, use the arrow keys and Enter. Unknown text reverts on blur." block=true>
            <div class="demo-grid-2">
                <Combobox
                    label="Camera"
                    options=options.clone()
                    selected=camera
                    on_select=Callback::new(move |v: String| b.log(format!("[UI] camera={v}")))
                />
                <Combobox
                    label="Tag (custom values allowed)"
                    options=vec!["perimeter", "indoor", "parking", "critical"]
                    selected=tag
                    placeholder="Add a tag"
                    allow_custom=true
                />
            </div>
            <span class="op-t-caption demo-muted">{move || format!("Selected camera: {}", camera.get())}</span>
        </Example>
        <Example title="Large list" note="20 000 options; the list renders only the visible window." block=true>
            <Combobox label="Entity" options=entities selected=entity placeholder="sensor.probe_…"/>
        </Example>
        <Example title="Rich options" note="Sections, icons and details; the field shows the label, the value is the entity id." block=true>
            <div class="demo-grid-2">
                <Combobox label="Device" options=devices selected=device on_select=Callback::new(move |v: String| b.log(format!("[UI] device={v}")))/>
                <Combobox
                    label="Brightness, % (typed value)"
                    options=vec!["25", "50", "75", "100"]
                    selected=level
                    parse_custom=Callback::new(|t: String| t.trim().parse::<u8>().ok().filter(|v| *v <= 100).map(|v| v.to_string()))
                />
            </div>
            <span class="op-t-caption demo-muted">{move || format!("Device: {}, brightness: {}", device.get(), level.get())}</span>
        </Example>
        <Example title="At the pointer" note="Right-click the area: ComboboxPopup opens at the pointer with the search focused (quick add, wire menus)." block=true>
            <div
                class="demo-popup-area"
                on:contextmenu=move |e| {
                    e.prevent_default();
                    at.set(PopoverAnchor::Point { x: f64::from(e.client_x()), y: f64::from(e.client_y()) });
                    popup.set(true);
                }
            >
                <span class="op-t-caption demo-muted">{move || format!("Added: {}", added.get())}</span>
            </div>
            <ComboboxPopup
                open=popup
                anchor=at
                options=nodes
                placeholder="Add a node"
                on_select=Callback::new(move |o: ComboOption| added.set(o.text().to_string()))
                on_custom=Callback::new(move |t: String| added.set(format!("custom: {t}")))
            />
        </Example>
        <Example title="Customization" block=true>
            <Combobox options=vec!["North", "South", "East", "West"] selected=RwSignal::new(String::new())
                label=l("Direction", "Напрямок") placeholder=l("Choose a direction", "Виберіть напрямок")
                height=12.0 padding_x=4.0 row_height=10.0 max_height=30.0
                prepend=|| view! { <Icon icon=icons::INFO/> }
                option=Callback::new(|item: ComboOption| view! { <strong>{item.text().to_string()}</strong> }.into_any())
                footer=|| view! { <span>{l("Four directions", "Чотири напрямки")}</span> }/>
        </Example>
    }
    .into_any()
}
