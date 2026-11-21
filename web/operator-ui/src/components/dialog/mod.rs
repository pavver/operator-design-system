//! Modal dialog: Dialog, the shell shared by every modal window.

use crate::components::action_button::ActionButton;
use crate::components::icon_button::IconButton;
use crate::components::scrollbar::ScrollView;
use crate::core::{
    ActionResult, AsyncAction, ControlState, Metric, PendingRun, Slot, root_class, root_style,
    sleep_ms, ui_icon, ui_text, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, icons};

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
    actions: Slot,
    /// Closes on Escape, scrim click, and the close button.
    #[prop(default = true.into(), into)]
    dismissible: Signal<bool>,

    /// Async confirm action; adds Cancel and confirm buttons.
    #[prop(optional, into)]
    on_confirm: Option<AsyncAction>,
    #[prop(optional, into)] confirm_text: MaybeProp<String>,
    #[prop(optional, into)] cancel_text: MaybeProp<String>,
    /// Shows the confirm button as dangerous.
    #[prop(optional)]
    destructive: bool,
    /// Uses the `alertdialog` role for a confirmation that interrupts the user.
    #[prop(optional)]
    alert: bool,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] header_padding: MaybeProp<f32>,
    #[prop(optional, into)] title_font_size: MaybeProp<f32>,
    #[prop(optional, into)] title_line_height: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] button_height: MaybeProp<f32>,
    #[prop(optional, into)] min_width: MaybeProp<f32>,
    #[prop(optional, into)] max_width: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] close_size: MaybeProp<f32>,
    #[prop(optional, into)] close_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] close_label: MaybeProp<String>,
    #[prop(optional)] close_icon: Option<IconData>,
    children: ChildrenFn,
) -> impl IntoView {
    let confirm_text = ui_text(confirm_text, "dialog.confirm", "OK");
    let cancel_text = ui_text(cancel_text, "dialog.cancel", "Cancel");
    let close_label = ui_text(close_label, "dialog.close", "Close");
    let close_icon = ui_icon(close_icon, "dialog.close", icons::CLOSE);
    let header = StoredValue::new(header);
    let footer = StoredValue::new(footer);
    let root_style = Signal::derive(root_style(
        vec![
            Metric::du("--op-dialog-max-height", max_height),
            Metric::du("--op-dialog-padding", padding),
            Metric::du("--op-dialog-header-padding", header_padding),
            Metric::tu("--op-dialog-title-font-size", title_font_size),
            Metric::tu("--op-dialog-title-line-height", title_line_height),
            Metric::du("--op-dialog-gap", gap),
            Metric::du("--op-dialog-button-height", button_height),
            Metric::du("--op-dialog-min-width", min_width),
            Metric::du("--op-dialog-max-width", max_width),
            Metric::tu("--op-dialog-font-size", font_size),
            Metric::tu("--op-dialog-line-height", line_height),
            Metric::du("--op-dialog-close-size", close_size),
            Metric::du("--op-dialog-close-icon-size", close_icon_size),
        ],
        style,
    ));
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
    crate::core::use_page_scroll_lock(open);
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

    let has_footer = actions.is_set() || has_confirm;
    // Rebuilt every time the dialog opens, so it captures cloneable parts only.
    let action_bar = move || {
        let extra = actions.run();
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
                class=root_class("op-dialog-scrim", class)
                style=move || root_style.get()
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
                    on:click=|ev| ev.stop_propagation()
                >
                    <div class="op-dialog-head">
                        <div class="op-dialog-title">{if header.with_value(Slot::is_set) { header.with_value(Slot::run).into_any() } else { (move || title.get()).into_any() }}</div>
                        <Show when=move || dismissible.get() && !has_confirm>
                            {move || view! { <IconButton icon=close_icon.get() label=close_label on_click=Callback::new(move |()| open.set(false))/> }}
                        </Show>
                    </div>
                    <ScrollView class="op-dialog-body" viewport_class="op-dialog-content op-scroll-contain">{move || children.with_value(|c| c())}</ScrollView>
                    {action_bar()}
                    {footer.with_value(Slot::is_set).then(|| view! { <div class="op-dialog-footer">{footer.with_value(Slot::run)}</div> })}
                    <PendingRun/>
                </div>
            </div>
        </Show>
    }
}
