//! Live examples for value fields.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn value_fields() -> AnyView {
    let temp = RwSignal::new(Some(21.5));
    let limit = RwSignal::new(None::<f64>);
    let delay = RwSignal::new(90_000_i64);
    let offset = RwSignal::new(-900_000_i64);
    let alarm = RwSignal::new(7 * 3600 + 30 * 60_u32);
    let when = RwSignal::new(
        LocalDateTime::new(2026, 11, 17, 7 * 3600).unwrap_or(LocalDateTime {
            year: 2026,
            month: 1,
            day: 1,
            second_of_day: 0,
        }),
    );
    let color = RwSignal::new("#2ec4b6".to_string());
    let swatches: Vec<String> = [
        "#e63946", "#f4a261", "#e9c46a", "#2a9d8f", "#2ec4b6", "#457b9d", "#6d597a", "#f1faee",
    ]
    .iter()
    .map(|s| (*s).to_string())
    .collect();
    view! {
        <Example title="Number with a unit" note="Arrow Up/Down steps; invalid or out-of-range text rolls back with the error pulse." block=true>
            <div class="demo-grid-2">
                <NumberField label="Target" value=temp unit=Signal::stored("°C".to_string()) step=0.5 min=5.0 max=30.0/>
                <NumberField label="Limit (optional)" value=limit unit=Signal::stored("kWh".to_string()) allow_empty=true placeholder="No limit"/>
            </div>
        </Example>
        <Example title="Duration and time" note="Durations accept 1h 30m, 90s, 1:30 and a sign; times accept 07:30 or 0730." block=true>
            <div class="demo-grid-2">
                <DurationField label="Delay" value=delay/>
                <DurationField label="Offset" value=offset step_ms=60_000/>
                <TimeField label="Alarm" value=alarm/>
            </div>
        </Example>
        <Example title="Date, time and colour" note="The arrow opens a calendar or the swatches; text input works too." block=true>
            <div class="demo-grid-2">
                <DateTimeField label="Start" value=when/>
                <ColorField label="Colour" value=color swatches=swatches/>
            </div>
        </Example>
        <span class="op-t-caption demo-muted">
            {move || format!(
                "temp={:?} limit={:?} delay={}ms offset={}ms alarm={}s when={:?} color={}",
                temp.get(), limit.get(), delay.get(), offset.get(), alarm.get(), when.get(), color.get()
            )}
        </span>
    }
    .into_any()
}
