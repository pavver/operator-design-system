//! Live examples for rating.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn rating() -> AnyView {
    let b = backend();
    let value = RwSignal::new(3u32);
    let commit_action =
        AsyncAction::new(move |v: u32| async move { b.call(format!("rating={v}")).await });
    view! {
        <Example
            title="Interactive"
            note="Inline control on page surface. Pointer hover previews the score; async save shows a spinner and result flash on the chosen star, with rollback on error."
            block=true
        >
            <Rating value=value on_commit=commit_action/>
            <p>{move || format!("Current rating: {}", value.get())}</p>
        </Example>
        <Example
            title="States"
            note="Pending spinner and result flash appear on the chosen star; focus ring highlights the focused star only."
            block=true
        >
            <Rating value=RwSignal::new(2) enabled=false/>
            <Rating value=RwSignal::new(4) readonly=true/>
            <Rating value=RwSignal::new(3) state=Some(ControlState::Pending)/>
            <Rating value=RwSignal::new(5) state=Some(ControlState::Success)/>
            <Rating value=RwSignal::new(1) state=Some(ControlState::Error)/>
        </Example>
        <Example title="Customization" block=true>
            <Rating value=RwSignal::new(2) max=7 size=12.0 gap=2.0 label=l("Quality", "Якість")/>
        </Example>
    }
    .into_any()
}
