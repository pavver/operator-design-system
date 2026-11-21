//! Data models and polymorphic types for chat messages.

use std::collections::HashMap;

/// Delivery and reception state of a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeliveryStatus {
    /// In transit / uploading.
    Sending,
    /// Sent to server.
    Sent,
    /// Delivered to recipient device.
    #[default]
    Delivered,
    /// Opened and read by recipient.
    Read,
    /// Delivery failure.
    Failed,
}

/// Time-to-live self-destruct countdown timer details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageTtlInfo {
    pub duration_seconds: u32,
    pub remaining_seconds: u32,
    pub is_encrypted: bool,
}

impl MessageTtlInfo {
    pub fn new(duration_seconds: u32, remaining_seconds: u32, is_encrypted: bool) -> Self {
        Self {
            duration_seconds,
            remaining_seconds,
            is_encrypted,
        }
    }
}

/// Emoji reaction counter attached to a message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageReaction {
    pub emoji: String,
    pub count: u32,
    pub is_selected_by_me: bool,
}

impl MessageReaction {
    pub fn new(emoji: impl Into<String>, count: u32, is_selected_by_me: bool) -> Self {
        Self {
            emoji: emoji.into(),
            count,
            is_selected_by_me,
        }
    }
}

/// Quoted message reply reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageReply {
    pub message_id: String,
    pub sender_name: String,
    pub excerpt_text: String,
    pub preview_image_url: Option<String>,
}

impl MessageReply {
    pub fn new(
        message_id: impl Into<String>,
        sender_name: impl Into<String>,
        excerpt_text: impl Into<String>,
    ) -> Self {
        Self {
            message_id: message_id.into(),
            sender_name: sender_name.into(),
            excerpt_text: excerpt_text.into(),
            preview_image_url: None,
        }
    }

    pub fn with_preview(mut self, url: impl Into<String>) -> Self {
        self.preview_image_url = Some(url.into());
        self
    }
}

/// Forwarded message origin attribution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardHeader {
    pub original_sender_name: String,
    pub original_channel_name: Option<String>,
}

impl ForwardHeader {
    pub fn new(original_sender_name: impl Into<String>) -> Self {
        Self {
            original_sender_name: original_sender_name.into(),
            original_channel_name: None,
        }
    }

    pub fn with_channel(mut self, channel: impl Into<String>) -> Self {
        self.original_channel_name = Some(channel.into());
        self
    }
}

/// Visual media item (photo or video) inside a media grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMediaItem {
    pub url: String,
    pub is_video: bool,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<u32>,
    pub caption: Option<String>,
    pub has_spoiler: bool,
    pub ttl: Option<MessageTtlInfo>,
}

impl ChatMediaItem {
    pub fn photo(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            is_video: false,
            thumbnail_url: None,
            duration_seconds: None,
            caption: None,
            has_spoiler: false,
            ttl: None,
        }
    }

    pub fn video(url: impl Into<String>, duration_seconds: u32) -> Self {
        Self {
            url: url.into(),
            is_video: true,
            thumbnail_url: None,
            duration_seconds: Some(duration_seconds),
            caption: None,
            has_spoiler: false,
            ttl: None,
        }
    }

    pub fn with_spoiler(mut self, spoiler: bool) -> Self {
        self.has_spoiler = spoiler;
        self
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }
}

/// Document or arbitrary file attachment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileAttachment {
    pub url: String,
    pub file_name: String,
    pub file_size: Option<u64>,
    pub mime_type: Option<String>,
}

/// Music or audio track attachment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioAttachment {
    pub url: String,
    pub title: String,
    pub performer: Option<String>,
    pub duration_seconds: Option<u32>,
    pub file_size: Option<u64>,
}

/// Voice memo message with waveform samples.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceAttachment {
    pub url: Option<String>,
    pub duration_seconds: u32,
    pub waveform: Vec<u8>,
}

/// Circular round video note attachment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoNoteAttachment {
    pub url: String,
    pub duration_seconds: u32,
    pub thumbnail_url: Option<String>,
}

/// Pluggable format for sticker graphics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StickerFormat {
    Raster,
    VectorSvg,
    Animated,
}

/// Standalone sticker graphic attachment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StickerAttachment {
    pub url: String,
    pub format: StickerFormat,
    pub emoji: Option<String>,
    pub pack_id: Option<String>,
    pub sticker_id: Option<String>,
}

/// Contact card attachment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactAttachment {
    pub name: String,
    pub phone_number: String,
    pub vcard: Option<String>,
}

/// Individual option in a poll.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PollOption {
    pub id: String,
    pub text: String,
    pub vote_count: u32,
    pub is_selected: bool,
}

/// Interactive voting poll card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PollAttachment {
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_votes: u32,
    pub is_closed: bool,
    pub is_anonymous: bool,
}

/// Web page link preview snippet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebPagePreview {
    pub url: String,
    pub site_name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub preview_image_url: Option<String>,
}

/// Financial payment invoice card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvoiceAttachment {
    pub title: String,
    pub description: String,
    pub currency: String,
    pub total_amount: String,
    pub photo_url: Option<String>,
    pub is_paid: bool,
    pub invoice_id: String,
}

/// Expiring story card reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryAttachment {
    pub author_name: String,
    pub author_avatar_url: Option<String>,
    pub media_url: String,
    pub is_video: bool,
    pub story_id: String,
}

/// Interactive button in an inline message keyboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
}

impl InlineKeyboardButton {
    pub fn callback(text: impl Into<String>, callback_data: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: None,
            callback_data: Some(callback_data.into()),
        }
    }

    pub fn link(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: Some(url.into()),
            callback_data: None,
        }
    }
}

/// Content payload variants for rich chat messages.
#[derive(Debug, Clone, PartialEq)]
pub enum MessageContent {
    /// Rich text parsed with the safe Operator Markdown renderer.
    Text(String),
    /// Monospaced syntax-tagged code block with copy action.
    CodeBlock { language: String, code: String },
    /// Stylized blockquote with accent bar and collapsible toggle.
    Blockquote { text: String, is_collapsible: bool },
    /// Visual media items arranged in a compact grid.
    MediaGrid(Vec<ChatMediaItem>),
    /// Document or file download attachment.
    File(FileAttachment),
    /// Audio track attachment.
    Audio(AudioAttachment),
    /// Voice memo with interactive waveform visualization.
    Voice(VoiceAttachment),
    /// Circular round video note.
    VideoNote(VideoNoteAttachment),
    /// Pluggable sticker graphic.
    Sticker(StickerAttachment),
    /// Contact card attachment.
    Contact(ContactAttachment),
    /// Interactive voting poll card.
    Poll(PollAttachment),
    /// Web page preview card with link attribution.
    WebPage(WebPagePreview),
    /// Payment or star invoice card.
    Invoice(InvoiceAttachment),
    /// Ephemeral story card.
    Story(StoryAttachment),
    /// Mixed content combining attachments with optional descriptive text.
    Mixed {
        attachments: Vec<MessageContent>,
        text: Option<String>,
    },
}

/// Grouping arrangement for consecutive messages from the same sender.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageGrouping {
    /// Standalone message with full vertical margins and headers.
    #[default]
    Single,
    /// First message in a consecutive group.
    First,
    /// Middle message in a consecutive group.
    Middle,
    /// Last message in a consecutive group.
    Last,
}

impl MessageGrouping {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::First => "first",
            Self::Middle => "middle",
            Self::Last => "last",
        }
    }
}

/// Pluggable registry for custom emoji packs provided by the host application.
#[derive(Clone, Default)]
pub struct CustomEmojiMap {
    pub entries: HashMap<String, String>,
}

impl CustomEmojiMap {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, url: impl Into<String>) {
        self.entries.insert(key.into(), url.into());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }
}

/// Complete high-level message data structure.
#[derive(Debug, Clone, PartialEq)]
pub struct OperatorMessage {
    pub id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub sender_role: Option<String>,
    pub avatar: Option<String>,
    pub timestamp: String,
    pub is_outgoing: bool,
    pub delivery_status: DeliveryStatus,
    pub content: MessageContent,
    pub reply_to: Option<MessageReply>,
    pub forward_info: Option<ForwardHeader>,
    pub is_edited: bool,
    pub reactions: Vec<MessageReaction>,
    pub ttl: Option<MessageTtlInfo>,
    pub grouping: MessageGrouping,
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
