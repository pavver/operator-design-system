//! TabPanels: animated panels synchronized with a Tabs selection signal.

use crate::core::{Metric, root_class, root_style};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("tab_panels.css");

/// Context shared between [`TabPanels`] and child [`TabPanel`] items.
#[derive(Clone, Copy)]
pub struct TabPanelsContext {
    pub value: Signal<usize>,
    pub animated: Signal<bool>,
}

/// Container for animated tab panels driven by a selected tab index.
#[component]
pub fn TabPanels(
    #[prop(into)] value: Signal<usize>,
    #[prop(default = true.into(), into)] animated: Signal<bool>,
    /// Padding in DU (`--op-tab-panels-padding`).
    #[prop(optional, into)]
    padding: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    provide_context(TabPanelsContext { value, animated });

    let metrics = vec![Metric::du("--op-tab-panels-padding", padding)];
    let class_fn = root_class("op-tab-panels", class);
    let style_fn = root_style(metrics, style);

    view! {
        <div
            class=class_fn
            style=style_fn
            data-animated=move || animated.get().then_some("")
        >
            {children()}
        </div>
    }
}

/// A single panel inside [`TabPanels`], visible when its index matches the selected tab.
#[component]
pub fn TabPanel(
    index: usize,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<TabPanelsContext>();
    let class_fn = root_class("op-tab-panel", class);
    let style_fn = root_style(vec![], style);

    let is_active = move || ctx.map(|c| c.value.get() == index).unwrap_or(false);

    view! {
        <div
            class=class_fn
            style=style_fn
            role="tabpanel"
            tabindex="0"
            data-hidden=move || (!is_active()).then_some("")
        >
            {children()}
        </div>
    }
}
