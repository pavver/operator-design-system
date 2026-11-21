//! Live examples for theme toggle.

use crate::kit::{Cell, Example};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn theme_toggle() -> AnyView {
    let local = RwSignal::new(false);
    view! {
        <Example title="Interactive" note="The switch in the top bar of this site is the same component.">
            <Cell label="Default"><ThemeToggle dark=local on_toggle=Callback::new(move |v: bool| local.set(v))/></Cell>
            <Cell label="Large, 24 × 10 DU">
                <ThemeToggle dark=local on_toggle=Callback::new(move |v: bool| local.set(v)) width=24.0 height=10.0/>
            </Cell>
        </Example>
        <Example title="Static">
            <Cell label="Day"><ThemeToggle dark=false/></Cell>
            <Cell label="Night"><ThemeToggle dark=true/></Cell>
        </Example>
    }
    .into_any()
}
