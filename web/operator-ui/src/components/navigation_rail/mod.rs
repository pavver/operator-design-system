//! NavigationRail: a vertical bar of top-level destinations.

use crate::components::primitives::Icon;
use crate::core::spawn_ripple;
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
    #[prop(default = "Navigation".into(), into)] label: String,
) -> impl IntoView {
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
        <nav class="op-rail" aria-label=label>
            <div
                class="op-rail-list"
                role="tablist"
                aria-orientation="vertical"
                on:keydown=move |e| match e.key().as_str() {
                    "ArrowDown" => {
                        e.prevent_default();
                        step(1);
                    }
                    "ArrowUp" => {
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
                                <span class="op-rail-icon"><Icon icon=item.icon size=dim::ICON_MEDIUM/></span>
                                <span class="op-rail-label op-t-caption">{item.label}</span>
                            </button>
                        }
                    }
                </For>
            </div>
        </nav>
    }
}
