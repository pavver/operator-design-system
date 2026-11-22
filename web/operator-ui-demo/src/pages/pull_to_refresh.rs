//! Live examples for pull-to-refresh.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn pull_to_refresh() -> AnyView {
    let last_refreshed = RwSignal::new(String::from("Initial state"));

    let on_refresh = AsyncAction::new(move |()| {
        Box::pin(async move {
            sleep_ms(1000).await;
            last_refreshed.set(format!("Refreshed at {}", "now"));
            ActionResult::Success
        })
    });

    view! {
        <Example title="Pull to Refresh" note="Touch gesture that triggers an async refresh with visual indicators." block=true>
            <div style="max-width: 500px; width: 100%; border: var(--op-dim-border) solid var(--op-border);">
                <PullToRefresh on_refresh=on_refresh>
                    <div style="padding: 24px; text-align: center; min-height: 180px; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 8px;">
                        <span style="font-weight: 700;">"Swipe or drag down from the top"</span>
                        <span style="color: var(--op-text-secondary); font-size: var(--op-type-caption-size);">
                            {move || last_refreshed.get()}
                        </span>
                    </div>
                </PullToRefresh>
            </div>
        </Example>
    }
    .into_any()
}
