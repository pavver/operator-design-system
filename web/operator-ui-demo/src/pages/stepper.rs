//! Live examples for stepper.

use crate::kit::{Cell, Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn stepper() -> AnyView {
    let b = backend();
    view! {
        <Example title="Interactive" note="Stays interactive while pending for continuous input.">
            <Cell label="Async (PTZ preset)">
                <Stepper value=RwSignal::new(3) min=0 max=9 on_async_change=AsyncAction::new(move |v: i32| b.call(format!("ptz preset={v}")))/>
            </Cell>
            <Cell label="Sync, 0..10"><Stepper value=RwSignal::new(5) min=0 max=10/></Cell>
        </Example>
        <Example title="Static states">
            <Cell label="Pending"><Stepper value=RwSignal::new(2) state=ControlState::Pending/></Cell>
            <Cell label="Success"><Stepper value=RwSignal::new(2) state=ControlState::Success/></Cell>
            <Cell label="Error"><Stepper value=RwSignal::new(2) state=ControlState::Error/></Cell>
            <Cell label="At min"><Stepper value=RwSignal::new(0) min=0/></Cell>
            <Cell label="Disabled"><Stepper value=RwSignal::new(2) enabled=false/></Cell>
        </Example>
    }
    .into_any()
}
