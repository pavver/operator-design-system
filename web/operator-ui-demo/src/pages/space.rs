//! Live examples for Space.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn space() -> AnyView {
    view! {
        <Example
            title="Flex row spacer"
            note="Pushes buttons to opposite ends of a container."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <div style="display:flex;align-items:center;padding:calc(var(--op-du) * 2);border:calc(var(--op-du) * 0.25) solid var(--op-border);">
                    <ActionButton text="Left Action" />
                    <Space />
                    <ActionButton text="Right Action" />
                </div>
            </div>
        </Example>
        <Example
            title="Custom minimum reservation"
            note="Reserves minimum space before expanding."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <div style="display:flex;align-items:center;padding:calc(var(--op-du) * 2);border:calc(var(--op-du) * 0.25) solid var(--op-border);">
                    <span>"Start"</span>
                    <Space min_width=8.0 />
                    <span>"End with min 8 DU space"</span>
                </div>
            </div>
        </Example>
    }
    .into_any()
}
