//! Thin pending progress bar at the edge of a container: LoadingBar.

use crate::core::{Metric, root_class, root_style, ui_text};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("loading_bar.css");

/// Where [`LoadingBar`] attaches to its parent container.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LoadingBarPosition {
    #[default]
    Top,
    Bottom,
}

impl LoadingBarPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// Thin progress bar indicating ongoing page or data loading activity.
#[component]
pub fn LoadingBar(
    /// Whether background activity is currently pending.
    #[prop(into)]
    pending: Signal<bool>,
    /// Position along the top or bottom edge of the parent container.
    #[prop(default = LoadingBarPosition::Top)]
    position: LoadingBarPosition,
    /// Bar thickness in DU (`--op-loading-bar-thickness`); defaults to `0.5 DU`.
    #[prop(optional, into)]
    thickness: MaybeProp<f32>,
    /// Accessible description; defaults to `"Loading page"`.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_label = ui_text(label, "loading-bar.loading", "Loading page");
    view! {
        <div
            class=root_class("op-loading-bar", class)
            role="progressbar"
            aria-label=move || resolved_label.get()
            aria-busy=move || pending.get().then_some("true")
            data-pending=move || pending.get().then_some("")
            data-position=position.as_str()
            style=root_style(vec![Metric::du("--op-loading-bar-thickness", thickness)], style)
        >
            <div class="op-loading-bar-indicator"></div>
        </div>
    }
}
