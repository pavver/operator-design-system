//! Live examples for graph.

use crate::i18n::l;
use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;
use std::collections::HashMap;

const NODE_W: f64 = 240.0;

#[derive(Clone, Copy)]
struct DemoNode {
    pos: RwSignal<(f64, f64)>,
    sockets: RwSignal<HashMap<String, f64>>,
}

impl DemoNode {
    fn new(x: f64, y: f64) -> Self {
        Self {
            pos: RwSignal::new((x, y)),
            sockets: RwSignal::new(HashMap::new()),
        }
    }

    fn record(self) -> Callback<Vec<SocketPosition>> {
        Callback::new(move |list: Vec<SocketPosition>| {
            self.sockets
                .set(list.into_iter().map(|(id, _, y)| (id, y)).collect());
        })
    }

    fn socket(self, id: &'static str, out: bool) -> Signal<(f64, f64)> {
        Signal::derive(move || {
            let (x, y) = self.pos.get();
            let dy = self.sockets.with(|s| s.get(id).copied().unwrap_or(0.0));
            (if out { x + NODE_W } else { x }, y + dy)
        })
    }
}

pub fn graph() -> AnyView {
    let camera = RwSignal::new(Camera {
        x: 40.0,
        y: 30.0,
        zoom: 1.0,
    });
    let clock = DemoNode::new(20.0, 40.0);
    let delay = DemoNode::new(340.0, 20.0);
    let text = DemoNode::new(340.0, 200.0);
    let dragging = StoredValue::new(None::<(DemoNode, f64, f64)>);
    let start_drag = move |node: DemoNode| {
        Callback::new(move |e: leptos::ev::PointerEvent| {
            e.stop_propagation();
            dragging.set_value(Some((
                node,
                f64::from(e.client_x()),
                f64::from(e.client_y()),
            )));
        })
    };
    let moved = window_event_listener(leptos::ev::pointermove, move |e| {
        if let Some((node, x, y)) = dragging.get_value() {
            let zoom = camera.get_untracked().zoom;
            let (nx, ny) = (f64::from(e.client_x()), f64::from(e.client_y()));
            node.pos
                .update(|p| *p = (p.0 + (nx - x) / zoom, p.1 + (ny - y) / zoom));
            dragging.set_value(Some((node, nx, ny)));
        }
    });
    let up = window_event_listener(leptos::ev::pointerup, move |_| dragging.set_value(None));
    on_cleanup(move || {
        moved.remove();
        up.remove();
    });
    let delay_value = RwSignal::new(2_000_i64);
    let template = RwSignal::new("Hello {{ name }}!\nThe row grows with the text.".to_string());
    let selected = RwSignal::new(false);
    let mini = Signal::derive(move || {
        [clock, delay, text]
            .iter()
            .map(|n| {
                let (x, y) = n.pos.get();
                MiniItem {
                    rect: WorldRect::new(x, y, NODE_W, 140.0),
                    color: None,
                }
            })
            .collect::<Vec<_>>()
    });
    let exec_color = Signal::stored("var(--op-text-primary)".to_string());
    let text_color = Signal::stored("#c678dd".to_string());
    let custom_camera = RwSignal::new(Camera::default());
    let source_node = DemoNode::new(45.0, 55.0);
    let target_node = DemoNode::new(365.0, 80.0);
    let row_height = RwSignal::new(12.0_f32);
    view! {
        <Example title="Node graph" note="Drag headers to move nodes, the background or the middle button to pan, the wheel to zoom. Hover the info icon for node help." block=true>
            <div class="demo-graph">
                <GraphCanvas camera=camera overlay=std::sync::Arc::new(move || view! { <Minimap items=mini/> }.into_any())>
                    <GraphFrame rect=WorldRect::new(0.0, -20.0, 300.0, 200.0) title="Triggers" color="#5b7cfa".to_string()/>
                    <WireLayer>
                        <Wire from=clock.socket("tick", true) to=delay.socket("in", false) exec=true color=exec_color/>
                        <Wire from=delay.socket("out", true) to=text.socket("go", false) exec=true color=exec_color label="2 s".to_string()/>
                    </WireLayer>
                    <NodeCard
                        x=Signal::derive(move || clock.pos.get().0)
                        y=Signal::derive(move || clock.pos.get().1)
                        width=NODE_W
                        title="Clock"
                        category="Time".to_string()
                        color="#5b7cfa".to_string()
                        selected=selected
                        on_header_down=start_drag(clock)
                        on_sockets=clock.record()
                        help=ViewFn::from(|| view! { <span class="op-t-body">"Emits an exec pulse every interval."</span> })
                    >
                        <NodeRow end_label="Tick".to_string() output=ViewFn::from(|| view! { <Socket id="tick" side=SocketSide::Out kind=SocketKind::Exec connected=true/> })/>
                        <NodeRow label="Interval".to_string()>
                            <span class="op-t-body">"1 s"</span>
                        </NodeRow>
                    </NodeCard>
                    <NodeCard
                        x=Signal::derive(move || delay.pos.get().0)
                        y=Signal::derive(move || delay.pos.get().1)
                        width=NODE_W
                        title="Delay"
                        category="Time".to_string()
                        color="#5b7cfa".to_string()
                        on_header_down=start_drag(delay)
                        on_sockets=delay.record()
                        help=ViewFn::from(|| view! {
                            <span class="op-t-row-title">"Delay"</span>
                            <span class="op-t-body">"Passes each trigger on after the duration."</span>
                        })
                    >
                        <NodeRow
                            label="In".to_string()
                            end_label="Out".to_string()
                            input=ViewFn::from(|| view! { <Socket id="in" side=SocketSide::In kind=SocketKind::Exec connected=true/> })
                            output=ViewFn::from(|| view! { <Socket id="out" side=SocketSide::Out kind=SocketKind::Exec connected=true/> })
                        />
                        <NodeRow label="Duration".to_string() input=ViewFn::from(|| view! { <Socket id="duration" side=SocketSide::In/> })>
                            <DurationField value=delay_value/>
                        </NodeRow>
                    </NodeCard>
                    <NodeCard
                        x=Signal::derive(move || text.pos.get().0)
                        y=Signal::derive(move || text.pos.get().1)
                        width=NODE_W
                        title="Template"
                        category="Text".to_string()
                        color="#c678dd".to_string()
                        on_header_down=start_drag(text)
                        on_sockets=text.record()
                    >
                        <NodeRow label="Go".to_string() input=ViewFn::from(|| view! { <Socket id="go" side=SocketSide::In kind=SocketKind::Exec connected=true/> })/>
                        <NodeRow label="Text".to_string() content=true output=ViewFn::from(move || view! { <Socket id="text" side=SocketSide::Out color=text_color/> })>
                            <textarea
                                class="demo-graph-text"
                                prop:value=move || template.get()
                                on:input=move |e| template.set(event_target_value(&e))
                            ></textarea>
                        </NodeRow>
                        <NodeRow end_label="Done".to_string() output=ViewFn::from(|| view! { <Socket id="done" side=SocketSide::Out kind=SocketKind::Exec/> })/>
                    </NodeCard>
                </GraphCanvas>
            </div>
        </Example>
        <Example title="Customization" note="Change the row height to check that the measured sockets and wires stay aligned." block=true>
            <label><input type="checkbox" on:change=move |e| row_height.set(if event_target_checked(&e) { 16.0 } else { 12.0 })/>" Taller rows"</label>
            <div class="demo-graph">
                <GraphCanvas camera=custom_camera grid=7.0 grid_width=0.5
                    style="--op-graph-header-background:var(--op-hover);--op-graph-socket-color:var(--op-accent);"
                    overlay=std::sync::Arc::new(move || view! {
                        <Minimap items=Signal::derive(move || vec![MiniItem { rect: WorldRect::new(45.0, 55.0, NODE_W, 180.0), color: None }, MiniItem { rect: WorldRect::new(365.0, 80.0, NODE_W, 180.0), color: None }])
                            minimap_width=38.0 minimap_height=24.0 minimap_offset_x=3.0 minimap_offset_y=3.0
                            label=l("Graph overview", "Огляд графа")/>
                    }.into_any())>
                    <GraphFrame rect=WorldRect::new(20.0, 15.0, 615.0, 255.0) title="Custom group"
                        frame_header_height=9.0 title_slot=|| view! { <strong>"Editing group"</strong> }/>
                    <WireLayer>
                        <Wire from=source_node.socket("value", true) to=target_node.socket("value", false)
                            wire_width=0.75 wire_label_padding_x=3.0 label="Value".to_string()/>
                    </WireLayer>
                    <NodeCard x=45.0 y=55.0 width=NODE_W title="Source" header_height=11.0
                        row_height=row_height socket_size=3.0 row_padding_x=4.0 on_sockets=source_node.record()
                        help=ViewFn::from(|| view! { <span>"Reusable graph node with custom geometry."</span> })
                        help_label=l("About the source", "Про джерело")
                        header=|| view! { <strong>"Source node"</strong> }
                        footer=|| view! { <span class="demo-muted">"Custom footer"</span> }>
                        <NodeRow end_label="Value".to_string() output=ViewFn::from(|| view! { <Socket id="value" side=SocketSide::Out connected=true/> })/>
                        <NodeRow label="Status".to_string()><span>"Ready"</span></NodeRow>
                    </NodeCard>
                    <NodeCard x=365.0 y=80.0 width=NODE_W title="Target" header_height=9.0 row_height=row_height
                        socket_size=3.0 on_sockets=target_node.record()
                        body=|| view! {
                            <NodeRow label="Value".to_string() input=ViewFn::from(|| view! { <Socket id="value" side=SocketSide::In connected=true/> })/>
                            <NodeRow label="Mode".to_string()><span>"Automatic"</span></NodeRow>
                        }/>
                </GraphCanvas>
            </div>
        </Example>
    }
    .into_any()
}
