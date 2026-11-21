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
    }
    .into_any()
}
