//! Live examples for status badge.

use crate::kit::{Cell, Example};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn status_badge() -> AnyView {
    view! {
        <Example title="All statuses at 4 DU">
            {PresenceStatus::ALL.into_iter().map(|s| view! { <Cell label=s.name()><StatusBadge status=s size=4.0/></Cell> }).collect_view()}
        </Example>
        <Example title="With text">
            <StatusBadge status=PresenceStatus::Online text=Signal::stored("Online".to_string())/>
            <StatusBadge status=PresenceStatus::Alarm text=Signal::stored("Motion alarm".to_string())/>
            <StatusBadge status=PresenceStatus::Offline text=Signal::stored("Offline 3 h".to_string())/>
            <StatusBadge status=PresenceStatus::SendingFile text=Signal::stored("Uploading clip".to_string())/>
        </Example>
        <Example title="Customization" note="Metric props and caption slot.">
            <StatusBadge
                status=PresenceStatus::Online
                size=5.0
                gap=3.0
                font_size=13.0
                text_slot=Slot::from(|| view! { <strong>"Custom Online Status"</strong> })
                aria_label="Active right now"
            />
        </Example>
    }
    .into_any()
}
