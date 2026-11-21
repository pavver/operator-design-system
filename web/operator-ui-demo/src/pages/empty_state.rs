//! Live examples for empty state.

use super::common::load_cycle_example;
use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn empty_state() -> AnyView {
    let b = backend();
    view! {
        {load_cycle_example()}
        <Example title="Full" block=true>
            <div class="demo-list">
                <EmptyState
                    title="No recordings yet"
                    description="Recordings appear here when motion is detected in an armed zone."
                    actions=ViewFn::from(move || view! { <ActionButton text="Arm zones" on_click=b.action("arm zones")/> })
                />
            </div>
        </Example>
        <Example title="Compact" block=true>
            <div class="demo-list">
                <EmptyState icon=icons::SEARCH title="No matches" description="Try a shorter query." compact=true/>
            </div>
        </Example>
    }
    .into_any()
}
