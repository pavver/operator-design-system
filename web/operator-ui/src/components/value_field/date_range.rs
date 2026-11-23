//! DateRangeField: a range of days with quick presets and two linked month calendars.

use super::calendar::{Calendar, MONTHS, today};
use super::parse::{DateRange, DateRangePreset, LocalDate, format_date_range, parse_date_range};
use super::parsed::ParsedField;
use super::portal_style::{field_popup_label, field_popup_style};
use crate::components::field::{FieldMetrics, FieldVariant};
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::primitives::Icon;
use crate::core::{ControlState, Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};
use std::sync::Arc;

const SHORT_MONTHS: &str = "Jan,Feb,Mar,Apr,May,Jun,Jul,Aug,Sep,Oct,Nov,Dec";

const MONTH_KEYS: [&str; 12] = [
    "calendar.january",
    "calendar.february",
    "calendar.march",
    "calendar.april",
    "calendar.may",
    "calendar.june",
    "calendar.july",
    "calendar.august",
    "calendar.september",
    "calendar.october",
    "calendar.november",
    "calendar.december",
];

fn next_month((y, m): (i32, u32), delta: i32) -> (i32, u32) {
    let index = y * 12 + i32::try_from(m).unwrap_or(1) - 1 + delta;
    (
        index.div_euclid(12),
        u32::try_from(index.rem_euclid(12)).unwrap_or(0) + 1,
    )
}

/// Input for a range of days (`12 Jul – 5 Nov 2026`) with quick presets and two calendars.
///
/// The first press on a day starts the range, the pointer previews it, and the second press
/// ends it; the order of the two days does not matter.
#[component]
pub fn DateRangeField(
    value: RwSignal<Option<DateRange>>,
    /// Quick ranges beside the calendars; `None` uses [`DateRangePreset::defaults`], an empty
    /// list hides the column.
    #[prop(optional)]
    presets: Option<Vec<DateRangePreset>>,
    /// Lets the field be cleared to `None`.
    #[prop(optional)]
    allow_empty: bool,
    /// Short month names for the text, January first.
    #[prop(optional)]
    short_month_names: Option<Vec<String>>,
    #[prop(optional)] month_names: Option<Vec<String>>,
    #[prop(optional)] weekday_names: Option<Vec<String>>,
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
    #[prop(optional, into)] presets_label: MaybeProp<String>,
    #[prop(optional, into)] start_hint: MaybeProp<String>,
    #[prop(optional, into)] end_hint: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_blur: MaybeProp<f32>,
    #[prop(optional, into)] calendar_cell_size: MaybeProp<f32>,
    #[prop(optional, into)] presets_width: MaybeProp<f32>,
    #[prop(optional, into)] preset_height: MaybeProp<f32>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<Option<DateRange>>>,
) -> impl IntoView {
    let placeholder = ui_text(placeholder, "date-range-field.placeholder", "");
    let aria_label = ui_text(aria_label, "date-range-field.label", "Date range");
    let picker_label = ui_text(picker_label, "date-range-field.pick", "Pick dates");
    let popup_label = ui_text(popup_label, "date-range-field.popup", "Date range");
    let presets_label = ui_text(presets_label, "date-range-field.presets", "Quick ranges");
    let start_hint = ui_text(
        start_hint,
        "date-range-field.start-hint",
        "Pick the first day",
    );
    let end_hint = ui_text(end_hint, "date-range-field.end-hint", "Pick the last day");
    let icon = ui_icon(icon, "date-range-field.expand", icons::ARROW_DROP_DOWN);
    let short_months = ui_text(
        short_month_names.map(|names| names.join(",")).into(),
        "date-range-field.months",
        SHORT_MONTHS,
    );
    let full_months: Vec<Signal<String>> = MONTHS
        .iter()
        .enumerate()
        .map(|(i, default)| {
            ui_text(
                month_names
                    .as_ref()
                    .and_then(|names| names.get(i))
                    .cloned()
                    .into(),
                MONTH_KEYS[i],
                default,
            )
        })
        .collect();
    let full_months = StoredValue::new(full_months);
    let shorts = move || -> Vec<String> {
        short_months
            .get_untracked()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };
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
    metrics.push(Metric::du(
        "--op-field-calendar-cell-size",
        calendar_cell_size,
    ));
    metrics.push(Metric::du("--op-field-range-presets-width", presets_width));
    metrics.push(Metric::du("--op-field-range-preset-height", preset_height));
    let custom_style = Signal::derive(root_style(metrics, style));

    let format = Callback::new(move |v: Option<DateRange>| {
        v.map(|r| format_date_range(r, &shorts()))
            .unwrap_or_default()
    });
    let parse = Callback::new(move |t: String| {
        if t.trim().is_empty() {
            return allow_empty.then_some(None);
        }
        let lists = vec![
            shorts(),
            full_months.with_value(|m| m.iter().map(|s| s.get_untracked()).collect()),
            MONTHS.iter().map(|s| (*s).to_string()).collect(),
        ];
        parse_date_range(&t, &lists, today().0).map(Some)
    });

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

    // The first press on a day; the pointer then previews the range until the second press.
    let first_day = RwSignal::new(None::<LocalDate>);
    let hover = RwSignal::new(None::<LocalDate>);
    let start = value.get_untracked().map_or_else(today, |r| r.start);
    let left = RwSignal::new((start.0, start.1));
    let right = RwSignal::new(next_month((start.0, start.1), 1));
    Effect::new(move |_| {
        let l = left.get();
        if right.get_untracked() != next_month(l, 1) {
            right.set(next_month(l, 1));
        }
    });
    Effect::new(move |_| {
        let r = right.get();
        if left.get_untracked() != next_month(r, -1) {
            left.set(next_month(r, -1));
        }
    });
    // Opening resets the selection and shows the months of the current range.
    Effect::new(move |_| {
        if open.get() {
            first_day.set(None);
            hover.set(None);
            let s = value.get_untracked().map_or_else(today, |r| r.start);
            left.set((s.0, s.1));
        }
    });
    let commit = move |r: DateRange| {
        if value.get_untracked() != Some(r) {
            value.set(Some(r));
            if let Some(cb) = on_commit {
                cb.run(Some(r));
            }
        }
        open.set(false);
    };
    let shown_range = Signal::derive(move || match first_day.get() {
        Some(a) => Some(DateRange::new(a, hover.get().unwrap_or(a))),
        None => value.get(),
    });
    let pick = Callback::new(move |d: LocalDate| {
        if blocked() {
            return;
        }
        match first_day.get_untracked() {
            None => first_day.set(Some(d)),
            Some(a) => commit(DateRange::new(a, d)),
        }
    });
    let on_hover = Callback::new(move |d: Option<LocalDate>| hover.set(d));

    let presets = presets.unwrap_or_else(DateRangePreset::defaults);
    let preset_labels = StoredValue::new(presets);
    let preset_text = move |p: &DateRangePreset| -> Signal<String> {
        match p {
            DateRangePreset::Today => {
                ui_text(MaybeProp::default(), "date-range-field.today", "Today")
            }
            DateRangePreset::Yesterday => ui_text(
                MaybeProp::default(),
                "date-range-field.yesterday",
                "Yesterday",
            ),
            DateRangePreset::ThisWeek => ui_text(
                MaybeProp::default(),
                "date-range-field.this-week",
                "This week",
            ),
            DateRangePreset::LastWeek => ui_text(
                MaybeProp::default(),
                "date-range-field.last-week",
                "Last week",
            ),
            DateRangePreset::ThisMonth => ui_text(
                MaybeProp::default(),
                "date-range-field.this-month",
                "This month",
            ),
            DateRangePreset::LastMonth => ui_text(
                MaybeProp::default(),
                "date-range-field.last-month",
                "Last month",
            ),
            DateRangePreset::LastDays(n) => {
                let n = *n;
                let t = ui_text(
                    MaybeProp::default(),
                    "date-range-field.last-days",
                    "Last {n} days",
                );
                Signal::derive(move || t.get().replace("{n}", &n.to_string()))
            }
            DateRangePreset::ThisYear => ui_text(
                MaybeProp::default(),
                "date-range-field.this-year",
                "This year",
            ),
            DateRangePreset::LastYear => ui_text(
                MaybeProp::default(),
                "date-range-field.last-year",
                "Last year",
            ),
            DateRangePreset::Custom(label, _) => Signal::stored(label.clone()),
        }
    };

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
    let calendar = move |shown: RwSignal<(i32, u32)>, first: bool| {
        view! {
            <Calendar
                selected=Signal::stored(None)
                range=shown_range
                on_pick=pick
                on_hover=on_hover
                month=shown
                hide_previous=!first
                hide_next=first
                month_names=months.get_value().unwrap_or_default()
                weekday_names=weekdays.get_value().unwrap_or_default()
            />
        }
    };
    view! {
        <div class=root_class("op-value-anchor op-date-range-field", class) style=move || custom_style.get() node_ref=host>
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
            <div class="op-range-popup" style=move || popup_style.get()>
                {move || {
                    let list = preset_labels.get_value();
                    (!list.is_empty()).then(|| view! {
                        <div class="op-range-presets" role="listbox" aria-label=move || presets_label.get()>
                            {list
                                .into_iter()
                                .map(|p| {
                                    let text = preset_text(&p);
                                    let preset = p.clone();
                                    let current = p.clone();
                                    view! {
                                        <button
                                            type="button"
                                            role="option"
                                            class="op-rippled op-range-preset"
                                            aria-selected=move || (value.get() == Some(current.resolve(today()))).to_string()
                                            disabled=blocked
                                            on:click=move |_| commit(preset.resolve(today()))
                                        >
                                            {move || text.get()}
                                        </button>
                                    }
                                })
                                .collect_view()}
                        </div>
                    })
                }}
                <div class="op-range-main">
                    <div class="op-range-months">
                        {calendar(left, true)}
                        {calendar(right, false)}
                    </div>
                    <div class="op-range-status op-t-caption" aria-live="polite">
                        {move || if first_day.get().is_some() { end_hint.get() } else { start_hint.get() }}
                    </div>
                </div>
            </div>
        </Popover>
    }
}
