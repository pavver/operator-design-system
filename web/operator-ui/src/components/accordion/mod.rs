//! Expandable sections: Accordion and Collapsible.

use crate::components::primitives::Icon;
use crate::core::{Metric, Slot, root_class, root_style, spawn_ripple, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::icons;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

pub(crate) const CSS: &str = include_str!("accordion.css");

#[derive(Clone, Copy)]
struct AccordionCtx {
    open: RwSignal<Option<u64>>,
}

static NEXT_SECTION: AtomicU64 = AtomicU64::new(1);

/// Group of [`Collapsible`] sections where only one is open at a time.
#[component]
pub fn Accordion(
    children: Children,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] header_height: MaybeProp<f32>,
    #[prop(optional, into)] padding_start: MaybeProp<f32>,
    #[prop(optional, into)] padding_end: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] content_padding_top: MaybeProp<f32>,
    #[prop(optional, into)] content_padding_end: MaybeProp<f32>,
    #[prop(optional, into)] content_padding_bottom: MaybeProp<f32>,
    #[prop(optional, into)] content_padding_start: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] subtitle_font_size: MaybeProp<f32>,
    #[prop(optional, into)] subtitle_line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    provide_context(AccordionCtx {
        open: RwSignal::new(None),
    });
    let metrics = vec![
        Metric::du("--op-accordion-header-height", header_height),
        Metric::du("--op-accordion-padding-start", padding_start),
        Metric::du("--op-accordion-padding-end", padding_end),
        Metric::du("--op-accordion-padding-y", padding_y),
        Metric::du("--op-accordion-gap", gap),
        Metric::du("--op-accordion-icon-size", icon_size),
        Metric::du("--op-accordion-content-padding-top", content_padding_top),
        Metric::du("--op-accordion-content-padding-end", content_padding_end),
        Metric::du(
            "--op-accordion-content-padding-bottom",
            content_padding_bottom,
        ),
        Metric::du(
            "--op-accordion-content-padding-start",
            content_padding_start,
        ),
        Metric::tu("--op-accordion-font-size", font_size),
        Metric::tu("--op-accordion-line-height", line_height),
        Metric::tu("--op-accordion-subtitle-font-size", subtitle_font_size),
        Metric::tu("--op-accordion-subtitle-line-height", subtitle_line_height),
    ];
    view! { <div class=root_class("op-accordion", class) style=root_style(metrics, style)>{header.run()}{actions.run()}{children()}{footer.run()}</div> }
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
    trailing: Slot,
    children: Children,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] expand_label: MaybeProp<String>,
    #[prop(optional, into)] collapse_label: MaybeProp<String>,
    #[prop(optional)] chevron_icon: Option<operator_ui_tokens::Icon>,
    #[prop(optional, into)] header_height: MaybeProp<f32>,
    #[prop(optional, into)] padding_start: MaybeProp<f32>,
    #[prop(optional, into)] padding_end: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] content_padding_top: MaybeProp<f32>,
    #[prop(optional, into)] content_padding_end: MaybeProp<f32>,
    #[prop(optional, into)] content_padding_bottom: MaybeProp<f32>,
    #[prop(optional, into)] content_padding_start: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] subtitle_font_size: MaybeProp<f32>,
    #[prop(optional, into)] subtitle_line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let expand_label = ui_text(expand_label, "accordion.expand", "Expand");
    let collapse_label = ui_text(collapse_label, "accordion.collapse", "Collapse");
    let chevron_icon = ui_icon(chevron_icon, "accordion.chevron", icons::CHEVRON_RIGHT);
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
    let metrics = vec![
        Metric::du("--op-accordion-header-height", header_height),
        Metric::du("--op-accordion-padding-start", padding_start),
        Metric::du("--op-accordion-padding-end", padding_end),
        Metric::du("--op-accordion-padding-y", padding_y),
        Metric::du("--op-accordion-gap", gap),
        Metric::du("--op-accordion-icon-size", icon_size),
        Metric::du("--op-accordion-content-padding-top", content_padding_top),
        Metric::du("--op-accordion-content-padding-end", content_padding_end),
        Metric::du(
            "--op-accordion-content-padding-bottom",
            content_padding_bottom,
        ),
        Metric::du(
            "--op-accordion-content-padding-start",
            content_padding_start,
        ),
        Metric::tu("--op-accordion-font-size", font_size),
        Metric::tu("--op-accordion-line-height", line_height),
        Metric::tu("--op-accordion-subtitle-font-size", subtitle_font_size),
        Metric::tu("--op-accordion-subtitle-line-height", subtitle_line_height),
    ];
    view! {
        <div class=root_class("op-collapse", class) style=root_style(metrics, style) data-open=move || is_open.get().then_some("")>
            <button
                type="button"
                class="op-rippled op-collapse-head"
                aria-expanded=move || is_open.get().to_string()
                aria-controls=panel_id.clone()
                aria-description=move || if is_open.get() { collapse_label.get() } else { expand_label.get() }
                on:pointerdown=move |ev| spawn_ripple(&ev)
                on:click=move |_| toggle()
            >
                <span class="op-collapse-chevron">{move || view! { <Icon icon=chevron_icon.get() style="--op-icon-size:var(--op-accordion-icon-size, var(--op-dim-icon-small))"/> }}</span>
                {if header.is_set() { header.run().into_any() } else { view! {
                <span class="op-collapse-titles">
                    <span class="op-collapse-title">{move || title.get()}</span>
                    {subtitle.map(|s| view! { <span class="op-collapse-subtitle">{move || s.get()}</span> })}
                </span>
                }.into_any() }}
                {trailing.is_set().then(|| view! { <span class="op-collapse-trailing">{trailing.run()}</span> })}
            </button>
            {actions.run()}
            <div class="op-collapse-panel" id=panel_id role="region" inert=move || (!is_open.get()).then_some("")>
                <div class="op-collapse-clip">
                    <div class="op-collapse-content">{children()}{footer.run()}</div>
                </div>
            </div>
        </div>
    }
}
