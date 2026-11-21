//! Live examples for popover.

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
    }
    .into_any()
}
