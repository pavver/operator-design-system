//! Modal confirmation dialog: ConfirmDialog, a preset of Dialog.

use crate::components::dialog::Dialog;
use crate::core::AsyncAction;
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("confirm_dialog.css");

/// Width of a confirmation dialog in DU.
const WIDTH: f32 = 75.0;

/// Modal confirmation dialog; the confirm action runs asynchronously and closes the dialog on success.
///
/// A [`Dialog`] with a message, fixed width, the `alertdialog` role, and Cancel and confirm buttons.
#[component]
pub fn ConfirmDialog(
    open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] message: Signal<String>,
    #[prop(default = "Delete".into(), into)] confirm_text: Signal<String>,
    #[prop(default = "Cancel".into(), into)] cancel_text: Signal<String>,
    #[prop(default = true)] destructive: bool,
    #[prop(into)] on_confirm: AsyncAction,
) -> impl IntoView {
    view! {
        <Dialog
            open=open
            title=title
            min_width=WIDTH
            max_width=WIDTH
            on_confirm=on_confirm
            confirm_text=confirm_text
            cancel_text=cancel_text
            destructive=destructive
            alert=true
        >
            <div class="op-dialog-message">{move || message.get()}</div>
        </Dialog>
    }
}
