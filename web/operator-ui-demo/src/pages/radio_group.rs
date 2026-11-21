//! Live examples for radio group.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn radio_group() -> AnyView {
    let b = backend();
    let mode = RwSignal::new("auto".to_string());
    let codec = RwSignal::new("h265".to_string());
    view! {
        <Example title="Customization" note="Metric props and slots customize this instance; its translated string follows the language selector.">
            <RadioGroup items=vec![GroupItem::new("a", "Option A"), GroupItem::new("b", "Option B")]
                selected=RwSignal::new("a".to_string()) label=crate::i18n::t("Selection")
                height=12.0 size=6.0 gap=3.0
                option=Callback::new(|item: GroupItem| view! { <strong>{item.name}</strong> }.into_any())/>
        </Example>
        <Example title="Async commit" note="The mark moves only after the request succeeds; the target shows a spinner while pending.">
            <RadioGroup
                label="Day/night mode"
                items=vec![GroupItem::new("auto", "Automatic"), GroupItem::new("day", "Always day"), GroupItem::new("night", "Always night")]
                selected=mode
                on_change=AsyncAction::new(move |v: String| b.call(format!("mode={v}")))
            />
        </Example>
        <Example title="Local, horizontal">
            <RadioGroup
                label="Codec"
                horizontal=true
                items=vec![GroupItem::new("h264", "H.264"), GroupItem::new("h265", "H.265"), GroupItem::new("mjpeg", "MJPEG")]
                selected=codec
            />
        </Example>
        <Example title="Disabled">
            <RadioGroup
                items=vec![GroupItem::new("a", "Option A"), GroupItem::new("b", "Option B")]
                selected=RwSignal::new("a".to_string())
                enabled=false
            />
        </Example>
    }
    .into_any()
}
