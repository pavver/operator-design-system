//! Live examples for form.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn form() -> AnyView {
    let b = backend();
    let name = RwSignal::new(String::new());
    let name_error = RwSignal::new(None::<String>);
    let name_rules = vec![
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

    let unavailable = RwSignal::new(String::new());
    let unavailable_error = RwSignal::new(None::<String>);
    let note = RwSignal::new(String::new());
    let note_error = RwSignal::new(None::<String>);

    view! {
        <Example title="Interactive" block=true>
            <Form on_submit=submit_action>
                <FormField value=name rules=name_rules error=name_error>
                    <TextField
                        value=name
                        label=l("Name", "Ім’я")
                        placeholder=l("Enter your name", "Введіть ваше ім’я")
                        error=Signal::derive(move || name_error.get())
                    />
                </FormField>
            </Form>
        </Example>
        <Example title="States" block=true>
            <Form enabled=false>
                <FormField value=unavailable error=unavailable_error rules=vec![]>
                    <TextField
                        value=unavailable
                        label=l("Unavailable", "Недоступно")
                        enabled=false
                    />
                </FormField>
            </Form>
        </Example>
        <Example title="Customization" block=true>
            <Form
                label=l("Profile", "Профіль")
                gap=4.0
                padding=5.0
                submit_label=l("Save profile", "Зберегти профіль")
            >
                <FormField value=note error=note_error rules=vec![]>
                    <TextField
                        value=note
                        label=l("Optional note", "Додаткова примітка")
                    />
                </FormField>
            </Form>
        </Example>
    }
    .into_any()
}
