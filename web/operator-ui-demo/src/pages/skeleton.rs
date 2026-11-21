//! Live examples for skeleton.

use super::common::load_cycle_example;
use crate::kit::{Cell, Example, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn skeleton() -> AnyView {
    let loaded = RwSignal::new(false);
    view! {
        {load_cycle_example()}
        <Example title="Shapes">
            <Cell label="Circle"><Skeleton circle=true width=10.0 height=10.0/></Cell>
            <Cell label="Block"><Skeleton width=30.0 height=17.0/></Cell>
            <Cell label="Text, 3 lines"><div style:width=du(50.0)><Skeleton lines=3/></div></Cell>
        </Example>
        <Example title="Loading a list row" block=true>
            <div class="demo-list">
                {move || {
                    if loaded.get() {
                        view! {
                            <ListItem
                                title="Gate camera"
                                subtitle="Online · 12 ms · H.265"
                                leading=ViewFn::from(|| view! { <Avatar id="cam-3" name="Gate" status=PresenceStatus::Online/> })
                            />
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="demo-skeleton-row">
                                <Skeleton circle=true width=8.0 height=8.0/>
                                <div class="demo-skeleton-text"><Skeleton lines=2/></div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
            <div>
                <ActionButton
                    text="Toggle loaded"
                    on_click=instant(move || loaded.update(|l| *l = !*l))
                />
            </div>
        </Example>
    }
    .into_any()
}
