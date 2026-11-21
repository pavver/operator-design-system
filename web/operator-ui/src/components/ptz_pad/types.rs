//! Types and commands for PTZ (Pan/Tilt/Zoom) camera control.

/// Direction for Pan and Tilt camera control.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PtzDirection {
    #[default]
    None,
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl PtzDirection {
    pub const fn is_active(self) -> bool {
        !matches!(self, Self::None)
    }

    pub const fn is_up(self) -> bool {
        matches!(self, Self::Up | Self::UpLeft | Self::UpRight)
    }

    pub const fn is_down(self) -> bool {
        matches!(self, Self::Down | Self::DownLeft | Self::DownRight)
    }

    pub const fn is_left(self) -> bool {
        matches!(self, Self::Left | Self::UpLeft | Self::DownLeft)
    }

    pub const fn is_right(self) -> bool {
        matches!(self, Self::Right | Self::UpRight | Self::DownRight)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Up => "up",
            Self::Down => "down",
            Self::Left => "left",
            Self::Right => "right",
            Self::UpLeft => "up-left",
            Self::UpRight => "up-right",
            Self::DownLeft => "down-left",
            Self::DownRight => "down-right",
        }
    }
}

/// Zoom direction for PTZ camera control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PtzZoomDirection {
    In,
    Out,
}

/// Unified PTZ command structure.
#[derive(Clone, Debug, PartialEq)]
pub enum PtzCommand {
    /// Directional movement with normalized speed in `[0.0, 1.0]`.
    Move { direction: PtzDirection, speed: f32 },
    /// Zoom adjustment.
    Zoom(PtzZoomDirection),
    /// Stop all active movement.
    Stop,
}
