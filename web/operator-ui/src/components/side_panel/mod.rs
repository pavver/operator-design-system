//! SidePanel: a titled panel at a screen edge that collapses to a narrow strip.

use crate::components::icon_button::IconButton;
use crate::core::{Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};

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
    /// Expanded width in DU (`--op-side-panel-width`).
    #[prop(optional, into)]
    width: MaybeProp<f32>,
    /// Collapsed width in DU (`--op-side-panel-collapsed-width`).
    #[prop(optional, into)]
    collapsed_width: MaybeProp<f32>,
    /// Header height in DU (`--op-side-panel-header-height`).
    #[prop(optional, into)]
    header_height: MaybeProp<f32>,
    /// Header start padding in DU (`--op-side-panel-header-padding-start`).
    #[prop(optional, into)]
    header_padding_start: MaybeProp<f32>,
    /// Header end padding in DU (`--op-side-panel-header-padding-end`).
    #[prop(optional, into)]
    header_padding_end: MaybeProp<f32>,
    /// Header items gap in DU (`--op-side-panel-header-gap`).
    #[prop(optional, into)]
    header_gap: MaybeProp<f32>,
    /// Strip top padding in DU (`--op-side-panel-strip-padding-top`).
    #[prop(optional, into)]
    strip_padding_top: MaybeProp<f32>,
    /// Title font size in TU (`--op-side-panel-title-font-size`).
    #[prop(optional, into)]
    title_font_size: MaybeProp<f32>,
    /// Slot to replace the header.
    #[prop(optional, into)]
    header: Slot,
    /// Views at the end of the header (actions).
    #[prop(optional, into)]
    actions: Slot,
    /// Slot below the panel body.
    #[prop(optional, into)]
    footer: Slot,
    /// Accessible label of the collapse button; defaults to `side-panel.collapse`.
    #[prop(optional, into)]
    collapse_label: MaybeProp<String>,
    /// Accessible label of the expand button; defaults to `side-panel.expand`.
    #[prop(optional, into)]
    expand_label: MaybeProp<String>,
    /// Icon for the collapse button.
    #[prop(optional)]
    collapse_icon: Option<IconData>,
    /// Icon for the expand button.
    #[prop(optional)]
    expand_icon: Option<IconData>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let (default_collapse, default_expand) = match side {
        PanelSide::Start => (icons::CHEVRON_LEFT, icons::CHEVRON_RIGHT),
        PanelSide::End => (icons::CHEVRON_RIGHT, icons::CHEVRON_LEFT),
    };
    let collapse_icon = ui_icon(collapse_icon, "side-panel.collapse", default_collapse);
    let expand_icon = ui_icon(expand_icon, "side-panel.expand", default_expand);
    let collapse_label = ui_text(collapse_label, "side-panel.collapse", "Collapse");
    let expand_label = ui_text(expand_label, "side-panel.expand", "Expand");

    let toggle = Callback::new(move |()| collapsed.update(|c| *c = !*c));
    let children = StoredValue::new(children);

    let root_metrics = vec![
        Metric::du("--op-side-panel-width", width),
        Metric::du("--op-side-panel-collapsed-width", collapsed_width),
        Metric::du("--op-side-panel-header-height", header_height),
        Metric::du("--op-side-panel-header-padding-start", header_padding_start),
        Metric::du("--op-side-panel-header-padding-end", header_padding_end),
        Metric::du("--op-side-panel-header-gap", header_gap),
        Metric::du("--op-side-panel-strip-padding-top", strip_padding_top),
        Metric::tu("--op-side-panel-title-font-size", title_font_size),
    ];
    let base_style = root_style(root_metrics, style);
    let full_style = move || {
        let mut out = if collapsed.get() {
            match collapsed_width.get() {
                Some(w) => format!("width:calc(var(--op-du) * {w});"),
                None => "width:var(--op-side-panel-collapsed-width,calc(var(--op-du) * 10));"
                    .to_string(),
            }
        } else {
            match width.get() {
                Some(w) => format!("width:calc(var(--op-du) * {w});"),
                None => "width:var(--op-side-panel-width,calc(var(--op-du) * 75));".to_string(),
            }
        };
        if let Some(s) = base_style() {
            out.push_str(&s);
        }
        Some(out)
    };

    view! {
        <aside
            class=root_class("op-side-panel", class)
            data-side=if side == PanelSide::End { "end" } else { "start" }
            data-collapsed=move || collapsed.get().then_some("")
            style=full_style
            aria-label=move || title.get()
        >
            <Show
                when=move || !collapsed.get()
                fallback=move || view! {
                    <div class="op-side-panel-strip">
                        <IconButton
                            icon=expand_icon.get()
                            label=expand_label
                            size=dim::ROW_COMPACT
                            on_click=toggle
                        />
                    </div>
                }
            >
                {if header.is_set() {
                    header.run()
                } else {
                    Some(view! {
                        <div class="op-side-panel-head">
                            <span class="op-side-panel-title op-t-section-title">{move || title.get()}</span>
                            {actions.run()}
                            <IconButton
                                icon=collapse_icon.get()
                                label=collapse_label
                                size=dim::ROW_COMPACT
                                on_click=toggle
                            />
                        </div>
                    }.into_any())
                }}
                <div class="op-side-panel-body">{children.with_value(|c| c())}</div>
                {footer.run()}
            </Show>
        </aside>
    }
}
