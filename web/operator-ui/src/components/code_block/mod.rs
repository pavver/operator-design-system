//! Monospaced block of code: CodeBlock.

use crate::components::icon_button::IconButton;
use crate::components::scrollbar::{ScrollAxis, ScrollView};
use crate::core::copy_action;
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};

pub(crate) const CSS: &str = include_str!("code_block.css");

/// Monospaced block of code or logs with a copy button.
#[component]
pub fn CodeBlock(
    #[prop(into)] code: Signal<String>,
    /// Label in the header, for example `rust` or `toml`.
    #[prop(optional, into)]
    language: Option<String>,
    #[prop(optional)] line_numbers: bool,
    /// Wraps long lines instead of scrolling horizontally.
    #[prop(optional)]
    wrap: bool,
    #[prop(default = true)] copy: bool,
    /// Maximum height in DU before the block scrolls.
    #[prop(optional)]
    max_height: Option<f32>,
) -> impl IntoView {
    let lines = move || {
        code.with(|c| {
            c.trim_end_matches('\n')
                .split('\n')
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
    };
    let has_head = language.is_some() || copy;
    view! {
        <div class="op-code" data-wrap=wrap.then_some("")>
            {has_head
                .then(|| {
                    view! {
                        <div class="op-code-head">
                            <span class="op-code-lang">{language.clone().unwrap_or_default()}</span>
                            {copy
                                .then(|| {
                                    view! {
                                        <IconButton
                                            icon=icons::CONTENT_COPY
                                            label="Copy code"
                                            size=dim::ROW_DENSE
                                            icon_size=3.5
                                            on_async_click=copy_action(code)
                                        />
                                    }
                                })}
                        </div>
                    }
                })}
            <ScrollView axis=ScrollAxis::Both max_height=max_height>
            <pre class="op-code-body">
                <code>
                    {move || {
                        lines()
                            .into_iter()
                            .enumerate()
                            .map(|(i, line)| {
                                view! {
                                    <span class="op-code-line">
                                        {line_numbers.then(|| view! { <span class="op-code-num" aria-hidden="true">{(i + 1).to_string()}</span> })}
                                        <span class="op-code-text">{line}</span>
                                    </span>
                                }
                            })
                            .collect_view()
                    }}
                </code>
            </pre>
            </ScrollView>
        </div>
    }
}
