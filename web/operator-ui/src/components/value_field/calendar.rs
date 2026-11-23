//! Localized month grid used by DateTimeField.
use super::parse::{DateRange, days_in_month, weekday};
use crate::components::primitives::Icon;
use crate::core::{Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};

pub(super) const MONTHS: [&str; 12] = [
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
pub(super) const WEEKDAYS: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

/// Today's local date from the browser clock.
pub(super) fn today() -> (i32, u32, u32) {
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
    /// Highlights a range of days: its ends are filled, the days between are tinted.
    #[prop(optional, into)]
    range: Signal<Option<DateRange>>,
    /// Runs with the day under the pointer, and with `None` when the pointer leaves the grid.
    #[prop(optional)]
    on_hover: Option<Callback<Option<(i32, u32, u32)>>>,
    /// The shown month as `(year, month)`; set it to link calendars or to jump.
    #[prop(optional)]
    month: Option<RwSignal<(i32, u32)>>,
    /// Hides the previous-month button (the first of two linked calendars keeps it).
    #[prop(optional)]
    hide_previous: bool,
    /// Hides the next-month button.
    #[prop(optional)]
    hide_next: bool,
    /// Month names, January first (English by default).
    #[prop(optional)]
    month_names: Option<Vec<String>>,
    /// Weekday names, Monday first (English by default).
    #[prop(optional)]
    weekday_names: Option<Vec<String>>,
    #[prop(optional, into)] previous_label: MaybeProp<String>,
    #[prop(optional, into)] next_label: MaybeProp<String>,
    #[prop(optional)] previous_icon: Option<IconData>,
    #[prop(optional)] next_icon: Option<IconData>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] cell_size: MaybeProp<f32>,
    #[prop(optional, into)] header_height: MaybeProp<f32>,
    #[prop(optional, into)] weekday_height: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] caption_font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] caption_line_height: MaybeProp<f32>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let now = today();
    let start = selected.get_untracked().unwrap_or(now);
    let shown = month.unwrap_or_else(|| RwSignal::new((start.0, start.1)));
    let month_keys = [
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
    let weekday_keys = [
        "calendar.monday",
        "calendar.tuesday",
        "calendar.wednesday",
        "calendar.thursday",
        "calendar.friday",
        "calendar.saturday",
        "calendar.sunday",
    ];
    let months: Vec<_> = MONTHS
        .iter()
        .enumerate()
        .map(|(i, default)| {
            ui_text(
                month_names
                    .as_ref()
                    .and_then(|names| names.get(i))
                    .cloned()
                    .into(),
                month_keys[i],
                default,
            )
        })
        .collect();
    let weekdays: Vec<_> = WEEKDAYS
        .iter()
        .enumerate()
        .map(|(i, default)| {
            ui_text(
                weekday_names
                    .as_ref()
                    .and_then(|names| names.get(i))
                    .cloned()
                    .into(),
                weekday_keys[i],
                default,
            )
        })
        .collect();
    let previous_label = ui_text(previous_label, "calendar.previous", "Previous month");
    let next_label = ui_text(next_label, "calendar.next", "Next month");
    let previous_icon = ui_icon(previous_icon, "calendar.previous", icons::CHEVRON_LEFT);
    let next_icon = ui_icon(next_icon, "calendar.next", icons::CHEVRON_RIGHT);
    let custom_style = root_style(
        vec![
            Metric::du("--op-field-calendar-padding", padding),
            Metric::du("--op-field-calendar-cell-size", cell_size),
            Metric::du("--op-field-calendar-header-height", header_height),
            Metric::du("--op-field-calendar-weekday-height", weekday_height),
            Metric::du("--op-field-icon-size", icon_size),
            Metric::tu("--op-field-font-size", font_size),
            Metric::tu("--op-field-caption-font-size", caption_font_size),
            Metric::tu("--op-field-line-height", line_height),
            Metric::tu("--op-field-caption-line-height", caption_line_height),
        ],
        style,
    );
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
            .map(|name| name.get())
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
        <div class=root_class("op-calendar", class) style=custom_style>
            {header.run()}
            <div class="op-calendar-head">
                <button type="button" class="op-rippled op-calendar-nav" style:visibility=hide_previous.then_some("hidden") aria-label=move || previous_label.get() on:click=move |_| shift(-1)>
                    {move || view! { <Icon icon=previous_icon.get() size=dim::ICON_SMALL attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-small));height:var(--op-field-icon-size, var(--op-dim-icon-small));"/> }}
                </button>
                <span class="op-calendar-title">{title}</span>
                <button type="button" class="op-rippled op-calendar-nav" style:visibility=hide_next.then_some("hidden") aria-label=move || next_label.get() on:click=move |_| shift(1)>
                    {move || view! { <Icon icon=next_icon.get() size=dim::ICON_SMALL attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-small));height:var(--op-field-icon-size, var(--op-dim-icon-small));"/> }}
                </button>
            </div>
            <div class="op-calendar-grid" role="grid" on:pointerleave=move |_| if let Some(cb) = on_hover { cb.run(None) }>
                {weekdays.into_iter().map(|w| view! { <span class="op-calendar-weekday op-t-caption">{move || w.get()}</span> }).collect_view()}
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
                                        aria-selected=move || (selected.get() == Some(date) || range.get().is_some_and(|r| r.start == date || r.end == date)).to_string()
                                        data-in-range=move || range.get().is_some_and(|r| r.contains(date)).then_some("")
                                        data-range-start=move || range.get().is_some_and(|r| r.start == date && r.end != date).then_some("")
                                        data-range-end=move || range.get().is_some_and(|r| r.end == date && r.start != date).then_some("")
                                        on:pointerenter=move |_| if let Some(cb) = on_hover { cb.run(Some(date)) }
                                        on:focus=move |_| if let Some(cb) = on_hover { cb.run(Some(date)) }
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
            {footer.run()}
        </div>
    }
}
