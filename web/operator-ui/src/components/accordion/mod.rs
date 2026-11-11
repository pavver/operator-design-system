//! Expandable sections: Accordion and Collapsible.

use crate::components::primitives::Icon;
use crate::core::spawn_ripple;
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

pub(crate) const CSS: &str = include_str!("accordion.css");

#[derive(Clone, Copy)]
struct AccordionCtx {
    open: RwSignal<Option<u64>>,
}

static NEXT_SECTION: AtomicU64 = AtomicU64::new(1);

/// Group of [`Collapsible`] sections where only one is open at a time.
#[component]
pub fn Accordion(children: Children) -> impl IntoView {
    provide_context(AccordionCtx {
        open: RwSignal::new(None),
    });
    view! { <div class="op-accordion">{children()}</div> }
}

/// Section with a header row that shows or hides its content.
#[component]
pub fn Collapsible(
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] subtitle: Option<Signal<String>>,
    /// External open state; inside an [`Accordion`] the group controls it.
    #[prop(optional, into)]
    open: Option<RwSignal<bool>>,
    #[prop(optional)] initially_open: bool,
    /// Extra content at the end of the header row, for example a counter.
    #[prop(optional, into)]
    trailing: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let id = NEXT_SECTION.fetch_add(1, AtomicOrdering::Relaxed);
    let group = use_context::<AccordionCtx>();
    let local = open.unwrap_or_else(|| RwSignal::new(initially_open));
    if let Some(g) = group.filter(|_| initially_open) {
        g.open.set(Some(id));
    }
    let is_open = Signal::derive(move || match group {
        Some(g) => g.open.get() == Some(id),
        None => local.get(),
    });
    let toggle = move || match group {
        Some(g) => g
            .open
            .update(|o| *o = if *o == Some(id) { None } else { Some(id) }),
        None => local.update(|o| *o = !*o),
    };
    let panel_id = format!("op-collapse-{id}");
    view! {
        <div class="op-collapse" data-open=move || is_open.get().then_some("")>
            <button
                type="button"
                class="op-rippled op-collapse-head"
                aria-expanded=move || is_open.get().to_string()
                aria-controls=panel_id.clone()
                on:pointerdown=move |ev| spawn_ripple(&ev)
                on:click=move |_| toggle()
            >
                <span class="op-collapse-chevron"><Icon icon=icons::CHEVRON_RIGHT size=dim::ICON_SMALL/></span>
                <span class="op-collapse-titles">
                    <span class="op-collapse-title">{move || title.get()}</span>
                    {subtitle.map(|s| view! { <span class="op-collapse-subtitle">{move || s.get()}</span> })}
                </span>
                {trailing.map(|t| view! { <span class="op-collapse-trailing">{t.run()}</span> })}
            </button>
            <div class="op-collapse-panel" id=panel_id role="region" inert=move || (!is_open.get()).then_some("")>
                <div class="op-collapse-clip">
                    <div class="op-collapse-content">{children()}</div>
                </div>
            </div>
        </div>
    }
}
