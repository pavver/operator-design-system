//! ColorPicker: spectrum, RGB, HSL and colour temperature editors with alpha, a hex input and
//! swatches. ColorField shows it in its popup.

use super::color_math::{
    Hsl, Hsv, Rgba, hsl_to_rgb, hsv_keeping, hsv_to_rgb, kelvin_to_rgb, parse_rgba, rgb_to_hsl,
    rgb_to_kelvin,
};
use crate::components::primitives::Icon;
use crate::core::{Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons};
use wasm_bindgen::JsCast;

/// True when the browser offers the EyeDropper API.
fn has_eyedropper() -> bool {
    web_sys::window()
        .and_then(|w| js_sys::Reflect::has(&w, &"EyeDropper".into()).ok())
        .unwrap_or(false)
}

/// Lets the user pick a colour anywhere on the screen; `None` when cancelled or unsupported.
async fn eyedrop() -> Option<String> {
    let window = web_sys::window()?;
    let ctor = js_sys::Reflect::get(&window, &"EyeDropper".into())
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    let dropper = js_sys::Reflect::construct(&ctor, &js_sys::Array::new()).ok()?;
    let open = js_sys::Reflect::get(&dropper, &"open".into())
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    let promise = open
        .call0(&dropper)
        .ok()?
        .dyn_into::<js_sys::Promise>()
        .ok()?;
    let result = wasm_bindgen_futures::JsFuture::from(promise).await.ok()?;
    js_sys::Reflect::get(&result, &"sRGBHex".into())
        .ok()?
        .as_string()
}

/// An editor tab of [`ColorPicker`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorMode {
    /// Saturation and brightness plane with a hue bar.
    Spectrum,
    /// Red, green and blue bars.
    Rgb,
    /// Hue, saturation and lightness bars.
    Hsl,
    /// A white-point bar from warm to cool, in kelvin.
    Temperature,
}

impl ColorMode {
    /// Every mode, in tab order.
    pub const ALL: [Self; 4] = [Self::Spectrum, Self::Rgb, Self::Hsl, Self::Temperature];
}

/// How an edit changes the colour temperature.
#[derive(Clone, Copy)]
enum Temp {
    Keep,
    Clear,
    Set(f64),
}

/// The pointer position inside the element that handles the event, as fractions of its size.
fn pointer_fraction(ev: &web_sys::PointerEvent) -> Option<(f64, f64)> {
    let el = ev.current_target()?.dyn_into::<web_sys::Element>().ok()?;
    let r = el.get_bounding_client_rect();
    if r.width() <= 0.0 || r.height() <= 0.0 {
        return None;
    }
    Some((
        ((f64::from(ev.client_x()) - r.left()) / r.width()).clamp(0.0, 1.0),
        ((f64::from(ev.client_y()) - r.top()) / r.height()).clamp(0.0, 1.0),
    ))
}

/// Keyboard step for a slider key: arrows by `step`, Page keys and Shift by ten steps, Home/End
/// to the ends. Returns the new fraction.
fn key_step(ev: &web_sys::KeyboardEvent, f: f64, step: f64, vertical: bool) -> Option<f64> {
    let big = if ev.shift_key() { 10.0 } else { 1.0 };
    let next = match ev.key().as_str() {
        "ArrowRight" if !vertical => f + step * big,
        "ArrowLeft" if !vertical => f - step * big,
        "ArrowUp" => f + step * big,
        "ArrowDown" => f - step * big,
        "PageUp" => f + step * 10.0,
        "PageDown" => f - step * 10.0,
        "Home" => 0.0,
        "End" => 1.0,
        _ => return None,
    };
    Some(next.clamp(0.0, 1.0))
}

fn gradient(stops: &[Rgba]) -> String {
    let stops: Vec<String> = stops.iter().map(|c| c.css_rgb(true)).collect();
    format!("linear-gradient(to right, {})", stops.join(", "))
}

fn opaque(c: Rgba) -> Rgba {
    Rgba { a: 1.0, ..c }
}

/// One channel: a caption, a bar and a number input.
#[component]
fn ChannelRow(
    label: Signal<String>,
    /// Position of the value along the bar.
    fraction: Signal<f64>,
    /// CSS background of the track.
    track: Signal<String>,
    /// Shows the checkerboard under the track (alpha).
    #[prop(optional)]
    checker: bool,
    /// The number shown in the input.
    number: Signal<String>,
    /// Unit after the number.
    unit: &'static str,
    /// Text of the input to a fraction, or `None` when it is not valid.
    parse: Callback<String, Option<f64>>,
    /// Keyboard step as a fraction of the bar.
    step: f64,
    on_change: Callback<(f64, bool)>,
    enabled: Signal<bool>,
) -> impl IntoView {
    let dragging = RwSignal::new(false);
    let input = NodeRef::<leptos::html::Input>::new();
    let valuetext = move || format!("{}{unit}", number.get());
    view! {
        <div class="op-color-channel">
            <span class="op-color-channel-label op-t-caption">{move || label.get()}</span>
            <div
                class="op-color-bar"
                class:op-color-checker=checker
                role="slider"
                tabindex=move || if enabled.get() { "0" } else { "-1" }
                aria-label=move || label.get()
                aria-valuemin="0"
                aria-valuemax="100"
                aria-valuenow=move || format!("{:.0}", fraction.get() * 100.0)
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
                    if let Some((x, _)) = pointer_fraction(&ev) {
                        on_change.run((x, false));
                    }
                }
                on:pointermove=move |ev| {
                    if dragging.get_untracked() && let Some((x, _)) = pointer_fraction(&ev) {
                        on_change.run((x, false));
                    }
                }
                on:pointerup=move |ev| {
                    if dragging.get_untracked() {
                        dragging.set(false);
                        if let Some((x, _)) = pointer_fraction(&ev) {
                            on_change.run((x, true));
                        }
                    }
                }
                on:pointercancel=move |_| {
                    if dragging.get_untracked() {
                        dragging.set(false);
                        on_change.run((fraction.get_untracked(), true));
                    }
                }
                on:keydown=move |ev| {
                    if !enabled.get_untracked() {
                        return;
                    }
                    if let Some(next) = key_step(&ev, fraction.get_untracked(), step, false) {
                        ev.prevent_default();
                        on_change.run((next, true));
                    }
                }
            >
                <span class="op-color-track" style:background=move || track.get()></span>
                <span class="op-color-bar-thumb" style:left=move || format!("{:.3}%", fraction.get() * 100.0)></span>
            </div>
            <label class="op-color-number">
                <input
                    node_ref=input
                    type="text"
                    inputmode="decimal"
                    aria-label=move || label.get()
                    disabled=move || !enabled.get()
                    prop:value=move || number.get()
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" && let Some(el) = input.get_untracked() {
                            let _ = el.blur();
                        }
                    }
                    on:change=move |ev| {
                        let text = event_target_value(&ev);
                        match parse.run(text) {
                            Some(f) => on_change.run((f.clamp(0.0, 1.0), true)),
                            None => {
                                if let Some(el) = input.get_untracked() {
                                    el.set_value(&number.get_untracked());
                                }
                            }
                        }
                    }
                />
                <span class="op-color-unit">{unit}</span>
            </label>
        </div>
    }
}

/// A number in `0..=max` as a fraction of `max`.
fn parse_in(max: f64) -> Callback<String, Option<f64>> {
    Callback::new(move |t: String| {
        let t = t
            .trim()
            .trim_end_matches(['%', '°', 'K', 'k'])
            .trim()
            .replace(',', ".");
        let v: f64 = t.parse().ok()?;
        (v.is_finite() && (0.0..=max).contains(&v)).then_some(v / max)
    })
}

/// Colour editor with spectrum, RGB, HSL and temperature modes, alpha, a hex input and swatches.
///
/// The value is `#rrggbb`, or `#rrggbbaa` with `alpha` when the colour is not opaque; it updates
/// live while a bar or the plane is dragged, and `on_commit` runs when an edit ends.
#[component]
pub fn ColorPicker(
    value: RwSignal<String>,
    /// Adds the opacity bar; the value gains an alpha byte when the colour is not opaque.
    #[prop(optional)]
    alpha: bool,
    /// Editor tabs in order; empty means every mode.
    #[prop(optional)]
    modes: Vec<ColorMode>,
    /// Warm end of the temperature bar, in kelvin.
    #[prop(default = 2000.0)]
    min_kelvin: f64,
    /// Cool end of the temperature bar, in kelvin.
    #[prop(default = 6500.0)]
    max_kelvin: f64,
    /// The colour temperature in kelvin while the colour comes from the temperature bar, `None`
    /// after any other edit.
    #[prop(optional)]
    temperature: Option<RwSignal<Option<u32>>>,
    /// Preset colours, in any form the hex input accepts.
    #[prop(optional)]
    swatches: Vec<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Runs when an edit ends: a bar or the plane is released, a number or the hex is entered, a
    /// swatch is picked or a key steps a bar.
    #[prop(optional)]
    on_commit: Option<Callback<String>>,
    #[prop(optional, into)] spectrum_label: MaybeProp<String>,
    #[prop(optional, into)] rgb_label: MaybeProp<String>,
    #[prop(optional, into)] hsl_label: MaybeProp<String>,
    #[prop(optional, into)] temperature_label: MaybeProp<String>,
    #[prop(optional, into)] plane_label: MaybeProp<String>,
    #[prop(optional, into)] hue_label: MaybeProp<String>,
    #[prop(optional, into)] saturation_label: MaybeProp<String>,
    #[prop(optional, into)] brightness_label: MaybeProp<String>,
    #[prop(optional, into)] lightness_label: MaybeProp<String>,
    #[prop(optional, into)] red_label: MaybeProp<String>,
    #[prop(optional, into)] green_label: MaybeProp<String>,
    #[prop(optional, into)] blue_label: MaybeProp<String>,
    #[prop(optional, into)] alpha_label: MaybeProp<String>,
    #[prop(optional, into)] hex_label: MaybeProp<String>,
    #[prop(optional, into)] swatches_label: MaybeProp<String>,
    /// Accessible name of the screen colour picker button, shown where the browser supports it.
    #[prop(optional, into)]
    eyedropper_label: MaybeProp<String>,
    #[prop(optional)] eyedropper_icon: Option<IconData>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] plane_height: MaybeProp<f32>,
    #[prop(optional, into)] bar_height: MaybeProp<f32>,
    #[prop(optional, into)] thumb_size: MaybeProp<f32>,
    #[prop(optional, into)] tab_height: MaybeProp<f32>,
    #[prop(optional, into)] number_width: MaybeProp<f32>,
    #[prop(optional, into)] swatch_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let spectrum_label = ui_text(spectrum_label, "color-picker.spectrum", "Spectrum");
    let rgb_label = ui_text(rgb_label, "color-picker.rgb", "RGB");
    let hsl_label = ui_text(hsl_label, "color-picker.hsl", "HSL");
    let temperature_label = ui_text(temperature_label, "color-picker.temperature", "Temperature");
    let plane_label = ui_text(
        plane_label,
        "color-picker.plane",
        "Saturation and brightness",
    );
    let hue_label = ui_text(hue_label, "color-picker.hue", "Hue");
    let saturation_label = ui_text(saturation_label, "color-picker.saturation", "Saturation");
    let brightness_label = ui_text(brightness_label, "color-picker.brightness", "Brightness");
    let lightness_label = ui_text(lightness_label, "color-picker.lightness", "Lightness");
    let red_label = ui_text(red_label, "color-picker.red", "Red");
    let green_label = ui_text(green_label, "color-picker.green", "Green");
    let blue_label = ui_text(blue_label, "color-picker.blue", "Blue");
    let alpha_label = ui_text(alpha_label, "color-picker.alpha", "Opacity");
    let hex_label = ui_text(hex_label, "color-picker.hex", "Colour code");
    let swatches_label = ui_text(swatches_label, "color-picker.swatches", "Presets");
    let eyedropper_label = ui_text(
        eyedropper_label,
        "color-picker.eyedropper",
        "Pick from the screen",
    );
    let eyedropper_icon = ui_icon(eyedropper_icon, "color-picker.eyedropper", icons::COLORIZE);
    let custom_style = root_style(
        vec![
            Metric::du("--op-field-color-width", width),
            Metric::du("--op-field-popup-padding", padding),
            Metric::du("--op-field-popup-gap", gap),
            Metric::du("--op-field-color-plane-height", plane_height),
            Metric::du("--op-field-color-bar-height", bar_height),
            Metric::du("--op-field-color-thumb-size", thumb_size),
            Metric::du("--op-field-color-tab-height", tab_height),
            Metric::du("--op-field-color-number-width", number_width),
            Metric::du("--op-field-swatch-size", swatch_size),
            Metric::tu("--op-field-font-size", font_size),
        ],
        style,
    );

    let modes = if modes.is_empty() {
        ColorMode::ALL.to_vec()
    } else {
        modes
    };
    let (min_k, max_k) = (min_kelvin.min(max_kelvin), max_kelvin.max(min_kelvin));
    let span_k = (max_k - min_k).max(1.0);
    let initial = parse_rgba(&value.get_untracked()).unwrap_or(Rgba {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    });
    let hsv = RwSignal::new(hsv_keeping(
        initial,
        Hsv {
            h: 0.0,
            s: 0.0,
            v: 1.0,
        },
    ));
    let a = RwSignal::new(if alpha { initial.a } else { 1.0 });
    let initial_k = temperature
        .and_then(|t| t.get_untracked())
        .map_or_else(|| rgb_to_kelvin(initial, min_k, max_k), f64::from);
    let kelvin = RwSignal::new(initial_k.clamp(min_k, max_k));
    let start_mode = if temperature.is_some_and(|t| t.get_untracked().is_some())
        && modes.contains(&ColorMode::Temperature)
    {
        ColorMode::Temperature
    } else {
        modes.first().copied().unwrap_or(ColorMode::Spectrum)
    };
    let mode = RwSignal::new(start_mode);
    let rgba = Memo::new(move |_| hsv_to_rgb(hsv.get(), a.get()));
    let hex = Memo::new(move |_| rgba.get().hex(alpha));

    // External changes of the value move the editors, keeping the hue of greys.
    Effect::new(move |_| {
        let Some(c) = parse_rgba(&value.get()) else {
            return;
        };
        if c.hex(alpha) == hex.get_untracked() {
            return;
        }
        hsv.set(hsv_keeping(c, hsv.get_untracked()));
        a.set(if alpha { c.a } else { 1.0 });
        kelvin.set(rgb_to_kelvin(c, min_k, max_k));
    });

    let apply = move |next: Hsv, next_a: f64, temp: Temp, commit: bool| {
        if !enabled.get_untracked() {
            return;
        }
        hsv.set(next);
        a.set(next_a.clamp(0.0, 1.0));
        match temp {
            Temp::Keep => {}
            Temp::Clear => {
                if let Some(t) = temperature {
                    t.set(None);
                }
            }
            Temp::Set(k) => {
                kelvin.set(k);
                if let Some(t) = temperature {
                    // The kelvin range is clamped, so the value fits.
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    t.set(Some(k.round() as u32));
                }
            }
        }
        let h = hex.get_untracked();
        if h != value.get_untracked() {
            value.set(h.clone());
        }
        if commit && let Some(cb) = on_commit {
            cb.run(h);
        }
    };
    let set_rgba = move |c: Rgba, commit: bool| {
        apply(
            hsv_keeping(c, hsv.get_untracked()),
            c.a,
            Temp::Clear,
            commit,
        );
    };
    // Hue never reaches 360 so the thumb stays at the end of the bar.
    let to_hue = |f: f64| (f * 360.0).min(359.999);

    // Spectrum plane.
    let plane_drag = RwSignal::new(false);
    let plane_set = move |(x, y): (f64, f64), commit: bool| {
        let c = hsv.get_untracked();
        apply(
            Hsv {
                h: c.h,
                s: x,
                v: 1.0 - y,
            },
            a.get_untracked(),
            Temp::Clear,
            commit,
        );
    };
    let plane_background = move || {
        let pure = hsv_to_rgb(
            Hsv {
                h: hsv.get().h,
                s: 1.0,
                v: 1.0,
            },
            1.0,
        );
        format!(
            "linear-gradient(to top, rgb(0, 0, 0), rgba(0, 0, 0, 0)), linear-gradient(to right, rgb(255, 255, 255), {})",
            pure.css_rgb(false)
        )
    };
    let plane_text = move || {
        let c = hsv.get();
        format!(
            "{} {:.0}%, {} {:.0}%",
            saturation_label.get(),
            c.s * 100.0,
            brightness_label.get(),
            c.v * 100.0
        )
    };
    let plane = move || {
        view! {
            <div
                class="op-color-plane"
                role="slider"
                tabindex=move || if enabled.get() { "0" } else { "-1" }
                aria-label=move || plane_label.get()
                aria-valuemin="0"
                aria-valuemax="100"
                aria-valuenow=move || format!("{:.0}", hsv.get().s * 100.0)
                aria-valuetext=plane_text
                aria-disabled=move || (!enabled.get()).then_some("true")
                data-dragging=move || plane_drag.get().then_some("")
                style:background=plane_background
                on:pointerdown=move |ev| {
                    if !enabled.get_untracked() || ev.button() != 0 {
                        return;
                    }
                    ev.prevent_default();
                    if let Some(el) = ev.current_target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
                        let _ = el.set_pointer_capture(ev.pointer_id());
                        let _ = el.focus();
                    }
                    plane_drag.set(true);
                    if let Some(p) = pointer_fraction(&ev) {
                        plane_set(p, false);
                    }
                }
                on:pointermove=move |ev| {
                    if plane_drag.get_untracked() && let Some(p) = pointer_fraction(&ev) {
                        plane_set(p, false);
                    }
                }
                on:pointerup=move |ev| {
                    if plane_drag.get_untracked() {
                        plane_drag.set(false);
                        if let Some(p) = pointer_fraction(&ev) {
                            plane_set(p, true);
                        }
                    }
                }
                on:pointercancel=move |_| {
                    if plane_drag.get_untracked() {
                        plane_drag.set(false);
                        let c = hsv.get_untracked();
                        plane_set((c.s, 1.0 - c.v), true);
                    }
                }
                on:keydown=move |ev| {
                    if !enabled.get_untracked() {
                        return;
                    }
                    let c = hsv.get_untracked();
                    let horizontal = matches!(ev.key().as_str(), "ArrowLeft" | "ArrowRight");
                    let vertical = matches!(ev.key().as_str(), "ArrowUp" | "ArrowDown");
                    if horizontal && let Some(s) = key_step(&ev, c.s, 0.01, false) {
                        ev.prevent_default();
                        plane_set((s, 1.0 - c.v), true);
                    } else if vertical && let Some(v) = key_step(&ev, c.v, 0.01, true) {
                        ev.prevent_default();
                        plane_set((c.s, 1.0 - v), true);
                    }
                }
            >
                <span
                    class="op-color-plane-thumb"
                    style:left=move || format!("{:.3}%", hsv.get().s * 100.0)
                    style:top=move || format!("{:.3}%", (1.0 - hsv.get().v) * 100.0)
                ></span>
            </div>
        }
    };

    let hue_track = Signal::stored(gradient(
        &(0..=6)
            .map(|i| {
                hsv_to_rgb(
                    Hsv {
                        h: f64::from(i) * 60.0,
                        s: 1.0,
                        v: 1.0,
                    },
                    1.0,
                )
            })
            .collect::<Vec<_>>(),
    ));
    let hue_row = move || {
        view! {
            <ChannelRow
                label=hue_label
                fraction=Signal::derive(move || hsv.get().h / 360.0)
                track=hue_track
                number=Signal::derive(move || format!("{:.0}", hsv.get().h.round() % 360.0))
                unit="°"
                parse=parse_in(360.0)
                step=1.0 / 360.0
                on_change=Callback::new(move |(f, commit): (f64, bool)| {
                    let c = hsv.get_untracked();
                    apply(Hsv { h: to_hue(f), ..c }, a.get_untracked(), Temp::Clear, commit);
                })
                enabled=enabled
            />
        }
    };

    let rgb_row = move |label: Signal<String>, index: usize| {
        let get = move |c: Rgba| match index {
            0 => c.r,
            1 => c.g,
            _ => c.b,
        };
        let with = move |c: Rgba, v: f64| match index {
            0 => Rgba { r: v, ..c },
            1 => Rgba { g: v, ..c },
            _ => Rgba { b: v, ..c },
        };
        view! {
            <ChannelRow
                label=label
                fraction=Signal::derive(move || get(rgba.get()))
                track=Signal::derive(move || {
                    let c = opaque(rgba.get());
                    gradient(&[with(c, 0.0), with(c, 1.0)])
                })
                number=Signal::derive(move || format!("{:.0}", get(rgba.get()) * 255.0))
                unit=""
                parse=parse_in(255.0)
                step=1.0 / 255.0
                on_change=Callback::new(move |(f, commit): (f64, bool)| set_rgba(with(rgba.get_untracked(), f), commit))
                enabled=enabled
            />
        }
    };

    // HSL keeps the editor hue so greys do not lose it.
    let hsl = Memo::new(move |_| Hsl {
        h: hsv.get().h,
        ..rgb_to_hsl(rgba.get())
    });
    let hsl_row = move |label: Signal<String>, lightness: bool| {
        let get = move |c: Hsl| if lightness { c.l } else { c.s };
        let with = move |c: Hsl, v: f64| {
            if lightness {
                Hsl { l: v, ..c }
            } else {
                Hsl { s: v, ..c }
            }
        };
        view! {
            <ChannelRow
                label=label
                fraction=Signal::derive(move || get(hsl.get()))
                track=Signal::derive(move || {
                    let c = hsl.get();
                    let stops: Vec<Rgba> = if lightness {
                        [0.0, 0.5, 1.0].iter().map(|l| hsl_to_rgb(Hsl { l: *l, ..c }, 1.0)).collect()
                    } else {
                        [0.0, 1.0].iter().map(|s| hsl_to_rgb(Hsl { s: *s, ..c }, 1.0)).collect()
                    };
                    gradient(&stops)
                })
                number=Signal::derive(move || format!("{:.0}", get(hsl.get()) * 100.0))
                unit="%"
                parse=parse_in(100.0)
                step=0.01
                on_change=Callback::new(move |(f, commit): (f64, bool)| {
                    let c = with(hsl.get_untracked(), f);
                    let next = hsl_to_rgb(c, a.get_untracked());
                    let mut h = hsv_keeping(next, hsv.get_untracked());
                    h.h = c.h;
                    apply(h, next.a, Temp::Clear, commit);
                })
                enabled=enabled
            />
        }
    };

    let temperature_row = move || {
        let k_track = Signal::stored(gradient(
            &(0..=6)
                .map(|i| kelvin_to_rgb(min_k + span_k * f64::from(i) / 6.0))
                .collect::<Vec<_>>(),
        ));
        view! {
            <ChannelRow
                label=temperature_label
                fraction=Signal::derive(move || (kelvin.get() - min_k) / span_k)
                track=k_track
                number=Signal::derive(move || format!("{:.0}", kelvin.get()))
                unit=" K"
                parse=Callback::new(move |t: String| {
                    let t = t.trim().trim_end_matches(['K', 'k']).trim().replace(',', ".");
                    let k: f64 = t.parse().ok()?;
                    (k.is_finite() && (min_k..=max_k).contains(&k)).then(|| (k - min_k) / span_k)
                })
                step=100.0 / span_k
                on_change=Callback::new(move |(f, commit): (f64, bool)| {
                    let k = min_k + span_k * f;
                    let c = kelvin_to_rgb(k);
                    apply(hsv_keeping(c, hsv.get_untracked()), a.get_untracked(), Temp::Set(k), commit);
                })
                enabled=enabled
            />
        }
    };

    let alpha_row = move || {
        view! {
            <ChannelRow
                label=alpha_label
                fraction=Signal::derive(move || a.get())
                track=Signal::derive(move || {
                    let c = rgba.get();
                    gradient(&[Rgba { a: 0.0, ..c }, opaque(c)])
                })
                checker=true
                number=Signal::derive(move || format!("{:.0}", a.get() * 100.0))
                unit="%"
                parse=parse_in(100.0)
                step=0.01
                on_change=Callback::new(move |(f, commit): (f64, bool)| apply(hsv.get_untracked(), f, Temp::Keep, commit))
                enabled=enabled
            />
        }
    };

    let mode_label = move |m: ColorMode| match m {
        ColorMode::Spectrum => spectrum_label.get(),
        ColorMode::Rgb => rgb_label.get(),
        ColorMode::Hsl => hsl_label.get(),
        ColorMode::Temperature => temperature_label.get(),
    };
    let tabs = (modes.len() > 1).then(|| {
        view! {
            <div class="op-color-tabs" role="tablist">
                {modes
                    .iter()
                    .map(|m| {
                        let m = *m;
                        view! {
                            <button
                                type="button"
                                role="tab"
                                class="op-rippled op-color-tab op-t-caption"
                                aria-selected=move || (mode.get() == m).to_string()
                                tabindex=move || if mode.get() == m { "0" } else { "-1" }
                                on:click=move |_| mode.set(m)
                            >
                                {move || mode_label(m)}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        }
    });
    let editor = move || match mode.get() {
        ColorMode::Spectrum => view! { {plane()} {hue_row()} }.into_any(),
        ColorMode::Rgb => view! {
            {rgb_row(red_label, 0)}
            {rgb_row(green_label, 1)}
            {rgb_row(blue_label, 2)}
        }
        .into_any(),
        ColorMode::Hsl => view! {
            {hue_row()}
            {hsl_row(saturation_label, false)}
            {hsl_row(lightness_label, true)}
        }
        .into_any(),
        ColorMode::Temperature => temperature_row().into_any(),
    };
    let readout = move || match mode.get() {
        ColorMode::Hsl => rgba.get().css_hsl(alpha),
        ColorMode::Temperature => format!("{:.0} K", kelvin.get()),
        ColorMode::Spectrum | ColorMode::Rgb => rgba.get().css_rgb(alpha),
    };

    let hex_input = NodeRef::<leptos::html::Input>::new();
    let swatches: Vec<(String, Rgba)> = swatches
        .iter()
        .filter_map(|s| parse_rgba(s))
        .map(|c| if alpha { c } else { opaque(c) })
        .map(|c| (c.hex(alpha), c))
        .collect();
    let has_swatches = !swatches.is_empty();
    view! {
        <div class=root_class("op-color-picker", class) style=custom_style>
            {header.run()}
            {tabs}
            <div class="op-color-editor">{editor}</div>
            {alpha.then(alpha_row)}
            <div class="op-color-code">
                <span class="op-color-sample op-color-checker">
                    <span style:background-color=move || rgba.get().css_rgb(true)></span>
                </span>
                <input
                    node_ref=hex_input
                    type="text"
                    class="op-color-hex"
                    spellcheck="false"
                    autocomplete="off"
                    aria-label=move || hex_label.get()
                    disabled=move || !enabled.get()
                    prop:value=move || hex.get()
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" && let Some(el) = hex_input.get_untracked() {
                            let _ = el.blur();
                        }
                    }
                    on:change=move |ev| match parse_rgba(&event_target_value(&ev)) {
                        Some(c) => set_rgba(if alpha { c } else { opaque(c) }, true),
                        None => {
                            if let Some(el) = hex_input.get_untracked() {
                                el.set_value(&hex.get_untracked());
                            }
                        }
                    }
                />
                <span class="op-color-readout op-t-caption">{readout}</span>
                {has_eyedropper().then(|| view! {
                    <button
                        type="button"
                        class="op-rippled op-color-eyedropper"
                        aria-label=move || eyedropper_label.get()
                        title=move || eyedropper_label.get()
                        disabled=move || !enabled.get()
                        on:click=move |_| spawn_local(async move {
                            if let Some(c) = eyedrop().await.and_then(|t| parse_rgba(&t)) {
                                let c = Rgba { a: a.get_untracked(), ..c };
                                set_rgba(c, true);
                            }
                        })
                    >
                        {move || view! { <Icon icon=eyedropper_icon.get() size=dim::ICON_SMALL attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-small));height:var(--op-field-icon-size, var(--op-dim-icon-small));"/> }}
                    </button>
                })}
            </div>
            {has_swatches.then(|| view! {
                <div class="op-color-swatches" role="listbox" aria-label=move || swatches_label.get()>
                    {swatches
                        .into_iter()
                        .map(|(code, c)| {
                            let current = code.clone();
                            view! {
                                <button
                                    type="button"
                                    role="option"
                                    class="op-color-swatch op-color-checker"
                                    aria-label=code.clone()
                                    aria-selected=move || (hex.get() == current).to_string()
                                    disabled=move || !enabled.get()
                                    on:click=move |_| set_rgba(c, true)
                                >
                                    <span style:background-color=c.css_rgb(true)></span>
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            })}
            {footer.run()}
        </div>
    }
}
