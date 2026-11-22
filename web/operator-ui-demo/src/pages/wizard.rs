//! Live examples for wizard.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn wizard() -> AnyView {
    let current = RwSignal::new(0usize);
    let name = RwSignal::new(String::new());

    let steps = vec![
        WizardStep::new("account", "Account").subtitle("Operator handle"),
        WizardStep::new("security", "Security").subtitle("Credentials"),
        WizardStep::new("confirm", "Confirm").subtitle("Review & apply"),
    ];

    let validator = Callback::new(move |step_idx: usize| {
        if step_idx == 0 {
            if name.get().trim().is_empty() {
                Err("Operator handle cannot be empty to proceed".to_string())
            } else {
                Ok(())
            }
        } else {
            Ok(())
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
            <div style="max-width: calc(var(--op-du) * 150); width: 100%; border: var(--op-dim-border) solid var(--op-border);">
                <Wizard
                    steps=steps
                    current=current
                    on_validate_step=validator
                    on_finish=on_finish
                >
                    <div style="padding: calc(var(--op-du) * 4) 0; min-height: calc(var(--op-du) * 30);">
                        {move || match current.get() {
                            0 => view! {
                                <div style="display: flex; flex-direction: column; gap: calc(var(--op-du) * 2);">
                                    <TextField
                                        label="Operator handle (required to proceed)"
                                        placeholder="operator-01"
                                        value=name
                                    />
                                </div>
                            }.into_any(),
                            1 => view! {
                                <div style="display: flex; flex-direction: column; gap: calc(var(--op-du) * 2);">
                                    <span style="font-weight: 500;">"Step 2: Security Keys"</span>
                                    <p style="color: var(--op-text-secondary); font-size: var(--op-type-caption-size);">
                                        "SSH ed25519 public key successfully verified against hardware security module."
                                    </p>
                                </div>
                            }.into_any(),
                            _ => view! {
                                <div style="display: flex; flex-direction: column; gap: calc(var(--op-du) * 2);">
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
