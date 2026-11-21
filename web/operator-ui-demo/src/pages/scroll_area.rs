//! Live examples for scroll area.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn scroll_area() -> AnyView {
    let rows = (1..=30)
        .map(|i| view! { <ListItem title=format!("Event {i:02}") subtitle="Motion detected · Gate North"/> })
        .collect_view();
    view! {
        <Example title="Stretch overscroll" note="Scroll to an edge and keep pulling with touch or the mouse wheel: the content stretches and springs back." block=true>
            <div class="demo-grid-2">
                <div class="demo-list">
                    <ScrollArea height=70.0 label="Events">{rows}</ScrollArea>
                </div>
                <div class="demo-list">
                    <ScrollArea height=70.0 label="Short list">
                        <ListItem title="Only two rows"/>
                        <ListItem title="Still stretches when pulled"/>
                    </ScrollArea>
                </div>
            </div>
        </Example>
    }
    .into_any()
}
