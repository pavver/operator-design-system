//! TimeField: a time of day.

use super::parse::{format_time, parse_time};
use super::parsed::ParsedField;
use crate::components::field::FieldVariant;
use crate::core::{ControlState, Slot, ui_text};
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

    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<u32>>,
) -> impl IntoView {
    let placeholder = ui_text(placeholder, "time-field.placeholder", "");
    let aria_label = ui_text(aria_label, "time-field.label", "Time");
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
            style=style
            enabled=enabled
            state=state
            input_mode="numeric"
            on_commit=on_commit
        />
    }
}
