//! Live examples for split button.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn split_button() -> AnyView {
    let b = backend();
    let save_actions = move || {
        vec![
            ActionStackItem::new("Save a copy", b.action("save a copy")).icon(icons::CONTENT_COPY),
            ActionStackItem::new("Export", b.action("export")).icon(icons::DOWNLOAD),
        ]
    };
    view! {
        <Example title="Interactive" block=true>
            <SplitButton text="Save" actions=save_actions() on_click=b.action("save")/>
        </Example>
        <Example title="Custom toggle icon" block=true>
            <SplitButton text="Share" toggle_icon=icons::MORE_VERT side=LayerSide::Above align=LayerAlign::End actions=save_actions() on_click=b.action("share")/>
        </Example>
        <Example title="States" block=true>
            <SplitButton text="Unavailable" enabled=false actions=vec![]/>
            <SplitButton text="Pending" state=Some(ControlState::Pending) actions=vec![]/>
        </Example>
        <Example title="Customization" block=true>
            <SplitButton text=l("Publish", "Опублікувати") height=12.0 toggle_size=12.0 icon_size=6.0 actions=vec![ActionStackItem::new("Save draft", b.action("save draft"))] on_click=b.action("publish")/>
        </Example>
    }.into_any()
}
