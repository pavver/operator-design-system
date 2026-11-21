//! Live examples for text field.

use crate::i18n::l;
use crate::kit::{Cell, Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn text_field() -> AnyView {
    let b = backend();
    let name = RwSignal::new("Gate-North".to_string());
    let saved = RwSignal::new("Gate-North".to_string());
    view! {
        <Example title="Interactive" note="Edit the first value to reveal the async SAVE button. It hides after a successful save." block=true>
            <div class="demo-stack demo-narrow">
                <TextField
                    value=name
                    initial_value=saved
                    placeholder="Device name"
                    label="Device name"
                    on_save=AsyncAction::new(move |v: String| async move {
                        let r = b.call(format!("rename to {v}")).await;
                        if r.is_success() {
                            saved.set(v);
                        }
                        r
                    })
                />
                <TextField value=RwSignal::new(String::new()) placeholder="rtsp://host:554/stream" label="Stream URL"/>
                <TextField value=RwSignal::new("hunter2".to_string()) placeholder="Password" password=true label="Password"/>
                <TextField value=RwSignal::new("Read-only value".to_string()) placeholder="" enabled=false/>
            </div>
        </Example>
        <Example title="Static states">
            {[("Focused", ControlState::Focused), ("Pending", ControlState::Pending), ("Success", ControlState::Success), ("Error", ControlState::Error)]
                .into_iter()
                .map(|(n, s)| view! {
                    <Cell label=n>
                        <div class="demo-field-cell"><TextField value=RwSignal::new("value".to_string()) placeholder="" state=s/></div>
                    </Cell>
                })
                .collect_view()}
        </Example>
        <Example title="Customization" block=true>
            <TextField value=RwSignal::new(String::new()) label=l("Account", "Обліковий запис")
                placeholder=l("Enter a name", "Введіть ім’я") height=12.0 padding_x=4.0 gap=2.0 font_size=16.0
                hint=l("Shown above the input", "Показано над полем")
                prepend=|| view! { <Icon icon=icons::INFO/> }/>
        </Example>
    }
    .into_any()
}
