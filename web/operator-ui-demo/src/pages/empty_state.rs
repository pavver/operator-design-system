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
        <Example title="Customization" note="Metric props, title and action slots." block=true>
            <div class="demo-list">
                <EmptyState
                    title_slot=Slot::from(|| view! { <span style="color:var(--op-accent);font-weight:700;">"Custom Empty View"</span> })
                    description="Customized spacing, icon size, and action slot."
                    icon_size=10.0
                    padding_y=6.0
                    actions=Slot::from(|| view! { <ActionButton text="Add new filter"/> })
                />
            </div>
        </Example>
    }
    .into_any()
}
