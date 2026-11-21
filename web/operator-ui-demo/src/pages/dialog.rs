//! Live examples for dialog.

use crate::kit::{Example, backend, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn dialog() -> AnyView {
    let b = backend();
    let open = RwSignal::new(false);
    let name = RwSignal::new("Gate North".to_string());
    let host = RwSignal::new("192.168.1.40".to_string());
    let save = AsyncAction::new(move |()| async move {
        b.call(format!("save camera {}", name.get_untracked()))
            .await
    });
    view! {
        <Example title="Form dialog" note="Save runs inside the dialog: success closes it, error keeps it open. Width follows clamp(75 DU, 90% of the viewport, 140 DU).">
            <ActionButton text="Edit camera…" icon=icons::EDIT on_click=instant(move || open.set(true))/>
            <Dialog
                open=open
                title="Edit camera"
                confirm_text="Save"
                on_confirm=save
            >
                <TextField value=name placeholder="Name" label="Name"/>
                <TextField value=host placeholder="Host" label="Host"/>
                <Banner tone=Tone::Info text="The camera reconnects after the change."/>
            </Dialog>
        </Example>
    }
    .into_any()
}
