//! Typed text fields: NumberField, DurationField, TimeField, DateTimeField and ColorField.
//!
//! Each one is a [`ParsedField`]: the text is parsed on Enter or blur; invalid text rolls back to
//! the last value and the field pulses the error state.

pub mod parse;
mod parsed;

#[cfg(feature = "color-field")]
mod color;
#[cfg(feature = "date-time-field")]
mod date_time;
#[cfg(feature = "duration-field")]
mod duration;
#[cfg(feature = "number-field")]
mod number;
#[cfg(feature = "time-field")]
mod time;

#[cfg(feature = "color-field")]
pub use color::ColorField;
#[cfg(feature = "date-time-field")]
pub use date_time::{Calendar, DateTimeField};
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
