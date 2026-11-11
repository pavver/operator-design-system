//! Styled text shared by CodeBlock and ConsoleTerminal: lines made of styled spans.
//!
//! Producers (the ANSI parser, the syntax highlighter) build [`StyledLine`]s;
//! [`span_view`] renders one span with the classes from `styled_text.css`.

use leptos::prelude::*;

/// Visual attributes of a span. Colors are ready CSS values, for example `var(--op-term-red)`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SpanStyle {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    /// Extra class, for example a syntax token class.
    pub class: Option<&'static str>,
}

/// A run of text with one style.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Span {
    pub text: String,
    pub style: SpanStyle,
}

/// One rendered line.
pub(crate) type StyledLine = Vec<Span>;

/// Appends text to a line, merging it into the last span when the style matches.
pub(crate) fn push_text(line: &mut StyledLine, text: &str, style: &SpanStyle) {
    if text.is_empty() {
        return;
    }
    match line.last_mut() {
        Some(last) if last.style == *style => last.text.push_str(text),
        _ => line.push(Span {
            text: text.to_owned(),
            style: style.clone(),
        }),
    }
}

/// Class list for a style; empty when the span is plain.
pub(crate) fn span_class(style: &SpanStyle) -> String {
    [
        (style.bold, "op-st-b"),
        (style.dim, "op-st-d"),
        (style.italic, "op-st-i"),
        (style.underline, "op-st-u"),
        (style.strike, "op-st-s"),
    ]
    .into_iter()
    .filter_map(|(on, class)| on.then_some(class))
    .chain(style.class)
    .collect::<Vec<_>>()
    .join(" ")
}

/// Inline style for the span colors; empty when both are default.
pub(crate) fn span_css(style: &SpanStyle) -> String {
    let mut css = String::new();
    if let Some(fg) = &style.fg {
        css.push_str("color:");
        css.push_str(fg);
        css.push(';');
    }
    if let Some(bg) = &style.bg {
        css.push_str("background-color:");
        css.push_str(bg);
        css.push(';');
    }
    css
}

/// Renders one span.
pub(crate) fn span_view(span: Span) -> impl IntoView {
    let class = span_class(&span.style);
    let css = span_css(&span.style);
    view! {
        <span class=(!class.is_empty()).then_some(class) style=(!css.is_empty()).then_some(css)>
            {span.text}
        </span>
    }
}

/// Renders the spans of one line.
pub(crate) fn line_view(line: StyledLine) -> impl IntoView {
    line.into_iter().map(span_view).collect_view()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_text_merges_equal_styles() {
        let mut line = StyledLine::new();
        let plain = SpanStyle::default();
        push_text(&mut line, "ab", &plain);
        push_text(&mut line, "c", &plain);
        assert_eq!(line.len(), 1);
        assert_eq!(line[0].text, "abc");
    }

    #[test]
    fn push_text_splits_on_style_change_and_skips_empty() {
        let mut line = StyledLine::new();
        let bold = SpanStyle {
            bold: true,
            ..SpanStyle::default()
        };
        push_text(&mut line, "a", &SpanStyle::default());
        push_text(&mut line, "", &bold);
        push_text(&mut line, "b", &bold);
        assert_eq!(line.len(), 2);
        assert_eq!(line[1].text, "b");
    }

    #[test]
    fn class_and_css_reflect_the_style() {
        let style = SpanStyle {
            fg: Some("red".into()),
            bold: true,
            underline: true,
            class: Some("op-syn-keyword"),
            ..SpanStyle::default()
        };
        assert_eq!(span_class(&style), "op-st-b op-st-u op-syn-keyword");
        assert_eq!(span_css(&style), "color:red;");
        assert_eq!(span_class(&SpanStyle::default()), "");
        assert_eq!(span_css(&SpanStyle::default()), "");
    }
}
