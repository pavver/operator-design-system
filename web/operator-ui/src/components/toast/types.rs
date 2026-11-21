use super::*;

/// Meaning of a message; maps to a signal color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Tone {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl Tone {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Short human-readable name, shown in the header of a tall toast.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Success => "Success",
            Self::Warning => "Warning",
            Self::Error => "Error",
        }
    }

    pub const fn icon(self) -> IconData {
        match self {
            Self::Info => icons::INFO,
            Self::Success => icons::CHECK_CIRCLE,
            Self::Warning => icons::WARNING,
            Self::Error => icons::ERROR,
        }
    }
}

/// Where the toast stack sits in the viewport.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ToastPosition {
    TopStart,
    TopCenter,
    TopEnd,
    BottomStart,
    #[default]
    BottomCenter,
    BottomEnd,
}

impl ToastPosition {
    pub const ALL: [Self; 6] = [
        Self::TopStart,
        Self::TopCenter,
        Self::TopEnd,
        Self::BottomStart,
        Self::BottomCenter,
        Self::BottomEnd,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopStart => "top-start",
            Self::TopCenter => "top-center",
            Self::TopEnd => "top-end",
            Self::BottomStart => "bottom-start",
            Self::BottomCenter => "bottom-center",
            Self::BottomEnd => "bottom-end",
        }
    }

    pub const fn is_top(self) -> bool {
        matches!(self, Self::TopStart | Self::TopCenter | Self::TopEnd)
    }
}

/// Maximum number of toasts on screen; older ones are dropped first.
pub const TOAST_LIMIT: usize = 3;

/// Content of one toast.
#[derive(Clone)]
pub struct ToastSpec {
    pub tone: Tone,
    pub text: String,
    /// Optional single action, for example `("Retry", callback)`.
    pub action: Option<(String, Callback<()>)>,
    /// Visible time in milliseconds; `None` uses the tone default.
    pub duration_ms: Option<u32>,
}

impl ToastSpec {
    pub fn new(tone: Tone, text: impl Into<String>) -> Self {
        Self {
            tone,
            text: text.into(),
            action: None,
            duration_ms: None,
        }
    }

    pub fn with_action(mut self, label: impl Into<String>, on: Callback<()>) -> Self {
        self.action = Some((label.into(), on));
        self
    }
}

#[derive(Clone)]
pub(super) struct ToastEntry {
    pub(super) id: u64,
    pub(super) spec: ToastSpec,
    pub(super) hovered: RwSignal<bool>,
    pub(super) expired: RwSignal<bool>,
    pub(super) leaving: RwSignal<bool>,
}
