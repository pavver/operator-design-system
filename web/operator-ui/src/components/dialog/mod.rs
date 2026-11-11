//! Modal dialog: Dialog, the shell shared by every modal window.

use crate::components::action_button::ActionButton;
use crate::components::icon_button::IconButton;
use crate::components::scrollbar::ScrollView;
use crate::core::{
    ActionResult, AsyncAction, ControlState, PendingRun, du, sleep_ms, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("dialog.css");

/// How long the dialog shows the result of the confirm action before it closes or unlocks.
const RESULT_MS: u32 = 500;
/// Delay between the end of an error result and unlocking the buttons.
const UNLOCK_MS: u32 = 300;

/// Modal window for a focused task: a form, details, or a choice.
///
/// Width follows the clamp model `clamp(min_width, 90% viewport, max_width)`.
///
/// With `on_confirm` the dialog owns its buttons: Cancel and the confirm button replace the close button.
/// The confirm action runs inside the dialog, its state shows on the dialog border,
/// dismissal is blocked while it runs, and the dialog closes itself after success.
#[component]
pub fn Dialog(
    open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    /// Extra buttons at the bottom, right-aligned, before Cancel and confirm.
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    /// Closes on Escape, scrim click, and the close button.
    #[prop(default = true.into(), into)]
    dismissible: Signal<bool>,
    #[prop(default = 75.0)] min_width: f32,
    #[prop(default = 140.0)] max_width: f32,
    /// Async confirm action; adds Cancel and confirm buttons.
    #[prop(optional, into)]
    on_confirm: Option<AsyncAction>,
    #[prop(default = "OK".into(), into)] confirm_text: Signal<String>,
    #[prop(default = "Cancel".into(), into)] cancel_text: Signal<String>,
    /// Shows the confirm button as dangerous.
    #[prop(optional)]
    destructive: bool,
    /// Uses the `alertdialog` role for a confirmation that interrupts the user.
    #[prop(optional)]
    alert: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let panel = NodeRef::<leptos::html::Div>::new();
    let children = StoredValue::new(children);
    // `confirming` locks the buttons and dismissal from the click until the unlock delay ends;
    // `running` covers only the request itself and drives the pending border.
    let confirming = RwSignal::new(false);
    let running = RwSignal::new(false);
    let result = RwSignal::new(ControlState::Idle);
    let border = Signal::derive(move || match result.get() {
        ControlState::Idle if running.get() => ControlState::Pending,
        s => s,
    });
    let visual = use_state_visual(border);
    let has_confirm = on_confirm.is_some();

    let dismiss = move || {
        if dismissible.get_untracked() && !confirming.get_untracked() {
            open.set(false);
        }
    };
    Effect::new(move |_| {
        if open.get() {
            confirming.set(false);
            running.set(false);
            result.set(ControlState::Idle);
            request_animation_frame(move || {
                if let Some(p) = panel.try_get_untracked().flatten() {
                    let _ = p.focus();
                }
            });
        }
    });

    let has_footer = actions.is_some() || has_confirm;
    // Rebuilt every time the dialog opens, so it captures cloneable parts only.
    let footer = move || {
        let extra = actions.clone().map(|a| a.run());
        let buttons = on_confirm.clone().map(|action| {
            let cancel = AsyncAction::new(move |()| async move {
                open.set(false);
                ActionResult::Success
            });
            let confirm = AsyncAction::new(move |()| {
                let action = action.clone();
                async move {
                    confirming.set(true);
                    running.set(true);
                    let outcome = action.call(()).await;
                    running.set(false);
                    result.set(outcome.clone().into());
                    let success = outcome.is_success();
                    spawn_local(async move {
                        sleep_ms(RESULT_MS).await;
                        if success {
                            open.set(false);
                        } else {
                            result.set(ControlState::Idle);
                            sleep_ms(UNLOCK_MS).await;
                            confirming.set(false);
                        }
                    });
                    outcome
                }
            });
            view! {
                <ActionButton
                    text=cancel_text
                    enabled=Signal::derive(move || !confirming.get())
                    on_click=cancel
                    class="op-dialog-btn"
                />
                <ActionButton text=confirm_text dangerous=destructive on_click=confirm class="op-dialog-btn"/>
            }
        });
        has_footer.then(|| view! { <div class="op-dialog-actions">{extra}{buttons}</div> })
    };

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
                    node_ref=panel
                    class="op-sb op-dialog"
                    role=if alert { "alertdialog" } else { "dialog" }
                    aria-modal="true"
                    aria-label=move || title.get()
                    tabindex="-1"
                    data-state=move || visual.state.get()
                    data-result=move || visual.result.get()
                    data-fading=move || visual.fading.get().then_some("")
                    style:width=format!(
                        "clamp(min({}, 100%), 90vw, {})",
                        du(min_width),
                        du(max_width),
                    )
                    on:click=|ev| ev.stop_propagation()
                >
                    <div class="op-dialog-head">
                        <div class="op-dialog-title">{move || title.get()}</div>
                        <Show when=move || dismissible.get() && !has_confirm>
                            <IconButton icon=icons::CLOSE label="Close" on_click=Callback::new(move |()| open.set(false))/>
                        </Show>
                    </div>
                    <ScrollView class="op-dialog-body" viewport_class="op-dialog-content op-scroll-contain">{move || children.with_value(|c| c())}</ScrollView>
                    {footer()}
                    <PendingRun/>
                </div>
            </div>
        </Show>
    }
}
