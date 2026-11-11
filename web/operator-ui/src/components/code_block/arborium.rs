//! Optional syntax highlighter for CodeBlock using arborium (tree-sitter).
//!
//! Capture name to Kind mapping table:
//!
//! | Capture pattern / prefix                     | Kind        | Token class           | Rationale |
//! |----------------------------------------------|-------------|-----------------------|-----------|
//! | `keyword*`, `include`, `repeat`, `exception` | Keyword     | `op-syn-keyword`      | Language control keywords and directives |
//! | `string*`, `character*`, `escape`            | String      | `op-syn-string`       | Quoted text, char literals, escape codes |
//! | `number*`, `float*`, `constant.numeric`      | Number      | `op-syn-number`       | Numeric literals |
//! | `comment*`                                   | Comment     | `op-syn-comment`      | Line, block, and doc comments |
//! | `type*`, `constructor*`, `structure`         | Type        | `op-syn-type`         | Types, classes, structs, constructors |
//! | `function*`, `method*`, `macro*`             | Function    | `op-syn-function`     | Calls, definitions, macros |
//! | `constant*`, `boolean*`                      | Constant    | `op-syn-constant`     | Named constants, booleans, literals |
//! | `property*`, `field*`, `label*`, `variable.member` | Property | `op-syn-property` | Properties, fields, labels, member vars |
//! | `attribute*`, `annotation*`, `tag*`, `namespace*`, `module*`, `lifetime*`, `preproc*` | Meta | `op-syn-meta` | Metadata, tags, namespaces, lifetimes, preprocessor |
//! | `punctuation*`, `operator*`                  | Punctuation | `op-syn-punctuation`  | Operators, delimiters, brackets |
//! | `variable*` (plain/param), `plain`, `text`   | None        | (unstyled)            | General identifiers and untagged text |
//! | `markup.*`, `diff.*`, `error`, `embedded`    | None        | (unstyled)            | Fallback for non-syntax or diff nodes |

use std::cell::RefCell;
use std::collections::HashMap;

use crate::components::code_block::highlight::{Kind, tokens_to_lines};
use crate::core::styled_text::StyledLine;

thread_local! {
    static HIGHLIGHTER: RefCell<arborium::Highlighter> =
        RefCell::new(arborium::Highlighter::new());
}

#[derive(Clone, Copy)]
struct ResolvedSpan {
    start: usize,
    end: usize,
    kind: Option<Kind>,
    pattern_index: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventKind {
    End,
    Start,
}

struct Event {
    pos: usize,
    kind: EventKind,
    span_idx: usize,
    end: usize,
}

/// Maps a tree-sitter capture name to our syntax [`Kind`].
///
/// Matching primarily inspects the first dotted segment of the capture name,
/// with explicit overrides for compound captures such as `variable.member`.
pub(crate) fn kind_for_capture(name: &str) -> Option<Kind> {
    let name = name.strip_prefix('@').unwrap_or(name);

    // Multi-segment overrides and special cases
    if name == "variable.member" || name == "variable.other.member" {
        return Some(Kind::Property);
    }
    if name == "function.macro" || name == "preproc.macro" {
        return Some(Kind::Function);
    }
    if name == "constant.numeric" || name.starts_with("constant.numeric.") {
        return Some(Kind::Number);
    }
    if name == "constant.builtin.boolean" {
        return Some(Kind::Constant);
    }
    if name == "keyword.operator" {
        return Some(Kind::Keyword);
    }

    let first = name.split('.').next().unwrap_or(name);
    match first {
        // Keywords and variants
        "keyword" | "include" | "conditional" | "repeat" | "exception" | "storageclass" => {
            Some(Kind::Keyword)
        }
        // Strings, characters, and escapes
        "string" | "character" | "escape" => Some(Kind::String),
        // Numbers and floats
        "number" | "float" => Some(Kind::Number),
        // Comments
        "comment" => Some(Kind::Comment),
        // Types and constructors
        "type" | "constructor" | "structure" => Some(Kind::Type),
        // Functions, methods, and macros
        "function" | "method" | "macro" => Some(Kind::Function),
        // Constants and booleans
        "constant" | "boolean" => Some(Kind::Constant),
        // Properties, fields, and labels
        "property" | "field" | "label" => Some(Kind::Property),
        // Metadata: attributes, annotations, tags, namespaces, lifetimes, preprocessors
        "attribute" | "annotation" | "tag" | "namespace" | "module" | "lifetime" | "preproc"
        | "define" => Some(Kind::Meta),
        // Punctuation and operators
        "punctuation" | "operator" | "delimiter" | "bracket" => Some(Kind::Punctuation),
        // Plain variables, parameters, text, markup, errors, etc. produce no token styling
        _ => None,
    }
}

/// Highlights `code` using arborium's tree-sitter parser.
///
/// Returns `None` if `language` is not supported (grammar not compiled in) or if parsing fails.
/// Never panics.
pub(crate) fn highlight(code: &str, language: &str) -> Option<Vec<StyledLine>> {
    let code = code.trim_end_matches('\n');

    let raw_spans = HIGHLIGHTER.with(|hl| {
        let mut hl = hl.try_borrow_mut().ok()?;
        hl.highlight_spans(language, code).ok()
    })?;

    // Filter and deduplicate spans with identical ranges.
    let mut deduped: HashMap<(usize, usize), ResolvedSpan> = HashMap::new();
    for span in raw_spans {
        let start = span.start as usize;
        let end = span.end as usize;
        if start >= end
            || end > code.len()
            || !code.is_char_boundary(start)
            || !code.is_char_boundary(end)
        {
            continue;
        }
        let kind = kind_for_capture(&span.capture);
        let resolved = ResolvedSpan {
            start,
            end,
            kind,
            pattern_index: span.pattern_index,
        };
        let key = (start, end);
        match deduped.get(&key) {
            Some(existing) => {
                let replace = match (resolved.kind.is_some(), existing.kind.is_some()) {
                    (true, false) => true,
                    (false, true) => false,
                    _ => resolved.pattern_index >= existing.pattern_index,
                };
                if replace {
                    deduped.insert(key, resolved);
                }
            }
            None => {
                deduped.insert(key, resolved);
            }
        }
    }

    let spans: Vec<ResolvedSpan> = deduped.into_values().collect();

    let mut events = Vec::with_capacity(spans.len() * 2);
    for (i, span) in spans.iter().enumerate() {
        events.push(Event {
            pos: span.start,
            kind: EventKind::Start,
            span_idx: i,
            end: span.end,
        });
        events.push(Event {
            pos: span.end,
            kind: EventKind::End,
            span_idx: i,
            end: span.end,
        });
    }

    events.sort_by(|a, b| {
        a.pos
            .cmp(&b.pos)
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| {
                if a.kind == EventKind::Start {
                    b.end.cmp(&a.end)
                } else {
                    std::cmp::Ordering::Equal
                }
            })
    });

    let mut last_pos = 0;
    let mut stack: Vec<usize> = Vec::new();
    let mut tokens: Vec<(Option<Kind>, &str)> = Vec::new();

    for event in events {
        let pos = event.pos;
        if pos > last_pos && pos <= code.len() {
            if let Some(text) = code.get(last_pos..pos) {
                let current_kind = stack
                    .last()
                    .and_then(|&idx| spans.get(idx).and_then(|s| s.kind));
                tokens.push((current_kind, text));
            }
            last_pos = pos;
        }
        match event.kind {
            EventKind::Start => stack.push(event.span_idx),
            EventKind::End => {
                if let Some(p) = stack.iter().rposition(|&idx| idx == event.span_idx) {
                    stack.remove(p);
                }
            }
        }
    }

    if last_pos < code.len()
        && let Some(text) = code.get(last_pos..code.len())
    {
        tokens.push((None, text));
    }

    Some(tokens_to_lines(tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_for_capture_mappings() {
        assert_eq!(kind_for_capture("keyword"), Some(Kind::Keyword));
        assert_eq!(kind_for_capture("keyword.function"), Some(Kind::Keyword));
        assert_eq!(
            kind_for_capture("keyword.control.repeat"),
            Some(Kind::Keyword)
        );
        assert_eq!(kind_for_capture("keyword.operator"), Some(Kind::Keyword));
        assert_eq!(kind_for_capture("include"), Some(Kind::Keyword));
        assert_eq!(kind_for_capture("conditional"), Some(Kind::Keyword));
        assert_eq!(kind_for_capture("repeat"), Some(Kind::Keyword));

        assert_eq!(kind_for_capture("string"), Some(Kind::String));
        assert_eq!(kind_for_capture("string.special"), Some(Kind::String));
        assert_eq!(kind_for_capture("character"), Some(Kind::String));
        assert_eq!(kind_for_capture("escape"), Some(Kind::String));

        assert_eq!(kind_for_capture("number"), Some(Kind::Number));
        assert_eq!(kind_for_capture("float"), Some(Kind::Number));
        assert_eq!(kind_for_capture("constant.numeric"), Some(Kind::Number));

        assert_eq!(kind_for_capture("comment"), Some(Kind::Comment));
        assert_eq!(
            kind_for_capture("comment.documentation"),
            Some(Kind::Comment)
        );

        assert_eq!(kind_for_capture("type"), Some(Kind::Type));
        assert_eq!(kind_for_capture("type.builtin"), Some(Kind::Type));
        assert_eq!(kind_for_capture("constructor"), Some(Kind::Type));
        assert_eq!(kind_for_capture("structure"), Some(Kind::Type));

        assert_eq!(kind_for_capture("function"), Some(Kind::Function));
        assert_eq!(kind_for_capture("function.builtin"), Some(Kind::Function));
        assert_eq!(kind_for_capture("method"), Some(Kind::Function));
        assert_eq!(kind_for_capture("macro"), Some(Kind::Function));
        assert_eq!(kind_for_capture("function.macro"), Some(Kind::Function));
        assert_eq!(kind_for_capture("preproc.macro"), Some(Kind::Function));

        assert_eq!(kind_for_capture("constant"), Some(Kind::Constant));
        assert_eq!(kind_for_capture("constant.builtin"), Some(Kind::Constant));
        assert_eq!(kind_for_capture("boolean"), Some(Kind::Constant));
        assert_eq!(
            kind_for_capture("constant.builtin.boolean"),
            Some(Kind::Constant)
        );

        assert_eq!(kind_for_capture("property"), Some(Kind::Property));
        assert_eq!(kind_for_capture("field"), Some(Kind::Property));
        assert_eq!(kind_for_capture("label"), Some(Kind::Property));
        assert_eq!(kind_for_capture("variable.member"), Some(Kind::Property));
        assert_eq!(
            kind_for_capture("variable.other.member"),
            Some(Kind::Property)
        );

        assert_eq!(kind_for_capture("attribute"), Some(Kind::Meta));
        assert_eq!(kind_for_capture("annotation"), Some(Kind::Meta));
        assert_eq!(kind_for_capture("tag"), Some(Kind::Meta));
        assert_eq!(kind_for_capture("namespace"), Some(Kind::Meta));
        assert_eq!(kind_for_capture("module"), Some(Kind::Meta));
        assert_eq!(kind_for_capture("lifetime"), Some(Kind::Meta));
        assert_eq!(kind_for_capture("preproc"), Some(Kind::Meta));
        assert_eq!(kind_for_capture("define"), Some(Kind::Meta));

        assert_eq!(kind_for_capture("punctuation"), Some(Kind::Punctuation));
        assert_eq!(
            kind_for_capture("punctuation.bracket"),
            Some(Kind::Punctuation)
        );
        assert_eq!(
            kind_for_capture("punctuation.delimiter"),
            Some(Kind::Punctuation)
        );
        assert_eq!(kind_for_capture("operator"), Some(Kind::Punctuation));

        // Unstyled captures
        assert_eq!(kind_for_capture("variable"), None);
        assert_eq!(kind_for_capture("variable.parameter"), None);
        assert_eq!(kind_for_capture("parameter"), None);
        assert_eq!(kind_for_capture("plain"), None);
        assert_eq!(kind_for_capture("text"), None);
        assert_eq!(kind_for_capture("unknown_custom_capture"), None);

        // Leading @ prefix handling
        assert_eq!(kind_for_capture("@keyword"), Some(Kind::Keyword));
        assert_eq!(kind_for_capture("@variable.member"), Some(Kind::Property));
    }

    #[test]
    fn test_unsupported_language_returns_none() {
        assert!(highlight("let x = 1;", "nonexistent_lang_12345").is_none());
    }

    #[test]
    fn test_rust_highlighting() {
        let snippet = "fn main() { let s = \"hello\"; }";
        if let Some(lines) = highlight(snippet, "rust") {
            let classes: Vec<&str> = lines
                .iter()
                .flat_map(|line| line.iter())
                .filter_map(|span| span.style.class)
                .collect();
            assert!(
                classes.contains(&"op-syn-keyword"),
                "expected keyword class"
            );
            assert!(classes.contains(&"op-syn-string"), "expected string class");
        }
    }
}
