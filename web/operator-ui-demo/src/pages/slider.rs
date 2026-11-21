//! Live examples for slider.

use crate::kit::{Cell, Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn slider() -> AnyView {
    let b = backend();
    view! {
        <Example title="Interactive" note="Drag or click the track; arrow keys work when focused. The commit after release runs async; on error the value rolls back." block=true>
            <div class="demo-stack demo-narrow">
                <div class="demo-labelled">
                    <span class="op-t-caption demo-muted">"Continuous 0..1"</span>
                    <Slider value=RwSignal::new(0.64f32) label="Brightness"/>
                </div>
                <div class="demo-labelled">
                    <span class="op-t-caption demo-muted">"Stepped 0..10, async commit"</span>
                    <Slider value=RwSignal::new(4.0f32) max=10.0 step=1.0 label="Zoom" on_commit=move |v: f32| b.call(format!("zoom={v}"))/>
                </div>
            </div>
        </Example>
        <Example title="Static states">
            {[("Pending", ControlState::Pending), ("Success", ControlState::Success), ("Error", ControlState::Error)]
                .into_iter()
                .map(|(n, s)| view! {
                    <Cell label=n><div class="demo-slider-cell"><Slider value=RwSignal::new(0.4) state=s/></div></Cell>
                })
                .collect_view()}
            <Cell label="Disabled">
                <div class="demo-slider-cell"><Slider value=RwSignal::new(5.0) max=10.0 step=1.0 enabled=false/></div>
            </Cell>
        </Example>
    }
    .into_any()
}
