//! Embedded log terminal: ConsoleTerminal.

use crate::components::scrollbar::ScrollView;
use crate::core::styled_text::line_view;
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
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
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] empty: Slot,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let label = ui_text(label, "console-terminal.label", "");
    let node = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        let count = logs.with(Vec::len);
        if let Some(el) = node.get()
            && count > 0
        {
            el.set_scroll_top(el.scroll_height());
        }
    });
    let metrics = vec![
        Metric::du("--op-console-terminal-height", height),
        Metric::du("--op-console-terminal-gap", gap),
        Metric::du("--op-console-terminal-padding", padding),
        Metric::tu("--op-console-terminal-font-size", font_size),
        Metric::tu("--op-console-terminal-line-height", line_height),
    ];
    view! {
        <div class=root_class("op-sb op-terminal", class) style=root_style(metrics, style) data-state="idle" role="log" aria-label=move || { let name = label.get(); (!name.is_empty()).then_some(name) }>
            {header.run()}
            <ScrollView node_ref=node viewport_class="op-terminal-log" focusable=true>
            {move || {
                if logs.with(Vec::is_empty) { return empty.run().into_any(); }
                logs.with(|entries| ansi::parse_lines(entries))
                    .into_iter()
                    .map(|line| view! { <div class="op-terminal-line">{line_view(line)}</div> })
                    .collect_view().into_any()
            }}
            </ScrollView>
            {footer.run()}
        </div>
    }
}
