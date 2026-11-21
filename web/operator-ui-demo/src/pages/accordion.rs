//! Live examples for accordion.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn accordion() -> AnyView {
    view! {
        <Example title="Accordion" note="Only one section is open at a time." block=true>
            <div class="demo-narrow">
                <Accordion>
                    <Collapsible title="Network" subtitle="DHCP · 192.168.1.40" initially_open=true>
                        <PropertyList>
                            <Property label="Mode" value="DHCP"/>
                            <Property label="Gateway" value="192.168.1.1"/>
                        </PropertyList>
                    </Collapsible>
                    <Collapsible title="Video" subtitle="H.265 · 4K · 25 fps">
                        <p class="demo-p">"Main stream settings."</p>
                    </Collapsible>
                    <Collapsible title="Alarms" trailing=ViewFn::from(|| view! { <Counter count=2u32 tone=CounterTone::Error/> })>
                        <p class="demo-p">"Two zones report motion."</p>
                    </Collapsible>
                </Accordion>
            </div>
        </Example>
        <Example title="Standalone collapsible" block=true>
            <div class="demo-narrow">
                <Collapsible title="Advanced options">
                    <AsyncCheckbox checked=false text=Signal::stored("Verbose logging".to_string())/>
                </Collapsible>
            </div>
        </Example>
    }
    .into_any()
}
