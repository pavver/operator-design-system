//! Live examples for wizard.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn wizard() -> AnyView {
    let current = RwSignal::new(0usize);
    let name = RwSignal::new(String::new());

    let steps = vec![
        WizardStep::new("account", "Account").subtitle("User details"),
        WizardStep::new("security", "Security").subtitle("Credentials"),
        WizardStep::new("confirm", "Confirm").subtitle("Review & apply"),
    ];

    let validator = Callback::new(move |step_idx: usize| {
        if step_idx == 0 {
            !name.get().trim().is_empty()
        } else {
            true
        }
    });

    let on_finish = AsyncAction::new(move |()| {
        Box::pin(async {
            sleep_ms(1000).await;
            ActionResult::Success
        })
    });

    view! {
        <Example title="Multi-step Wizard" note="Step header with validation per step, navigation buttons, and asynchronous finish with Operator states." block=true>
            <div style="max-width: 600px; width: 100%; border: var(--op-dim-border) solid var(--op-border);">
                <Wizard
                    steps=steps
                    current=current
                    on_validate_step=validator
                    on_finish=on_finish
                >
                    <div style="padding: 16px 0; min-height: 120px;">
                        {move || match current.get() {
                            0 => view! {
                                <div style="display: flex; flex-direction: column; gap: 8px;">
                                    <label style="font-size: var(--op-type-caption-size); color: var(--op-text-secondary);">
                                        "Enter operator handle (required to proceed):"
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="e.g. pavver"
                                        style="padding: 8px; background: var(--op-surface-alt); border: var(--op-dim-border) solid var(--op-border); color: var(--op-text-primary);"
                                        prop:value=move || name.get()
                                        on:input=move |ev| name.set(event_target_value(&ev))
                                    />
                                </div>
                            }.into_any(),
                            1 => view! {
                                <div style="display: flex; flex-direction: column; gap: 8px;">
                                    <span style="font-weight: 500;">"Step 2: Security Keys"</span>
                                    <p style="color: var(--op-text-secondary); font-size: var(--op-type-caption-size);">
                                        "SSH ed25519 public key successfully verified against hardware security module."
                                    </p>
                                </div>
                            }.into_any(),
                            _ => view! {
                                <div style="display: flex; flex-direction: column; gap: 8px;">
                                    <span style="font-weight: 500;">"Step 3: Confirmation"</span>
                                    <p style="color: var(--op-text-secondary); font-size: var(--op-type-caption-size);">
                                        "Click Finish to initialize operator session. The button will transition through running and success states."
                                    </p>
                                </div>
                            }.into_any(),
                        }}
                    </div>
                </Wizard>
            </div>
        </Example>
    }
    .into_any()
}
