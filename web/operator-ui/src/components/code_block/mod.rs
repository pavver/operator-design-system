//! Monospaced block of code: CodeBlock.

use crate::components::icon_button::IconButton;
use crate::components::scrollbar::{ScrollAxis, ScrollView};
use crate::core::styled_text::line_view;
use crate::core::{Metric, Slot, copy_action, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::icons;

#[cfg(feature = "arborium")]
mod arborium;
mod highlight;
mod langs;

pub(crate) const CSS: &str = include_str!("code_block.css");

/// Monospaced block of code or logs with a copy button.
///
/// Known languages are highlighted: Rust, Kotlin, TOML, JSON, YAML, and shell.
#[component]
pub fn CodeBlock(
    #[prop(into)] code: Signal<String>,
    /// Label in the header, for example `rust` or `toml`; also selects the syntax highlighting.
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
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] copy_label: MaybeProp<String>,
    #[prop(optional)] copy_icon: Option<operator_ui_tokens::Icon>,
    #[prop(optional, into)] header_height: MaybeProp<f32>,
    #[prop(optional, into)] header_gap: MaybeProp<f32>,
    #[prop(optional, into)] header_padding_start: MaybeProp<f32>,
    #[prop(optional, into)] header_padding_end: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] number_width: MaybeProp<f32>,
    #[prop(optional, into)] number_padding: MaybeProp<f32>,
    #[prop(optional, into)] copy_size: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] header_font_size: MaybeProp<f32>,
    #[prop(optional, into)] header_letter_spacing: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let copy_label = ui_text(copy_label, "code-block.copy", "Copy code");
    let copy_icon = ui_icon(copy_icon, "code-block.copy", icons::CONTENT_COPY);
    let lang = language.as_deref().and_then(highlight::find);
    #[cfg(feature = "arborium")]
    let lines = {
        let lang_name = language.clone();
        move || {
            code.with(|c| {
                if let Some(name) = lang_name.as_deref()
                    && let Some(lines) = arborium::highlight(c, name)
                {
                    lines
                } else {
                    highlight::highlight(c, lang)
                }
            })
        }
    };
    #[cfg(not(feature = "arborium"))]
    let lines = move || code.with(|c| highlight::highlight(c, lang));
    let has_head = language.is_some() || copy || header.is_set() || actions.is_set();
    let metrics = vec![
        Metric::du("--op-code-block-max-height", max_height.into()),
        Metric::du("--op-code-block-header-height", header_height),
        Metric::du("--op-code-block-header-gap", header_gap),
        Metric::du("--op-code-block-header-padding-start", header_padding_start),
        Metric::du("--op-code-block-header-padding-end", header_padding_end),
        Metric::du("--op-code-block-padding", padding),
        Metric::du("--op-code-block-number-width", number_width),
        Metric::du("--op-code-block-number-padding", number_padding),
        Metric::du("--op-code-block-copy-size", copy_size),
        Metric::du("--op-code-block-icon-size", icon_size),
        Metric::tu("--op-code-block-font-size", font_size),
        Metric::tu("--op-code-block-line-height", line_height),
        Metric::tu("--op-code-block-header-font-size", header_font_size),
        Metric::tu(
            "--op-code-block-header-letter-spacing",
            header_letter_spacing,
        ),
    ];
    view! {
        <div class=root_class("op-code", class) style=root_style(metrics, style) data-wrap=wrap.then_some("")>
            {has_head
                .then(|| {
                    view! {
                        <div class="op-code-head">
                            {if header.is_set() { header.run().into_any() } else {
                                view! { <span class="op-code-lang">{language.clone().unwrap_or_default()}</span> }.into_any()
                            }}
                            {actions.run()}
                            {copy
                                .then_some({
                                    move || view! {
                                        <IconButton
                                            icon=copy_icon.get()
                                            label=copy_label
                                            style="--op-button-size:var(--op-code-block-copy-size, var(--op-dim-row-dense));--op-button-icon-size:var(--op-code-block-icon-size, calc(var(--op-du) * 3.5))"
                                            on_async_click=copy_action(code)
                                        />
                                    }
                                })}
                        </div>
                    }
                })}
            {if content.is_set() { content.run().into_any() } else { view! {
            <ScrollView axis=ScrollAxis::Both class="op-code-scroll" style="--op-scrollbar-max-height: var(--op-code-block-max-height, 100%);">
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
                                        <span class="op-code-text">{line_view(line)}</span>
                                    </span>
                                }
                            })
                            .collect_view()
                    }}
                </code>
            </pre>
            </ScrollView>
            }.into_any() }}
            {footer.run()}
        </div>
    }
}
