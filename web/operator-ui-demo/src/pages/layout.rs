//! Live examples for layout.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn layout() -> AnyView {
    let start_open = RwSignal::new(true);

    view! {
        <Example title="App Shell Layout" note="App shell in container mode with header, drawer, scrollable page container, footer, and PageScroller." block=true>
            <div style="height: calc(var(--op-du) * 95); width: 100%; border: var(--op-dim-border) solid var(--op-divider); position: relative;">
                <Layout
                    container=true
                    start_drawer_open=start_open
                    start_drawer_width=32.0
                    header=Slot::from(move || view! {
                        <div style="display: flex; align-items: center; justify-content: space-between; height: 100%; padding: 0 calc(var(--op-du) * 3);">
                            <span style="font-weight: 700;">"Operator Console"</span>
                            <ActionButton
                                text=Signal::derive(move || if start_open.get() { "Close Drawer".to_string() } else { "Open Drawer".to_string() })
                                on_click=AsyncAction::new(move |()| {
                                    start_open.update(|v| *v = !*v);
                                    Box::pin(async { ActionResult::Success })
                                })
                            />
                        </div>
                    })
                    start_drawer=Slot::from(move || view! {
                        <div style="padding: calc(var(--op-du) * 3); display: flex; flex-direction: column; gap: calc(var(--op-du) * 2);">
                            <span style="font-size: var(--op-type-caption-size); color: var(--op-text-secondary);">"NAVIGATION"</span>
                            <span>"Dashboard"</span>
                            <span>"Telemetry"</span>
                            <span>"Settings"</span>
                        </div>
                    })
                    footer=Slot::from(move || view! {
                        <div style="display: flex; align-items: center; height: 100%; padding: 0 calc(var(--op-du) * 3); font-size: var(--op-type-caption-size); color: var(--op-text-secondary);">
                            "Status: Nominal · 24 nodes active"
                        </div>
                    })
                >
                    <div style="padding: calc(var(--op-du) * 4); min-height: calc(var(--op-du) * 200); display: flex; flex-direction: column; gap: calc(var(--op-du) * 4);">
                        <span style="font-weight: 700; font-size: var(--op-type-section-title-size);">"Main Content Area"</span>
                        <p style="color: var(--op-text-secondary); max-width: calc(var(--op-du) * 150);">
                            "Scroll down to see the PageScroller button appear at the bottom-right corner. Layout provides structured regions: header, footer, start and end drawers, and page content."
                        </p>
                        <div style="height: calc(var(--op-du) * 150); background: var(--op-surface-alt); padding: calc(var(--op-du) * 4); border: var(--op-dim-border) solid var(--op-border);">
                            "Scrollable telemetry workspace..."
                        </div>
                    </div>
                    <PageSticky position=StickyPosition::TopEnd offset_x=4.0 offset_y=4.0>
                        <span style="background: var(--op-accent); color: var(--op-on-accent); padding: calc(var(--op-du) * 1) calc(var(--op-du) * 2); font-size: calc(var(--op-tu) * 11); font-weight: 700;">
                            "LIVE"
                        </span>
                    </PageSticky>
                    <PageScroller scroll_offset=15.0 />
                </Layout>
            </div>
        </Example>
    }
    .into_any()
}
