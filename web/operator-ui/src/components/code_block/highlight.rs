//! Lightweight syntax highlighter for CodeBlock.
//!
//! Each language is a declarative [`Lang`] (comments, quotes, word lists, a few flags),
//! in the spirit of highlight.js modes; one scanner handles every language.
//! The result is display-grade: it never fails and never rejects input.

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

fn ident_len(s: &str) -> usize {
    s.char_indices()
        .find(|&(_, c)| !is_ident(c))
        .map_or(s.len(), |(i, _)| i)
}

/// Digits, radix prefixes, separators, fractions, exponents, and type suffixes.
fn number_len(s: &str) -> usize {
    let bytes = s.as_bytes();
    let hex = s.starts_with("0x") || s.starts_with("0X");
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let next_digit = bytes.get(i + 1).is_some_and(u8::is_ascii_digit);
        let after_exp = i > 0 && matches!(bytes[i - 1], b'e' | b'E') && !hex;
        let ok = b.is_ascii_alphanumeric()
            || b == b'_'
            || (b == b'.' && next_digit)
            || ((b == b'+' || b == b'-') && after_exp && next_digit);
        if !ok {
            break;
        }
        i += 1;
    }
    i.max(1)
}

fn block_comment_len(s: &str, open: &str, close: &str, nested: bool) -> usize {
    let mut depth = 0usize;
    let mut i = 0;
    while i < s.len() {
        let rest = &s[i..];
        if rest.starts_with(open) && (nested || depth == 0) {
            depth += 1;
            i += open.len();
        } else if rest.starts_with(close) {
            depth = depth.saturating_sub(1);
            i += close.len();
            if depth == 0 {
                return i;
            }
        } else {
            i += rest.chars().next().map_or(1, char::len_utf8);
        }
    }
    s.len()
}

fn quoted_len(s: &str, q: &Quote) -> usize {
    let mut i = q.open.len();
    while i < s.len() {
        let rest = &s[i..];
        if rest.starts_with(q.close) {
            return i + q.close.len();
        }
        let Some(c) = rest.chars().next() else { break };
        if c == '\n' && !q.multiline {
            return i;
        }
        i += c.len_utf8();
        if c == '\\' && q.escapes {
            i += s[i..].chars().next().map_or(0, char::len_utf8);
        }
    }
    s.len()
}

fn variable_len(s: &str) -> usize {
    let rest = &s[1..];
    if rest.starts_with('{') {
        return rest.find('}').map_or(s.len(), |i| i + 2);
    }
    match rest.chars().next() {
        Some(c) if is_ident_start(c) => 1 + ident_len(rest),
        Some(c) if c.is_ascii_digit() || "@#?$!*-".contains(c) => 2,
        _ => 1,
    }
}

/// `key:` at the start of a YAML line, with no quote or comment inside the key.
fn yaml_key_len(s: &str) -> Option<usize> {
    let line = &s[..s.find('\n').unwrap_or(s.len())];
    let colon = line
        .char_indices()
        .take_while(|&(_, c)| !"#\"'{[,&*!|>".contains(c))
        .find(|&(i, c)| c == ':' && line[i + 1..].chars().next().is_none_or(char::is_whitespace))
        .map(|(i, _)| i)?;
    (colon > 0).then_some(colon)
}

/// Bare or dotted TOML key followed by `=`.
fn toml_key_len(s: &str) -> Option<usize> {
    let len = s
        .char_indices()
        .find(|&(_, c)| !(is_ident(c) || c == '-' || c == '.'))
        .map_or(s.len(), |(i, _)| i);
    let after = s[len..].trim_start_matches([' ', '\t']);
    (len > 0 && after.starts_with('=') && !after.starts_with("==")).then_some(len)
}

/// Rust-only tokens: raw and byte strings, char literals, lifetimes, attributes.
fn rust_special(s: &str, prev: Option<char>) -> Option<(Kind, usize)> {
    if prev.is_some_and(is_ident) {
        return None;
    }
    let body = s.strip_prefix('b').unwrap_or(s);
    let prefix = s.len() - body.len();
    if let Some(raw) = body.strip_prefix('r') {
        let hashes = raw.len() - raw.trim_start_matches('#').len();
        if raw[hashes..].starts_with('"') {
            let close = format!("\"{}", "#".repeat(hashes));
            let start = prefix + 1 + hashes + 1;
            let end = s[start..]
                .find(&close)
                .map_or(s.len(), |i| start + i + close.len());
            return Some((Kind::String, end));
        }
    }
    if prefix == 1 && body.starts_with('"') {
        let q = Quote {
            open: "\"",
            close: "\"",
            escapes: true,
            multiline: true,
        };
        return Some((Kind::String, 1 + quoted_len(body, &q)));
    }
    if let Some(after_quote) = body.strip_prefix('\'') {
        let q = Quote {
            open: "'",
            close: "'",
            escapes: true,
            multiline: false,
        };
        let len = quoted_len(body, &q);
        let inner = body.get(1..len.saturating_sub(1)).unwrap_or("");
        let is_char = body[..len].ends_with('\'')
            && len > 2
            && (inner.starts_with('\\') || inner.chars().count() == 1);
        if is_char {
            return Some((Kind::String, prefix + len));
        }
        if prefix == 0 && after_quote.starts_with(is_ident_start) {
            return Some((Kind::Meta, 1 + ident_len(after_quote)));
        }
    }
    if prefix == 0 && (s.starts_with("#[") || s.starts_with("#![")) {
        let mut depth = 0usize;
        for (i, c) in s.char_indices() {
            match c {
                '[' => depth += 1,
                ']' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return Some((Kind::Meta, i + 1));
                    }
                }
                _ => {}
            }
        }
        return Some((Kind::Meta, s.len()));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tokens of one language as `(class or "", text)`, without whitespace.
    fn tokens(lang: &str, code: &str) -> Vec<(&'static str, String)> {
        highlight(code, find(lang))
            .into_iter()
            .flatten()
            .map(|s| (s.style.class.unwrap_or(""), s.text))
            .filter(|(_, t)| !t.trim().is_empty())
            .collect()
    }

    fn kind_of(lang: &str, code: &str, text: &str) -> &'static str {
        tokens(lang, code)
            .into_iter()
            .find(|(_, t)| t.trim() == text)
            .map_or("missing", |(k, _)| k)
    }

    #[test]
    fn unknown_language_is_plain_and_keeps_lines() {
        let lines = highlight("a\nb\n\nc\n", find("cobol"));
        assert_eq!(lines.len(), 4);
        assert!(lines[2].is_empty());
        assert_eq!(lines[3][0].style, SpanStyle::default());
    }

    #[test]
    fn find_uses_aliases_and_ignores_case() {
        assert!(find("RS").is_some());
        assert!(find("kts").is_some());
        assert!(find("yml").is_some());
        assert!(find("sh").is_some());
    }

    #[test]
    fn rust_tokens() {
        let code = "#[derive(Debug)]\npub fn main() -> Result<u8, E> {\n    let s = r#\"raw \"q\"\"#; // note\n    let c = '\\n'; let l: &'a str = \"x\";\n    println!(\"{}\", MAX_LEN + 0x1F + 1.5e-3);\n}";
        assert_eq!(kind_of("rust", code, "#[derive(Debug)]"), "op-syn-meta");
        assert_eq!(kind_of("rust", code, "pub"), "op-syn-keyword");
        assert_eq!(kind_of("rust", code, "main"), "op-syn-function");
        assert_eq!(kind_of("rust", code, "Result"), "op-syn-type");
        assert_eq!(kind_of("rust", code, "u8"), "op-syn-type");
        assert_eq!(kind_of("rust", code, "r#\"raw \"q\"\"#"), "op-syn-string");
        assert_eq!(kind_of("rust", code, "// note"), "op-syn-comment");
        assert_eq!(kind_of("rust", code, "'\\n'"), "op-syn-string");
        assert_eq!(kind_of("rust", code, "'a"), "op-syn-meta");
        assert_eq!(kind_of("rust", code, "println!"), "op-syn-function");
        assert_eq!(kind_of("rust", code, "MAX_LEN"), "op-syn-constant");
        assert_eq!(kind_of("rust", code, "0x1F"), "op-syn-number");
        assert_eq!(kind_of("rust", code, "1.5e-3"), "op-syn-number");
    }

    #[test]
    fn nested_block_comments_and_multiline_strings_keep_lines() {
        let lines = highlight("/* a /* b */ c */ x\n\"one\ntwo\"", find("rust"));
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0][0].text, "/* a /* b */ c */");
        assert_eq!(lines[0][0].style.class, Some("op-syn-comment"));
        assert_eq!(lines[1][0].style.class, Some("op-syn-string"));
        assert_eq!(lines[2][0].text, "two\"");
        assert_eq!(lines[2][0].style.class, Some("op-syn-string"));
    }

    #[test]
    fn ranges_are_not_floats() {
        assert_eq!(kind_of("rust", "0..10", "0"), "op-syn-number");
        assert_eq!(kind_of("rust", "0..10", "10"), "op-syn-number");
    }

    #[test]
    fn kotlin_tokens() {
        let code = "@Composable\nfun Screen(name: String) {\n    val n = 10L\n    if (name == null) return \"\"\"multi\"\"\"\n}";
        assert_eq!(kind_of("kotlin", code, "@Composable"), "op-syn-meta");
        assert_eq!(kind_of("kotlin", code, "fun"), "op-syn-keyword");
        assert_eq!(kind_of("kotlin", code, "Screen"), "op-syn-function");
        assert_eq!(kind_of("kotlin", code, "String"), "op-syn-type");
        assert_eq!(kind_of("kotlin", code, "10L"), "op-syn-number");
        assert_eq!(kind_of("kotlin", code, "null"), "op-syn-constant");
        assert_eq!(
            kind_of("kotlin", code, "\"\"\"multi\"\"\""),
            "op-syn-string"
        );
    }

    #[test]
    fn toml_tokens() {
        let code = "[server.http] # main\nport = 8080\nname = \"op\"\nflags.debug = true";
        assert_eq!(kind_of("toml", code, "[server.http]"), "op-syn-meta");
        assert_eq!(kind_of("toml", code, "# main"), "op-syn-comment");
        assert_eq!(kind_of("toml", code, "port"), "op-syn-property");
        assert_eq!(kind_of("toml", code, "8080"), "op-syn-number");
        assert_eq!(kind_of("toml", code, "flags.debug"), "op-syn-property");
        assert_eq!(kind_of("toml", code, "true"), "op-syn-constant");
    }

    #[test]
    fn json_keys_and_values() {
        let code = "{\"id\": 7, \"name\" : \"cam\", \"on\": false, \"x\": null}";
        assert_eq!(kind_of("json", code, "\"id\""), "op-syn-property");
        assert_eq!(kind_of("json", code, "\"name\""), "op-syn-property");
        assert_eq!(kind_of("json", code, "\"cam\""), "op-syn-string");
        assert_eq!(kind_of("json", code, "false"), "op-syn-constant");
    }

    #[test]
    fn yaml_tokens() {
        let code = "---\nserver: &base\n  host: example.com # prod\n  - name: cam\n    url: http://x:80\nuse: *base\nok: yes";
        assert_eq!(kind_of("yaml", code, "---"), "op-syn-meta");
        assert_eq!(kind_of("yaml", code, "server"), "op-syn-property");
        assert_eq!(kind_of("yaml", code, "&base"), "op-syn-meta");
        assert_eq!(kind_of("yaml", code, "name"), "op-syn-property");
        assert_eq!(kind_of("yaml", code, "url"), "op-syn-property");
        assert_eq!(kind_of("yaml", code, "# prod"), "op-syn-comment");
        assert_eq!(kind_of("yaml", code, "*base"), "op-syn-meta");
        assert_eq!(kind_of("yaml", code, "yes"), "op-syn-constant");
    }

    #[test]
    fn shell_tokens() {
        let code = "# deploy\nexport PORT=8080\nif [ \"$1\" = up ]; then echo ${HOME} $#; fi";
        assert_eq!(kind_of("bash", code, "# deploy"), "op-syn-comment");
        assert_eq!(kind_of("bash", code, "export"), "op-syn-function");
        assert_eq!(kind_of("bash", code, "if"), "op-syn-keyword");
        assert_eq!(kind_of("bash", code, "\"$1\""), "op-syn-string");
        assert_eq!(kind_of("bash", code, "${HOME}"), "op-syn-property");
        assert_eq!(kind_of("bash", code, "$#"), "op-syn-property");
    }

    #[test]
    fn unterminated_constructs_do_not_panic() {
        for lang in ["rust", "kotlin", "toml", "json", "yaml", "bash"] {
            for code in [
                "\"abc", "/* open", "r#\"x", "'", "#[x", "${", "0x", "é'ü", "key:",
            ] {
                let lines = highlight(code, find(lang));
                let text: String = lines.into_iter().flatten().map(|s| s.text).collect();
                assert_eq!(text, code, "{lang}: {code}");
            }
        }
    }
}
