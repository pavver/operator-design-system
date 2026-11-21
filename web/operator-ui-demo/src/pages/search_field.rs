//! Live examples for search field.

use super::common::SEARCH_CAMERAS as CAMERAS;
use crate::i18n::l;
use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn search_field() -> AnyView {
    let b = backend();
    let query = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let results = RwSignal::new(CAMERAS.iter().map(|s| (*s).to_string()).collect::<Vec<_>>());
    let search = Callback::new(move |q: String| {
        pending.set(true);
        leptos::task::spawn_local(async move {
            sleep_ms(400).await;
            let q = q.to_lowercase();
            results.set(
                CAMERAS
                    .iter()
                    .filter(|c| c.to_lowercase().contains(&q))
                    .map(|s| (*s).to_string())
                    .collect(),
            );
            pending.set(false);
            b.log(format!("[UI] search \"{q}\""));
        });
    });
    view! {
        <Example title="Live search" note="The query is reported 300 ms after the last key press; the spinner shows while results load. Escape clears." block=true>
            <div class="demo-stack demo-narrow">
                <p class="demo-p">{l("Search cameras", "Пошук камер")}</p>
                <SearchField value=query placeholder=l("Search cameras", "Пошук камер") clear_label=l("Clear search", "Очистити пошук") on_search=search pending=pending/>
                <div class="demo-list">
                    {move || {
                        let list = results.get();
                        if list.is_empty() {
                            view! { <EmptyState icon=icons::SEARCH title="No cameras match" compact=true/> }.into_any()
                        } else {
                            list.into_iter().map(|c| view! { <ListItem title=c/> }).collect_view().into_any()
                        }
                    }}
                </div>
            </div>
        </Example>
    }
    .into_any()
}
