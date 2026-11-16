//! TimeField: a time of day.

use super::parse::{format_time, parse_time};
use super::parsed::ParsedField;
use crate::core::ControlState;
use leptos::prelude::*;

/// Time-of-day input (`07:30`, `0730`, `7:30:15`); the value is seconds since midnight.
#[component]
pub fn TimeField(
    value: RwSignal<u32>,
    /// Always shows seconds.
    #[prop(optional)]
    seconds: bool,
    /// Arrow Up/Down step in seconds; wraps around midnight.
    #[prop(default = 60)]
    step_s: u32,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<u32>>,
) -> impl IntoView {
    let format = Callback::new(move |v: u32| format_time(v, seconds));
    let parse = Callback::new(move |t: String| parse_time(&t));
    let on_step = Callback::new(move |(v, delta): (u32, i32)| {
        let day = 86_400_i64;
        let next = i64::from(v) + i64::from(step_s) * i64::from(delta);
        u32::try_from(next.rem_euclid(day)).unwrap_or(0)
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
            input_mode="numeric"
            class="op-time-field"
            on_commit=on_commit
        />
    }
}
