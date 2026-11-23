//! Colour models used by ColorField and ColorPicker: RGB, HSV, HSL, colour temperature and alpha.
//!
//! Pure functions without a colour library; channels are `f64` in `0..=1` except hue (degrees).

/// An sRGB colour with alpha; every channel is in `0..=1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

/// Hue in degrees `0..360`, saturation and value in `0..=1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

/// Hue in degrees `0..360`, saturation and lightness in `0..=1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

fn unit(v: f64) -> f64 {
    if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) }
}

fn hue(h: f64) -> f64 {
    if h.is_nan() { 0.0 } else { h.rem_euclid(360.0) }
}

/// A channel in `0..=1` as a byte.
pub fn to_byte(v: f64) -> u8 {
    // The clamp keeps the value inside `0..=255`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let b = (unit(v) * 255.0).round() as u8;
    b
}

impl Rgba {
    /// An opaque colour from bytes.
    pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: f64::from(r) / 255.0,
            g: f64::from(g) / 255.0,
            b: f64::from(b) / 255.0,
            a: f64::from(a) / 255.0,
        }
    }

    /// The channels as bytes `[r, g, b, a]`.
    pub fn bytes(self) -> [u8; 4] {
        [
            to_byte(self.r),
            to_byte(self.g),
            to_byte(self.b),
            to_byte(self.a),
        ]
    }

    /// The same colour with every channel clamped into `0..=1`.
    pub fn clamped(self) -> Self {
        Self {
            r: unit(self.r),
            g: unit(self.g),
            b: unit(self.b),
            a: unit(self.a),
        }
    }

    /// `#rrggbb`, or `#rrggbbaa` when `alpha` is set and the colour is not opaque.
    pub fn hex(self, alpha: bool) -> String {
        let [r, g, b, a] = self.bytes();
        if alpha && a < 255 {
            format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
        } else {
            format!("#{r:02x}{g:02x}{b:02x}")
        }
    }

    /// `rgb(r, g, b)`, or `rgba(r, g, b, a)` when `alpha` is set and the colour is not opaque.
    pub fn css_rgb(self, alpha: bool) -> String {
        let [r, g, b, _] = self.bytes();
        if alpha && to_byte(self.a) < 255 {
            format!("rgba({r}, {g}, {b}, {})", round_to(self.a, 2))
        } else {
            format!("rgb({r}, {g}, {b})")
        }
    }

    /// `hsl(h, s%, l%)`, with alpha as in [`Rgba::css_rgb`].
    pub fn css_hsl(self, alpha: bool) -> String {
        let c = rgb_to_hsl(self);
        let (h, s, l) = (c.h.round(), (c.s * 100.0).round(), (c.l * 100.0).round());
        if alpha && to_byte(self.a) < 255 {
            format!("hsla({h}, {s}%, {l}%, {})", round_to(self.a, 2))
        } else {
            format!("hsl({h}, {s}%, {l}%)")
        }
    }
}

fn round_to(v: f64, digits: i32) -> f64 {
    let k = 10_f64.powi(digits);
    (v * k).round() / k
}

pub fn rgb_to_hsv(c: Rgba) -> Hsv {
    let (r, g, b) = (unit(c.r), unit(c.g), unit(c.b));
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    Hsv {
        h: hue_of(r, g, b, max, d),
        s: if max <= 0.0 { 0.0 } else { d / max },
        v: max,
    }
}

fn hue_of(r: f64, g: f64, b: f64, max: f64, d: f64) -> f64 {
    if d <= 0.0 {
        return 0.0;
    }
    let h = if (max - r).abs() < f64::EPSILON {
        ((g - b) / d).rem_euclid(6.0)
    } else if (max - g).abs() < f64::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    hue(h * 60.0)
}

pub fn hsv_to_rgb(c: Hsv, a: f64) -> Rgba {
    let (h, s, v) = (hue(c.h), unit(c.s), unit(c.v));
    let k = |n: f64| {
        let k = (n + h / 60.0).rem_euclid(6.0);
        v - v * s * k.min(4.0 - k).clamp(0.0, 1.0)
    };
    Rgba {
        r: k(5.0),
        g: k(3.0),
        b: k(1.0),
        a: unit(a),
    }
}

pub fn rgb_to_hsl(c: Rgba) -> Hsl {
    let (r, g, b) = (unit(c.r), unit(c.g), unit(c.b));
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let l = (max + min) / 2.0;
    let s = if d <= 0.0 {
        0.0
    } else {
        d / (1.0 - (2.0 * l - 1.0).abs())
    };
    Hsl {
        h: hue_of(r, g, b, max, d),
        s: unit(s),
        l,
    }
}

pub fn hsl_to_rgb(c: Hsl, a: f64) -> Rgba {
    let (h, s, l) = (hue(c.h), unit(c.s), unit(c.l));
    let k = |n: f64| {
        let k = (n + h / 30.0).rem_euclid(12.0);
        l - s * l.min(1.0 - l) * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0)
    };
    Rgba {
        r: k(0.0),
        g: k(8.0),
        b: k(4.0),
        a: unit(a),
    }
}

/// HSV with the hue and saturation of `fallback` kept where the colour has none (greys, black).
pub fn hsv_keeping(c: Rgba, fallback: Hsv) -> Hsv {
    let mut hsv = rgb_to_hsv(c);
    if hsv.v <= 0.0 {
        hsv.s = fallback.s;
    }
    if hsv.s <= 0.0 || hsv.v <= 0.0 {
        hsv.h = fallback.h;
    }
    hsv
}

/// The colour of a black body at `kelvin` (1000 K to 40 000 K), after Tanner Helland's fit.
pub fn kelvin_to_rgb(kelvin: f64) -> Rgba {
    let t = kelvin.clamp(1000.0, 40_000.0) / 100.0;
    let r = if t <= 66.0 {
        255.0
    } else {
        329.698_727_446 * (t - 60.0).powf(-0.133_204_759_2)
    };
    let g = if t <= 66.0 {
        99.470_802_586_1 * t.ln() - 161.119_568_166_1
    } else {
        288.122_169_528_3 * (t - 60.0).powf(-0.075_514_849_2)
    };
    let b = if t >= 66.0 {
        255.0
    } else if t <= 19.0 {
        0.0
    } else {
        138.517_731_223_1 * (t - 10.0).ln() - 305.044_792_730_7
    };
    Rgba {
        r: unit(r / 255.0),
        g: unit(g / 255.0),
        b: unit(b / 255.0),
        a: 1.0,
    }
}

/// The temperature in `min..=max` kelvin whose colour is closest to `c`, by its blue-to-red ratio.
pub fn rgb_to_kelvin(c: Rgba, min: f64, max: f64) -> f64 {
    let ratio = |c: Rgba| c.b / c.r.max(1e-6);
    let target = ratio(c);
    let (mut lo, mut hi) = (min.min(max), max.max(min));
    for _ in 0..40 {
        let mid = (lo + hi) / 2.0;
        if ratio(kelvin_to_rgb(mid)) < target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) / 2.0
}

/// A colour from text: `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa` (the `#` is optional),
/// `rgb()`/`rgba()`, `hsl()`/`hsla()`, `hsv()` or a temperature such as `2700K`.
pub fn parse_rgba(text: &str) -> Option<Rgba> {
    let t = text.trim().to_ascii_lowercase();
    if let Some(k) = t.strip_suffix('k') {
        let k: f64 = k.trim().parse().ok()?;
        return (1000.0..=40_000.0).contains(&k).then(|| kelvin_to_rgb(k));
    }
    if let Some((name, args)) = function(&t) {
        return parse_function(name, &args);
    }
    parse_hex(&t)
}

fn parse_hex(t: &str) -> Option<Rgba> {
    let hex = t.strip_prefix('#').unwrap_or(t);
    if hex.is_empty() || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let digits: Vec<u8> = match hex.len() {
        3 | 4 => hex
            .chars()
            .map(|c| u8::from_str_radix(&format!("{c}{c}"), 16).ok())
            .collect::<Option<_>>()?,
        6 | 8 => (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
            .collect::<Option<_>>()?,
        _ => return None,
    };
    let a = digits.get(3).copied().unwrap_or(255);
    Some(Rgba::from_bytes(digits[0], digits[1], digits[2], a))
}

/// `name(a, b, c[, d])` with commas, spaces or a `/` before alpha.
fn function(t: &str) -> Option<(&str, Vec<&str>)> {
    let open = t.find('(')?;
    let body = t[open + 1..].strip_suffix(')')?;
    let args = body
        .split([',', ' ', '/'])
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .collect();
    Some((t[..open].trim(), args))
}

/// A number, or a percentage of `full`.
fn amount(arg: &str, full: f64) -> Option<f64> {
    match arg.strip_suffix('%') {
        Some(p) => Some(p.trim().parse::<f64>().ok()? / 100.0 * full),
        None => arg.trim_end_matches("deg").parse().ok(),
    }
}

fn parse_function(name: &str, args: &[&str]) -> Option<Rgba> {
    if !(3..=4).contains(&args.len()) {
        return None;
    }
    let alpha = match args.get(3) {
        Some(a) => amount(a, 1.0)?,
        None => 1.0,
    };
    if !(0.0..=1.0).contains(&alpha) {
        return None;
    }
    // Percentages map to the full range; bare saturation and lightness are percentages too.
    let percent = |arg: &str| -> Option<f64> {
        let v = amount(arg, 100.0)?;
        (0.0..=100.0).contains(&v).then_some(v / 100.0)
    };
    match name {
        "rgb" | "rgba" => {
            let ch = |arg: &str| -> Option<f64> {
                let v = amount(arg, 255.0)?;
                (0.0..=255.0).contains(&v).then_some(v / 255.0)
            };
            Some(Rgba {
                r: ch(args[0])?,
                g: ch(args[1])?,
                b: ch(args[2])?,
                a: alpha,
            })
        }
        "hsl" | "hsla" => Some(hsl_to_rgb(
            Hsl {
                h: amount(args[0], 360.0)?,
                s: percent(args[1])?,
                l: percent(args[2])?,
            },
            alpha,
        )),
        "hsv" | "hsva" | "hsb" => Some(hsv_to_rgb(
            Hsv {
                h: amount(args[0], 360.0)?,
                s: percent(args[1])?,
                v: percent(args[2])?,
            },
            alpha,
        )),
        _ => None,
    }
}
