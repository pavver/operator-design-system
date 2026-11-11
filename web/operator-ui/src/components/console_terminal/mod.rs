//! Embedded log terminal: ConsoleTerminal.

use crate::components::scrollbar::ScrollView;
use crate::core::du;
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("console_terminal.css");

fn log_tone(line: &str) -> &'static str {
    if line.contains("[CRITICAL]") || line.contains("[ERROR]") {
        "error"
    } else if line.contains("[WARNING]") {
        "warning"
    } else if line.contains("[DEBUG]") {
        "debug"
    } else if line.contains("[SUCCESS]") {
        "success"
    } else if ["[API]", "[UI]", "[TX]", "[RX]"]
        .iter()
        .any(|t| line.contains(t))
    {
        "io"
    } else {
        "plain"
    }
}

/// Embedded log terminal that auto-scrolls to the newest line.
#[component]
pub fn ConsoleTerminal(
    #[prop(into)] logs: Signal<Vec<String>>,
    /// Height in DU.
    #[prop(default = 45.0)]
    height: f32,
) -> impl IntoView {
    let node = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        let count = logs.with(Vec::len);
        if let Some(el) = node.get()
            && count > 0
        {
            el.set_scroll_top(el.scroll_height());
        }
    });
    view! {
        <div class="op-sb op-terminal" data-state="idle" role="log" style:height=du(height)>
            <ScrollView node_ref=node viewport_class="op-terminal-log" focusable=true>
            {move || {
                logs.get()
                    .into_iter()
                    .map(|line| {
                        let tone = log_tone(&line);
                        view! { <div class="op-terminal-line" data-tone=tone>{line}</div> }
                    })
                    .collect_view()
            }}
            </ScrollView>
        </div>
    }
}
