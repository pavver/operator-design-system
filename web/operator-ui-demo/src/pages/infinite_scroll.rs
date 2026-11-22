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

    view! {
        <Example title="Infinite Scroll" note="Loads additional items incrementally when the bottom sentinel enters the viewport." block=true>
            <div style="height: 300px; width: 100%; max-width: 500px; overflow-y: auto; border: var(--op-dim-border) solid var(--op-border); padding: 8px;">
                <InfiniteScroll
                    on_load=load_more
                    loading=loading
                    has_more=has_more
                    error=error
                >
                    <div style="display: flex; flex-direction: column; gap: 8px;">
                        {move || items.get().into_iter().map(|item| view! {
                            <div style="padding: 10px; background: var(--op-surface-alt); border: var(--op-dim-border) solid var(--op-border); font-size: var(--op-type-caption-size);">
                                {item}
                            </div>
                        }).collect_view()}
                    </div>
                </InfiniteScroll>
            </div>
        </Example>
    }
    .into_any()
}
