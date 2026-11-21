//! NumberField: a number with an optional unit.

use super::parse::{format_number, parse_number, step_number};
use super::parsed::ParsedField;
use crate::components::field::FieldVariant;
use crate::core::{ControlState, Metric, Slot, root_style, ui_text};
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
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] variant: Signal<FieldVariant>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] prepend: Slot,
    #[prop(optional, into)] append: Slot,
    #[prop(optional, into)] before: Slot,
    #[prop(optional, into)] after: Slot,
    #[prop(optional, into)] label_slot: Slot,
    #[prop(optional, into)] hint_slot: Slot,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] slot_size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] unit_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] unit_gap: MaybeProp<f32>,

    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<Option<f64>>>,
) -> impl IntoView {
    let placeholder = ui_text(placeholder, "number-field.placeholder", "");
    let aria_label = ui_text(aria_label, "number-field.label", "Number");
    let custom_style = Signal::derive(root_style(
        vec![
            Metric::du("--op-field-unit-padding-x", unit_padding_x),
            Metric::du("--op-field-unit-gap", unit_gap),
        ],
        style,
    ));
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
            hint=hint
            error=error
            variant=variant
            aria_label=aria_label
            prepend=prepend
            append=append
            before=before
            after=after
            label_slot=label_slot
            hint_slot=hint_slot
            height=height
            padding_x=padding_x
            slot_size=slot_size
            gap=gap
            font_size=font_size
            class=class
            style=custom_style
            enabled=enabled
            state=state
            input_mode="decimal"
            on_commit=on_commit
            trailing=trailing
        />
    }
}
