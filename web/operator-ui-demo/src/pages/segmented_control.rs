//! Live examples for segmented control.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn segmented() -> AnyView {
    let b = backend();
    view! {
        <Example title="Variants" note="The async variant shows a spinner in the target segment and slides back on error." block=true>
            <div class="demo-stack demo-narrow">
                <SegmentedControl items=vec![SegmentItem::text("Auto"), SegmentItem::text("Day"), SegmentItem::text("Night")] selected=RwSignal::new(1usize)/>
                <SegmentedControl
                    items=vec![SegmentItem::item("Live", icons::VIDEOCAM), SegmentItem::item("Search", icons::SEARCH), SegmentItem::icon(icons::SETTINGS)]
                    selected=RwSignal::new(0usize)
                />
                <SegmentedControl
                    items=vec![SegmentItem::text("SD"), SegmentItem::text("HD"), SegmentItem::text("4K")]
                    selected=RwSignal::new(0usize)
                    on_select=AsyncAction::new(move |i: usize| b.call(format!("stream quality={i}")))
                />
                <SegmentedControl items=vec![SegmentItem::text("Auto"), SegmentItem::text("Day"), SegmentItem::text("Night")] selected=RwSignal::new(0) enabled=false/>
            </div>
        </Example>
    }
    .into_any()
}
