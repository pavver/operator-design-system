//! Live examples for code field.

use crate::i18n::l;
use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

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
    let customized = RwSignal::new("{{ sensor | unknown }}".to_string());
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
        <Example title="Customization" note="Custom typography, gutter, completion rows and diagnostic help; type {{ to open completion." block=true>
            <CodeField value=customized highlight=Callback::new(demo_highlight) complete=Callback::new(demo_complete)
                padding_x=4.0 padding_y=3.0 font_size=15.0 line_height=23.0 row_height=9.0
                gutter_width=7.0 gutter_gap=2.0
                placeholder=l("Write an expression", "Введіть вираз")
                aria_label=l("Expression editor", "Редактор виразів")
                style="--op-field-background:var(--op-surface-alt);--op-field-syntax-property:var(--op-accent);"
                gutter=|| view! { <span class="demo-muted">"1"</span> }
                diagnostics_hover=|| view! { <span>"Choose upper, lower, round or default from completion."</span> }
                completion_item=Callback::new(|item: ComboOption| view! {
                    <strong>{item.value}</strong><span class="demo-muted">{item.detail}</span>
                }.into_any())
            />
        </Example>
    }
    .into_any()
}
