//! NodeCard: a graph node as a header and a stack of rows with sockets.

use crate::components::hover_card::HoverCard;
use crate::components::primitives::Icon;
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

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
) -> impl IntoView {
    view! {
        <span
            class="op-socket"
            data-socket=id
            data-side=if side == SocketSide::In { "in" } else { "out" }
            data-kind=if kind == SocketKind::Exec { "exec" } else { "data" }
            data-connected=move || connected.get().then_some("")
            data-highlighted=move || highlighted.get().then_some("")
            data-missing=move || missing.get().then_some("")
            style=move || color.map(|c| format!("--op-socket-color: {}", c.get()))
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
    #[prop(optional)] input: Option<ViewFn>,
    #[prop(optional)] output: Option<ViewFn>,
    /// Grows with its content instead of keeping the row height.
    #[prop(optional)]
    content: bool,
    /// Marks the row (a missing required input).
    #[prop(optional, into)]
    error: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let has_control = children.is_some();
    view! {
        <div
            class="op-node-row"
            data-content=content.then_some("")
            data-error=move || error.get().then_some("")
            data-wide=(!has_control && label.is_some() != end_label.is_some()).then_some("")
        >
            {input.map(|s| view! { <span class="op-node-socket-slot" data-side="in">{s.run()}</span> })}
            {label.map(|l| view! { <span class="op-node-label">{move || l.get()}</span> })}
            {children.map(|c| view! { <span class="op-node-control">{c()}</span> })}
            {end_label.map(|l| view! { <span class="op-node-end-label">{move || l.get()}</span> })}
            {output.map(|s| view! { <span class="op-node-socket-slot" data-side="out">{s.run()}</span> })}
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
    #[prop(default = "About this node".into(), into)]
    help_label: String,
    /// Views at the end of the header (run button, status).
    #[prop(optional)]
    actions: Option<ChildrenFn>,
    /// Pointer down on the header, for dragging the node.
    #[prop(optional)]
    on_header_down: Option<Callback<leptos::ev::PointerEvent>>,
    #[prop(optional)] on_sockets: Option<Callback<Vec<SocketPosition>>>,
    children: Children,
) -> impl IntoView {
    let card = NodeRef::<leptos::html::Div>::new();
    if let Some(report) = on_sockets {
        Effect::new(move |_| {
            let Some(el) = card.get() else { return };
            let el: web_sys::HtmlElement = el.unchecked_ref::<web_sys::HtmlElement>().clone();
            let measure = {
                let el = el.clone();
                move || report.run(measure_sockets(&el))
            };
            measure();
            let on_resize = Closure::<dyn FnMut()>::new(measure);
            let observer = web_sys::ResizeObserver::new(on_resize.as_ref().unchecked_ref()).ok();
            if let Some(o) = &observer {
                o.observe(&el);
            }
            let keep = SendWrapper::new((observer, on_resize));
            on_cleanup(move || {
                if let Some(o) = &keep.0 {
                    o.disconnect();
                }
            });
        });
    }
    let help_label = StoredValue::new(help_label);
    view! {
        <div
            node_ref=card
            class="op-node"
            data-selected=move || selected.get().then_some("")
            data-error=move || error.get().then_some("")
            style:left=move || format!("{}px", x.get())
            style:top=move || format!("{}px", y.get())
            style:width=move || format!("{}px", width.get())
            style=move || color.map(|c| format!("--op-node-color: {}", c.get()))
        >
            <div
                class="op-node-head"
                on:pointerdown=move |e| {
                    if let Some(cb) = on_header_down {
                        cb.run(e);
                    }
                }
            >
                <span class="op-node-title">{move || title.get()}</span>
                {category.map(|c| view! { <span class="op-node-category op-t-caption">{move || c.get()}</span> })}
                {help.map(|h| view! {
                    <span class="op-node-info" on:pointerdown=|e| e.stop_propagation()>
                        <HoverCard card=h label=help_label.get_value()>
                            <span class="op-node-info-icon" tabindex="0" aria-label=help_label.get_value()>
                                <Icon icon=icons::INFO size=dim::ICON_SMALL/>
                            </span>
                        </HoverCard>
                    </span>
                })}
                {actions.map(|a| view! { <span class="op-node-actions" on:pointerdown=|e| e.stop_propagation()>{a()}</span> })}
            </div>
            <div class="op-node-body">{children()}</div>
        </div>
    }
}

/// Socket centres of a card in world units, independent of the camera zoom.
fn measure_sockets(card: &web_sys::HtmlElement) -> Vec<SocketPosition> {
    let rect = card.get_bounding_client_rect();
    let width = f64::from(card.offset_width());
    let scale = if width > 0.0 {
        rect.width() / width
    } else {
        1.0
    };
    let Ok(list) = card.query_selector_all("[data-socket]") else {
        return Vec::new();
    };
    (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|n| n.dyn_into::<web_sys::Element>().ok())
        .filter_map(|el| {
            let id = el.get_attribute("data-socket")?;
            let side = if el.get_attribute("data-side").as_deref() == Some("out") {
                SocketSide::Out
            } else {
                SocketSide::In
            };
            let r = el.get_bounding_client_rect();
            let y = (r.top() + r.height() / 2.0 - rect.top()) / scale;
            Some((id, side, y))
        })
        .collect()
}
