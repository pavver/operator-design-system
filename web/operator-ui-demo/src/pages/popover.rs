//! Live examples for popover.

use crate::i18n::l;
use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn popover() -> AnyView {
    let open = RwSignal::new(false);
    let host = NodeRef::<leptos::html::Div>::new();
    let anchor = Signal::derive(move || {
        host.get()
            .map(|el| PopoverAnchor::of(&el))
            .unwrap_or(PopoverAnchor::Point { x: 0.0, y: 0.0 })
    });
    let menu_items = vec![
        MenuItem::new("open", "Open"),
        MenuItem::new("rename", "Rename"),
        MenuItem::new("delete", "Delete").danger().separated(),
    ];
    let camera = RwSignal::new("Gate North".to_string());
    let custom_open = RwSignal::new(false);
    let custom_host = NodeRef::<leptos::html::Div>::new();
    let custom_anchor = Signal::derive(move || {
        custom_host
            .get()
            .map(|el| PopoverAnchor::of(&el))
            .unwrap_or(PopoverAnchor::Point { x: 0.0, y: 0.0 })
    });
    view! {
        <Example title="Escapes clipping" note="The box clips and transforms its content; the layers still open fully on top." block=true>
            <div class="demo-clip-box">
                <div node_ref=host class="demo-row">
                    <ActionButton text="Toggle popover" on_click=crate::kit::instant(move || open.update(|o| *o = !*o))/>
                    <MenuButton items=menu_items on_select=Callback::new(|_: String| {})/>
                    <Tooltip text="A tooltip inside a clipped box">
                        <span class="op-t-body">Hover me</span>
                    </Tooltip>
                </div>
                <DropdownMenu
                    label="Camera"
                    options=vec!["Gate North".into(), "Gate South".into(), "Parking".into(), "Lobby".into()]
                    selected=camera
                />
            </div>
            <Popover open=open anchor=anchor gap=1.0 class="demo-popover">
                <span class="op-t-body">Any content in a floating layer.</span>
            </Popover>
        </Example>
        <Example title="Customization" note="The layer exposes offsets, slots, and a reactive accessible name.">
            <div node_ref=custom_host>
                <ActionButton text=l("Open preview", "Відкрити перегляд") on_click=crate::kit::instant(move || custom_open.update(|v| *v = !*v))/>
            </div>
            <Popover open=custom_open anchor=custom_anchor placement=Placement::BelowStart gap=2.0 offset_x=2.0 offset_y=1.0
                class="demo-popover" aria_label=l("Preview", "Перегляд") role="dialog"
                header=move || view! { <strong>{l("Preview", "Перегляд")}</strong> }
                footer=move || view! { <small>{l("Escape closes this preview", "Escape закриває цей перегляд")}</small> }
                style="--op-popover-background: var(--op-surface-alt); --op-popover-text: var(--op-text-primary); background-color: var(--op-popover-background);"
            >
                <span>{l("Anchored content", "Вміст біля елемента")}</span>
            </Popover>
        </Example>
    }
    .into_any()
}
