//! Wires, frames and the minimap of a graph.

use super::camera::WorldRect;
use super::route::{
    Point, route_midpoint_with_handle as route_midpoint, route_path_with_handle as route_path,
};
use crate::core::{Metric, Slot, root_class, root_style};
use leptos::prelude::*;
use operator_ui_tokens::DU_POINTS;

/// Look of a [`Wire`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WireState {
    #[default]
    Normal,
    Selected,
    /// Being dragged from a socket.
    Preview,
    /// Kept although an end lost its socket.
    Detached,
}

/// SVG layer for [`Wire`]s in world coordinates; place it first in the canvas world.
#[component]
pub fn WireLayer(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] layer_size: MaybeProp<f32>,
) -> impl IntoView {
    view! { <svg class=root_class("op-wires", class) style=root_style(vec![Metric::du("--op-graph-layer-size", layer_size)], style) aria-hidden="true">{children()}</svg> }
}

/// A connection between two world points, optionally through route points.
#[component]
pub fn Wire(
    #[prop(into)] from: Signal<Point>,
    #[prop(into)] to: Signal<Point>,
    #[prop(optional, into)] points: Signal<Vec<Point>>,
    /// Wire colour, any CSS colour (a type colour).
    #[prop(optional, into)]
    color: Option<Signal<String>>,
    #[prop(optional, into)] state: Signal<WireState>,
    /// Control flow wires are drawn thicker.
    #[prop(optional)]
    exec: bool,
    /// Text in a box at the middle of the wire (a conversion, a value).
    #[prop(optional, into)]
    label: Option<Signal<String>>,
    #[prop(optional)] on_pointerdown: Option<Callback<leptos::ev::PointerEvent>>,
    #[prop(optional, into)] wire_width: MaybeProp<f32>,
    #[prop(optional, into)] wire_exec_width: MaybeProp<f32>,
    #[prop(optional, into)] wire_hit_width: MaybeProp<f32>,
    #[prop(optional, into)] wire_dash: MaybeProp<f32>,
    #[prop(optional, into)] wire_dash_gap: MaybeProp<f32>,
    #[prop(optional, into)] wire_detached_dash: MaybeProp<f32>,
    #[prop(optional, into)] wire_label_width: MaybeProp<f32>,
    #[prop(optional, into)] wire_label_height: MaybeProp<f32>,
    #[prop(optional, into)] wire_label_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] wire_handle_min: MaybeProp<f32>,
    #[prop(optional, into)] border_width: MaybeProp<f32>,
    #[prop(optional, into)] caption_font_size: MaybeProp<f32>,
    #[prop(optional, into)] caption_line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let custom_style = root_style(
        vec![
            Metric::du("--op-graph-wire-width", wire_width),
            Metric::du("--op-graph-wire-exec-width", wire_exec_width),
            Metric::du("--op-graph-wire-hit-width", wire_hit_width),
            Metric::du("--op-graph-wire-dash", wire_dash),
            Metric::du("--op-graph-wire-dash-gap", wire_dash_gap),
            Metric::du("--op-graph-wire-detached-dash", wire_detached_dash),
            Metric::du("--op-graph-wire-label-width", wire_label_width),
            Metric::du("--op-graph-wire-label-height", wire_label_height),
            Metric::du("--op-graph-wire-label-padding-x", wire_label_padding_x),
            Metric::du("--op-graph-wire-handle-min", wire_handle_min),
            Metric::du("--op-graph-border-width", border_width),
            Metric::tu("--op-graph-caption-font-size", caption_font_size),
            Metric::tu("--op-graph-caption-line-height", caption_line_height),
        ],
        style,
    );
    let d = move || {
        points.with(|p| {
            route_path(
                from.get(),
                p,
                to.get(),
                f64::from(wire_handle_min.get().unwrap_or(10.0) * DU_POINTS),
            )
        })
    };
    let mid = move || {
        points.with(|p| {
            route_midpoint(
                from.get(),
                p,
                to.get(),
                f64::from(wire_handle_min.get().unwrap_or(10.0) * DU_POINTS),
            )
        })
    };
    let state_attr = move || match state.get() {
        WireState::Normal => "normal",
        WireState::Selected => "selected",
        WireState::Preview => "preview",
        WireState::Detached => "detached",
    };
    view! {
        <g
            class=root_class("op-wire", class)
            data-state=state_attr
            data-exec=exec.then_some("")
            style=move || format!("{}{}", color.map(|c| format!("--op-graph-wire-color:{};", c.get())).unwrap_or_default(), custom_style().unwrap_or_default())
        >
            <path class="op-wire-hit" d=d on:pointerdown=move |e| {
                if let Some(cb) = on_pointerdown {
                    cb.run(e);
                }
            }></path>
            <path class="op-wire-line" d=d></path>
            {label.map(|l| view! {
                <foreignObject
                    class="op-wire-label-box"
                    x=move || mid().0
                    y=move || mid().1
                    width=move || wire_label_width.get().unwrap_or(100.0) * DU_POINTS
                    height=move || wire_label_height.get().unwrap_or(6.0) * DU_POINTS
                >
                    <div class="op-wire-label-wrap"><span class="op-wire-label op-t-caption">{move || l.get()}</span></div>
                </foreignObject>
            })}
        </g>
    }
}

/// A titled region behind nodes, in world units.
#[component]
pub fn GraphFrame(
    #[prop(into)] rect: Signal<WorldRect>,
    #[prop(into)] title: Signal<String>,
    /// Frame colour, any CSS colour.
    #[prop(optional, into)]
    color: Option<Signal<String>>,
    #[prop(optional, into)] selected: Signal<bool>,
    /// Pointer down on the header, for dragging the frame.
    #[prop(optional)]
    on_header_down: Option<Callback<leptos::ev::PointerEvent>>,
    /// Views at the end of the header.
    #[prop(optional)]
    actions: Option<ChildrenFn>,
    #[prop(optional, into)] title_slot: Slot,
    #[prop(optional, into)] frame_header_height: MaybeProp<f32>,
    #[prop(optional, into)] frame_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] frame_gap: MaybeProp<f32>,
    #[prop(optional, into)] border_width: MaybeProp<f32>,
    #[prop(optional, into)] emphasis_width: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let custom_style = root_style(
        vec![
            Metric::du("--op-graph-frame-header-height", frame_header_height),
            Metric::du("--op-graph-frame-padding-x", frame_padding_x),
            Metric::du("--op-graph-frame-gap", frame_gap),
            Metric::du("--op-graph-border-width", border_width),
            Metric::du("--op-graph-emphasis-width", emphasis_width),
            Metric::tu("--op-graph-font-size", font_size),
            Metric::tu("--op-graph-line-height", line_height),
        ],
        style,
    );
    view! {
        <div
            class=root_class("op-graph-frame", class)
            data-selected=move || selected.get().then_some("")
            style=move || { let r = rect.get(); format!("left:{}px;top:{}px;width:{}px;height:{}px;{}{}", r.x, r.y, r.w, r.h, color.map(|c| format!("--op-graph-frame-color:{};", c.get())).unwrap_or_default(), custom_style().unwrap_or_default()) }
        >
            <div class="op-graph-frame-head" on:pointerdown=move |e| {
                if let Some(cb) = on_header_down {
                    cb.run(e);
                }
            }>
                <span class="op-graph-frame-title">{if title_slot.is_set() { title_slot.run().into_any() } else { (move || title.get()).into_any() }}</span>
                {actions.map(|a| a())}
            </div>
        </div>
    }
}
