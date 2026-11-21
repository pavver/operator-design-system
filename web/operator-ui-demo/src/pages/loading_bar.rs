//! Live examples for LoadingBar.

use crate::kit::Example;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui::prelude::*;

pub fn loading_bar() -> AnyView {
    let pending = RwSignal::new(false);
    let trigger = Callback::new(move |()| {
        pending.set(true);
        spawn_local(async move {
            sleep_ms(2500).await;
            pending.set(false);
        });
    });

    view! {
        <Example
            title="Page or container loading bar"
            note="Thin top indicator showing background page activity."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <div>
                    <ActionButton
                        text="Simulate Page Fetch"
                        on_click=AsyncAction::from(move |()| async move {
                            trigger.run(());
                            ActionResult::Success
                        })
                    />
                </div>
                <div style="position:relative;height:calc(var(--op-du) * 20);border:calc(var(--op-du) * 0.25) solid var(--op-border);padding:calc(var(--op-du) * 3);background-color:var(--op-surface);">
                    <LoadingBar pending=pending />
                    <p>"Card content with simulated network requests driving the top bar."</p>
                </div>
            </div>
        </Example>
    }
    .into_any()
}
