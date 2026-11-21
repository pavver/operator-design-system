//! Media item models and layout configurations for Gallery.

pub use crate::components::gallery_viewer::{GalleryMediaItem, GalleryViewerItem, MediaItem};

/// Gallery grid arrangement strategy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GalleryLayout {
    /// Responsive column grid adapting to container width.
    #[default]
    Grid,
    /// Compact chat-attachment bubble layout from Telegram media sections.
    Telegram,
}
