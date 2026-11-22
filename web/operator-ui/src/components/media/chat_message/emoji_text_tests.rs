use super::emoji_text::{EmojiToken, get_isolated_emojis, parse_emoji_tokens};
use crate::components::chat_message::types::CustomEmojiMap;

#[test]
fn isolated_emojis_recognizes_one_to_three_emojis() {
    assert_eq!(get_isolated_emojis("👍"), Some(vec!["👍".to_string()]));
    assert_eq!(
        get_isolated_emojis(" 👍   🎉 "),
        Some(vec!["👍".to_string(), "🎉".to_string()])
    );
    assert_eq!(
        get_isolated_emojis("🔥 :rocket: ⭐"),
        Some(vec![
            "🔥".to_string(),
            ":rocket:".to_string(),
            "⭐".to_string()
        ])
    );
}

#[test]
fn isolated_emojis_rejects_empty_more_than_three_or_text() {
    assert_eq!(get_isolated_emojis(""), None);
    assert_eq!(get_isolated_emojis("   "), None);
    assert_eq!(get_isolated_emojis("👍 hello"), None);
    assert_eq!(get_isolated_emojis("hello 👍"), None);
    assert_eq!(get_isolated_emojis("👍 👍 👍 👍"), None);
    assert_eq!(get_isolated_emojis(":invalid shortcode:"), None);
}

#[test]
fn isolated_emojis_handles_modifiers_and_zwj() {
    // Thumbs up with skin tone
    let skin_tone = "👍🏽";
    assert_eq!(
        get_isolated_emojis(skin_tone),
        Some(vec![skin_tone.to_string()])
    );
}

#[test]
fn parse_emoji_tokens_without_map_returns_text() {
    assert_eq!(
        parse_emoji_tokens("hello :fire:", None),
        vec![EmojiToken::Text("hello :fire:".into())]
    );
}

#[test]
fn parse_emoji_tokens_replaces_shortcodes_and_unicode() {
    let mut map = CustomEmojiMap::new();
    map.insert("fire", "https://example.com/fire.svg");
    map.insert("⭐", "https://example.com/star.svg");

    let tokens = parse_emoji_tokens("hot :fire: and ⭐ cool", Some(&map));
    assert_eq!(
        tokens,
        vec![
            EmojiToken::Text("hot ".into()),
            EmojiToken::CustomEmoji {
                url: "https://example.com/fire.svg".into(),
                label: ":fire:".into(),
            },
            EmojiToken::Text(" and ".into()),
            EmojiToken::CustomEmoji {
                url: "https://example.com/star.svg".into(),
                label: "⭐".into(),
            },
            EmojiToken::Text(" cool".into()),
        ]
    );
}
