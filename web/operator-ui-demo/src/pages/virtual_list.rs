//! Live examples for virtual list.

use crate::kit::{Example, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn virtual_list() -> AnyView {
    let rows: Vec<String> = (1..=20_000)
        .map(|i| format!("sensor.probe_{i:05}"))
        .collect();
    let rows = Signal::stored(rows);
    let handle = VirtualListHandle::new(operator_ui::tokens::dim::ROW_COMPACT);
    view! {
        <Example
            title="20 000 rows"
            note="Only the visible rows and a few above and below exist in the DOM."
            block=true
        >
            <div class="demo-row">
                <ActionButton
                    text="Jump to row 15 000"
                    on_click=instant(move || handle.reveal(14_999, true))
                />
            </div>
            <VirtualList
                items=rows
                handle=handle
                height=Some(60.0)
                focusable=true
                key=|row: &String| row.clone()
                let:row
            >
                <div class="demo-vlist-row">
                    <span class="op-t-caption demo-muted">{row.0 + 1}</span>
                    <span>{row.1}</span>
                </div>
            </VirtualList>
        </Example>
        <Example
            title="Customization"
            note="Metric props and header/footer slots customize VirtualList."
            block=true
        >
            <div
                class="demo-resize"
                style="height: calc(var(--op-du) * 50);"
            >
                <VirtualList
                    items=rows
                    handle=VirtualListHandle::new(operator_ui::tokens::dim::ROW_STANDARD)
                    height=45.0
                    header=Slot::from(|| view! {
                        <div
                            class="op-t-caption demo-muted"
                            style="padding: calc(var(--op-du) * 1);"
                        >
                            "Header"
                        </div>
                    })
                    footer=Slot::from(|| view! {
                        <div
                            class="op-t-caption demo-muted"
                            style="padding: calc(var(--op-du) * 1);"
                        >
                            "Footer"
                        </div>
                    })
                    key=|row: &String| row.clone()
                    let:row
                >
                    <div class="demo-vlist-row">
                        <span class="op-t-caption demo-muted">{row.0 + 1}</span>
                        <span>{row.1}</span>
                    </div>
                </VirtualList>
            </div>
        </Example>
    }
    .into_any()
}
