//! NumberField: a number with an optional unit.

use super::parse::{format_number, parse_number, step_number};
use super::parsed::ParsedField;
use crate::core::ControlState;
use leptos::prelude::*;

/// Number input with a unit after the value; arrows step it within the bounds.
///
/// The value is `None` when the field is empty and `allow_empty` is set.
#[component]
pub fn NumberField(
    value: RwSignal<Option<f64>>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    /// Arrow Up/Down step.
    #[prop(default = 1.0)]
    step: f64,
    /// Fraction digits shown at most.
    #[prop(default = 3)]
    decimals: u8,
    /// Unit after the value, for example `°C` or `kWh`.
    #[prop(optional, into)]
    unit: Option<Signal<String>>,
    /// Accepts an empty field as no value.
    #[prop(optional)]
    allow_empty: bool,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<Option<f64>>>,
) -> impl IntoView {
    let format = Callback::new(move |v: Option<f64>| {
        v.map(|v| format_number(v, decimals)).unwrap_or_default()
    });
    let parse = Callback::new(move |t: String| {
        if t.trim().is_empty() {
            return allow_empty.then_some(None);
        }
        let v = parse_number(&t)?;
        let in_range = min.is_none_or(|m| v >= m) && max.is_none_or(|m| v <= m);
        in_range.then_some(Some(v))
    });
    let on_step = Callback::new(move |(v, delta): (Option<f64>, i32)| {
        Some(step_number(
            v.unwrap_or_else(|| min.unwrap_or(0.0)),
            delta,
            step,
            min,
            max,
        ))
    });
    let trailing = unit.map(|u| -> ChildrenFn {
        std::sync::Arc::new(move || {
            view! { <span class="op-value-unit">{move || u.get()}</span> }.into_any()
        })
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
            input_mode="decimal"
            class="op-number-field"
            on_commit=on_commit
            trailing=trailing
        />
    }
}
