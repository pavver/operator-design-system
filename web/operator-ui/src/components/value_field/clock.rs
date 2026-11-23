//! ClockPicker: a clock dial for picking hours, then minutes, by dragging the hand.

use super::parse::{dial_angle, dial_hour, dial_minute};
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Fractions of the dial radius for the outer and inner rings.
const OUTER: f64 = 0.8;
const INNER: f64 = 0.52;

/// Which hand the dial sets.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Hand {
    Hours,
    Minutes,
}

/// The pointer offset from the dial centre as fractions of its radius.
fn offset(ev: &web_sys::PointerEvent) -> Option<(f64, f64)> {
    let el = ev.current_target()?.dyn_into::<web_sys::Element>().ok()?;
    let r = el.get_bounding_client_rect();
    let half = r.width().min(r.height()) / 2.0;
    if half <= 0.0 {
        return None;
    }
    Some((
        (f64::from(ev.client_x()) - r.left() - r.width() / 2.0) / half,
        (f64::from(ev.client_y()) - r.top() - r.height() / 2.0) / half,
    ))
}

/// Position on the dial for a clockwise `angle` at `radius` (a fraction of the dial radius), as
/// CSS `left` and `top` percentages.
fn place(angle: f64, radius: f64) -> (String, String) {
    let a = angle.to_radians();
    (
        format!("{:.3}%", 50.0 + 50.0 * radius * a.sin()),
        format!("{:.3}%", 50.0 - 50.0 * radius * a.cos()),
    )
}

/// Clock dial for a time of day: pick the hour, then the minute.
///
/// The value is seconds since midnight; seconds are kept. In 24-hour mode the inner ring holds
/// `00` and `13`–`23`; with `hour12` the dial shows `12` and `1`–`11` with an AM/PM switch.
#[component]
pub fn ClockPicker(
    value: RwSignal<u32>,
    /// Shows a 12-hour dial with AM and PM.
    #[prop(optional)]
    hour12: bool,
    /// Minutes snap to this step on the dial.
    #[prop(default = 1)]
    minute_step: u32,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Runs when a hand is released, a key changes the time or AM/PM switches.
    #[prop(optional)]
    on_commit: Option<Callback<u32>>,
    /// Runs when the minute hand is released or Enter is pressed on the minutes.
    #[prop(optional)]
    on_done: Option<Callback<()>>,
    #[prop(optional, into)] hours_label: MaybeProp<String>,
    #[prop(optional, into)] minutes_label: MaybeProp<String>,
    #[prop(optional, into)] am_label: MaybeProp<String>,
    #[prop(optional, into)] pm_label: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] number_size: MaybeProp<f32>,
    #[prop(optional, into)] hand_width: MaybeProp<f32>,
    #[prop(optional, into)] readout_font_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let hours_label = ui_text(hours_label, "clock-picker.hours", "Hours");
    let minutes_label = ui_text(minutes_label, "clock-picker.minutes", "Minutes");
    let am_label = ui_text(am_label, "clock-picker.am", "AM");
    let pm_label = ui_text(pm_label, "clock-picker.pm", "PM");
    let custom_style = root_style(
        vec![
            Metric::du("--op-field-clock-size", size),
            Metric::du("--op-field-popup-padding", padding),
            Metric::du("--op-field-popup-gap", gap),
            Metric::du("--op-field-clock-number-size", number_size),
            Metric::du("--op-field-clock-hand-width", hand_width),
            Metric::tu("--op-field-clock-readout-font-size", readout_font_size),
            Metric::tu("--op-field-font-size", font_size),
        ],
        style,
    );
    let minute_step = minute_step.clamp(1, 30);
    let hand = RwSignal::new(Hand::Hours);
    let dragging = RwSignal::new(false);
    let hour = move || value.get() / 3600 % 24;
    let minute = move || value.get() / 60 % 60;
    let pm = move || hour() >= 12;

    let set = move |h: u32, m: u32, commit: bool| {
        if !enabled.get_untracked() {
            return;
        }
        let next = (h % 24) * 3600 + (m % 60) * 60 + value.get_untracked() % 60;
        if next != value.get_untracked() {
            value.set(next);
        }
        if commit && let Some(cb) = on_commit {
            cb.run(next);
        }
    };
    let pick = move |(dx, dy): (f64, f64), commit: bool| {
        let angle = dial_angle(dx, dy);
        let now = value.get_untracked();
        let (h, m) = (now / 3600 % 24, now / 60 % 60);
        match hand.get_untracked() {
            Hand::Hours => {
                let inner = !hour12 && dx.hypot(dy) < f64::midpoint(OUTER, INNER);
                set(dial_hour(angle, inner, hour12, h >= 12), m, commit);
            }
            Hand::Minutes => set(h, dial_minute(angle, minute_step), commit),
        }
    };
    let release = move || {
        if hand.get_untracked() == Hand::Hours {
            hand.set(Hand::Minutes);
        } else if let Some(done) = on_done {
            done.run(());
        }
    };

    // The selected number, its ring and the hand angle.
    let hand_angle = move || match hand.get() {
        Hand::Hours => f64::from(hour() % 12) * 30.0,
        Hand::Minutes => f64::from(minute()) * 6.0,
    };
    let hand_radius = move || {
        let h = hour();
        if hand.get() == Hand::Hours && !hour12 && (h == 0 || h > 12) {
            INNER
        } else {
            OUTER
        }
    };
    let numbers = move || {
        let mut out: Vec<(String, f64, f64, bool)> = Vec::new();
        match hand.get() {
            Hand::Hours => {
                let h = hour();
                for i in 0..12_u32 {
                    let angle = f64::from(i) * 30.0;
                    let outer = if i == 0 { 12 } else { i };
                    let outer_value = if hour12 {
                        i + if h >= 12 { 12 } else { 0 }
                    } else {
                        outer
                    };
                    out.push((outer.to_string(), angle, OUTER, h == outer_value));
                    if !hour12 {
                        let inner = if i == 0 { 0 } else { i + 12 };
                        out.push((format!("{inner:02}"), angle, INNER, h == inner));
                    }
                }
            }
            Hand::Minutes => {
                let m = minute();
                for i in 0..12_u32 {
                    out.push((
                        format!("{:02}", i * 5),
                        f64::from(i) * 30.0,
                        OUTER,
                        m == i * 5,
                    ));
                }
            }
        }
        out
    };
    let valuetext = move || match hand.get() {
        Hand::Hours => format!("{} {:02}", hours_label.get(), hour()),
        Hand::Minutes => format!("{} {:02}", minutes_label.get(), minute()),
    };
    let shown_hour = move || {
        let h = hour();
        if hour12 {
            let h = h % 12;
            format!("{:02}", if h == 0 { 12 } else { h })
        } else {
            format!("{h:02}")
        }
    };
    let on_key = move |ev: web_sys::KeyboardEvent| {
        if !enabled.get_untracked() {
            return;
        }
        let (h, m) = (hour_untracked(value), value.get_untracked() / 60 % 60);
        let delta: i64 = match ev.key().as_str() {
            "ArrowUp" | "ArrowRight" => 1,
            "ArrowDown" | "ArrowLeft" => -1,
            "PageUp" => 5,
            "PageDown" => -5,
            "Enter" => {
                ev.prevent_default();
                release();
                return;
            }
            _ => return,
        };
        ev.prevent_default();
        match hand.get_untracked() {
            Hand::Hours => {
                let next = (i64::from(h) + delta).rem_euclid(24);
                set(u32::try_from(next).unwrap_or(0), m, true);
            }
            Hand::Minutes => {
                let next = (i64::from(m) + delta * i64::from(minute_step)).rem_euclid(60);
                set(h, u32::try_from(next).unwrap_or(0), true);
            }
        }
    };

    view! {
        <div class=root_class("op-clock-picker", class) style=custom_style>
            {header.run()}
            <div class="op-clock-readout">
                <button
                    type="button"
                    class="op-rippled op-clock-part"
                    aria-label=move || hours_label.get()
                    aria-pressed=move || (hand.get() == Hand::Hours).to_string()
                    on:click=move |_| hand.set(Hand::Hours)
                >
                    {shown_hour}
                </button>
                <span class="op-clock-colon" aria-hidden="true">":"</span>
                <button
                    type="button"
                    class="op-rippled op-clock-part"
                    aria-label=move || minutes_label.get()
                    aria-pressed=move || (hand.get() == Hand::Minutes).to_string()
                    on:click=move |_| hand.set(Hand::Minutes)
                >
                    {move || format!("{:02}", minute())}
                </button>
                {hour12.then(|| view! {
                    <div class="op-clock-meridiem" role="group">
                        <button
                            type="button"
                            class="op-rippled op-clock-period op-t-caption"
                            aria-pressed=move || (!pm()).to_string()
                            disabled=move || !enabled.get()
                            on:click=move |_| if pm() { set(hour_untracked(value) - 12, value.get_untracked() / 60 % 60, true) }
                        >
                            {move || am_label.get()}
                        </button>
                        <button
                            type="button"
                            class="op-rippled op-clock-period op-t-caption"
                            aria-pressed=move || pm().to_string()
                            disabled=move || !enabled.get()
                            on:click=move |_| if !pm() { set(hour_untracked(value) + 12, value.get_untracked() / 60 % 60, true) }
                        >
                            {move || pm_label.get()}
                        </button>
                    </div>
                })}
            </div>
            <div
                class="op-clock-dial"
                role="slider"
                tabindex=move || if enabled.get() { "0" } else { "-1" }
                aria-label=move || match hand.get() { Hand::Hours => hours_label.get(), Hand::Minutes => minutes_label.get() }
                aria-valuemin="0"
                aria-valuemax=move || if hand.get() == Hand::Hours { "23" } else { "59" }
                aria-valuenow=move || if hand.get() == Hand::Hours { hour() } else { minute() }
                aria-valuetext=valuetext
                aria-disabled=move || (!enabled.get()).then_some("true")
                data-dragging=move || dragging.get().then_some("")
                on:pointerdown=move |ev| {
                    if !enabled.get_untracked() || ev.button() != 0 {
                        return;
                    }
                    ev.prevent_default();
                    if let Some(el) = ev.current_target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
                        let _ = el.set_pointer_capture(ev.pointer_id());
                        let _ = el.focus();
                    }
                    dragging.set(true);
                    if let Some(p) = offset(&ev) {
                        pick(p, false);
                    }
                }
                on:pointermove=move |ev| {
                    if dragging.get_untracked() && let Some(p) = offset(&ev) {
                        pick(p, false);
                    }
                }
                on:pointerup=move |ev| {
                    if dragging.get_untracked() {
                        dragging.set(false);
                        if let Some(p) = offset(&ev) {
                            pick(p, true);
                        }
                        release();
                    }
                }
                on:pointercancel=move |_| dragging.set(false)
                on:keydown=on_key
            >
                <span
                    class="op-clock-hand"
                    style:height=move || format!("{:.3}%", hand_radius() * 50.0)
                    style:transform=move || format!("translateX(-50%) rotate({:.3}deg)", hand_angle())
                ></span>
                <span class="op-clock-center"></span>
                {move || {
                    let (left, top) = place(hand_angle(), hand_radius());
                    view! { <span class="op-clock-tip" style:left=left style:top=top></span> }
                }}
                {move || numbers()
                    .into_iter()
                    .map(|(text, angle, radius, selected)| {
                        let (left, top) = place(angle, radius);
                        view! {
                            <span
                                class="op-clock-number"
                                data-inner=(radius < OUTER).then_some("")
                                data-selected=selected.then_some("")
                                style:left=left
                                style:top=top
                                aria-hidden="true"
                            >
                                {text}
                            </span>
                        }
                    })
                    .collect_view()}
            </div>
            {footer.run()}
        </div>
    }
}

fn hour_untracked(value: RwSignal<u32>) -> u32 {
    value.get_untracked() / 3600 % 24
}
