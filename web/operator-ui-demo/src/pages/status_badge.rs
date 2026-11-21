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
    }
    .into_any()
}
