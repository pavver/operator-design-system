//! Emoji text renderer and standalone isolated big emoji viewer.

use crate::components::chat_message::delivery_status::DeliveryStatusView;
use crate::components::chat_message::reactions::ReactionsRow;
use crate::components::chat_message::types::{CustomEmojiMap, DeliveryStatus, MessageReaction};
use crate::core::{root_class, root_style};
use leptos::prelude::*;

/// Checks if a character belongs to standard Unicode emoji ranges.
pub fn is_emoji_char(c: char) -> bool {
    matches!(c,
        '\u{2600}'..='\u{27BF}'   // Miscellaneous Symbols and Dingbats
        | '\u{2B50}'..='\u{2B55}' // Stars and shapes
        | '\u{1F1E6}'..='\u{1F1FF}' // Regional indicator flags
        | '\u{1F300}'..='\u{1F5FF}' // Miscellaneous Symbols and Pictographs
        | '\u{1F600}'..='\u{1F64F}' // Emoticons
        | '\u{1F680}'..='\u{1F6FF}' // Transport and Map Symbols
        | '\u{1F700}'..='\u{1F77F}' // Alchemical Symbols
        | '\u{1F780}'..='\u{1F7FF}' // Geometric Shapes Extended
        | '\u{1F800}'..='\u{1F8FF}' // Supplemental Arrows-C
        | '\u{1F900}'..='\u{1F9FF}' // Supplemental Symbols and Pictographs
        | '\u{1FA00}'..='\u{1FA6F}' // Chess and Games
        | '\u{1FA70}'..='\u{1FAFF}' // Symbols and Pictographs Extended-A
    )
}

fn parse_shortcode(chars: &[char], i: usize) -> Option<(String, usize)> {
    if chars[i] != ':' {
        return None;
    }
    let len = chars.len();
    let mut j = i + 1;
    while j < len && chars[j] != ':' && !chars[j].is_whitespace() {
        j += 1;
    }
    if j < len && chars[j] == ':' && j > i + 1 {
        let code: String = chars[i..=j].iter().collect();
        Some((code, j + 1))
    } else {
        None
    }
}

fn parse_unicode_emoji(chars: &[char], i: usize) -> (String, usize) {
    let start = i;
    let len = chars.len();
    let mut j = i + 1;
    while j < len {
        let c = chars[j];
        if c == '\u{200D}'
            || c == '\u{FE0F}'
            || ('\u{1F3FB}'..='\u{1F3FF}').contains(&c)
            || is_emoji_char(c)
        {
            j += 1;
        } else {
            break;
        }
    }
    (chars[start..j].iter().collect(), j)
}

fn parse_single_emoji(chars: &[char], i: usize) -> Option<(String, usize)> {
    if chars[i] == ':' {
        parse_shortcode(chars, i)
    } else if is_emoji_char(chars[i]) {
        Some(parse_unicode_emoji(chars, i))
    } else {
        None
    }
}

/// Tests whether text consists exclusively of 1 to 3 isolated emojis.
pub fn get_isolated_emojis(text: &str) -> Option<Vec<String>> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    let chars: Vec<char> = trimmed.chars().collect();
    let mut emojis = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        let (emoji, next_i) = parse_single_emoji(&chars, i)?;
        emojis.push(emoji);
        if emojis.len() > 3 {
            return None;
        }
        i = next_i;
    }

    if (1..=3).contains(&emojis.len()) {
        Some(emojis)
    } else {
        None
    }
}

/// Token representing either raw text or a replaced custom emoji.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmojiToken {
    /// Normal text snippet.
    Text(String),
    /// Custom emoji image/svg URL with fallback label.
    CustomEmoji { url: String, label: String },
}

fn match_custom_emoji(
    chars: &[char],
    i: usize,
    map: &CustomEmojiMap,
) -> Option<(EmojiToken, usize)> {
    if chars[i] == ':' {
        let (code, end) = parse_shortcode(chars, i)?;
        let name = &code[1..code.len() - 1];
        if let Some(url) = map.get(name) {
            return Some((
                EmojiToken::CustomEmoji {
                    url: url.clone(),
                    label: code,
                },
                end,
            ));
        }
    } else if is_emoji_char(chars[i]) {
        let start = i;
        let mut k = i + 1;
        while k < chars.len()
            && (chars[k] == '\u{FE0F}' || ('\u{1F3FB}'..='\u{1F3FF}').contains(&chars[k]))
        {
            k += 1;
        }
        let glyph: String = chars[start..k].iter().collect();
        if let Some(url) = map.get(&glyph) {
            return Some((
                EmojiToken::CustomEmoji {
                    url: url.clone(),
                    label: glyph,
                },
                k,
            ));
        }
    }
    None
}

/// Splits text into normal text and custom emoji tokens using the provided map.
pub fn parse_emoji_tokens(text: &str, map: Option<&CustomEmojiMap>) -> Vec<EmojiToken> {
    let Some(map) = map else {
        return vec![EmojiToken::Text(text.to_string())];
    };

    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut plain_buf = String::new();
    let mut i = 0;

    while i < chars.len() {
        if let Some((tok, next_i)) = match_custom_emoji(&chars, i, map) {
            if !plain_buf.is_empty() {
                tokens.push(EmojiToken::Text(std::mem::take(&mut plain_buf)));
            }
            tokens.push(tok);
            i = next_i;
            continue;
        }

        plain_buf.push(chars[i]);
        i += 1;
    }

    if !plain_buf.is_empty() {
        tokens.push(EmojiToken::Text(plain_buf));
    }

    tokens
}

/// Text renderer replacing registered custom emojis with high-resolution graphics.
#[component]
pub fn EmojiText(
    /// Input text string.
    #[prop(into)]
    text: MaybeProp<String>,
    /// Optional local custom emoji registry map.
    #[prop(optional)]
    emoji_map: Option<CustomEmojiMap>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    // Check context for app-wide custom emoji registry if not provided in props
    let context_map = use_context::<CustomEmojiMap>();
    let effective_map = emoji_map.or(context_map);

    let parsed_tokens = move || {
        let content = text.get().unwrap_or_default();
        parse_emoji_tokens(&content, effective_map.as_ref())
    };

    view! {
        <span class=root_class("op-emoji-text", class) style=root_style(vec![], style)>
            {move || {
                parsed_tokens().into_iter().map(|token| match token {
                    EmojiToken::Text(t) => view! { <span>{t}</span> }.into_any(),
                    EmojiToken::CustomEmoji { url, label } => view! {
                        <img
                            class="op-chat-custom-emoji"
                            src=url
                            alt=label.clone()
                            aria-label=label
                        />
                    }.into_any(),
                }).collect_view()
            }}
        </span>
    }
}

/// Standalone bubble-less presentation for messages containing only 1-3 emojis.
#[component]
pub fn IsolatedEmojiView(
    /// The isolated emojis (1 to 3 items).
    emojis: Vec<String>,
    /// Timestamp stamp string.
    #[prop(into)]
    timestamp: String,
    /// Outgoing delivery status.
    #[prop(optional, into)]
    delivery_status: MaybeProp<DeliveryStatus>,
    /// Message direction flag.
    #[prop(optional)]
    is_outgoing: bool,
    /// Attached reaction counts.
    #[prop(optional)]
    reactions: Vec<MessageReaction>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let size_du = match emojis.len() {
        1 => 20.0,
        2 => 14.0,
        _ => 11.0,
    };

    let emoji_size_style = format!("--op-isolated-emoji-size: calc(var(--op-du) * {size_du});");
    let combined_style = move || {
        let extra = style.get().unwrap_or_default();
        format!("{emoji_size_style}{extra}")
    };

    view! {
        <div
            class=root_class("op-isolated-emoji-container", class)
            data-sent=is_outgoing.then_some("")
            style=combined_style
        >
            <div class="op-isolated-emoji-row">
                {emojis.into_iter().map(|emoji| {
                    view! {
                        <span class="op-isolated-emoji-item">
                            {emoji}
                        </span>
                    }
                }).collect_view()}
                <div class="op-isolated-emoji-badge">
                    <span class="op-chat-message-time">{timestamp.clone()}</span>
                    {is_outgoing.then(|| {
                        delivery_status.get().map(|st| view! {
                            <DeliveryStatusView status=st />
                        })
                    })}
                </div>
            </div>
            {(!reactions.is_empty()).then(|| {
                let reactions_sig = Signal::derive(move || reactions.clone());
                view! {
                    <ReactionsRow reactions=reactions_sig />
                }
            })}
        </div>
    }
}
