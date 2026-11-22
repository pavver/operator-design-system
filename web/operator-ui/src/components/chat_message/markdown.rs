//! Small safe Markdown parser and renderer without raw HTML.

use leptos::prelude::*;

/// Inline text token for rich formatting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineToken {
    /// Plain text string.
    Text(String),
    /// Bold text (`**text**`).
    Bold(String),
    /// Italic text (`*text*`).
    Italic(String),
    /// Bold and italic text (`***text***`).
    BoldItalic(String),
    /// Strikethrough text (`~~text~~`).
    Strike(String),
    /// Inline code segment (`` `code` ``).
    Code(String),
    /// Spoiler text (`||spoiler||`), click to unmask.
    Spoiler(String),
    /// Hyperlink (`[label](url)`).
    Link { label: String, url: String },
}

/// Block-level structured token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockToken {
    /// Heading line (`# Title`).
    Heading(Vec<InlineToken>),
    /// Blockquote line or group (`> quote`).
    Quote(Vec<InlineToken>),
    /// Bullet list item (`- item` or `* item`).
    ListItem(Vec<InlineToken>),
    /// Standard paragraph of inline tokens.
    Paragraph(Vec<InlineToken>),
}

struct DelimitedSpan {
    delim: &'static [char],
    ctor: fn(String) -> InlineToken,
    disallow_next: Option<char>,
}

const SPANS: &[DelimitedSpan] = &[
    DelimitedSpan {
        delim: &['*', '*', '*'],
        ctor: InlineToken::BoldItalic,
        disallow_next: None,
    },
    DelimitedSpan {
        delim: &['*', '*'],
        ctor: InlineToken::Bold,
        disallow_next: None,
    },
    DelimitedSpan {
        delim: &['*'],
        ctor: InlineToken::Italic,
        disallow_next: Some('*'),
    },
    DelimitedSpan {
        delim: &['~', '~'],
        ctor: InlineToken::Strike,
        disallow_next: None,
    },
    DelimitedSpan {
        delim: &['`'],
        ctor: InlineToken::Code,
        disallow_next: None,
    },
    DelimitedSpan {
        delim: &['|', '|'],
        ctor: InlineToken::Spoiler,
        disallow_next: None,
    },
];

fn find_delimiter(chars: &[char], delim: &[char], start: usize) -> Option<usize> {
    let dlen = delim.len();
    if dlen == 0 || start + dlen > chars.len() {
        return None;
    }
    let mut i = start;
    while i + dlen <= chars.len() {
        if chars[i..].starts_with(delim) {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn try_parse_span(chars: &[char], i: usize) -> Option<(InlineToken, usize)> {
    for span in SPANS {
        if !chars[i..].starts_with(span.delim) {
            continue;
        }
        if let Some(disallow) = span.disallow_next
            && chars.get(i + span.delim.len()) == Some(&disallow)
        {
            continue;
        }
        let content_start = i + span.delim.len();
        if let Some(end) = find_delimiter(chars, span.delim, content_start) {
            let inner: String = chars[content_start..end].iter().collect();
            return Some(((span.ctor)(inner), end + span.delim.len()));
        }
    }
    None
}

fn try_parse_link(chars: &[char], i: usize) -> Option<(InlineToken, usize)> {
    if chars.get(i) != Some(&'[') {
        return None;
    }
    let label_end = find_delimiter(chars, &[']'], i + 1)?;
    if chars.get(label_end + 1) != Some(&'(') {
        return None;
    }
    let url_end = find_delimiter(chars, &[')'], label_end + 2)?;
    let label: String = chars[i + 1..label_end].iter().collect();
    let raw_url: String = chars[label_end + 2..url_end].iter().collect();
    Some((
        InlineToken::Link {
            label,
            url: sanitize_url(&raw_url),
        },
        url_end + 1,
    ))
}

/// Parses inline markdown syntax safely into structured tokens.
pub fn parse_inline_markdown(input: &str) -> Vec<InlineToken> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut plain_buf = String::new();

    while i < len {
        if let Some((tok, next_i)) = try_parse_span(&chars, i).or_else(|| try_parse_link(&chars, i))
        {
            if !plain_buf.is_empty() {
                tokens.push(InlineToken::Text(std::mem::take(&mut plain_buf)));
            }
            tokens.push(tok);
            i = next_i;
            continue;
        }

        // An unclosed double star cannot open an italic span; emit both stars as plain text.
        if chars[i..].starts_with(&['*', '*']) {
            plain_buf.push('*');
            plain_buf.push('*');
            i += 2;
            continue;
        }

        plain_buf.push(chars[i]);
        i += 1;
    }

    if !plain_buf.is_empty() {
        tokens.push(InlineToken::Text(plain_buf));
    }

    tokens
}

fn sanitize_url(raw: &str) -> String {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("javascript:") || lower.starts_with("data:text/html") {
        "#".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Parses multi-line markdown text into block tokens.
pub fn parse_markdown_blocks(raw_text: &str) -> Vec<BlockToken> {
    let mut blocks = Vec::new();
    let lines = raw_text.lines();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(heading_text) = trimmed.strip_prefix("# ") {
            blocks.push(BlockToken::Heading(parse_inline_markdown(
                heading_text.trim(),
            )));
        } else if let Some(quote_text) = trimmed.strip_prefix("> ") {
            blocks.push(BlockToken::Quote(parse_inline_markdown(quote_text.trim())));
        } else if let Some(list_text) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        {
            blocks.push(BlockToken::ListItem(parse_inline_markdown(
                list_text.trim(),
            )));
        } else {
            blocks.push(BlockToken::Paragraph(parse_inline_markdown(trimmed)));
        }
    }

    blocks
}

/// Renders an inline token into a safe Leptos view.
#[component]
pub fn RenderInline(token: InlineToken) -> impl IntoView {
    match token {
        InlineToken::Text(t) => view! { <span>{t}</span> }.into_any(),
        InlineToken::Bold(t) => view! { <strong>{t}</strong> }.into_any(),
        InlineToken::Italic(t) => view! { <em>{t}</em> }.into_any(),
        InlineToken::BoldItalic(t) => view! { <strong><em>{t}</em></strong> }.into_any(),
        InlineToken::Strike(t) => view! { <s>{t}</s> }.into_any(),
        InlineToken::Code(t) => view! { <code class="op-chat-inline-code">{t}</code> }.into_any(),
        InlineToken::Spoiler(t) => {
            let revealed = RwSignal::new(false);
            view! {
                <span
                    class="op-chat-spoiler"
                    data-revealed=move || revealed.get().then_some("")
                    role="button"
                    tabindex="0"
                    on:click=move |_| revealed.set(!revealed.get())
                >
                    {t}
                </span>
            }
            .into_any()
        }
        InlineToken::Link { label, url } => view! {
            <a
                class="op-chat-link"
                href=url
                target="_blank"
                rel="noopener noreferrer"
            >
                {label}
            </a>
        }
        .into_any(),
    }
}

/// Safe Markdown content renderer component.
#[component]
pub fn SafeMarkdown(
    /// Raw text input with Markdown syntax.
    #[prop(into)]
    text: String,
) -> impl IntoView {
    let blocks = parse_markdown_blocks(&text);
    view! {
        <div class="op-chat-markdown">
            {blocks.into_iter().map(|block| match block {
                BlockToken::Heading(inlines) => view! {
                    <div class="op-chat-heading">
                        {inlines.into_iter().map(|tok| view! { <RenderInline token=tok /> }).collect_view()}
                    </div>
                }.into_any(),
                BlockToken::Quote(inlines) => view! {
                    <blockquote class="op-chat-quote">
                        {inlines.into_iter().map(|tok| view! { <RenderInline token=tok /> }).collect_view()}
                    </blockquote>
                }.into_any(),
                BlockToken::ListItem(inlines) => view! {
                    <div class="op-chat-list-item">
                        <span class="op-chat-bullet">"• "</span>
                        {inlines.into_iter().map(|tok| view! { <RenderInline token=tok /> }).collect_view()}
                    </div>
                }.into_any(),
                BlockToken::Paragraph(inlines) => view! {
                    <div class="op-chat-paragraph">
                        {inlines.into_iter().map(|tok| view! { <RenderInline token=tok /> }).collect_view()}
                    </div>
                }.into_any(),
            }).collect_view()}
        </div>
    }
}
