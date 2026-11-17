//! Example for the project-specific graph components.

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
    }
    .into_any()
}

const VARS: [&str; 4] = ["name", "temperature", "battery_power", "sensor"];
const FILTERS: [&str; 4] = ["upper", "lower", "round", "default"];

/// A tiny Jinja-like highlighter for the demo: `{{ … }}` blocks with variables, filters, numbers.
fn demo_highlight(text: String) -> Vec<CodeSpan> {
    let mut spans = Vec::new();
    let mut rest = 0;
    while let Some(open) = text[rest..].find("{{").map(|i| i + rest) {
        let close = text[open..].find("}}").map(|i| i + open);
        spans.push(CodeSpan::new(open..open + 2, CodeToken::Punctuation));
        let end = close.unwrap_or(text.len());
        if close.is_none() {
            spans.push(CodeSpan::new(open..end, CodeToken::Error).message("Unclosed {{ block"));
        }
        let mut after_pipe = false;
        let mut i = open + 2;
        while i < end {
            let c = text[i..].chars().next().unwrap_or(' ');
            if c.is_alphabetic() || c == '_' {
                let len = text[i..end]
                    .find(|ch: char| !(ch.is_alphanumeric() || ch == '_'))
                    .unwrap_or(end - i);
                let word = &text[i..i + len];
                if after_pipe {
                    spans.push(CodeSpan::new(i..i + len, CodeToken::Function));
                    if !FILTERS.contains(&word) {
                        spans.push(
                            CodeSpan::new(i..i + len, CodeToken::Error)
                                .message(format!("Unknown filter: {word}")),
                        );
                    }
                } else {
                    spans.push(CodeSpan::new(i..i + len, CodeToken::Property));
                }
                after_pipe = false;
                i += len;
            } else if c.is_ascii_digit() {
                let len = text[i..end]
                    .find(|ch: char| !(ch.is_ascii_digit() || ch == '.'))
                    .unwrap_or(end - i);
                spans.push(CodeSpan::new(i..i + len, CodeToken::Number));
                i += len;
            } else {
                if c == '|' {
                    after_pipe = true;
                    spans.push(CodeSpan::new(i..i + 1, CodeToken::Keyword));
                }
                i += c.len_utf8();
            }
        }
        if let Some(close) = close {
            spans.push(CodeSpan::new(close..close + 2, CodeToken::Punctuation));
            rest = close + 2;
        } else {
            break;
        }
    }
    spans
}

/// Offers variables after `{{` and filters after `|`, filtered by the word at the caret.
fn demo_complete((text, caret): (String, usize)) -> Option<CodeCompletion> {
    let before = &text[..caret];
    let open = before.rfind("{{")?;
    if before[open..].contains("}}") {
        return None;
    }
    let start = before
        .rfind(|c: char| !(c.is_alphanumeric() || c == '_'))
        .map_or(0, |i| {
            i + before[i..].chars().next().map_or(1, char::len_utf8)
        });
    let word = &before[start..];
    let trimmed = before[..start].trim_end();
    let pool: &[&str] = if trimmed.ends_with('|') {
        &FILTERS
    } else if trimmed.ends_with("{{") {
        &VARS
    } else {
        return None;
    };
    let options: Vec<ComboOption> = pool
        .iter()
        .filter(|o| o.starts_with(word) && **o != word)
        .map(|o| {
            ComboOption::new(*o).detail(if trimmed.ends_with('|') {
                "filter"
            } else {
                "variable"
            })
        })
        .collect();
    Some(CodeCompletion {
        range: start..caret,
        options,
    })
}

pub fn code_field() -> AnyView {
    let source =
        RwSignal::new("Hello {{ name | upper }}, it is {{ temperature | round }} °C.".to_string());
    let committed = RwSignal::new(source.get_untracked());
    view! {
        <Example title="Template field" note="Type {{ for variables or | for filters; Ctrl+Space asks again. Unknown filters are underlined; Ctrl+Enter or leaving commits." block=true>
            <CodeField
                value=source
                highlight=Callback::new(demo_highlight)
                complete=Callback::new(demo_complete)
                on_commit=Callback::new(move |v: String| committed.set(v))
                label="Template"
            />
            <span class="op-t-caption demo-muted">{move || format!("Committed: {}", committed.get())}</span>
        </Example>
    }
    .into_any()
}
