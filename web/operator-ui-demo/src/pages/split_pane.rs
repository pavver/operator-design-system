//! Live examples for split pane.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn split_pane() -> AnyView {
    let ratio = RwSignal::new(0.35f32);
    let list = ViewFn::from(|| {
        view! {
            <div class="demo-pane">
                {["Gate North", "Gate South", "Yard East", "Parking A"]
                    .into_iter()
                    .map(|c| view! { <ListItem title=c/> })
                    .collect_view()}
            </div>
        }
    });
    let detail = ViewFn::from(|| {
        view! {
            <div class="demo-pane demo-sheet-pad">
                <PropertyList>
                    <Property label="Camera" value="Gate North"/>
                    <Property label="Status" value="Online"/>
                </PropertyList>
            </div>
        }
    });
    let (list2, detail2) = (list.clone(), detail.clone());
    let (list3, detail3) = (list.clone(), detail.clone());
    view! {
        <Example title="Horizontal" note="Drag the divider, use the arrow keys when it is focused, or double-click to reset." block=true>
            <div class="demo-split-box">
                <SplitPane start=list end=detail ratio=ratio/>
            </div>
            <span class="op-t-caption demo-muted">{move || format!("First pane: {:.0}%", ratio.get() * 100.0)}</span>
        </Example>
        <Example title="Vertical" block=true>
            <div class="demo-split-box demo-split-tall">
                <SplitPane start=detail2 end=list2 vertical=true/>
            </div>
        </Example>
        <Example title="Customization" note="Custom handle size, custom label, and custom grip slot." block=true>
            <div class="demo-split-box">
                <SplitPane
                    start=list3
                    end=detail3
                    handle_size=3.0
                    label="Custom divider"
                    grip=Slot::from(move || view! { <span class="demo-muted">"⋮"</span> })
                />
            </div>
        </Example>
    }
    .into_any()
}
