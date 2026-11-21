//! Live examples for counter.

use crate::kit::{Cell, Example};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn counter() -> AnyView {
    let unread = RwSignal::new(3i32);
    let count = Signal::derive(move || u32::try_from(unread.get()).unwrap_or(0));
    view! {
        <Example title="Tones and overflow">
            <Cell label="Accent"><Counter count=3u32/></Cell>
            <Cell label="Neutral"><Counter count=27u32 tone=CounterTone::Neutral/></Cell>
            <Cell label="Error"><Counter count=1u32 tone=CounterTone::Error/></Cell>
            <Cell label="Over max"><Counter count=1250u32/></Cell>
            <Cell label="Zero shown"><Counter count=0u32 show_zero=true tone=CounterTone::Neutral/></Cell>
        </Example>
        <Example title="On an icon button" note="Click the inbox to mark everything read.">
            <WithCounter count=count>
                <IconButton icon=icons::INBOX label="Inbox" on_click=Callback::new(move |()| unread.set(0))/>
            </WithCounter>
            <Stepper value=unread min=0 max=150/>
        </Example>
        <Example title="Customization" note="Metric props (min_width, height, font_size) and custom label.">
            <Cell label="Custom size">
                <Counter count=7u32 min_width=6.0 height=6.0 font_size=14.0 aria_label="7 unread notifications"/>
            </Cell>
        </Example>
    }
    .into_any()
}
