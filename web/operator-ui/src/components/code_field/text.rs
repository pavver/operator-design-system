//! Code text logic: offsets, highlight segments and completion splicing (pure).

use std::ops::Range;

/// Syntax role of a highlighted range; each maps to a `syntax_*` colour token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeToken {
    Keyword,
    String,
    Number,
    Comment,
    Type,
    Function,
    Constant,
    Property,
    Meta,
    Punctuation,
    /// An error range: a wavy `error` underline.
    Error,
    /// A warning range: a wavy `warning` underline.
    Warning,
}

impl CodeToken {
    pub fn class(self) -> &'static str {
        match self {
            Self::Keyword => "op-code-keyword",
            Self::String => "op-code-string",
            Self::Number => "op-code-number",
            Self::Comment => "op-code-comment",
            Self::Type => "op-code-type",
            Self::Function => "op-code-function",
            Self::Constant => "op-code-constant",
            Self::Property => "op-code-property",
            Self::Meta => "op-code-meta",
            Self::Punctuation => "op-code-punctuation",
            Self::Error => "op-code-error",
            Self::Warning => "op-code-warning",
        }
    }

    /// Diagnostics are drawn over syntax colours instead of replacing them.
    pub fn is_diagnostic(self) -> bool {
        matches!(self, Self::Error | Self::Warning)
    }
}

/// A highlighted byte range of the text, with an optional message (diagnostics).
#[derive(Clone, Debug, PartialEq)]
pub struct CodeSpan {
    pub range: Range<usize>,
    pub token: CodeToken,
    pub message: Option<String>,
}

impl CodeSpan {
    pub fn new(range: Range<usize>, token: CodeToken) -> Self {
        Self {
            range,
            token,
            message: None,
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

/// A run of text with its classes, ready to render.
#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    pub text: String,
    pub classes: Vec<&'static str>,
}

/// Splits `text` into runs: syntax spans colour the text, diagnostic spans underline it on top.
///
/// Ranges are clamped to the text and to char boundaries; overlapping syntax spans keep the
/// earlier one.
pub fn segments(text: &str, spans: &[CodeSpan]) -> Vec<Segment> {
    let clamp = |i: usize| {
        let mut i = i.min(text.len());
        while !text.is_char_boundary(i) {
            i -= 1;
        }
        i
    };
    let mut cuts = vec![0, text.len()];
    for s in spans {
        cuts.push(clamp(s.range.start));
        cuts.push(clamp(s.range.end));
    }
    cuts.sort_unstable();
    cuts.dedup();
    let mut syntax: Vec<&CodeSpan> = spans.iter().filter(|s| !s.token.is_diagnostic()).collect();
    syntax.sort_by_key(|s| s.range.start);
    let mut out: Vec<Segment> = Vec::new();
    for pair in cuts.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if a >= b {
            continue;
        }
        let covers = |s: &CodeSpan| clamp(s.range.start) <= a && clamp(s.range.end) >= b;
        let mut classes: Vec<&'static str> = syntax
            .iter()
            .find(|s| covers(s))
            .map(|s| vec![s.token.class()])
            .unwrap_or_default();
        classes.extend(
            spans
                .iter()
                .filter(|s| s.token.is_diagnostic())
                .filter(|s| covers(s))
                .map(|s| s.token.class()),
        );
        match out.last_mut() {
            Some(last) if last.classes == classes => last.text.push_str(&text[a..b]),
            _ => out.push(Segment {
                text: text[a..b].to_string(),
                classes,
            }),
        }
    }
    out
}

/// UTF-16 offset (DOM selection) to a byte offset in `text`, on a char boundary.
pub fn utf16_to_byte(text: &str, utf16: usize) -> usize {
    let mut units = 0;
    for (byte, ch) in text.char_indices() {
        if units >= utf16 {
            return byte;
        }
        units += ch.len_utf16();
        if units > utf16 {
            return byte;
        }
    }
    text.len()
}

/// Byte offset in `text` to a UTF-16 offset (DOM selection).
pub fn byte_to_utf16(text: &str, byte: usize) -> usize {
    text.char_indices()
        .take_while(|(b, ch)| b + ch.len_utf8() <= byte)
        .map(|(_, ch)| ch.len_utf16())
        .sum()
}

/// `text` with `range` replaced by `insert`, and the caret byte offset after the insert.
pub fn splice(text: &str, range: Range<usize>, insert: &str) -> (String, usize) {
    let start = range.start.min(text.len());
    let end = range.end.min(text.len()).max(start);
    let mut out = String::with_capacity(text.len() + insert.len());
    out.push_str(&text[..start]);
    out.push_str(insert);
    out.push_str(&text[end..]);
    (out, start + insert.len())
}

/// The first diagnostic message, errors before warnings.
pub fn first_message(spans: &[CodeSpan]) -> Option<(CodeToken, String)> {
    let pick = |t: CodeToken| {
        spans
            .iter()
            .find(|s| s.token == t && s.message.is_some())
            .and_then(|s| s.message.clone().map(|m| (t, m)))
    };
    pick(CodeToken::Error).or_else(|| pick(CodeToken::Warning))
}
