//! State and playback types for VideoStream.

/// Operational state of a video stream playback shell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VideoStreamState {
    /// Idle state with no stream attached; displays an offline camera placeholder.
    Idle,
    /// Actively establishing or buffering the incoming video stream.
    Connecting,
    /// Actively receiving and playing video frames.
    Playing,
    /// Video frame stream has stalled or buffer is underrun.
    Stalled,
    /// Stream failed or encountered a network/media decode error.
    Error(String),
}

impl VideoStreamState {
    pub const fn is_playing(&self) -> bool {
        matches!(self, Self::Playing)
    }

    pub const fn is_connecting(&self) -> bool {
        matches!(self, Self::Connecting)
    }

    pub const fn is_stalled(&self) -> bool {
        matches!(self, Self::Stalled)
    }

    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Connecting => "connecting",
            Self::Playing => "playing",
            Self::Stalled => "stalled",
            Self::Error(_) => "error",
        }
    }
}
