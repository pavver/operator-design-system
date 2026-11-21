//! Live examples for switch.

use crate::kit::{Cell, Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

fn async_switch(label: &'static str, value: RwSignal<bool>) -> impl IntoView {
    let b = backend();
    view! {
        <AsyncSwitch
            checked=value
            label=label
            on_toggle=AsyncAction::new(move |v: bool| async move {
                let r = b.call(format!("{label}={v}")).await;
                if r.is_success() {
                    value.set(v);
                }
                r
            })
        />
    }
}

pub fn switch() -> AnyView {
    view! {
        <Example title="Interactive" note="The value commits only after success; the thumb parks in the center with a spinner while pending.">
            <Cell label="Night mode">{async_switch("night_mode", RwSignal::new(true))}</Cell>
            <Cell label="IR light">{async_switch("ir_light", RwSignal::new(false))}</Cell>
        </Example>
        <Example title="Static states">
            <Cell label="On"><AsyncSwitch checked=true/></Cell>
            <Cell label="Off"><AsyncSwitch checked=false/></Cell>
            <Cell label="Pressed"><AsyncSwitch checked=true state=ControlState::Pressed/></Cell>
            <Cell label="Pending"><AsyncSwitch checked=true state=ControlState::Pending/></Cell>
            <Cell label="Success"><AsyncSwitch checked=true state=ControlState::Success/></Cell>
            <Cell label="Error"><AsyncSwitch checked=false state=ControlState::Error/></Cell>
            <Cell label="Disabled on"><AsyncSwitch checked=true enabled=false/></Cell>
            <Cell label="Disabled off"><AsyncSwitch checked=false enabled=false/></Cell>
        </Example>
    }
    .into_any()
}
