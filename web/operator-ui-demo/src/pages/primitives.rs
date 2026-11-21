//! Live examples for primitives.

use crate::kit::{Cell, Example};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn primitives() -> AnyView {
    view! {
        <Example title="Icon and ProgressIndicator">
            <Cell label="Icon 4 DU"><Icon icon=icons::SETTINGS size=4.0/></Cell>
            <Cell label="Icon 6 DU"><Icon icon=icons::SETTINGS/></Cell>
            <Cell label="Spinner 6 DU"><ProgressIndicator/></Cell>
            <Cell label="Spinner pending"><ProgressIndicator size=4.0 stroke=dim::SPINNER_STROKE color="var(--op-pending)"/></Cell>
        </Example>
        <Example title="Card and Divider" block=true>
            <div class="demo-grid-2">
                <Card>
                    <div class="op-t-row-title">"Surface card"</div>
                    <div class="op-t-body-small demo-muted">"Primary container background."</div>
                </Card>
                <Card alt=true>
                    <div class="op-t-row-title">"Alt card"</div>
                    <div class="op-t-body-small demo-muted">"Sunken background for displays."</div>
                </Card>
            </div>
            <Divider/>
        </Example>
        <Example title="Card Sections and Customization" block=true>
            <Card
                padding=4.0
                gap=3.0
                header=Slot::from(|| view! { <div class="op-t-row-title">"Card Header"</div> })
                actions=Slot::from(|| view! { <ActionButton text="Confirm"/> })
                footer=Slot::from(|| view! { <span class="op-t-caption demo-muted">"Last updated 5m ago"</span> })
            >
                <div class="op-t-body-small">"This card uses Quasar-style container sections: header, body (default children), actions, and footer."</div>
            </Card>
        </Example>
    }
    .into_any()
}
