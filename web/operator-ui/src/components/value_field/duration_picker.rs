//! DurationPicker: one column per unit with increase and decrease buttons and a number input.

use super::parse::{DURATION_UNITS_MS, join_duration, split_duration};
use crate::components::primitives::Icon;
use crate::core::{Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};

/// Duration editor: days, hours, minutes, seconds and milliseconds as columns of steppers.
///
/// The value is milliseconds. The largest shown unit takes everything above it; the buttons
/// move the whole duration by one unit, so minutes carry into hours.
#[component]
pub fn DurationPicker(
    value: RwSignal<i64>,
    /// Adds a sign switch; without it the value never goes below zero.
    #[prop(optional)]
    signed: bool,
    /// Shows a days column.
    #[prop(optional)]
    days: bool,
    /// Shows a seconds column.
    #[prop(default = true)]
    seconds: bool,
    /// Shows a milliseconds column.
    #[prop(optional)]
    milliseconds: bool,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Runs after every change.
    #[prop(optional)]
    on_commit: Option<Callback<i64>>,
    #[prop(optional, into)] days_label: MaybeProp<String>,
    #[prop(optional, into)] hours_label: MaybeProp<String>,
    #[prop(optional, into)] minutes_label: MaybeProp<String>,
    #[prop(optional, into)] seconds_label: MaybeProp<String>,
    #[prop(optional, into)] milliseconds_label: MaybeProp<String>,
    /// Accessible name of the increase buttons; `{unit}` is replaced by the column name.
    #[prop(optional, into)]
    increase_label: MaybeProp<String>,
    /// Accessible name of the decrease buttons; `{unit}` is replaced by the column name.
    #[prop(optional, into)]
    decrease_label: MaybeProp<String>,
    #[prop(optional, into)] negative_label: MaybeProp<String>,
    #[prop(optional)] increase_icon: Option<IconData>,
    #[prop(optional)] decrease_icon: Option<IconData>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] column_width: MaybeProp<f32>,
    #[prop(optional, into)] button_height: MaybeProp<f32>,
    #[prop(optional, into)] number_height: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let names = [
        ui_text(days_label, "duration-picker.days", "Days"),
        ui_text(hours_label, "duration-picker.hours", "Hours"),
        ui_text(minutes_label, "duration-picker.minutes", "Minutes"),
        ui_text(seconds_label, "duration-picker.seconds", "Seconds"),
        ui_text(
            milliseconds_label,
            "duration-picker.milliseconds",
            "Milliseconds",
        ),
    ];
    let increase_label = ui_text(
        increase_label,
        "duration-picker.increase",
        "Increase {unit}",
    );
    let decrease_label = ui_text(
        decrease_label,
        "duration-picker.decrease",
        "Decrease {unit}",
    );
    let negative_label = ui_text(negative_label, "duration-picker.negative", "Negative");
    let increase_icon = ui_icon(increase_icon, "duration-picker.increase", icons::ADD);
    let decrease_icon = ui_icon(decrease_icon, "duration-picker.decrease", icons::REMOVE);
    let custom_style = root_style(
        vec![
            Metric::du("--op-field-popup-padding", padding),
            Metric::du("--op-field-popup-gap", gap),
            Metric::du("--op-field-duration-column-width", column_width),
            Metric::du("--op-field-duration-button-height", button_height),
            Metric::du("--op-field-duration-number-height", number_height),
            Metric::du("--op-field-icon-size", icon_size),
            Metric::tu("--op-field-font-size", font_size),
        ],
        style,
    );
    let shown = [days, true, true, seconds, milliseconds];
    let set = move |next: i64| {
        if !enabled.get_untracked() {
            return;
        }
        let next = if signed { next } else { next.max(0) };
        if next != value.get_untracked() {
            value.set(next);
            if let Some(cb) = on_commit {
                cb.run(next);
            }
        }
    };
    let negative = move || value.get() < 0;
    // Moves the magnitude by `delta` units, keeping the sign.
    let step = move |unit: usize, delta: i64| {
        let v = value.get_untracked();
        let sign = if v < 0 { -1 } else { 1 };
        let magnitude = (v.saturating_abs() + delta * DURATION_UNITS_MS[unit]).max(0);
        set(sign * magnitude);
    };
    let icon_style = "width:var(--op-field-icon-size, var(--op-dim-icon-small));height:var(--op-field-icon-size, var(--op-dim-icon-small));";
    let column = move |unit: usize| {
        let name = names[unit];
        let part = move || split_duration(value.get(), shown)[unit];
        let input = NodeRef::<leptos::html::Input>::new();
        view! {
            <div class="op-duration-column" role="group" aria-label=move || name.get()>
                <button
                    type="button"
                    class="op-rippled op-duration-step"
                    aria-label=move || increase_label.get().replace("{unit}", &name.get())
                    disabled=move || !enabled.get()
                    on:click=move |_| step(unit, 1)
                >
                    {move || view! { <Icon icon=increase_icon.get() size=dim::ICON_SMALL attr:style=icon_style/> }}
                </button>
                <input
                    node_ref=input
                    type="text"
                    inputmode="numeric"
                    class="op-duration-number"
                    aria-label=move || name.get()
                    disabled=move || !enabled.get()
                    prop:value=move || {
                        let p = part();
                        if unit == 0 || unit == 1 { p.to_string() } else if unit == 4 { format!("{p:03}") } else { format!("{p:02}") }
                    }
                    on:keydown=move |ev| match ev.key().as_str() {
                        "ArrowUp" => {
                            ev.prevent_default();
                            step(unit, 1);
                        }
                        "ArrowDown" => {
                            ev.prevent_default();
                            step(unit, -1);
                        }
                        "Enter" => {
                            if let Some(el) = input.get_untracked() {
                                let _ = el.blur();
                            }
                        }
                        _ => {}
                    }
                    on:change=move |ev| {
                        let parsed = event_target_value(&ev).trim().parse::<i64>().ok().filter(|n| *n >= 0);
                        match parsed {
                            Some(n) => {
                                let v = value.get_untracked();
                                let mut parts = split_duration(v, shown);
                                parts[unit] = n;
                                set(join_duration(parts, v < 0));
                            }
                            None => {
                                if let Some(el) = input.get_untracked() {
                                    el.set_value(&part().to_string());
                                }
                            }
                        }
                    }
                />
                <span class="op-duration-unit op-t-caption">{move || name.get()}</span>
                <button
                    type="button"
                    class="op-rippled op-duration-step"
                    aria-label=move || decrease_label.get().replace("{unit}", &name.get())
                    disabled=move || !enabled.get()
                    on:click=move |_| step(unit, -1)
                >
                    {move || view! { <Icon icon=decrease_icon.get() size=dim::ICON_SMALL attr:style=icon_style/> }}
                </button>
            </div>
        }
    };
    view! {
        <div class=root_class("op-duration-picker", class) style=custom_style>
            {header.run()}
            <div class="op-duration-columns">
                {signed.then(|| view! {
                    <button
                        type="button"
                        class="op-rippled op-duration-sign"
                        aria-label=move || negative_label.get()
                        aria-pressed=move || negative().to_string()
                        disabled=move || !enabled.get()
                        on:click=move |_| set(-value.get_untracked())
                    >
                        {move || if negative() { "−" } else { "+" }}
                    </button>
                })}
                {(0..5).filter(|i| shown[*i]).map(column).collect_view()}
            </div>
            {footer.run()}
        </div>
    }
}
