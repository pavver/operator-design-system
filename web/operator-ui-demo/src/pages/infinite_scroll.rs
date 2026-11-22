//! Live examples for infinite-scroll.

use crate::kit::Example;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui::prelude::*;

pub fn infinite_scroll() -> AnyView {
    let items = RwSignal::new(
        (1..=10)
            .map(|i| format!("Event log entry #{i}"))
            .collect::<Vec<_>>(),
    );
    let loading = RwSignal::new(false);
    let has_more = RwSignal::new(true);
    let error = RwSignal::new(Option::<String>::None);

    let load_more = Callback::new(move |()| {
        if loading.get() || !has_more.get() {
            return;
        }
        loading.set(true);
        spawn_local(async move {
            sleep_ms(800).await;
            let current_len = items.get().len();
            if current_len >= 30 {
                has_more.set(false);
            } else {
                items.update(|list| {
                    for i in 1..=5 {
                        list.push(format!("Event log entry #{}", current_len + i));
                    }
                });
            }
            loading.set(false);
        });
    });

    let on_retry = Callback::new(move |()| {
        error.set(None);
        load_more.run(());
    });

    view! {
        <Example title="Infinite Scroll" note="Loads additional items incrementally when the bottom sentinel enters the viewport." block=true>
            <div style="display: flex; flex-direction: column; gap: calc(var(--op-du) * 2); width: 100%; max-width: calc(var(--op-du) * 125);">
                <div>
                    <ActionButton
                        text="Simulate Fetch Error"
                        on_click=AsyncAction::new(move |()| {
                            error.set(Some("Connection timeout reaching log ingest node".to_string()));
                            Box::pin(async { ActionResult::Success })
                        })
                    />
                </div>
                <div style="height: calc(var(--op-du) * 75); width: 100%; overflow-y: auto; border: var(--op-dim-border) solid var(--op-border); padding: calc(var(--op-du) * 2);">
                    <InfiniteScroll
                        on_load=load_more
                        on_retry=on_retry
                        loading=loading
                        has_more=has_more
                        error=error
                    >
                        <div style="display: flex; flex-direction: column; gap: calc(var(--op-du) * 2);">
                            {move || items.get().into_iter().map(|item| view! {
                                <div style="padding: calc(var(--op-du) * 2.5); background: var(--op-surface-alt); border: var(--op-dim-border) solid var(--op-border); font-size: var(--op-type-caption-size);">
                                    {item}
                                </div>
                            }).collect_view()}
                        </div>
                    </InfiniteScroll>
                </div>
            </div>
        </Example>
    }
    .into_any()
}
