//! NodeCard: a graph node as a header and a stack of rows with sockets.

use crate::components::hover_card::HoverCard;
use crate::components::primitives::Icon;
use crate::core::{Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};

/// Shape of a [`Socket`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SocketKind {
    /// A value: a square.
    #[default]
    Data,
    /// Control flow: a triangle pointing in the flow direction.
    Exec,
}

/// Node edge a socket sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocketSide {
    In,
    Out,
}

/// Measured socket centre: id, side and vertical offset from the card top in world units.
pub type SocketPosition = (String, SocketSide, f64);

/// Connection point on the edge of a [`NodeRow`].
#[component]
pub fn Socket(
    /// Unique within the card; reported by [`NodeCard`] `on_sockets`.
    #[prop(into)]
    id: String,
    side: SocketSide,
    #[prop(optional)] kind: SocketKind,
    /// Socket colour (a type colour), any CSS colour.
    #[prop(optional, into)]
    color: Option<Signal<String>>,
    #[prop(optional, into)] connected: Signal<bool>,
    /// Marks a valid drop target while a wire is dragged.
    #[prop(optional, into)]
    highlighted: Signal<bool>,
    /// Marks a required input without a value.
    #[prop(optional, into)]
    missing: Signal<bool>,
    #[prop(optional)] on_pointerdown: Option<Callback<leptos::ev::PointerEvent>>,
    #[prop(optional, into)] socket_size: MaybeProp<f32>,
    #[prop(optional, into)] socket_border_width: MaybeProp<f32>,
    #[prop(optional, into)] socket_outline_width: MaybeProp<f32>,
    #[prop(optional, into)] socket_outline_offset: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let custom_style = root_style(
        vec![
            Metric::du("--op-graph-socket-size", socket_size),
            Metric::du("--op-graph-socket-border-width", socket_border_width),
            Metric::du("--op-graph-socket-outline-width", socket_outline_width),
            Metric::du("--op-graph-socket-outline-offset", socket_outline_offset),
        ],
        style,
    );
    view! {
        <span
            class=root_class("op-socket", class)
            data-socket=id
            data-side=if side == SocketSide::In { "in" } else { "out" }
            data-kind=if kind == SocketKind::Exec { "exec" } else { "data" }
            data-connected=move || connected.get().then_some("")
            data-highlighted=move || highlighted.get().then_some("")
            data-missing=move || missing.get().then_some("")
            style=move || format!("{}{}", color.map(|c| format!("--op-graph-socket-color:{};", c.get())).unwrap_or_default(), custom_style().unwrap_or_default())
            on:pointerdown=move |e| {
                if let Some(cb) = on_pointerdown {
                    e.stop_propagation();
                    cb.run(e);
                }
            }
        ></span>
    }
}

/// One row of a [`NodeCard`]: an optional input socket, a label, a control or an end label, and an
/// optional output socket.
///
/// Rows share the card's two-column grid: labels form the first column, controls the second.
/// A fixed row is `row_standard` tall; a `content` row grows with its control (text, lists).
#[component]
pub fn NodeRow(
    #[prop(optional, into)] label: Option<Signal<String>>,
    /// Label at the end edge (an output name).
    #[prop(optional, into)]
    end_label: Option<Signal<String>>,
    #[prop(optional, into)] input: Slot,
    #[prop(optional, into)] output: Slot,
    /// Grows with its content instead of keeping the row height.
    #[prop(optional)]
    content: bool,
    /// Marks the row (a missing required input).
    #[prop(optional, into)]
    error: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] row_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] row_padding_y: MaybeProp<f32>,
    #[prop(optional, into)] row_gap: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let has_control = children.is_some();
    view! {
        <div
            class=root_class("op-node-row", class)
            style=root_style(vec![
                Metric::du("--op-graph-row-height", row_height),
                Metric::du("--op-graph-row-padding-x", row_padding_x),
                Metric::du("--op-graph-row-padding-y", row_padding_y),
                Metric::du("--op-graph-row-gap", row_gap),
                Metric::tu("--op-graph-font-size", font_size),
                Metric::tu("--op-graph-line-height", line_height)
            ], style)
            data-content=content.then_some("")
            data-error=move || error.get().then_some("")
            data-wide=(!has_control && label.is_some() != end_label.is_some()).then_some("")
        >
            {input.is_set().then(|| view! { <span class="op-node-socket-slot" data-side="in">{input.run()}</span> })}
            {label.map(|l| view! { <span class="op-node-label">{move || l.get()}</span> })}
            {children.map(|c| view! { <span class="op-node-control">{c()}</span> })}
            {end_label.map(|l| view! { <span class="op-node-end-label">{move || l.get()}</span> })}
            {output.is_set().then(|| view! { <span class="op-node-socket-slot" data-side="out">{output.run()}</span> })}
        </div>
    }
}

/// A node on a [`GraphCanvas`](super::GraphCanvas): a header and a stack of [`NodeRow`]s.
///
/// The card is positioned in world units. It measures its socket centres and reports them through
/// `on_sockets` whenever its size changes, so wires follow rows that grow with content.
#[component]
pub fn NodeCard(
    #[prop(into)] x: Signal<f64>,
    #[prop(into)] y: Signal<f64>,
    /// Width in world units.
    #[prop(into)]
    width: Signal<f64>,
    #[prop(into)] title: Signal<String>,
    /// Category text in the header.
    #[prop(optional, into)]
    category: Option<Signal<String>>,
    /// Category colour: the header accent and the category text, any CSS colour.
    #[prop(optional, into)]
    color: Option<Signal<String>>,
    #[prop(optional, into)] selected: Signal<bool>,
    /// Marks the node (a missing input, a failed run).
    #[prop(optional, into)]
    error: Signal<bool>,
    /// Help shown in a hover card behind an `info` icon in the header.
    #[prop(optional)]
    help: Option<ViewFn>,
    /// Accessible name of the help card.
    #[prop(optional, into)]
    help_label: MaybeProp<String>,
    #[prop(optional)] help_icon: Option<IconData>,
    /// Views at the end of the header (run button, status).
    #[prop(optional)]
    actions: Option<ChildrenFn>,
    /// Pointer down on the header, for dragging the node.
    #[prop(optional)]
    on_header_down: Option<Callback<leptos::ev::PointerEvent>>,
    #[prop(optional)] on_sockets: Option<Callback<Vec<SocketPosition>>>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] body: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] header_height: MaybeProp<f32>,
    #[prop(optional, into)] header_padding_start: MaybeProp<f32>,
    #[prop(optional, into)] header_padding_end: MaybeProp<f32>,
    #[prop(optional, into)] header_gap: MaybeProp<f32>,
    #[prop(optional, into)] body_padding_y: MaybeProp<f32>,
    #[prop(optional, into)] label_width: MaybeProp<f32>,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] row_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] row_padding_y: MaybeProp<f32>,
    #[prop(optional, into)] row_gap: MaybeProp<f32>,
    #[prop(optional, into)] socket_size: MaybeProp<f32>,
    #[prop(optional, into)] border_width: MaybeProp<f32>,
    #[prop(optional, into)] emphasis_width: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] caption_font_size: MaybeProp<f32>,
    #[prop(optional, into)] caption_line_height: MaybeProp<f32>,
    #[prop(optional, into)] footer_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] footer_padding_y: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let card = NodeRef::<leptos::html::Div>::new();
    if let Some(report) = on_sockets {
        super::sockets::observe_sockets(card, report);
    }
    let help_label = ui_text(help_label, "graph.node-help", "About this node");
    let help_icon = ui_icon(help_icon, "graph.node-help", icons::INFO);
    let custom_style = root_style(
        vec![
            Metric::du("--op-graph-header-height", header_height),
            Metric::du("--op-graph-header-padding-start", header_padding_start),
            Metric::du("--op-graph-header-padding-end", header_padding_end),
            Metric::du("--op-graph-header-gap", header_gap),
            Metric::du("--op-graph-body-padding-y", body_padding_y),
            Metric::du("--op-graph-label-width", label_width),
            Metric::du("--op-graph-row-height", row_height),
            Metric::du("--op-graph-row-padding-x", row_padding_x),
            Metric::du("--op-graph-row-padding-y", row_padding_y),
            Metric::du("--op-graph-row-gap", row_gap),
            Metric::du("--op-graph-socket-size", socket_size),
            Metric::du("--op-graph-border-width", border_width),
            Metric::du("--op-graph-emphasis-width", emphasis_width),
            Metric::du("--op-graph-icon-size", icon_size),
            Metric::tu("--op-graph-font-size", font_size),
            Metric::tu("--op-graph-line-height", line_height),
            Metric::tu("--op-graph-caption-font-size", caption_font_size),
            Metric::tu("--op-graph-caption-line-height", caption_line_height),
            Metric::du("--op-graph-footer-padding-x", footer_padding_x),
            Metric::du("--op-graph-footer-padding-y", footer_padding_y),
        ],
        style,
    );
    view! {
        <div
            node_ref=card
            class=root_class("op-node", class)
            data-selected=move || selected.get().then_some("")
            data-error=move || error.get().then_some("")
            style=move || format!("left:{}px;top:{}px;width:{}px;{}{}", x.get(), y.get(), width.get(), color.map(|c| format!("--op-graph-node-color:{};", c.get())).unwrap_or_default(), custom_style().unwrap_or_default())
        >
            <div
                class="op-node-head"
                on:pointerdown=move |e| {
                    if let Some(cb) = on_header_down {
                        cb.run(e);
                    }
                }
            >
                {if header.is_set() { header.run() } else { Some(view! {
                <span class="op-node-title">{move || title.get()}</span>
                {category.map(|c| view! { <span class="op-node-category op-t-caption">{move || c.get()}</span> })}
                }.into_any()) }}
                {help.map(|h| view! {
                    <span class="op-node-info" on:pointerdown=|e| e.stop_propagation()>
                        <HoverCard card=h aria_label=help_label>
                            <span class="op-node-info-icon" tabindex="0" aria-label=move || help_label.get()>
                                {move || view! { <Icon icon=help_icon.get() size=dim::ICON_SMALL attr:style="width:var(--op-graph-icon-size, var(--op-dim-icon-small));height:var(--op-graph-icon-size, var(--op-dim-icon-small));"/> }}
                            </span>
                        </HoverCard>
                    </span>
                })}
                {actions.map(|a| view! { <span class="op-node-actions" on:pointerdown=|e| e.stop_propagation()>{a()}</span> })}
            </div>
            <div class="op-node-body">{if body.is_set() { body.run() } else { children.map(|c| c().into_any()) }}</div>
            {footer.is_set().then(|| view! { <div class="op-node-footer">{footer.run()}</div> })}
        </div>
    }
}
