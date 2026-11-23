//! DateTimeField and its month Calendar.

use super::calendar::Calendar;
use super::parse::{LocalDateTime, format_date_time, parse_date_time};
use super::parsed::ParsedField;
use super::portal_style::{field_popup_label, field_popup_style};
use crate::components::field::FieldMetrics;
use crate::components::field::FieldVariant;
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::primitives::Icon;
use crate::core::{ControlState, Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::dim;
use operator_ui_tokens::{Icon as IconData, icons};
use std::sync::Arc;

/// Local date and time input (`2026-11-17 07:30`) with a calendar for the date.
#[component]
pub fn DateTimeField(
    value: RwSignal<LocalDateTime>,
    /// Always shows seconds.
    #[prop(optional)]
    seconds: bool,
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
    #[prop(optional, into)] picker_label: MaybeProp<String>,
    #[prop(optional, into)] popup_label: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_blur: MaybeProp<f32>,
    #[prop(optional, into)] calendar_padding: MaybeProp<f32>,
    #[prop(optional, into)] calendar_cell_size: MaybeProp<f32>,
    #[prop(optional, into)] calendar_header_height: MaybeProp<f32>,
    #[prop(optional, into)] calendar_weekday_height: MaybeProp<f32>,

    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<LocalDateTime>>,
    #[prop(optional)] month_names: Option<Vec<String>>,
    #[prop(optional)] weekday_names: Option<Vec<String>>,
) -> impl IntoView {
    let placeholder = ui_text(placeholder, "date-time-field.placeholder", "");
    let aria_label = ui_text(aria_label, "date-time-field.label", "Date and time");
    let picker_label = ui_text(picker_label, "date-time-field.pick", "Pick a date");
    let popup_label = ui_text(popup_label, "date-time-field.popup", "Calendar");
    let icon = ui_icon(icon, "date-time-field.expand", icons::ARROW_DROP_DOWN);
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
    metrics.push(Metric::du("--op-field-calendar-padding", calendar_padding));
    metrics.push(Metric::du(
        "--op-field-calendar-cell-size",
        calendar_cell_size,
    ));
    metrics.push(Metric::du(
        "--op-field-calendar-header-height",
        calendar_header_height,
    ));
    metrics.push(Metric::du(
        "--op-field-calendar-weekday-height",
        calendar_weekday_height,
    ));
    let custom_style = Signal::derive(root_style(metrics, style));
    let format = Callback::new(move |v: LocalDateTime| format_date_time(v, seconds));
    let parse = Callback::new(move |t: String| parse_date_time(&t));
    let open = RwSignal::new(false);
    let popup_ref = NodeRef::<leptos::html::Div>::new();
    field_popup_label(popup_ref, popup_label);
    Effect::new(move |_| {
        if blocked() {
            open.set(false);
        }
    });
    let host = NodeRef::<leptos::html::Div>::new();
    let popup_style = field_popup_style(host, open, custom_style);
    let anchor = Signal::derive(move || {
        host.get()
            .map(|el| {
                el.query_selector(".op-field")
                    .ok()
                    .flatten()
                    .map(|field| PopoverAnchor::of(&field))
                    .unwrap_or_else(|| PopoverAnchor::of(&el))
            })
            .unwrap_or(PopoverAnchor::Point { x: 0.0, y: 0.0 })
    });
    let selected = Signal::derive(move || {
        let v = value.get();
        Some((v.year, v.month, v.day))
    });
    let pick = Callback::new(move |(y, m, d): (i32, u32, u32)| {
        let current = value.get_untracked();
        if let Some(next) = LocalDateTime::new(y, m, d, current.second_of_day) {
            value.set(next);
            if let Some(cb) = on_commit {
                cb.run(next);
            }
        }
        open.set(false);
    });
    let trailing: ChildrenFn = Arc::new(move || {
        view! {
            <button
                type="button"
                class="op-rippled op-field-reveal"
                aria-label=move || picker_label.get()
                aria-expanded=move || open.get().to_string()
                disabled=blocked
                on:mousedown=|ev| ev.prevent_default()
                on:click=move |_| open.update(|o| *o = !*o)
            >
                {move || view! { <Icon icon=icon.get() size=dim::ICON_FIELD attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-field));height:var(--op-field-icon-size, var(--op-dim-icon-field));"/> }}
            </button>
        }
        .into_any()
    });
    let months = StoredValue::new(month_names);
    let weekdays = StoredValue::new(weekday_names);
    view! {
        <div class=root_class("op-value-anchor op-date-time-field", class) style=move || custom_style.get() node_ref=host>
            <ParsedField
                value=value
                format=format
                parse=parse
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
                trailing=Some(trailing)
            />
        </div>
        <Popover dismiss_on_scroll=true node_ref=popup_ref open=open anchor=anchor placement=Placement::BelowStart gap=0.0 class="op-value-popup" role="dialog" label=popup_label.get()>
            <div style=move || popup_style.get()>
            <Calendar
                selected=selected
                on_pick=pick
                month_names=months.get_value().unwrap_or_default()
                weekday_names=weekdays.get_value().unwrap_or_default()
            />
            </div>
        </Popover>
    }
}
