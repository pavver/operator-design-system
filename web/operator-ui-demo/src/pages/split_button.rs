//! Live examples for split button.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn split_button() -> AnyView {
    let selected = RwSignal::new(String::new());
    let b = backend();
    view! {
        <Example title="Interactive" block=true>
            <SplitButton text="Save" items=vec![MenuItem::new("copy", "Save a copy"), MenuItem::new("export", "Export")] on_select=Callback::new(move |id| selected.set(id)) on_click=b.action("save")/>
            <p role="status">{move || selected.get()}</p>
        </Example>
        <Example title="States" block=true>
            <SplitButton text="Unavailable" enabled=false items=vec![] on_select=Callback::new(|_| {})/>
            <SplitButton text="Pending" state=Some(ControlState::Pending) items=vec![] on_select=Callback::new(|_| {})/>
        </Example>
        <Example title="Customization" block=true>
            <SplitButton text=l("Publish", "Опублікувати") height=12.0 toggle_size=12.0 icon_size=6.0 items=vec![MenuItem::new("draft", "Save draft")] on_select=Callback::new(move |id| selected.set(id)) on_click=b.action("publish")/>
        </Example>
    }.into_any()
}
