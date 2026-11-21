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
        <Example title="Interactive" block=true>
            <Rating value=value on_commit=commit_action/>
            <p>{move || value.get()}</p>
        </Example>
        <Example title="States" block=true>
            <Rating value=RwSignal::new(2) enabled=false/>
            <Rating value=RwSignal::new(4) readonly=true/>
            <Rating value=RwSignal::new(3) state=Some(ControlState::Pending)/>
        </Example>
        <Example title="Customization" block=true>
            <Rating value=RwSignal::new(2) max=7 size=12.0 gap=2.0 label=l("Quality", "Якість")/>
        </Example>
    }
    .into_any()
}
