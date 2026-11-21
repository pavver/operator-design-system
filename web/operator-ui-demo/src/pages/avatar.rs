//! Live examples for avatar.

use crate::kit::{Cell, Example};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn avatar() -> AnyView {
    view! {
        <Example title="Kinds" note="Color is derived from the id with the same hash on every platform.">
            <Cell label="Direct"><Avatar id="andrii" name="Andrii" kind=AvatarKind::Direct/></Cell>
            <Cell label="Threaded group"><Avatar id="ops-room" name="Ops" kind=AvatarKind::ThreadedGroup/></Cell>
            <Cell label="Channel"><Avatar id="news" name="News" kind=AvatarKind::Channel/></Cell>
            <Cell label="Simple group"><Avatar id="family" name="Family" kind=AvatarKind::SimpleGroup/></Cell>
        </Example>
        <Example title="Sizes and status">
            <Cell label="6 DU"><Avatar id="a1" name="Ihor" size=6.0 status=PresenceStatus::Online/></Cell>
            <Cell label="8 DU"><Avatar id="a2" name="Marta" size=8.0 status=PresenceStatus::Alarm/></Cell>
            <Cell label="10 DU"><Avatar id="a3" name="Taras" size=10.0 status=PresenceStatus::RecordingAudio/></Cell>
            <Cell label="14 DU"><Avatar id="a4" name="Zoia" size=14.0 status=PresenceStatus::SendingFile/></Cell>
            <Cell label="Loading"><Avatar id="a5" name="Bot" size=10.0 image_url="debug_loading"/></Cell>
        </Example>
        <Example title="Customization" note="Custom badge slot, metric props and aria-label.">
            <Cell label="Custom badge slot">
                <Avatar
                    id="custom-1"
                    name="Security Team"
                    size=12.0
                    font_size=20.0
                    border_width=0.5
                    badge=Slot::from(|| view! { <span style="background:var(--op-accent);color:var(--op-on-accent);padding:0 4px;font-size:10px;font-weight:700;">"ADMIN"</span> })
                    aria_label="Security administrator avatar"
                />
            </Cell>
        </Example>
    }
    .into_any()
}
