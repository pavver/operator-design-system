//! Live examples for segmented control.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn segmented() -> AnyView {
    let b = backend();
    view! {
        <Example title="Customization" note="Metric props and slots customize this instance; its translated string follows the language selector.">
            <SegmentedControl items=vec![SegmentItem::text("Auto"), SegmentItem::text("Manual")]
                selected=RwSignal::new(0usize) label=crate::i18n::t("Selection")
                height=12.0 gap=3.0 font_size=15.0
                option=Callback::new(|item: SegmentItem| view! { <strong>{item.text}</strong> }.into_any())/>
        </Example>
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
