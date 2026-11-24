//! DurationField: a signed duration in milliseconds, with a DurationPicker popup.

use super::duration_picker::DurationPicker;
use super::parse::{format_duration, parse_duration};
use super::parsed::ParsedField;
use super::picker_popup::PickerPopup;
use crate::components::field::{FieldMetrics, FieldVariant};
use crate::components::popover::{Placement, Popover};
use crate::core::{ControlState, Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

/// Duration input: `1h 30m`, `90s`, `1:30`, `-5m`, with a [`DurationPicker`] popup; the value is
/// in milliseconds.
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
    #[prop(optional, into)] day_unit: MaybeProp<String>,
    #[prop(optional, into)] hour_unit: MaybeProp<String>,
    #[prop(optional, into)] minute_unit: MaybeProp<String>,
    #[prop(optional, into)] second_unit: MaybeProp<String>,
    #[prop(optional, into)] millisecond_unit: MaybeProp<String>,

    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<i64>>,
    /// Hides the picker button; the field takes text only.
    #[prop(optional)]
    no_picker: bool,
    /// Shows a days column in the picker.
    #[prop(optional)]
    picker_days: bool,
    /// Shows a seconds column in the picker.
    #[prop(default = true)]
    picker_seconds: bool,
    /// Shows a milliseconds column in the picker.
    #[prop(optional)]
    picker_milliseconds: bool,
    #[prop(optional, into)] picker_label: MaybeProp<String>,
    #[prop(optional, into)] popup_label: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_blur: MaybeProp<f32>,
) -> impl IntoView {
    let placeholder = ui_text(placeholder, "duration-field.placeholder", "");
    let aria_label = ui_text(aria_label, "duration-field.label", "Duration");
    let units = [
        ui_text(day_unit, "duration-field.day-unit", "d"),
        ui_text(hour_unit, "duration-field.hour-unit", "h"),
        ui_text(minute_unit, "duration-field.minute-unit", "m"),
        ui_text(second_unit, "duration-field.second-unit", "s"),
        ui_text(millisecond_unit, "duration-field.millisecond-unit", "ms"),
    ];
    let format = Callback::new(move |v| localize_duration(v, &units.map(|unit| unit.get())));
    let parse = Callback::new(move |t: String| {
        parse_localized_duration(&t, default_unit_ms, &units.map(|unit| unit.get()))
            .filter(|v| signed || *v >= 0)
    });
    let on_step = Callback::new(move |(v, delta): (i64, i32)| {
        let next = v.saturating_add(step_ms.saturating_mul(i64::from(delta)));
        if signed { next } else { next.max(0) }
    });
    let picker_label = ui_text(picker_label, "duration-field.pick", "Pick a duration");
    let popup_label = ui_text(popup_label, "duration-field.popup", "Duration");
    let icon = ui_icon(icon, "duration-field.expand", icons::ARROW_DROP_DOWN);
    let blocked = move || {
        !enabled.get()
            || matches!(
                state.get(),
                Some(ControlState::Disabled | ControlState::Pending)
            )
    };
    let mut metrics = FieldMetrics {
        height,
        padding_x,
        slot_size,
        gap,
        font_size,
    }
    .vars();
    metrics.push(Metric::du("--op-field-icon-size", icon_size));
    metrics.push(Metric::du("--op-field-popup-shadow-y", popup_shadow_y));
    metrics.push(Metric::du(
        "--op-field-popup-shadow-blur",
        popup_shadow_blur,
    ));
    let custom_style = Signal::derive(root_style(metrics, style));
    let popup = PickerPopup::new(custom_style, popup_label, blocked);
    let trailing = (!no_picker).then(|| popup.button(picker_label, icon, blocked));
    let commit = Callback::new(move |v: i64| {
        if let Some(cb) = on_commit {
            cb.run(v);
        }
    });
    view! {
        <div class=root_class("op-value-anchor op-duration-field", class) style=move || custom_style.get() node_ref=popup.host>
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
            enabled=enabled
            state=state
            on_commit=on_commit
            trailing=trailing
        />
        </div>
        <Popover dismiss_on_scroll=true node_ref=popup.popup_ref open=popup.open anchor=popup.anchor placement=Placement::BelowStart class="op-value-popup" role="dialog" label=popup_label.get()>
            <div style=move || popup.style.get()>
                <DurationPicker
                    value=value
                    signed=signed
                    days=picker_days
                    seconds=picker_seconds
                    milliseconds=picker_milliseconds
                    enabled=Signal::derive(move || !blocked())
                    on_commit=commit
                />
            </div>
        </Popover>
    }
}

const UNITS: [&str; 5] = ["d", "h", "m", "s", "ms"];

fn localize_duration(value: i64, units: &[String; 5]) -> String {
    format_duration(value)
        .split_whitespace()
        .map(|part| {
            // Milliseconds must precede seconds when matching suffixes.
            for i in [4, 0, 1, 2, 3] {
                if let Some(number) = part.strip_suffix(UNITS[i]) {
                    return format!("{number}{}", units[i]);
                }
            }
            part.to_string()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_localized_duration(text: &str, default_unit_ms: i64, units: &[String; 5]) -> Option<i64> {
    if let Some(value) = parse_duration(text, default_unit_ms) {
        return Some(value);
    }
    let mut aliases: Vec<_> = units
        .iter()
        .enumerate()
        .filter(|(_, unit)| !unit.is_empty())
        .collect();
    aliases.sort_by_key(|(_, unit)| std::cmp::Reverse(unit.len()));
    let mut rest = text.trim();
    let mut normalized = String::new();
    while !rest.is_empty() {
        let number_end =
            rest.find(|c: char| !(c.is_ascii_digit() || matches!(c, '+' | '-' | '.' | ',' | ' ')))?;
        normalized.push_str(&rest[..number_end]);
        rest = &rest[number_end..];
        let (i, unit) = aliases
            .iter()
            .find(|(_, unit)| rest.starts_with(unit.as_str()))?;
        normalized.push_str(UNITS[*i]);
        normalized.push(' ');
        rest = rest[unit.len()..].trim_start();
    }
    parse_duration(&normalized, default_unit_ms)
}

#[cfg(test)]
mod tests {
    use super::{localize_duration, parse_localized_duration};

    #[test]
    fn translated_duration_units_round_trip() {
        let units = ["д", "год", "хв", "с", "мс"].map(str::to_string);
        for value in [0, -90_000, 86_403_250, 9_000_000_000] {
            let text = localize_duration(value, &units);
            assert_eq!(parse_localized_duration(&text, 1_000, &units), Some(value));
        }
        assert_eq!(localize_duration(90_000, &units), "1хв 30с");
        assert_eq!(
            parse_localized_duration("1:30", 1_000, &units),
            Some(90_000)
        );
    }
}
