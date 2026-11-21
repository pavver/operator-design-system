//! Live examples for popup edit.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn popup_edit() -> AnyView {
    let b = backend();
    let value = RwSignal::new("Click to rename".to_string());
    let save_action =
        AsyncAction::new(move |name: String| async move { b.call(format!("rename={name}")).await });
    view! {
        <Example title="Interactive" block=true>
            <PopupEdit value=value on_save=save_action validate=Callback::new(|v: String| v.trim().is_empty().then(|| "A name is required".to_string()))/>
        </Example>
        <Example title="States" block=true>
            <PopupEdit value=RwSignal::new("Unavailable".to_string()) enabled=false/>
        </Example>
        <Example title="Customization" block=true>
            <PopupEdit value=RwSignal::new("Custom editor".to_string()) label=l("Edit name", "Змінити назву") width=88.0 padding=5.0 gap=3.0/>
        </Example>
    }.into_any()
}
