//! CSS length conversions for design and type units.

/// Converts Design Units into a CSS length.
pub fn du(value: f32) -> String {
    format!("calc(var(--op-du) * {value})")
}

/// Converts Type Units into a CSS length.
pub fn tu(value: f32) -> String {
    format!("calc(var(--op-tu) * {value})")
}
