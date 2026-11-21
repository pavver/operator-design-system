//! Lightweight syntax highlighter for CodeBlock.
//!
//! Each language is a declarative [`Lang`] (comments, quotes, word lists, a few flags),
//! in the spirit of highlight.js modes; one scanner handles every language.
//! The result is display-grade: it never fails and never rejects input.

#[path = "scanner_helpers.rs"]
mod scanner_helpers;
use scanner_helpers::*;

use crate::core::styled_text::{SpanStyle, StyledLine, push_text};

/// Token class, rendered with the `syntax_*` tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
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
}

impl Kind {
    pub(crate) fn class(self) -> &'static str {
        match self {
            Kind::Keyword => "op-syn-keyword",
            Kind::String => "op-syn-string",
            Kind::Number => "op-syn-number",
            Kind::Comment => "op-syn-comment",
            Kind::Type => "op-syn-type",
            Kind::Function => "op-syn-function",
            Kind::Constant => "op-syn-constant",
            Kind::Property => "op-syn-property",
            Kind::Meta => "op-syn-meta",
            Kind::Punctuation => "op-syn-punctuation",
        }
    }
}

/// A string delimiter pair.
pub(crate) struct Quote {
    pub open: &'static str,
    pub close: &'static str,
    /// Backslash escapes the next character.
    pub escapes: bool,
    /// May span several lines; otherwise ends at the end of the line.
    pub multiline: bool,
}

/// How keys of key/value formats are recognized at the start of a line.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Keys {
    None,
    /// `key = value` (TOML).
    Equals,
    /// `key: value` (YAML).
    Colon,
}

/// Declarative description of a language.
pub(crate) struct Lang {
    pub names: &'static [&'static str],
    pub line_comments: &'static [&'static str],
    pub block_comment: Option<(&'static str, &'static str)>,
    pub nested_comments: bool,
    /// `#` starts a comment only at the start of a word (shell, YAML).
    pub hash_needs_boundary: bool,
    /// Longest delimiters first.
    pub quotes: &'static [Quote],
    pub keywords: &'static [&'static str],
    pub constants: &'static [&'static str],
    pub types: &'static [&'static str],
    /// Built-in commands or functions, shown as functions.
    pub builtins: &'static [&'static str],
    /// Capitalized words are types; ALL_CAPS words are constants.
    pub capitalized_types: bool,
    pub keys: Keys,
    /// A string followed by `:` is a key (JSON).
    pub string_keys: bool,
    /// `[table]` and `[[array]]` headers at the start of a line (TOML).
    pub sections: bool,
    /// `@Annotation` (Kotlin).
    pub annotations: bool,
    /// `$var`, `${var}` (shell).
    pub variables: bool,
    /// `&anchor`, `*alias`, `---` (YAML).
    pub yaml: bool,
    /// Raw strings, char literals and lifetimes, `macro!`, `#[attribute]` (Rust).
    pub rust: bool,
}

impl Lang {
    /// Plain description; languages override what they need.
    pub(crate) const BASE: Lang = Lang {
        names: &[],
        line_comments: &[],
        block_comment: None,
        nested_comments: false,
        hash_needs_boundary: false,
        quotes: &[],
        keywords: &[],
        constants: &[],
        types: &[],
        builtins: &[],
        capitalized_types: false,
        keys: Keys::None,
        string_keys: false,
        sections: false,
        annotations: false,
        variables: false,
        yaml: false,
        rust: false,
    };
}

/// Finds a language by name or alias, ignoring case.
pub(crate) fn find(name: &str) -> Option<&'static Lang> {
    let name = name.trim().to_ascii_lowercase();
    super::langs::ALL
        .iter()
        .copied()
        .find(|lang| lang.names.contains(&name.as_str()))
}

/// Splits code into lines; highlights them when `lang` is known.
pub(crate) fn highlight(code: &str, lang: Option<&Lang>) -> Vec<StyledLine> {
    let code = code.trim_end_matches('\n');
    let tokens = match lang {
        Some(lang) => Scanner::new(code, lang).run(),
        None => vec![(None, code)],
    };
    tokens_to_lines(tokens)
}

/// Turns classified text runs into lines, splitting runs that contain `\n`.
pub(crate) fn tokens_to_lines<'a>(
    tokens: impl IntoIterator<Item = (Option<Kind>, &'a str)>,
) -> Vec<StyledLine> {
    let mut lines = vec![StyledLine::new()];
    for (kind, text) in tokens {
        let style = SpanStyle {
            class: kind.map(Kind::class),
            ..SpanStyle::default()
        };
        let mut parts = text.split('\n');
        if let Some(first) = parts.next()
            && let Some(line) = lines.last_mut()
        {
            push_text(line, first, &style);
        }
        for part in parts {
            let mut line = StyledLine::new();
            push_text(&mut line, part, &style);
            lines.push(line);
        }
    }
    lines
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

const PUNCTUATION: &str = "{}[]()<>=+-*/%!&|^~?:;,.@#$\\";

struct Scanner<'a> {
    src: &'a str,
    lang: &'a Lang,
    pos: usize,
    /// Only whitespace so far on the current line.
    line_start: bool,
    /// Token kind and byte range; adjacent tokens of one kind are merged.
    out: Vec<(Option<Kind>, usize, usize)>,
}

impl<'a> Scanner<'a> {
    fn new(src: &'a str, lang: &'a Lang) -> Self {
        Scanner {
            src,
            lang,
            pos: 0,
            line_start: true,
            out: Vec::new(),
        }
    }

    fn rest(&self) -> &'a str {
        &self.src[self.pos..]
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn prev(&self) -> Option<char> {
        self.src[..self.pos].chars().next_back()
    }

    fn emit(&mut self, kind: Option<Kind>, len: usize) {
        let start = self.pos;
        let mut end = (start + len).min(self.src.len());
        while !self.src.is_char_boundary(end) {
            end += 1;
        }
        self.pos = end;
        if end == start {
            return;
        }
        match self.out.last_mut() {
            Some((k, _, prev_end)) if *k == kind && *prev_end == start => *prev_end = end,
            _ => self.out.push((kind, start, end)),
        }
    }

    fn run(mut self) -> Vec<(Option<Kind>, &'a str)> {
        while let Some(c) = self.peek() {
            if c == '\n' {
                self.emit(None, 1);
                self.line_start = true;
                continue;
            }
            if c.is_whitespace() {
                self.emit(None, c.len_utf8());
                continue;
            }
            if self.line_start && self.line_start_token() {
                continue;
            }
            self.line_start = false;
            self.token(c);
        }
        self.mark_string_keys();
        let src = self.src;
        self.out
            .into_iter()
            .map(|(kind, start, end)| (kind, &src[start..end]))
            .collect()
    }

    /// Constructs recognized only at the start of a line; true when one was emitted.
    fn line_start_token(&mut self) -> bool {
        let rest = self.rest();
        if self.lang.yaml {
            if rest.starts_with("---") || rest.starts_with("...") {
                self.emit(Some(Kind::Meta), 3);
                self.line_start = false;
                return true;
            }
            if rest.starts_with("- ") {
                // A list item may start with a key, so stay at the line start.
                self.emit(Some(Kind::Punctuation), 1);
                return true;
            }
        }
        if self.lang.sections && rest.starts_with('[') {
            let end = rest.find('\n').unwrap_or(rest.len());
            let close = rest[..end].rfind(']').map_or(end, |i| i + 1);
            self.emit(Some(Kind::Meta), close);
            self.line_start = false;
            return true;
        }
        let key_len = match self.lang.keys {
            Keys::Colon => yaml_key_len(rest),
            Keys::Equals => toml_key_len(rest),
            Keys::None => None,
        };
        if let Some(len) = key_len {
            self.emit(Some(Kind::Property), len);
            self.line_start = false;
            return true;
        }
        false
    }

    fn token(&mut self, c: char) {
        let rest = self.rest();
        let lang = self.lang;
        if let Some(len) = self.comment_len(rest, c) {
            self.emit(Some(Kind::Comment), len);
        } else if let Some(len) = lang.rust.then(|| rust_special(rest, self.prev())).flatten() {
            self.emit(Some(len.0), len.1);
        } else if let Some(len) = self.string_len(rest) {
            self.emit(Some(Kind::String), len);
        } else if c.is_ascii_digit() && !self.prev().is_some_and(is_ident) {
            self.emit(Some(Kind::Number), number_len(rest));
        } else if is_ident_start(c) {
            self.word(rest);
        } else if lang.annotations && c == '@' && rest[1..].starts_with(is_ident_start) {
            self.emit(Some(Kind::Meta), 1 + ident_len(&rest[1..]));
        } else if lang.variables && c == '$' {
            self.emit(Some(Kind::Property), variable_len(rest));
        } else if lang.yaml
            && (c == '&' || c == '*')
            && !self.prev().is_some_and(|p| !p.is_whitespace())
            && rest[1..].starts_with(is_ident_start)
        {
            self.emit(Some(Kind::Meta), 1 + ident_len(&rest[1..]));
        } else if PUNCTUATION.contains(c) {
            self.emit(Some(Kind::Punctuation), 1);
        } else {
            self.emit(None, c.len_utf8());
        }
    }

    fn comment_len(&self, rest: &str, c: char) -> Option<usize> {
        let lang = self.lang;
        if let Some((open, close)) = lang.block_comment
            && rest.starts_with(open)
        {
            return Some(block_comment_len(rest, open, close, lang.nested_comments));
        }
        let at_boundary = !self.prev().is_some_and(|p| !p.is_whitespace());
        lang.line_comments
            .iter()
            .any(|m| rest.starts_with(m) && (c != '#' || !lang.hash_needs_boundary || at_boundary))
            .then(|| rest.find('\n').unwrap_or(rest.len()))
    }

    fn string_len(&self, rest: &str) -> Option<usize> {
        self.lang
            .quotes
            .iter()
            .find(|q| rest.starts_with(q.open))
            .map(|q| quoted_len(rest, q))
    }

    fn word(&mut self, rest: &str) {
        let lang = self.lang;
        let len = ident_len(rest);
        let word = &rest[..len];
        let after = &rest[len..];
        let kind = if lang.keywords.contains(&word) {
            Some(Kind::Keyword)
        } else if lang.constants.contains(&word) {
            Some(Kind::Constant)
        } else if lang.types.contains(&word) {
            Some(Kind::Type)
        } else if lang.builtins.contains(&word) {
            Some(Kind::Function)
        } else if lang.rust && after.starts_with('!') && !after.starts_with("!=") {
            self.emit(Some(Kind::Function), len + 1);
            return;
        } else if after.trim_start_matches([' ', '\t']).starts_with('(') {
            Some(Kind::Function)
        } else if lang.capitalized_types && word.starts_with(char::is_uppercase) {
            let all_caps = word.len() > 1 && word.chars().all(|c| !c.is_lowercase());
            Some(if all_caps { Kind::Constant } else { Kind::Type })
        } else {
            None
        };
        self.emit(kind, len);
    }

    /// JSON: a string followed by `:` is a key.
    fn mark_string_keys(&mut self) {
        if !self.lang.string_keys {
            return;
        }
        for i in 0..self.out.len() {
            if self.out[i].0 != Some(Kind::String) {
                continue;
            }
            let src = self.src;
            let next = self.out[i + 1..]
                .iter()
                .find(|(k, a, b)| k.is_some() || !src[*a..*b].trim().is_empty());
            if next.is_some_and(|(_, a, _)| src[*a..].starts_with(':')) {
                self.out[i].0 = Some(Kind::Property);
            }
        }
    }
}

#[cfg(test)]
#[path = "highlight_tests.rs"]
mod tests;
