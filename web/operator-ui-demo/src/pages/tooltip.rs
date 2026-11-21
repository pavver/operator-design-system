//! Live examples for tooltip.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn tooltip() -> AnyView {
    view! {
        <Example title="On icon buttons" note="Hover for 500 ms, focus with Tab, or long-press on touch.">
            <Tooltip text="Refresh presets"><IconButton icon=icons::REFRESH label="Refresh" on_click=Callback::new(|()| {})/></Tooltip>
            <Tooltip text="Camera settings"><IconButton icon=icons::SETTINGS label="Settings" on_click=Callback::new(|()| {})/></Tooltip>
            <Tooltip text="Delete the selected recording from the device"><IconButton icon=icons::DELETE label="Delete" on_click=Callback::new(|()| {})/></Tooltip>
        </Example>
        <Example title="On text">
            <Tooltip text="Round-trip time to the device">
                <span class="docs-dotted" tabindex="0">"RTT 12 ms"</span>
            </Tooltip>
        </Example>
    }
    .into_any()
}
