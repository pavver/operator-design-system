//! Live examples for value fields.

use crate::i18n::l;
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
    let overlay = RwSignal::new("#e6394699".to_string());
    let lamp = RwSignal::new("#ffb46b".to_string());
    let lamp_kelvin = RwSignal::new(Some(3000_u32));
    let picked = RwSignal::new("#457b9d".to_string());
    let swatches: Vec<String> = [
        "#e63946", "#f4a261", "#e9c46a", "#2a9d8f", "#2ec4b6", "#457b9d", "#6d597a", "#f1faee",
    ]
    .iter()
    .map(|s| (*s).to_string())
    .collect();
    let swatches = StoredValue::new(swatches);
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
                <ColorField label="Colour" value=color swatches=swatches.get_value()/>
            </div>
        </Example>
        <Example title="Colour picker modes" note="Spectrum, RGB, HSL and temperature editors; the code updates live while dragging. Opacity adds an alpha byte." block=true>
            <div class="demo-grid-2">
                <ColorField label=l("Overlay (with opacity)", "Накладання (з прозорістю)") value=overlay alpha=true swatches=swatches.get_value()/>
                <ColorField label=l("Lamp (temperature)", "Лампа (температура)") value=lamp temperature=lamp_kelvin
                    modes=vec![ColorMode::Temperature, ColorMode::Spectrum] min_kelvin=2200.0 max_kelvin=6500.0/>
            </div>
            <span class="op-t-caption demo-muted">
                {move || format!("overlay={} lamp={} kelvin={:?}", overlay.get(), lamp.get(), lamp_kelvin.get())}
            </span>
        </Example>
        <Example title="Inline ColorPicker" note="The editor alone, for panels and dialogs." block=true>
            <div class="demo-stack">
                <ColorPicker value=picked alpha=true swatches=swatches.get_value()/>
                <span class="op-t-caption demo-muted">{move || picked.get()}</span>
            </div>
        </Example>
        <span class="op-t-caption demo-muted">
            {move || format!(
                "temp={:?} limit={:?} delay={}ms offset={}ms alarm={}s when={:?} color={}",
                temp.get(), limit.get(), delay.get(), offset.get(), alarm.get(), when.get(), color.get()
            )}
        </span>
        <Example title="Customization" block=true>
            <div class="demo-grid-2">
                <NumberField value=RwSignal::new(Some(12.0)) label=l("Amount", "Кількість")
                    height=12.0 padding_x=4.0 unit_gap=2.0 unit=Signal::stored("kg".to_string())
                    prepend=|| view! { <Icon icon=icons::INFO/> }/>
                <DurationField value=RwSignal::new(90_000_i64) label=l("Duration", "Тривалість")
                    height=12.0 gap=2.0 minute_unit=l("m", "хв") second_unit=l("s", "с")
                    after=|| view! { <span>{l("elapsed", "минуло")}</span> }/>
                <TimeField value=RwSignal::new(27_000_u32) label=l("Time", "Час") height=12.0 font_size=16.0
                    before=|| view! { <Icon icon=icons::INFO/> }/>
                <DateTimeField value=when label=l("Start date", "Дата початку") height=12.0 icon_size=6.0 calendar_cell_size=10.0
                    picker_label=l("Open calendar", "Відкрити календар") append=|| view! { <span>"local"</span> }/>
                <ColorField value=color label=l("Colour", "Колір") height=12.0 preview_size=6.0 swatch_size=10.0
                    plane_height=32.0 bar_height=8.0 before=|| view! { <Icon icon=icons::INFO/> }/>
                <ParsedField value=RwSignal::new(42_i32) format=Callback::new(|v: i32| v.to_string())
                    parse=Callback::new(|text: String| text.parse::<i32>().ok())
                    label=l("Custom parser", "Власний парсер") height=12.0 padding_x=4.0
                    append=|| view! { <span>{l("items", "елементів")}</span> }/>
                <Calendar selected=Signal::stored(Some((2026, 11, 17))) on_pick=Callback::new(|_| {})
                    cell_size=10.0 padding=3.0 next_label=l("Next month", "Наступний місяць")
                    footer=|| view! { <span>{l("Choose a day", "Виберіть день")}</span> }/>
            </div>
        </Example>
    }
    .into_any()
}
