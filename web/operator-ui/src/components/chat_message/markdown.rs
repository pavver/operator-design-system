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

/// Parses inline markdown syntax safely into structured tokens.
pub fn parse_inline_markdown(input: &str) -> Vec<InlineToken> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut plain_buf = String::new();

    while i < len {
        // 1. Bold Italic: ***text***
        if i + 2 < len
            && chars[i] == '*'
            && chars[i + 1] == '*'
            && chars[i + 2] == '*'
            && let Some(end) = find_triple_star(&chars, i + 3)
        {
            if !plain_buf.is_empty() {
                tokens.push(InlineToken::Text(std::mem::take(&mut plain_buf)));
            }
            let inner: String = chars[i + 3..end].iter().collect();
            tokens.push(InlineToken::BoldItalic(inner));
            i = end + 3;
            continue;
        }

        // 2. Bold: **text**
        if i + 1 < len
            && chars[i] == '*'
            && chars[i + 1] == '*'
            && let Some(end) = find_double_star(&chars, i + 2)
        {
            if !plain_buf.is_empty() {
                tokens.push(InlineToken::Text(std::mem::take(&mut plain_buf)));
            }
            let inner: String = chars[i + 2..end].iter().collect();
            tokens.push(InlineToken::Bold(inner));
            i = end + 2;
            continue;
        }

        // 3. Italic: *text* (ensure not immediately followed by star)
        if chars[i] == '*'
            && (i + 1 >= len || chars[i + 1] != '*')
            && let Some(end) = find_single_star(&chars, i + 1)
        {
            if !plain_buf.is_empty() {
                tokens.push(InlineToken::Text(std::mem::take(&mut plain_buf)));
            }
            let inner: String = chars[i + 1..end].iter().collect();
            tokens.push(InlineToken::Italic(inner));
            i = end + 1;
            continue;
        }

        // 4. Strikethrough: ~~text~~
        if i + 1 < len
            && chars[i] == '~'
            && chars[i + 1] == '~'
            && let Some(end) = find_double_tilde(&chars, i + 2)
        {
            if !plain_buf.is_empty() {
                tokens.push(InlineToken::Text(std::mem::take(&mut plain_buf)));
            }
            let inner: String = chars[i + 2..end].iter().collect();
            tokens.push(InlineToken::Strike(inner));
            i = end + 2;
            continue;
        }

        // 5. Inline Code: `code`
        if chars[i] == '`'
            && let Some(end) = find_backtick(&chars, i + 1)
        {
            if !plain_buf.is_empty() {
                tokens.push(InlineToken::Text(std::mem::take(&mut plain_buf)));
            }
            let inner: String = chars[i + 1..end].iter().collect();
            tokens.push(InlineToken::Code(inner));
            i = end + 1;
            continue;
        }

        // 6. Spoiler: ||spoiler||
        if i + 1 < len
            && chars[i] == '|'
            && chars[i + 1] == '|'
            && let Some(end) = find_double_pipe(&chars, i + 2)
        {
            if !plain_buf.is_empty() {
                tokens.push(InlineToken::Text(std::mem::take(&mut plain_buf)));
            }
            let inner: String = chars[i + 2..end].iter().collect();
            tokens.push(InlineToken::Spoiler(inner));
            i = end + 2;
            continue;
        }

        // 7. Link: [label](url)
        if chars[i] == '['
            && let Some(label_end) = find_char(&chars, ']', i + 1)
            && label_end + 1 < len
            && chars[label_end + 1] == '('
            && let Some(url_end) = find_char(&chars, ')', label_end + 2)
        {
            if !plain_buf.is_empty() {
                tokens.push(InlineToken::Text(std::mem::take(&mut plain_buf)));
            }
            let label: String = chars[i + 1..label_end].iter().collect();
            let raw_url: String = chars[label_end + 2..url_end].iter().collect();
            let safe_url = sanitize_url(&raw_url);
            tokens.push(InlineToken::Link {
                label,
                url: safe_url,
            });
            i = url_end + 1;
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

fn find_triple_star(chars: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 2 < chars.len() {
        if chars[i] == '*' && chars[i + 1] == '*' && chars[i + 2] == '*' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_double_star(chars: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < chars.len() {
        if chars[i] == '*' && chars[i + 1] == '*' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_single_star(chars: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    while i < chars.len() {
        if chars[i] == '*' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_double_tilde(chars: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < chars.len() {
        if chars[i] == '~' && chars[i + 1] == '~' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_backtick(chars: &[char], start: usize) -> Option<usize> {
    find_char(chars, '`', start)
}

fn find_double_pipe(chars: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < chars.len() {
        if chars[i] == '|' && chars[i + 1] == '|' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_char(chars: &[char], target: char, start: usize) -> Option<usize> {
    let mut i = start;
    while i < chars.len() {
        if chars[i] == target {
            return Some(i);
        }
        i += 1;
    }
    None
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
