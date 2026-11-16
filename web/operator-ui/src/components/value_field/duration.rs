//! DurationField: a signed duration in milliseconds.

use super::parse::{format_duration, parse_duration};
use super::parsed::ParsedField;
use crate::core::ControlState;
use leptos::prelude::*;

/// Duration input: `1h 30m`, `90s`, `1:30`, `-5m`; the value is in milliseconds.
#[component]
pub fn DurationField(
    value: RwSignal<i64>,
    /// Allows negative durations (offsets such as `-15m`).
    #[prop(default = true)]
    signed: bool,
    /// Unit of a bare number, in milliseconds (seconds by default).
    #[prop(default = 1_000)]
    default_unit_ms: i64,
    /// Arrow Up/Down step in milliseconds.
    #[prop(default = 1_000)]
    step_ms: i64,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<i64>>,
) -> impl IntoView {
    let format = Callback::new(format_duration);
    let parse = Callback::new(move |t: String| {
        parse_duration(&t, default_unit_ms).filter(|v| signed || *v >= 0)
    });
    let on_step = Callback::new(move |(v, delta): (i64, i32)| {
        let next = v.saturating_add(step_ms.saturating_mul(i64::from(delta)));
        if signed { next } else { next.max(0) }
    });
    view! {
        <ParsedField
            value=value
            format=format
            parse=parse
            on_step=Some(on_step)
            placeholder=placeholder
            label=label
            enabled=enabled
            state=state
            class="op-duration-field"
            on_commit=on_commit
        />
    }
}
