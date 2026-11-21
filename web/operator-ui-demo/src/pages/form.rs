//! Live examples for form.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn form() -> AnyView {
    let b = backend();
    let name = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);
    let rules = vec![
        Callback::new(|v: String| {
            v.trim()
                .is_empty()
                .then(|| "A name is required".to_string())
        }),
        Callback::new(|v: String| {
            (v.len() < 3).then(|| "Use at least three characters".to_string())
        }),
    ];
    let submit_action = AsyncAction::new(move |()| {
        let name = name.get_untracked();
        async move { b.call(format!("submit {name}")).await }
    });
    view! {
        <Example title="Interactive" block=true>
            <Form on_submit=submit_action>
                <FormField value=name rules=rules error=error>
                    <TextField value=name label="Name"/>
                </FormField>
            </Form>
        </Example>
        <Example title="States" block=true>
            <Form enabled=false>
                <TextField value=RwSignal::new(String::new()) label="Unavailable"/>
            </Form>
        </Example>
        <Example title="Customization" block=true>
            <Form label=l("Profile", "Профіль") gap=4.0 padding=5.0 submit_label=l("Save profile", "Зберегти профіль")>
                <TextField value=RwSignal::new(String::new()) label="Optional note"/>
            </Form>
        </Example>
    }.into_any()
}
