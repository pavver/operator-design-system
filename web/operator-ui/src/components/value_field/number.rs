//! NumberField: a number with an optional unit.

use super::parse::{format_number, parse_number, step_number};
use super::parsed::ParsedField;
use crate::components::field::FieldVariant;
use crate::components::primitives::Icon;
use crate::core::{ControlState, Metric, Slot, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};

/// Number input with a unit after the value; arrows, and the optional − and + buttons, step it
/// within the bounds.
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
    /// Shows − and + buttons after the value.
    #[prop(optional)]
    steppers: bool,
    #[prop(optional, into)] increase_label: MaybeProp<String>,
    #[prop(optional, into)] decrease_label: MaybeProp<String>,
    #[prop(optional)] increase_icon: Option<IconData>,
    #[prop(optional)] decrease_icon: Option<IconData>,
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
    let increase_label = ui_text(increase_label, "number-field.increase", "Increase");
    let decrease_label = ui_text(decrease_label, "number-field.decrease", "Decrease");
    let increase_icon = ui_icon(increase_icon, "number-field.increase", icons::ADD);
    let decrease_icon = ui_icon(decrease_icon, "number-field.decrease", icons::REMOVE);
    let blocked = move || {
        !enabled.get()
            || matches!(
                state.get(),
                Some(ControlState::Disabled | ControlState::Pending)
            )
    };
    let press = move |delta: i32| {
        if blocked() {
            return;
        }
        let next = on_step.run((value.get_untracked(), delta));
        if next != value.get_untracked() {
            value.set(next);
            if let Some(cb) = on_commit {
                cb.run(next);
            }
        }
    };
    let at_min = move || matches!((value.get(), min), (Some(v), Some(m)) if v <= m);
    let at_max = move || matches!((value.get(), max), (Some(v), Some(m)) if v >= m);
    let icon_style = "width:var(--op-field-icon-size, var(--op-dim-icon-field));height:var(--op-field-icon-size, var(--op-dim-icon-field));";
    let trailing = (unit.is_some() || steppers).then(|| -> ChildrenFn {
        std::sync::Arc::new(move || {
            view! {
                {unit.map(|u| view! { <span class="op-value-unit">{move || u.get()}</span> })}
                {steppers.then(|| view! {
                    <button
                        type="button"
                        class="op-rippled op-field-reveal op-number-step"
                        aria-label=move || decrease_label.get()
                        disabled=move || blocked() || at_min()
                        on:click=move |_| press(-1)
                    >
                        {move || view! { <Icon icon=decrease_icon.get() size=dim::ICON_FIELD attr:style=icon_style/> }}
                    </button>
                    <button
                        type="button"
                        class="op-rippled op-field-reveal op-number-step"
                        aria-label=move || increase_label.get()
                        disabled=move || blocked() || at_max()
                        on:click=move |_| press(1)
                    >
                        {move || view! { <Icon icon=increase_icon.get() size=dim::ICON_FIELD attr:style=icon_style/> }}
                    </button>
                })}
            }
            .into_any()
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
