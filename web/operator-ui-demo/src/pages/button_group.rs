//! Live examples for ButtonGroup.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn button_group() -> AnyView {
    view! {
        <Example
            title="Standard joined buttons"
            note="Adjacent buttons share borders seamlessly."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ButtonGroup>
                    <ActionButton text="Copy" />
                    <ActionButton text="Cut" />
                    <ActionButton text="Paste" />
                </ButtonGroup>
            </div>
        </Example>
        <Example
            title="Icon buttons group"
            note="Joined icon actions."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ButtonGroup>
                    <IconButton
                        icon=icons::EDIT
                        label="Edit"
                    />
                    <IconButton
                        icon=icons::REFRESH
                        label="Refresh"
                    />
                    <IconButton
                        icon=icons::DELETE
                        label="Delete"
                    />
                </ButtonGroup>
            </div>
        </Example>
        <Example
            title="Spread mode"
            note="Stretches buttons to fill available width."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ButtonGroup spread=true>
                    <ActionButton text="Left" />
                    <ActionButton text="Center" />
                    <ActionButton text="Right" />
                </ButtonGroup>
            </div>
        </Example>
        <Example
            title="Vertical mode"
            note="Vertical stack sharing horizontal borders."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ButtonGroup vertical=true>
                    <ActionButton text="Option A" />
                    <ActionButton text="Option B" />
                    <ActionButton text="Option C" />
                </ButtonGroup>
            </div>
        </Example>
    }
    .into_any()
}
