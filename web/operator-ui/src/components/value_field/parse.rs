//! Text forms of typed field values: parsing and formatting without a date library.

/// A number from text; accepts a comma as the decimal separator and spaces between digit groups.
pub fn parse_number(text: &str) -> Option<f64> {
    let cleaned: String = text
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '_')
        .map(|c| if c == ',' { '.' } else { c })
        .collect();
    if cleaned.is_empty() {
        return None;
    }
    cleaned.parse::<f64>().ok().filter(|v| v.is_finite())
}

/// A number with at most `decimals` fraction digits and no trailing zeros.
pub fn format_number(value: f64, decimals: u8) -> String {
    let s = format!("{value:.*}", usize::from(decimals));
    let s = if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    };
    if s == "-0" { "0".to_string() } else { s }
}

/// `value` moved by `delta` steps, snapped to the step grid and clamped to the range.
pub fn step_number(value: f64, delta: i32, step: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let step = if step > 0.0 { step } else { 1.0 };
    let snapped = (value / step).round() * step;
    let next = if (snapped - value).abs() > step * 1e-9 && delta != 0 {
        // Off-grid values move to the nearest grid line in the step direction first.
        if delta > 0 {
            (value / step).ceil() * step
        } else {
            (value / step).floor() * step
        }
    } else {
        snapped + f64::from(delta) * step
    };
    clamp(next, min, max)
}

/// `value` limited to the optional bounds.
pub fn clamp(value: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let v = min.map_or(value, |m| value.max(m));
    max.map_or(v, |m| v.min(m))
}

const MS_SECOND: i64 = 1_000;
const MS_MINUTE: i64 = 60 * MS_SECOND;
const MS_HOUR: i64 = 60 * MS_MINUTE;
const MS_DAY: i64 = 24 * MS_HOUR;

/// A signed duration in milliseconds as `-1d 2h 3m 4s 500ms`; zero is `0s`.
pub fn format_duration(ms: i64) -> String {
    if ms == 0 {
        return "0s".to_string();
    }
    let sign = if ms < 0 { "-" } else { "" };
    let mut rest = ms.unsigned_abs();
    let mut parts = Vec::new();
    for (unit, size) in [
        ("d", MS_DAY),
        ("h", MS_HOUR),
        ("m", MS_MINUTE),
        ("s", MS_SECOND),
        ("ms", 1),
    ] {
        let size = size.unsigned_abs();
        let n = rest / size;
        rest %= size;
        if n > 0 {
            parts.push(format!("{n}{unit}"));
        }
    }
    format!("{sign}{}", parts.join(" "))
}

/// A signed duration in milliseconds from text.
///
/// Accepts unit parts (`1h 30m`, `90s`, `1.5h`, `250ms`, `2d`), clock forms (`1:30` = minutes and
/// seconds, `1:02:03` = hours, minutes and seconds) and a bare number in `default_unit_ms`.
/// A leading `-` makes the duration negative.
pub fn parse_duration(text: &str, default_unit_ms: i64) -> Option<i64> {
    let t = text.trim();
    let (negative, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest.trim_start()),
        None => (false, t.strip_prefix('+').unwrap_or(t).trim_start()),
    };
    if body.is_empty() {
        return None;
    }
    let ms = if body.contains(':') {
        parse_clock(body)?
    } else if let Some(n) = parse_number(body) {
        round_ms(n * to_f64(default_unit_ms))?
    } else {
        parse_unit_parts(body)?
    };
    Some(if negative { -ms } else { ms })
}

fn validate_clock_parts(h: i64, m: i64, s: f64, max_m: i64) -> Option<(i64, i64, f64)> {
    if h >= 0 && (0..=max_m).contains(&m) && (0.0..60.0).contains(&s) {
        Some((h, m, s))
    } else {
        None
    }
}

fn parse_clock_components(parts: &[&str]) -> Option<(i64, i64, f64)> {
    match parts {
        [m, s] => validate_clock_parts(0, (*m).parse().ok()?, parse_number(s)?, i64::MAX),
        [h, m, s] => {
            validate_clock_parts((*h).parse().ok()?, (*m).parse().ok()?, parse_number(s)?, 59)
        }
        _ => None,
    }
}

fn parse_clock(body: &str) -> Option<i64> {
    let parts: Vec<&str> = body.split(':').map(str::trim).collect();
    let (h, m, s) = parse_clock_components(&parts)?;
    let whole = h
        .checked_mul(MS_HOUR)?
        .checked_add(m.checked_mul(MS_MINUTE)?)?;
    whole.checked_add(round_ms(s * 1_000.0)?)
}

fn parse_unit_parts(body: &str) -> Option<i64> {
    let mut total: i64 = 0;
    let mut rest = body.trim();
    let mut any = false;
    while !rest.is_empty() {
        let num_end = rest
            .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == ','))
            .unwrap_or(rest.len());
        if num_end == 0 {
            return None;
        }
        let n = parse_number(&rest[..num_end])?;
        rest = rest[num_end..].trim_start();
        let unit_end = rest
            .find(|c: char| !c.is_alphabetic())
            .unwrap_or(rest.len());
        let size = unit_size(&rest[..unit_end].to_lowercase())?;
        rest = rest[unit_end..].trim_start();
        total = total.checked_add(round_ms(n * to_f64(size))?)?;
        any = true;
    }
    any.then_some(total)
}

fn unit_size(unit: &str) -> Option<i64> {
    Some(match unit {
        "ms" => 1,
        "s" | "sec" | "secs" | "second" | "seconds" => MS_SECOND,
        "m" | "min" | "mins" | "minute" | "minutes" => MS_MINUTE,
        "h" | "hr" | "hrs" | "hour" | "hours" => MS_HOUR,
        "d" | "day" | "days" => MS_DAY,
        _ => return None,
    })
}

#[allow(clippy::cast_precision_loss)]
fn to_f64(v: i64) -> f64 {
    v as f64
}

#[allow(clippy::cast_possible_truncation)]
fn round_ms(v: f64) -> Option<i64> {
    (v.is_finite() && v.abs() < 9.0e15).then(|| v.round() as i64)
}

/// Seconds since midnight as `HH:MM`, or `HH:MM:SS` when `seconds` is set or the seconds are not zero.
pub fn format_time(second_of_day: u32, seconds: bool) -> String {
    let s = second_of_day % 86_400;
    let (h, m, sec) = (s / 3600, s / 60 % 60, s % 60);
    if seconds || sec != 0 {
        format!("{h:02}:{m:02}:{sec:02}")
    } else {
        format!("{h:02}:{m:02}")
    }
}

/// Seconds since midnight from `H:MM`, `H:MM:SS`, `HHMM` or `H`.
pub fn parse_time(text: &str) -> Option<u32> {
    let t = text.trim();
    let parts: Vec<&str> = if t.contains(':') {
        t.split(':').map(str::trim).collect()
    } else if t.len() == 4 && t.chars().all(|c| c.is_ascii_digit()) {
        vec![&t[..2], &t[2..]]
    } else {
        vec![t]
    };
    let nums: Vec<u32> = parts
        .iter()
        .map(|p| p.parse::<u32>().ok())
        .collect::<Option<_>>()?;
    let (h, m, s) = match nums.as_slice() {
        [h] => (*h, 0, 0),
        [h, m] => (*h, *m, 0),
        [h, m, s] => (*h, *m, *s),
        _ => return None,
    };
    (h < 24 && m < 60 && s < 60).then_some(h * 3600 + m * 60 + s)
}

/// A local calendar date and time of day, without a time zone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalDateTime {
    pub year: i32,
    /// 1 to 12.
    pub month: u32,
    /// 1 to the month length.
    pub day: u32,
    pub second_of_day: u32,
}

impl LocalDateTime {
    /// A valid date and time, or `None`.
    pub fn new(year: i32, month: u32, day: u32, second_of_day: u32) -> Option<Self> {
        ((1..=12).contains(&month)
            && day >= 1
            && day <= days_in_month(year, month)
            && second_of_day < 86_400)
            .then_some(Self {
                year,
                month,
                day,
                second_of_day,
            })
    }
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Days since 1970-01-01 (proleptic Gregorian calendar).
pub fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = i64::from(year) - i64::from(month <= 2);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let m = i64::from(month);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Day of the week, Monday = 0 to Sunday = 6.
pub fn weekday(year: i32, month: u32, day: u32) -> u32 {
    // 1970-01-01 was a Thursday (3).
    u32::try_from((days_from_civil(year, month, day) + 3).rem_euclid(7)).unwrap_or(0)
}

/// `YYYY-MM-DD HH:MM`, with `:SS` when `seconds` is set or the seconds are not zero.
pub fn format_date_time(v: LocalDateTime, seconds: bool) -> String {
    format!(
        "{:04}-{:02}-{:02} {}",
        v.year,
        v.month,
        v.day,
        format_time(v.second_of_day, seconds)
    )
}

/// A date and time from `YYYY-MM-DD HH:MM[:SS]` (also with `T`), or a date alone at midnight.
pub fn parse_date_time(text: &str) -> Option<LocalDateTime> {
    let t = text.trim();
    let (date, time) = match t.find(['T', 't', ' ']) {
        Some(i) => (&t[..i], t[i + 1..].trim()),
        None => (t, ""),
    };
    let mut d = date.split(['-', '.', '/']).map(str::trim);
    let (y, m, day) = (d.next()?, d.next()?, d.next()?);
    if d.next().is_some() || y.len() != 4 {
        return None;
    }
    let second = if time.is_empty() {
        0
    } else {
        parse_time(time)?
    };
    LocalDateTime::new(y.parse().ok()?, m.parse().ok()?, day.parse().ok()?, second)
}

/// A colour as lowercase `#rrggbb` from any form [`parse_rgba`](super::color_math::parse_rgba)
/// accepts; alpha is dropped.
pub fn parse_color(text: &str) -> Option<String> {
    super::color_math::parse_rgba(text).map(|c| c.hex(false))
}
