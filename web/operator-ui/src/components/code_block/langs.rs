//! Language descriptions for the built-in highlighter.
//!
//! Word lists follow each language's reference; constructs a line scanner cannot see
//! (string templates, heredocs, YAML block scalars) stay plain.

use super::highlight::{Keys, Lang, Quote};

const fn quote(open: &'static str, close: &'static str, escapes: bool, multiline: bool) -> Quote {
    Quote {
        open,
        close,
        escapes,
        multiline,
    }
}

pub(crate) static RUST: Lang = Lang {
    names: &["rust", "rs"],
    line_comments: &["//"],
    block_comment: Some(("/*", "*/")),
    nested_comments: true,
    quotes: &[quote("\"", "\"", true, true)],
    keywords: &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
        "pub", "ref", "return", "self", "static", "struct", "super", "trait", "type", "union",
        "unsafe", "use", "where", "while", "yield",
    ],
    constants: &["true", "false"],
    types: &[
        "Self", "bool", "char", "str", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16",
        "u32", "u64", "u128", "usize", "f32", "f64",
    ],
    capitalized_types: true,
    rust: true,
    ..Lang::BASE
};

pub(crate) static KOTLIN: Lang = Lang {
    names: &["kotlin", "kt", "kts"],
    line_comments: &["//"],
    block_comment: Some(("/*", "*/")),
    nested_comments: true,
    quotes: &[
        quote("\"\"\"", "\"\"\"", false, true),
        quote("\"", "\"", true, false),
        quote("'", "'", true, false),
    ],
    keywords: &[
        "abstract",
        "annotation",
        "as",
        "break",
        "by",
        "catch",
        "class",
        "companion",
        "const",
        "constructor",
        "continue",
        "crossinline",
        "data",
        "do",
        "else",
        "enum",
        "external",
        "final",
        "finally",
        "for",
        "fun",
        "if",
        "import",
        "in",
        "infix",
        "init",
        "inline",
        "inner",
        "interface",
        "internal",
        "is",
        "lateinit",
        "noinline",
        "object",
        "open",
        "operator",
        "out",
        "override",
        "package",
        "private",
        "protected",
        "public",
        "reified",
        "return",
        "sealed",
        "super",
        "suspend",
        "tailrec",
        "this",
        "throw",
        "try",
        "typealias",
        "val",
        "value",
        "var",
        "vararg",
        "when",
        "where",
        "while",
    ],
    constants: &["true", "false", "null"],
    types: &[
        "Any", "Array", "Boolean", "Byte", "Char", "Double", "Float", "Int", "List", "Long", "Map",
        "Nothing", "Set", "Short", "String", "Unit",
    ],
    capitalized_types: true,
    annotations: true,
    ..Lang::BASE
};

pub(crate) static TOML: Lang = Lang {
    names: &["toml"],
    line_comments: &["#"],
    quotes: &[
        quote("\"\"\"", "\"\"\"", true, true),
        quote("'''", "'''", false, true),
        quote("\"", "\"", true, false),
        quote("'", "'", false, false),
    ],
    constants: &["true", "false", "inf", "nan"],
    keys: Keys::Equals,
    sections: true,
    ..Lang::BASE
};

pub(crate) static JSON: Lang = Lang {
    names: &["json", "jsonc", "json5"],
    line_comments: &["//"],
    block_comment: Some(("/*", "*/")),
    quotes: &[quote("\"", "\"", true, false)],
    constants: &["true", "false", "null"],
    string_keys: true,
    ..Lang::BASE
};

pub(crate) static YAML: Lang = Lang {
    names: &["yaml", "yml"],
    line_comments: &["#"],
    hash_needs_boundary: true,
    quotes: &[
        quote("\"", "\"", true, false),
        quote("'", "'", false, false),
    ],
    constants: &[
        "true", "false", "True", "False", "TRUE", "FALSE", "yes", "no", "on", "off", "null",
        "Null", "NULL", "~",
    ],
    keys: Keys::Colon,
    string_keys: true,
    yaml: true,
    ..Lang::BASE
};

pub(crate) static SHELL: Lang = Lang {
    names: &["bash", "sh", "shell", "zsh", "console"],
    line_comments: &["#"],
    hash_needs_boundary: true,
    quotes: &[quote("\"", "\"", true, true), quote("'", "'", false, true)],
    keywords: &[
        "case", "do", "done", "elif", "else", "esac", "fi", "for", "function", "if", "in",
        "select", "then", "time", "until", "while",
    ],
    constants: &["true", "false"],
    builtins: &[
        "alias", "cd", "echo", "eval", "exec", "exit", "export", "local", "printf", "read",
        "readonly", "return", "set", "shift", "source", "test", "trap", "unset", "wait",
    ],
    variables: true,
    ..Lang::BASE
};

/// Every built-in language.
pub(crate) static ALL: [&Lang; 6] = [&RUST, &KOTLIN, &TOML, &JSON, &YAML, &SHELL];
