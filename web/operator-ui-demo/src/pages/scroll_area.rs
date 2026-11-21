//! Live examples for scroll area.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn scroll_area() -> AnyView {
    let rows = (1..=30)
        .map(|i| view! { <ListItem title=format!("Event {i:02}") subtitle="Motion detected · Gate North"/> })
        .collect_view();
    let custom_rows = (1..=10)
        .map(|i| view! { <ListItem title=format!("Custom Row {i}")/> })
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
        <Example title="Customization" note="Custom metrics, header/footer slots, and custom label." block=true>
            <div class="demo-list">
                <ScrollArea
                    height=50.0
                    label="Custom scroll area"
                    header=Slot::from(move || view! { <div class="op-t-caption demo-muted demo-sheet-pad">"Fixed Header"</div> })
                    footer=Slot::from(move || view! { <div class="op-t-caption demo-muted demo-sheet-pad">"Fixed Footer"</div> })
                >
                    {custom_rows}
                </ScrollArea>
            </div>
        </Example>
    }
    .into_any()
}
