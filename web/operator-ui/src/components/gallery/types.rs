//! Media item models and layout configurations for Gallery.

/// Model describing a media item displayed inside a Gallery.
#[derive(Clone, Debug, PartialEq)]
pub struct GalleryMediaItem {
    /// Unique item identifier.
    pub id: String,
    /// High-resolution image or video stream URL.
    pub src: String,
    /// Optional preview thumbnail URL (used for faster grid rendering and video posters).
    pub thumbnail: Option<String>,
    /// Optional title or filename.
    pub title: Option<String>,
    /// Optional descriptive caption text.
    pub caption: Option<String>,
    /// True if the item is a video rather than a photograph.
    pub is_video: bool,
    /// Preferred aspect ratio (width / height); defaults to `1.0` or layout default.
    pub aspect_ratio: Option<f32>,
}

impl GalleryMediaItem {
    pub fn image(id: impl Into<String>, src: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            src: src.into(),
            thumbnail: None,
            title: None,
            caption: None,
            is_video: false,
            aspect_ratio: None,
        }
    }

    pub fn video(id: impl Into<String>, src: impl Into<String>, thumbnail: Option<String>) -> Self {
        Self {
            id: id.into(),
            src: src.into(),
            thumbnail,
            title: None,
            caption: None,
            is_video: true,
            aspect_ratio: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn with_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = Some(ratio);
        self
    }
}

/// Gallery grid arrangement strategy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GalleryLayout {
    /// Responsive column grid adapting to container width.
    #[default]
    Grid,
    /// Compact chat-attachment bubble layout from Telegram media sections.
    Telegram,
}
