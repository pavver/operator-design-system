//! Live examples for confirm dialog.

use crate::i18n::l;
use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn confirm_dialog() -> AnyView {
    let b = backend();
    let delete_open = RwSignal::new(false);
    let reboot_open = RwSignal::new(false);
    let opener = move |s: RwSignal<bool>| {
        AsyncAction::new(move |()| async move {
            s.set(true);
            ActionResult::Success
        })
    };
    let custom_open = RwSignal::new(false);
    view! {
        <Example title="Destructive and neutral" note="Confirm runs async: success closes after 500 ms, error pulses and unlocks the dialog.">
            <ActionButton text="Delete recording…" dangerous=true on_click=opener(delete_open)/>
            <ActionButton text="Reboot device…" on_click=opener(reboot_open)/>
            <ConfirmDialog
                open=delete_open
                title="Delete recording?"
                message="The clip from 14:02 will be removed from the device and cannot be restored."
                on_confirm=b.action("delete recording")
            />
            <ConfirmDialog
                open=reboot_open
                title="Reboot device?"
                message="The stream will be unavailable for about 40 seconds."
                confirm_text="Reboot"
                destructive=false
                on_confirm=b.action("reboot device")
            />
        </Example>
        <Example title="Customization">
            <ActionButton text=l("Confirm action", "Підтвердити дію") on_click=opener(custom_open)/>
            <ConfirmDialog open=custom_open title=l("Apply settings?", "Застосувати налаштування?")
                message=l("The device will reconnect.", "Пристрій під’єднається повторно.")
                confirm_text=l("Apply", "Застосувати") cancel_text=l("Keep editing", "Редагувати далі")
                width=90.0 padding=5.0 gap=4.0 destructive=false on_confirm=b.action("apply settings")
                header=move || view! { <strong>{l("Apply settings?", "Застосувати налаштування?")}</strong> }
                footer=move || view! { <small>{l("Your current settings stay available.", "Поточні налаштування залишаться доступними.")}</small> }
                style="--op-confirm-dialog-background: var(--op-surface-alt);"
            />
        </Example>
    }
    .into_any()
}
