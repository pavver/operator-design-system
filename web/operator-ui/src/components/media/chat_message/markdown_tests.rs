use super::markdown::{BlockToken, InlineToken, parse_inline_markdown, parse_markdown_blocks};

fn text(s: &str) -> InlineToken {
    InlineToken::Text(s.to_string())
}

#[test]
fn plain_text_is_one_token() {
    assert_eq!(
        parse_inline_markdown("hello world"),
        vec![text("hello world")]
    );
}

#[test]
fn every_delimited_span_becomes_its_token() {
    let cases = [
        ("***a***", InlineToken::BoldItalic("a".into())),
        ("**a**", InlineToken::Bold("a".into())),
        ("*a*", InlineToken::Italic("a".into())),
        ("~~a~~", InlineToken::Strike("a".into())),
        ("`a`", InlineToken::Code("a".into())),
        ("||a||", InlineToken::Spoiler("a".into())),
    ];
    for (input, token) in cases {
        assert_eq!(parse_inline_markdown(input), vec![token], "{input}");
    }
}

#[test]
fn spans_keep_the_surrounding_text() {
    assert_eq!(
        parse_inline_markdown("a **b** c `d`"),
        vec![
            text("a "),
            InlineToken::Bold("b".into()),
            text(" c "),
            InlineToken::Code("d".into())
        ]
    );
}

#[test]
fn unclosed_delimiters_stay_text() {
    for input in ["**abc", "~~abc", "`abc", "||abc", "*abc", "[label](url"] {
        assert_eq!(parse_inline_markdown(input), vec![text(input)], "{input}");
    }
}

#[test]
fn a_double_star_without_its_pair_is_not_italic() {
    assert_eq!(parse_inline_markdown("**a*"), vec![text("**a*")]);
}

#[test]
fn an_unclosed_triple_star_falls_back_to_bold() {
    assert_eq!(
        parse_inline_markdown("***a**"),
        vec![InlineToken::Bold("*a".into())]
    );
}

#[test]
fn links_are_parsed_and_unsafe_urls_neutralized() {
    assert_eq!(
        parse_inline_markdown("[docs](https://example.com)"),
        vec![InlineToken::Link {
            label: "docs".into(),
            url: "https://example.com".into()
        }]
    );
    assert_eq!(
        parse_inline_markdown("[x]( JavaScript:alert(1)"),
        vec![InlineToken::Link {
            label: "x".into(),
            url: "#".into()
        }]
    );
    assert_eq!(
        parse_inline_markdown("[label] (url)"),
        vec![text("[label] (url)")]
    );
}

#[test]
fn blocks_follow_line_prefixes_and_skip_blank_lines() {
    let blocks = parse_markdown_blocks("# Title\n\n> quote\n- one\n* two\nplain");
    assert_eq!(
        blocks,
        vec![
            BlockToken::Heading(vec![text("Title")]),
            BlockToken::Quote(vec![text("quote")]),
            BlockToken::ListItem(vec![text("one")]),
            BlockToken::ListItem(vec![text("two")]),
            BlockToken::Paragraph(vec![text("plain")]),
        ]
    );
}
