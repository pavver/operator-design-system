//! Modal confirmation dialog: ConfirmDialog, a preset of Dialog.

use crate::components::dialog::Dialog;
use crate::core::{AsyncAction, Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("confirm_dialog.css");

/// Modal confirmation dialog; the confirm action runs asynchronously and closes the dialog on success.
///
/// A [`Dialog`] with a message, fixed width, the `alertdialog` role, and Cancel and confirm buttons.
#[component]
pub fn ConfirmDialog(
    open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] message: Signal<String>,
    #[prop(optional, into)] confirm_text: MaybeProp<String>,
    #[prop(optional, into)] cancel_text: MaybeProp<String>,
    #[prop(default = true)] destructive: bool,
    #[prop(into)] on_confirm: AsyncAction,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] header_padding: MaybeProp<f32>,
    #[prop(optional, into)] title_font_size: MaybeProp<f32>,
    #[prop(optional, into)] title_line_height: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] button_height: MaybeProp<f32>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
) -> impl IntoView {
    let confirm_text = ui_text(confirm_text, "confirm-dialog.confirm", "Delete");
    let cancel_text = ui_text(cancel_text, "confirm-dialog.cancel", "Cancel");
    let class = Signal::derive(root_class("op-confirm-dialog", class));
    let style = Signal::derive(root_style(
        vec![
            Metric::du("--op-confirm-dialog-max-height", max_height),
            Metric::tu("--op-confirm-dialog-font-size", font_size),
            Metric::tu("--op-confirm-dialog-line-height", line_height),
            Metric::du("--op-confirm-dialog-padding", padding),
            Metric::du("--op-confirm-dialog-header-padding", header_padding),
            Metric::tu("--op-confirm-dialog-title-font-size", title_font_size),
            Metric::tu("--op-confirm-dialog-title-line-height", title_line_height),
            Metric::du("--op-confirm-dialog-gap", gap),
            Metric::du("--op-confirm-dialog-button-height", button_height),
            Metric::du("--op-confirm-dialog-width", width),
        ],
        style,
    ));
    view! {
        <Dialog
            open=open
            title=title
            class=class
            style=style
            header=header
            actions=actions
            footer=footer
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
