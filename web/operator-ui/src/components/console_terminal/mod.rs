//! Embedded log terminal: ConsoleTerminal.

use crate::components::scrollbar::ScrollView;
use crate::core::du;
use crate::core::styled_text::line_view;
use leptos::prelude::*;

mod ansi;

pub(crate) const CSS: &str = include_str!("console_terminal.css");

/// Embedded log terminal that auto-scrolls to the newest line.
///
/// Colors and text attributes come from xterm SGR escape sequences in the log text.
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
                logs.with(|entries| ansi::parse_lines(entries))
                    .into_iter()
                    .map(|line| view! { <div class="op-terminal-line">{line_view(line)}</div> })
                    .collect_view()
            }}
            </ScrollView>
        </div>
    }
}
