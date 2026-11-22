//! Live examples for layout.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn layout() -> AnyView {
    let start_open = RwSignal::new(true);

    view! {
        <Example title="App Shell Layout" note="App shell in container mode with header, drawer, scrollable page container, footer, and PageScroller." block=true>
            <div style="height: 380px; width: 100%; border: var(--op-dim-border) solid var(--op-divider); position: relative;">
                <Layout
                    container=true
                    start_drawer_open=start_open
                    start_drawer_width=32.0
                    header=Slot::from(move || view! {
                        <div style="display: flex; align-items: center; justify-content: space-between; height: 100%; padding: 0 12px;">
                            <span style="font-weight: 700;">"Operator Console"</span>
                            <button
                                type="button"
                                style="background: transparent; border: 1px solid var(--op-border); color: var(--op-text-primary); padding: 4px 8px; cursor: pointer;"
                                on:click=move |_| start_open.update(|v| *v = !*v)
                            >
                                {move || if start_open.get() { "Close Drawer" } else { "Open Drawer" }}
                            </button>
                        </div>
                    })
                    start_drawer=Slot::from(move || view! {
                        <div style="padding: 12px; display: flex; flex-direction: column; gap: 8px;">
                            <span style="font-size: var(--op-type-caption-size); color: var(--op-text-secondary);">"NAVIGATION"</span>
                            <span>"Dashboard"</span>
                            <span>"Telemetry"</span>
                            <span>"Settings"</span>
                        </div>
                    })
                    footer=Slot::from(move || view! {
                        <div style="display: flex; align-items: center; height: 100%; padding: 0 12px; font-size: var(--op-type-caption-size); color: var(--op-text-secondary);">
                            "Status: Nominal · 24 nodes active"
                        </div>
                    })
                >
                    <div style="padding: 16px; min-height: 800px; display: flex; flex-direction: column; gap: 16px;">
                        <span style="font-weight: 700; font-size: var(--op-type-section-title-size);">"Main Content Area"</span>
                        <p style="color: var(--op-text-secondary); max-width: 600px;">
                            "Scroll down to see the PageScroller button appear at the bottom-right corner. Layout provides structured regions: header, footer, start and end drawers, and page content."
                        </p>
                        <div style="height: 600px; background: var(--op-surface-alt); padding: 16px; border: var(--op-dim-border) solid var(--op-border);">
                            "Scrollable telemetry workspace..."
                        </div>
                    </div>
                    <PageSticky position=StickyPosition::TopEnd offset_x=4.0 offset_y=4.0>
                        <span style="background: var(--op-accent); color: var(--op-on-accent); padding: 4px 8px; font-size: 11px; font-weight: 700;">
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
