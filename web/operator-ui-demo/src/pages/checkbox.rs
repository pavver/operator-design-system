//! Live examples for checkbox.

use crate::kit::{Cell, Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn checkbox() -> AnyView {
    let b = backend();
    let autofocus = RwSignal::new(true);
    view! {
        <Example title="Interactive">
            <AsyncCheckbox
                checked=autofocus
                text=Signal::stored("Autofocus".to_string())
                on_change=AsyncAction::new(move |v: bool| async move {
                    let r = b.call(format!("autofocus={v}")).await;
                    if r.is_success() {
                        autofocus.set(v);
                    }
                    r
                })
            />
        </Example>
        <Example title="Static states">
            <Cell label="Unchecked"><AsyncCheckbox checked=false/></Cell>
            <Cell label="Checked"><AsyncCheckbox checked=true/></Cell>
            <Cell label="Pressed"><AsyncCheckbox checked=false state=ControlState::Pressed/></Cell>
            <Cell label="Pending"><AsyncCheckbox checked=false state=ControlState::Pending/></Cell>
            <Cell label="Success"><AsyncCheckbox checked=true state=ControlState::Success/></Cell>
            <Cell label="Error"><AsyncCheckbox checked=false state=ControlState::Error/></Cell>
            <Cell label="Disabled"><AsyncCheckbox checked=true enabled=false/></Cell>
        </Example>
    }
    .into_any()
}
