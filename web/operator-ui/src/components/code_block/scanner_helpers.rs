//! Private scanners used by the lightweight syntax highlighter.

use super::{Kind, Quote, is_ident, is_ident_start};

pub(super) fn ident_len(s: &str) -> usize {
    s.char_indices()
        .find(|&(_, c)| !is_ident(c))
        .map_or(s.len(), |(i, _)| i)
}

/// Digits, radix prefixes, separators, fractions, exponents, and type suffixes.
pub(super) fn number_len(s: &str) -> usize {
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

pub(super) fn block_comment_len(s: &str, open: &str, close: &str, nested: bool) -> usize {
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

pub(super) fn quoted_len(s: &str, q: &Quote) -> usize {
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

pub(super) fn variable_len(s: &str) -> usize {
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
pub(super) fn yaml_key_len(s: &str) -> Option<usize> {
    let line = &s[..s.find('\n').unwrap_or(s.len())];
    let colon = line
        .char_indices()
        .take_while(|&(_, c)| !"#\"'{[,&*!|>".contains(c))
        .find(|&(i, c)| c == ':' && line[i + 1..].chars().next().is_none_or(char::is_whitespace))
        .map(|(i, _)| i)?;
    (colon > 0).then_some(colon)
}

/// Bare or dotted TOML key followed by `=`.
pub(super) fn toml_key_len(s: &str) -> Option<usize> {
    let len = s
        .char_indices()
        .find(|&(_, c)| !(is_ident(c) || c == '-' || c == '.'))
        .map_or(s.len(), |(i, _)| i);
    let after = s[len..].trim_start_matches([' ', '\t']);
    (len > 0 && after.starts_with('=') && !after.starts_with("==")).then_some(len)
}

/// Rust-only tokens: raw and byte strings, char literals, lifetimes, attributes.
pub(super) fn rust_special(s: &str, prev: Option<char>) -> Option<(Kind, usize)> {
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
