//! Live examples for tooltip.

use crate::i18n::l;
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
        <Example title="Customization">
            <Tooltip text=l("Connection details", "Відомості про з’єднання") max_width=60.0 padding_x=4.0 padding_y=2.0 offset=3.0
                content=move || view! { <strong>{l("Round-trip time to the device", "Час обміну даними з пристроєм")}</strong> }
                style="--op-tooltip-background: var(--op-surface);"
            >
                <span tabindex="0" class="docs-dotted">{l("Connection", "З’єднання")}</span>
            </Tooltip>
        </Example>
    }
    .into_any()
}
