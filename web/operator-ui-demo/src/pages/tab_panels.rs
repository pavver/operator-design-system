//! Live examples for tab-panels.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn tab_panels() -> AnyView {
    let selected = RwSignal::new(0usize);

    view! {
        <Example title="Animated Tab Panels" note="TabPanels driven by an existing Tabs component selection signal." block=true>
            <div style="display: flex; flex-direction: column; width: 100%; gap: 12px;">
                <Tabs
                    variant=TabsVariant::Folder
                    items=vec![
                        TabItem::new("Overview"),
                        TabItem::new("Metrics").count(4),
                        TabItem::new("Logs"),
                    ]
                    selected=selected
                />
                <TabPanels value=selected padding=3.0>
                    <TabPanel index=0>
                        <div style="padding: 16px; background: var(--op-surface-alt); border: var(--op-dim-border) solid var(--op-border);">
                            <span style="font-weight: 700;">"Overview Panel"</span>
                            <p style="color: var(--op-text-secondary); margin-top: 8px;">
                                "System running smoothly. All services operational."
                            </p>
                        </div>
                    </TabPanel>
                    <TabPanel index=1>
                        <div style="padding: 16px; background: var(--op-surface-alt); border: var(--op-dim-border) solid var(--op-border);">
                            <span style="font-weight: 700;">"Metrics Panel"</span>
                            <p style="color: var(--op-text-secondary); margin-top: 8px;">
                                "CPU: 14% · Memory: 2.1 GB / 8 GB · Network: 420 KB/s."
                            </p>
                        </div>
                    </TabPanel>
                    <TabPanel index=2>
                        <div style="padding: 16px; background: var(--op-surface-alt); border: var(--op-dim-border) solid var(--op-border);">
                            <span style="font-weight: 700;">"Logs Panel"</span>
                            <p style="color: var(--op-text-secondary); margin-top: 8px;">
                                "[01:34:22] Cluster synchronized with leader node."
                            </p>
                        </div>
                    </TabPanel>
                </TabPanels>
            </div>
        </Example>
    }
    .into_any()
}
