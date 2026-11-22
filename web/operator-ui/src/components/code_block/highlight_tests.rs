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
    let code = "#![no_std]\n#[derive(Debug)]\npub fn main() -> Result<u8, E> {\n    let s = r#\"raw \"q\"\"#; // note\n    let b = b\"byte\"; let bc = b'k';\n    let c = '\\n'; let l: &'a str = \"x\";\n    println!(\"{}\", MAX_LEN + 0x1F + 1.5e-3);\n}";
    assert_eq!(kind_of("rust", code, "#![no_std]"), "op-syn-meta");
    assert_eq!(kind_of("rust", code, "#[derive(Debug)]"), "op-syn-meta");
    assert_eq!(kind_of("rust", code, "pub"), "op-syn-keyword");
    assert_eq!(kind_of("rust", code, "main"), "op-syn-function");
    assert_eq!(kind_of("rust", code, "Result"), "op-syn-type");
    assert_eq!(kind_of("rust", code, "u8"), "op-syn-type");
    assert_eq!(kind_of("rust", code, "r#\"raw \"q\"\"#"), "op-syn-string");
    assert_eq!(kind_of("rust", code, "b\"byte\""), "op-syn-string");
    assert_eq!(kind_of("rust", code, "b'k'"), "op-syn-string");
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
