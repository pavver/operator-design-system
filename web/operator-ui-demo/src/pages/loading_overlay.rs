//! Live examples for LoadingOverlay.

use crate::kit::Example;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui::prelude::*;

pub fn loading_overlay() -> AnyView {
    let showing = RwSignal::new(false);
    let trigger = Callback::new(move |()| {
        showing.set(true);
        spawn_local(async move {
            sleep_ms(2000).await;
            showing.set(false);
        });
    });

    view! {
        <Example title="Covering container" note="Blocks inputs while operation is pending." block=true>
            <div class="demo-stack demo-narrow">
                <div>
                    <ActionButton text="Trigger 2s Operation" on_click=AsyncAction::from(move |()| async move {
                        trigger.run(());
                        ActionResult::Success
                    }) />
                </div>
                <div style="position:relative;padding:calc(var(--op-du) * 4);border:calc(var(--op-du) * 0.25) solid var(--op-border);background-color:var(--op-surface);">
                    <h4>"Container Subject to Loading"</h4>
                    <p style="margin-top:calc(var(--op-du) * 2);">
                        "This content and any inputs within are obscured and blocked while the overlay is showing."
                    </p>
                    <LoadingOverlay showing=showing label="Processing dataset..." />
                </div>
            </div>
        </Example>
    }
    .into_any()
}
