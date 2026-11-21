//! Live examples for text area.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn text_area() -> AnyView {
    view! {
        <Example title="Auto-growing" note="Grows from 3 to 8 lines, then scrolls." block=true>
            <div class="demo-stack demo-narrow">
                <TextArea value=RwSignal::new(String::new()) placeholder="Describe the incident" label="Incident notes"/>
                <TextArea
                    value=RwSignal::new("Motion at the north gate after 22:00.".to_string())
                    placeholder="Message"
                    label="Message"
                    max_length=120
                    min_rows=2
                />
            </div>
        </Example>
        <Example title="States" block=true>
            <div class="demo-grid-2">
                <TextArea value=RwSignal::new("Saved".to_string()) state=ControlState::Success min_rows=2/>
                <TextArea value=RwSignal::new("Rejected".to_string()) state=ControlState::Error min_rows=2/>
                <TextArea value=RwSignal::new("Sending…".to_string()) state=ControlState::Pending min_rows=2/>
                <TextArea value=RwSignal::new("Read only".to_string()) enabled=false min_rows=2/>
            </div>
        </Example>
    }
    .into_any()
}
