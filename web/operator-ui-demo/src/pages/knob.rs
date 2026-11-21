//! Live examples for knob.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn knob() -> AnyView {
    let b = backend();
    let commit_action =
        AsyncAction::new(move |v: f32| async move { b.call(format!("volume={v}")).await });
    view! {
        <Example title="Interactive" block=true>
            <Knob value=RwSignal::new(40.0) label="Volume" on_commit=commit_action/>
        </Example>
        <Example title="States" block=true>
            <Knob value=RwSignal::new(40.0) enabled=false/>
            <Knob value=RwSignal::new(40.0) state=Some(ControlState::Pending)/>
            <Knob value=RwSignal::new(40.0) state=Some(ControlState::Error)/>
        </Example>
        <Example title="Customization" block=true>
            <Knob value=RwSignal::new(0.5) min=0.0 max=1.0 step=0.05 size=32.0 thickness=1.0 font_size=18.0 label=l("Gain", "Підсилення")/>
        </Example>
    }.into_any()
}
