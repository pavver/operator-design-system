//! Live examples for virtual list.

use crate::kit::{Example, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn virtual_list() -> AnyView {
    let rows: Vec<String> = (1..=20_000)
        .map(|i| format!("sensor.probe_{i:05}"))
        .collect();
    let handle = VirtualListHandle::new(operator_ui::tokens::dim::ROW_COMPACT);
    view! {
        <Example title="20 000 rows" note="Only the visible rows and a few above and below exist in the DOM." block=true>
            <div class="demo-row">
                <ActionButton text="Jump to row 15 000" on_click=instant(move || handle.reveal(14_999, true))/>
            </div>
            <VirtualList
                items=Signal::stored(rows)
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
    }
    .into_any()
}
