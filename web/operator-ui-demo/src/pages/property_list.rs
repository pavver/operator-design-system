//! Live examples for property list.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn property_list() -> AnyView {
    view! {
        <Example title="Device details" note="Narrow containers stack the label above the value." block=true>
            <div class="demo-grid-2">
                <PropertyList>
                    <Property label="Model" value="DS-2CD2143G2-I"/>
                    <Property label="Firmware" value="V5.7.3 build 220112"/>
                    <Property label="Address" value="192.168.1.40" copyable=true/>
                    <Property label="Serial" value="DS2CD2143-20220112AAWRJ3" copyable=true/>
                    <Property label="Status">
                        <StatusBadge status=PresenceStatus::Online text=Signal::stored("Online".to_string())/>
                    </Property>
                </PropertyList>
                <div style:max-width=du(55.0)>
                    <PropertyList>
                        <Property label="Uptime" value="12 d 4 h"/>
                        <Property label="Storage" value="1.2 / 2 TB"/>
                    </PropertyList>
                </div>
            </div>
        </Example>
    }
    .into_any()
}
