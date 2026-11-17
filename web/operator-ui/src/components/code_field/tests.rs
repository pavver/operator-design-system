use super::text::*;

#[test]
fn segments_colour_and_underline() {
    let text = "{{ name | upper }}";
    let spans = vec![
        CodeSpan::new(0..2, CodeToken::Punctuation),
        CodeSpan::new(3..7, CodeToken::Property),
        CodeSpan::new(10..15, CodeToken::Function),
        CodeSpan::new(3..15, CodeToken::Error).message("unknown filter"),
    ];
    let segs = segments(text, &spans);
    let joined: String = segs.iter().map(|s| s.text.as_str()).collect();
    assert_eq!(joined, text);
    let name = segs
        .iter()
        .find(|s| s.text == "name")
        .expect("name segment");
    assert_eq!(name.classes, ["op-code-property", "op-code-error"]);
    let gap = segs.iter().find(|s| s.text == " | ").expect("gap");
    assert_eq!(gap.classes, ["op-code-error"]);
    assert_eq!(segs[0].classes, ["op-code-punctuation"]);
    assert_eq!(
        first_message(&spans),
        Some((CodeToken::Error, "unknown filter".into()))
    );
}

#[test]
fn segments_clamp_to_char_boundaries() {
    let text = "ає";
    let segs = segments(text, &[CodeSpan::new(1..100, CodeToken::String)]);
    let joined: String = segs.iter().map(|s| s.text.as_str()).collect();
    assert_eq!(joined, text);
}

#[test]
fn offsets_convert_across_wide_chars() {
    let text = "a😀b";
    assert_eq!(utf16_to_byte(text, 1), 1);
    assert_eq!(utf16_to_byte(text, 3), 5);
    assert_eq!(utf16_to_byte(text, 2), 1);
    assert_eq!(byte_to_utf16(text, 5), 3);
    assert_eq!(byte_to_utf16(text, 100), 4);
}

#[test]
fn splice_replaces_and_places_the_caret() {
    assert_eq!(
        splice("{{ na }}", 3..5, "name"),
        ("{{ name }}".to_string(), 7)
    );
    assert_eq!(splice("ab", 5..9, "c"), ("abc".to_string(), 3));
}
