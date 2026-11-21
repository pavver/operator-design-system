//! Live examples for pagination.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn pagination() -> AnyView {
    let page = RwSignal::new(6usize);
    let small = RwSignal::new(1usize);
    let big = RwSignal::new(37usize);
    let custom = RwSignal::new(2usize);
    view! {
        <Example title="Many pages" block=true>
            <Pagination page=page total=24usize/>
            <span class="op-t-caption demo-muted">{move || format!("Page {} of 24", page.get())}</span>
        </Example>
        <Example title="Few pages and compact" block=true>
            <Pagination page=small total=4usize/>
            <Pagination page=page total=24usize compact=true/>
        </Example>
        <Example title="Type a page" note="Click the current page, type a number, and press Enter; values past the end go to the last page. Escape restores the page." block=true>
            <Pagination page=big total=480usize/>
            <span class="op-t-caption demo-muted">{move || format!("Page {} of 480", big.get())}</span>
            <Pagination page=big total=480usize compact=true/>
        </Example>
        <Example title="Customization" note="Custom metrics, before/after slots, and custom labels." block=true>
            <Pagination
                page=custom
                total=10usize
                gap=2.0
                page_size=10.0
                before=Slot::from(move || view! { <span class="op-t-caption demo-muted">"Page:"</span> })
                label="Custom pagination"
            />
        </Example>
    }
    .into_any()
}
