//! Vertical list of dated entries: Timeline and TimelineEntry.

use crate::components::primitives::Icon;
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::Icon as IconData;

pub(crate) const CSS: &str = include_str!("timeline.css");

/// Vertical sequence of dated events or milestones.
#[component]
pub fn Timeline(
    /// Dense layout with reduced spacing between entries.
    #[prop(optional)]
    dense: bool,
    /// Gap between entries in DU (`--op-timeline-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Connecting line width in DU (`--op-timeline-line-width`).
    #[prop(optional, into)]
    line_width: MaybeProp<f32>,
    /// Accessible label; defaults to `"Timeline"`.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let resolved_label = ui_text(label, "timeline.label", "Timeline");
    view! {
        <ol
            class=root_class("op-timeline", class)
            aria-label=move || resolved_label.get()
            data-dense=dense.then_some("")
            style=root_style(
                vec![
                    Metric::du("--op-timeline-gap", gap),
                    Metric::du("--op-timeline-line-width", line_width),
                ],
                style,
            )
        >
            {children()}
        </ol>
    }
}

/// One milestone or event entry in a [`Timeline`].
#[component]
pub fn TimelineEntry(
    /// Title of the milestone.
    #[prop(optional, into)]
    title: MaybeProp<String>,
    /// Date or timestamp string.
    #[prop(optional, into)]
    date: MaybeProp<String>,
    /// Semantic tone of the entry dot: `"default"`, `"pending"`, `"success"`, `"warning"`, `"error"`, `"accent"`.
    #[prop(optional, into)]
    tone: MaybeProp<String>,
    /// Icon inside the entry dot.
    #[prop(optional)]
    icon: Option<IconData>,
    /// Custom title slot.
    #[prop(optional, into)]
    title_slot: Slot,
    /// Custom date slot.
    #[prop(optional, into)]
    date_slot: Slot,
    /// Custom icon or node slot.
    #[prop(optional, into)]
    icon_slot: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let tone_val = move || tone.get().unwrap_or_else(|| "default".to_string());
    view! {
        <li
            class=root_class("op-timeline-entry", class)
            data-tone=tone_val
            style=root_style(vec![], style)
        >
            <div class="op-timeline-rail">
                <div
                    class="op-timeline-line"
                    aria-hidden="true"
                ></div>
                <div
                    class="op-timeline-dot"
                    aria-hidden="true"
                >
                    {if icon_slot.is_set() {
                        icon_slot.run()
                    } else {
                        icon.map(|ic| view! {
                            <Icon
                                icon=ic
                                size=4.0
                            />
                        }.into_any())
                    }}
                </div>
            </div>
            <div class="op-timeline-content">
                {(date_slot.is_set() || date.get().is_some()).then(|| view! {
                    <div class="op-timeline-date">
                        {if date_slot.is_set() {
                            date_slot.run()
                        } else {
                            date.get().map(|d| d.into_any())
                        }}
                    </div>
                })}
                {(title_slot.is_set() || title.get().is_some()).then(|| view! {
                    <div class="op-timeline-title">
                        {if title_slot.is_set() {
                            title_slot.run()
                        } else {
                            title.get().map(|t| t.into_any())
                        }}
                    </div>
                })}
                {children.map(|c| view! {
                    <div class="op-timeline-body">
                        {c()}
                    </div>
                })}
            </div>
        </li>
    }
}
