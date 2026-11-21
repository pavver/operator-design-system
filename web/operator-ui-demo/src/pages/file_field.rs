//! Live examples for file field.
use crate::i18n::l;
use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn file_field() -> AnyView {
    let files = RwSignal::new(Vec::<PickedFile>::new());
    let rejected = RwSignal::new(String::new());
    let on_rejected = Callback::new(move |names: Vec<String>| {
        rejected.set(format!("Rejected: {}", names.join(", ")));
    });
    view! {
        <Example title="Interactive" block=true>
            <FileField value=files label="Attachments" accept=".txt,image/*" on_rejected=on_rejected/>
            <p role="status">{move || rejected.get()}</p>
        </Example>
        <Example title="States" block=true>
            <FileField value=RwSignal::new(vec![]) enabled=false/>
            <FileField value=RwSignal::new(vec![]) state=Some(ControlState::Pending)/>
        </Example>
        <Example title="Customization" block=true>
            <FileField value=RwSignal::new(vec![]) label=l("Documents", "Документи") height=12.0 chip_gap=2.0 max_files=3 max_size=1048576.0 hint="Up to three files, each at most 1 MiB"/>
        </Example>
    }.into_any()
}
