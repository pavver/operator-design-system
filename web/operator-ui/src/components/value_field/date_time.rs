//! DateTimeField and its month Calendar.

use super::parse::{LocalDateTime, days_in_month, format_date_time, parse_date_time, weekday};
use super::parsed::ParsedField;
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::primitives::Icon;
use crate::core::ControlState;
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};
use std::sync::Arc;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

/// Today's local date from the browser clock.
fn today() -> (i32, u32, u32) {
    let d = js_sys::Date::new_0();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    (d.get_full_year() as i32, d.get_month() + 1, d.get_date())
}

/// Month grid for picking a day; weeks start on Monday.
#[component]
pub fn Calendar(
    /// The selected day.
    #[prop(into)]
    selected: Signal<Option<(i32, u32, u32)>>,
    on_pick: Callback<(i32, u32, u32)>,
    /// Month names, January first (English by default).
    #[prop(optional)]
    month_names: Option<Vec<String>>,
    /// Weekday names, Monday first (English by default).
    #[prop(optional)]
    weekday_names: Option<Vec<String>>,
) -> impl IntoView {
    let now = today();
    let start = selected.get_untracked().unwrap_or(now);
    let shown = RwSignal::new((start.0, start.1));
    let months: Vec<String> =
        month_names.unwrap_or_else(|| MONTHS.iter().map(|s| (*s).to_string()).collect());
    let weekdays: Vec<String> =
        weekday_names.unwrap_or_else(|| WEEKDAYS.iter().map(|s| (*s).to_string()).collect());
    let shift = move |delta: i32| {
        shown.update(|(y, m)| {
            let index = *y * 12 + i32::try_from(*m).unwrap_or(1) - 1 + delta;
            *y = index.div_euclid(12);
            *m = u32::try_from(index.rem_euclid(12)).unwrap_or(0) + 1;
        });
    };
    let title = move || {
        let (y, m) = shown.get();
        let name = months
            .get(usize::try_from(m).unwrap_or(1) - 1)
            .cloned()
            .unwrap_or_default();
        format!("{name} {y}")
    };
    let cells = move || {
        let (y, m) = shown.get();
        let lead = weekday(y, m, 1);
        let mut out: Vec<Option<u32>> = (0..lead).map(|_| None).collect();
        out.extend((1..=days_in_month(y, m)).map(Some));
        out
    };
    view! {
        <div class="op-calendar">
            <div class="op-calendar-head">
                <button type="button" class="op-rippled op-calendar-nav" aria-label="Previous month" on:click=move |_| shift(-1)>
                    <Icon icon=icons::CHEVRON_LEFT size=dim::ICON_SMALL/>
                </button>
                <span class="op-calendar-title">{title}</span>
                <button type="button" class="op-rippled op-calendar-nav" aria-label="Next month" on:click=move |_| shift(1)>
                    <Icon icon=icons::CHEVRON_RIGHT size=dim::ICON_SMALL/>
                </button>
            </div>
            <div class="op-calendar-grid" role="grid">
                {weekdays.into_iter().map(|w| view! { <span class="op-calendar-weekday op-t-caption">{w}</span> }).collect_view()}
                {move || {
                    let (y, m) = shown.get();
                    cells()
                        .into_iter()
                        .map(|cell| match cell {
                            None => view! { <span></span> }.into_any(),
                            Some(d) => {
                                let date = (y, m, d);
                                view! {
                                    <button
                                        type="button"
                                        class="op-rippled op-calendar-day"
                                        data-today=(date == now).then_some("")
                                        aria-selected=move || (selected.get() == Some(date)).to_string()
                                        on:click=move |_| on_pick.run(date)
                                    >
                                        {d}
                                    </button>
                                }
                                .into_any()
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}

/// Local date and time input (`2026-11-17 07:30`) with a calendar for the date.
#[component]
pub fn DateTimeField(
    value: RwSignal<LocalDateTime>,
    /// Always shows seconds.
    #[prop(optional)]
    seconds: bool,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<LocalDateTime>>,
    #[prop(optional)] month_names: Option<Vec<String>>,
    #[prop(optional)] weekday_names: Option<Vec<String>>,
) -> impl IntoView {
    let format = Callback::new(move |v: LocalDateTime| format_date_time(v, seconds));
    let parse = Callback::new(move |t: String| parse_date_time(&t));
    let open = RwSignal::new(false);
    let host = NodeRef::<leptos::html::Div>::new();
    let anchor = Signal::derive(move || {
        host.get()
            .map(|el| PopoverAnchor::of(&el))
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
                aria-label="Pick a date"
                aria-expanded=move || open.get().to_string()
                disabled=move || !enabled.get()
                on:mousedown=|ev| ev.prevent_default()
                on:click=move |_| open.update(|o| *o = !*o)
            >
                <Icon icon=icons::ARROW_DROP_DOWN size=5.0/>
            </button>
        }
        .into_any()
    });
    let months = StoredValue::new(month_names);
    let weekdays = StoredValue::new(weekday_names);
    view! {
        <div class="op-value-anchor" node_ref=host>
            <ParsedField
                value=value
                format=format
                parse=parse
                placeholder=placeholder
                label=label
                enabled=enabled
                state=state
                class="op-date-time-field"
                on_commit=on_commit
                trailing=Some(trailing)
            />
        </div>
        <Popover open=open anchor=anchor placement=Placement::BelowStart gap=0.0 class="op-value-popup" role="dialog" label="Calendar">
            <Calendar
                selected=selected
                on_pick=pick
                month_names=months.get_value().unwrap_or_else(|| MONTHS.iter().map(|s| (*s).to_string()).collect())
                weekday_names=weekdays.get_value().unwrap_or_else(|| WEEKDAYS.iter().map(|s| (*s).to_string()).collect())
            />
        </Popover>
    }
}
