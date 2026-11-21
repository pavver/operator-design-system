//! Item models for GalleryViewer.

#[cfg(feature = "gallery")]
use crate::components::gallery::GalleryMediaItem;

/// Model for a media item shown in the GalleryViewer.
#[derive(Clone, Debug, PartialEq)]
pub struct GalleryViewerItem {
    pub id: String,
    pub src: String,
    pub thumbnail: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub is_video: bool,
}

impl GalleryViewerItem {
    pub fn image(id: impl Into<String>, src: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            src: src.into(),
            thumbnail: None,
            title: None,
            caption: None,
            is_video: false,
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
}

#[cfg(feature = "gallery")]
impl From<GalleryMediaItem> for GalleryViewerItem {
    fn from(item: GalleryMediaItem) -> Self {
        Self {
            id: item.id,
            src: item.src,
            thumbnail: item.thumbnail,
            title: item.title,
            caption: item.caption,
            is_video: item.is_video,
        }
    }
}
