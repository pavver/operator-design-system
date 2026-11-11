//! Modal confirmation dialog: ConfirmDialog.

use crate::components::action_button::ActionButton;
use crate::core::{
    ActionResult, AsyncAction, ControlState, PendingRun, sleep_ms, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;

pub(crate) const CSS: &str = include_str!("confirm_dialog.css");

/// Modal confirmation dialog; the confirm action runs asynchronously and closes the dialog on success.
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
    let confirming = RwSignal::new(false);
    let dialog_state = RwSignal::new(ControlState::Idle);
    let border = Signal::derive(move || {
        let s = dialog_state.get();
        if s != ControlState::Idle {
            s
        } else if confirming.get() {
            ControlState::Pending
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(border);

    Effect::new(move |_| {
        if open.get() {
            confirming.set(false);
            dialog_state.set(ControlState::Idle);
        }
    });

    let dismiss = move || {
        if !confirming.get_untracked() {
            open.set(false);
        }
    };
    let cancel = AsyncAction::new(move |()| async move {
        open.set(false);
        ActionResult::Success
    });
    let confirm = AsyncAction::new(move |()| {
        let action = on_confirm.clone();
        async move {
            confirming.set(true);
            dialog_state.set(ControlState::Pending);
            let result = action.call(()).await;
            dialog_state.set(result.clone().into());
            let success = result.is_success();
            spawn_local(async move {
                sleep_ms(500).await;
                if success {
                    open.set(false);
                } else {
                    dialog_state.set(ControlState::Idle);
                    sleep_ms(300).await;
                    confirming.set(false);
                }
            });
            result
        }
    });

    view! {
        <Show when=move || open.get()>
            <div
                class="op-dialog-scrim"
                on:click=move |_| dismiss()
                on:keydown=move |ev| {
                    if ev.key() == "Escape" {
                        dismiss();
                    }
                }
            >
                <div
                    class="op-sb op-dialog"
                    role="alertdialog"
                    aria-modal="true"
                    data-state=move || visual.state.get()
                    data-result=move || visual.result.get()
                    data-fading=move || visual.fading.get().then_some("")
                    on:click=|ev| ev.stop_propagation()
                >
                    <div class="op-dialog-title">{move || title.get()}</div>
                    <div class="op-dialog-message">{move || message.get()}</div>
                    <div class="op-dialog-actions">
                        <ActionButton
                            text=cancel_text
                            enabled=Signal::derive(move || !confirming.get())
                            on_click=cancel.clone()
                            class="op-dialog-btn"
                        />
                        <ActionButton
                            text=confirm_text
                            dangerous=destructive
                            on_click=confirm.clone()
                            class="op-dialog-btn"
                        />
                    </div>
                    <PendingRun/>
                </div>
            </div>
        </Show>
    }
}
