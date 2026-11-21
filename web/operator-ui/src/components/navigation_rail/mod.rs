//! NavigationRail: a vertical bar of top-level destinations.

use crate::components::primitives::Icon;
use crate::core::{Metric, ScopedSlot, Slot, root_class, root_style, spawn_ripple, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim};

pub(crate) const CSS: &str = include_str!("navigation_rail.css");

/// One destination of a [`NavigationRail`].
#[derive(Clone, Debug, PartialEq)]
pub struct RailItem {
    pub id: String,
    pub label: String,
    pub icon: IconData,
    /// Colour of the icon tile, for categories with their own colour (any CSS colour).
    pub color: Option<String>,
}

impl RailItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, icon: IconData) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon,
            color: None,
        }
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// Vertical bar of destinations with an icon over a short label; the selected one is marked.
///
/// Arrow Up and Down move between destinations, as in a tab list.
#[component]
pub fn NavigationRail(
    #[prop(into)] items: Signal<Vec<RailItem>>,
    selected: RwSignal<String>,
    #[prop(optional)] on_select: Option<Callback<String>>,
    #[prop(optional, into)] label: MaybeProp<String>,
    /// Lays the destinations out in a row of top tabs (narrow layouts).
    #[prop(optional, into)]
    horizontal: Signal<bool>,
    /// Rail width in DU (`--op-navigation-rail-width`).
    #[prop(optional, into)]
    width: MaybeProp<f32>,
    /// Destination item height in DU (`--op-navigation-rail-item-height`).
    #[prop(optional, into)]
    item_height: MaybeProp<f32>,
    /// Horizontal padding of destination items in DU (`--op-navigation-rail-item-padding-x`).
    #[prop(optional, into)]
    item_padding_x: MaybeProp<f32>,
    /// Gap between icon tile and label in DU (`--op-navigation-rail-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Size of the icon square tile in DU (`--op-navigation-rail-icon-tile-size`).
    #[prop(optional, into)]
    icon_tile_size: MaybeProp<f32>,
    /// Size of the icon glyph in DU (`--op-navigation-rail-icon-size`).
    #[prop(optional, into)]
    icon_size: MaybeProp<f32>,
    /// Selection indicator bar thickness in DU (`--op-navigation-rail-indicator-thickness`).
    #[prop(optional, into)]
    indicator_thickness: MaybeProp<f32>,
    /// Font size of labels in TU (`--op-navigation-rail-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    /// Slot above the destinations list.
    #[prop(optional, into)]
    header: Slot,
    /// Slot below the destinations list.
    #[prop(optional, into)]
    footer: Slot,
    /// Custom rendering for each destination item.
    #[prop(optional)]
    option: Option<ScopedSlot<RailItem>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let label = ui_text(label, "navigation-rail.label", "Navigation");
    let root_metrics = vec![
        Metric::du("--op-navigation-rail-width", width),
        Metric::du("--op-navigation-rail-item-height", item_height),
        Metric::du("--op-navigation-rail-item-padding-x", item_padding_x),
        Metric::du("--op-navigation-rail-gap", gap),
        Metric::du("--op-navigation-rail-icon-tile-size", icon_tile_size),
        Metric::du("--op-navigation-rail-icon-size", icon_size),
        Metric::du(
            "--op-navigation-rail-indicator-thickness",
            indicator_thickness,
        ),
        Metric::tu("--op-navigation-rail-font-size", font_size),
    ];
    let icon_size_val = Signal::derive(move || icon_size.get().unwrap_or(dim::ICON_MEDIUM));

    let pick = move |id: String| {
        selected.set(id.clone());
        if let Some(cb) = on_select {
            cb.run(id);
        }
    };
    let step = move |delta: isize| {
        let ids: Vec<String> = items.with_untracked(|i| i.iter().map(|r| r.id.clone()).collect());
        if ids.is_empty() {
            return;
        }
        let n = ids.len().cast_signed();
        let at = selected
            .with_untracked(|s| ids.iter().position(|i| i == s))
            .map_or(0, usize::cast_signed);
        let next = (at + delta).rem_euclid(n).cast_unsigned();
        if let Some(id) = ids.get(next) {
            pick(id.clone());
        }
    };
    view! {
        <nav
            class=root_class("op-rail", class)
            style=root_style(root_metrics, style)
            aria-label=move || label.get()
            data-horizontal=move || horizontal.get().then_some("")
        >
            {header.run()}
            <div
                class="op-rail-list"
                role="tablist"
                aria-orientation=move || if horizontal.get() { "horizontal" } else { "vertical" }
                on:keydown=move |e| match (e.key().as_str(), horizontal.get_untracked()) {
                    ("ArrowDown", false) | ("ArrowRight", true) => {
                        e.prevent_default();
                        step(1);
                    }
                    ("ArrowUp", false) | ("ArrowLeft", true) => {
                        e.prevent_default();
                        step(-1);
                    }
                    _ => {}
                }
            >
                <For each=move || items.get() key=|item| item.id.clone() let:item>
                    {
                        let id = item.id.clone();
                        let current = item.id.clone();
                        let is_selected = Signal::derive(move || selected.with(|s| *s == current));
                        let item_view = match &option {
                            Some(slot) => slot.run(item.clone()),
                            None => view! {
                                <span class="op-rail-icon"><Icon icon=item.icon size=icon_size_val/></span>
                                <span class="op-rail-label op-t-caption">{item.label}</span>
                            }
                            .into_any(),
                        };
                        view! {
                            <button
                                type="button"
                                role="tab"
                                class="op-rippled op-rail-item"
                                aria-selected=move || is_selected.get().to_string()
                                tabindex=move || if is_selected.get() { "0" } else { "-1" }
                                style=item.color.map(|c| format!("--op-rail-color: {c}"))
                                on:pointerdown=move |ev| spawn_ripple(&ev)
                                on:click=move |_| pick(id.clone())
                            >
                                {item_view}
                            </button>
                        }
                    }
                </For>
            </div>
            {footer.run()}
        </nav>
    }
}
