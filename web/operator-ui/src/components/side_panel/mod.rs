//! SidePanel: a titled panel at a screen edge that collapses to a narrow strip.

use crate::components::icon_button::IconButton;
use crate::core::du;
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};

pub(crate) const CSS: &str = include_str!("side_panel.css");

/// Screen edge a [`SidePanel`] is attached to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PanelSide {
    #[default]
    Start,
    End,
}

/// Panel with a header; the header button collapses it to a strip that keeps only the expand button.
#[component]
pub fn SidePanel(
    #[prop(into)] title: Signal<String>,
    collapsed: RwSignal<bool>,
    #[prop(optional)] side: PanelSide,
    /// Expanded width in DU.
    #[prop(default = 75.0)]
    width: f32,
    /// Views at the end of the header (actions).
    #[prop(optional)]
    actions: Option<ChildrenFn>,
    children: ChildrenFn,
) -> impl IntoView {
    let (collapse_icon, expand_icon) = match side {
        PanelSide::Start => (icons::CHEVRON_LEFT, icons::CHEVRON_RIGHT),
        PanelSide::End => (icons::CHEVRON_RIGHT, icons::CHEVRON_LEFT),
    };
    let toggle = Callback::new(move |()| collapsed.update(|c| *c = !*c));
    let children = StoredValue::new(children);
    let actions = StoredValue::new(actions);
    view! {
        <aside
            class="op-side-panel"
            data-side=if side == PanelSide::End { "end" } else { "start" }
            data-collapsed=move || collapsed.get().then_some("")
            style:width=move || if collapsed.get() { du(dim::ROW_STANDARD) } else { du(width) }
            aria-label=move || title.get()
        >
            <Show
                when=move || !collapsed.get()
                fallback=move || view! {
                    <div class="op-side-panel-strip">
                        <IconButton icon=expand_icon label=title.get_untracked() size=dim::ROW_COMPACT on_click=toggle/>
                    </div>
                }
            >
                <div class="op-side-panel-head">
                    <span class="op-side-panel-title op-t-section-title">{move || title.get()}</span>
                    {actions.with_value(|a| a.as_ref().map(|a| a()))}
                    <IconButton icon=collapse_icon label="Collapse" size=dim::ROW_COMPACT on_click=toggle/>
                </div>
                <div class="op-side-panel-body">{children.with_value(|c| c())}</div>
            </Show>
        </aside>
    }
}
