//! Typed text fields: NumberField, DurationField, TimeField, DateTimeField and ColorField.
//!
//! Each one is a [`ParsedField`]: the text is parsed on Enter or blur; invalid text rolls back to
//! the last value and the field pulses the error state.

pub mod color_math;
pub mod parse;
mod parsed;
// Reuse the popup helper without adding dropdown-menu as a value-field dependency.
#[cfg(all(
    any(
        feature = "color-field",
        feature = "date-time-field",
        feature = "time-field"
    ),
    not(feature = "dropdown-menu")
))]
#[path = "../dropdown_menu/portal_style.rs"]
mod portal_style;
#[cfg(all(
    any(
        feature = "color-field",
        feature = "date-time-field",
        feature = "time-field"
    ),
    feature = "dropdown-menu"
))]
use super::dropdown_menu::portal_style;

#[cfg(feature = "date-time-field")]
mod calendar;
#[cfg(feature = "time-field")]
mod clock;
#[cfg(feature = "color-field")]
mod color;
#[cfg(feature = "color-field")]
mod color_picker;
#[cfg(feature = "date-time-field")]
mod date_time;
#[cfg(feature = "duration-field")]
mod duration;
#[cfg(feature = "number-field")]
mod number;
#[cfg(feature = "time-field")]
mod time;

#[cfg(feature = "date-time-field")]
pub use calendar::Calendar;
#[cfg(feature = "time-field")]
pub use clock::ClockPicker;
#[cfg(feature = "color-field")]
pub use color::ColorField;
#[cfg(feature = "color-field")]
pub use color_picker::{ColorMode, ColorPicker};
#[cfg(feature = "date-time-field")]
pub use date_time::DateTimeField;
#[cfg(feature = "duration-field")]
pub use duration::DurationField;
#[cfg(feature = "number-field")]
pub use number::NumberField;
pub use parsed::ParsedField;
#[cfg(feature = "time-field")]
pub use time::TimeField;

pub(crate) const CSS: &str = include_str!("value_field.css");

#[cfg(test)]
mod tests;
