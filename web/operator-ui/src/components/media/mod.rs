//! Media components extension: chat, messaging, media capture and playback, gallery, and camera control.

#[cfg(feature = "chat-input")]
pub mod chat_input;
#[cfg(feature = "chat-message")]
pub mod chat_message;
#[cfg(feature = "emoji-picker")]
pub mod emoji_picker;
#[cfg(feature = "gallery")]
pub mod gallery;
#[cfg(feature = "gallery-viewer")]
pub mod gallery_viewer;
#[cfg(feature = "gif-picker")]
pub mod gif_picker;
#[cfg(feature = "media-keyboard")]
pub mod media_keyboard;
#[cfg(feature = "media-record-button")]
pub mod media_record_button;
#[cfg(feature = "ptt-button")]
pub mod ptt_button;
#[cfg(feature = "ptz-pad")]
pub mod ptz_pad;
#[cfg(feature = "reaction-picker")]
pub mod reaction_picker;
#[cfg(feature = "sticker-picker")]
pub mod sticker_picker;
#[cfg(feature = "video-stream")]
pub mod video_stream;
