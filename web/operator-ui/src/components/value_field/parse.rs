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

/// Clockwise angle in degrees from twelve o'clock for a pointer offset from the dial centre
/// (`dy` grows downwards).
pub fn dial_angle(dx: f64, dy: f64) -> f64 {
    dx.atan2(-dy).to_degrees().rem_euclid(360.0)
}

/// The hour under `angle`: in 24-hour mode the inner ring holds `0` and `13..=23`, the outer
/// ring `12` and `1..=11`; in 12-hour mode `pm` adds twelve.
pub fn dial_hour(angle: f64, inner: bool, hour12: bool, pm: bool) -> u32 {
    // The angle is reduced to `0..360`, so the index fits.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let index = ((angle.rem_euclid(360.0) / 30.0).round() as u32) % 12;
    if hour12 {
        index + if pm { 12 } else { 0 }
    } else if inner {
        if index == 0 { 0 } else { index + 12 }
    } else if index == 0 {
        12
    } else {
        index
    }
}

/// The minute under `angle`, rounded to a multiple of `step`.
pub fn dial_minute(angle: f64, step: u32) -> u32 {
    let step = f64::from(step.clamp(1, 30));
    // The angle is reduced to `0..360`, so the minute fits.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let m = ((angle.rem_euclid(360.0) / 6.0 / step).round() * step) as u32;
    m % 60
}

/// A calendar day as `(year, month, day)`.
pub type LocalDate = (i32, u32, u32);

/// The date `days` after 1970-01-01 (inverse of [`days_from_civil`]).
pub fn civil_from_days(days: i64) -> LocalDate {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    (
        i32::try_from(y).unwrap_or(0),
        u32::try_from(m).unwrap_or(1),
        u32::try_from(d).unwrap_or(1),
    )
}

/// `date` moved by `days`.
pub fn add_days(date: LocalDate, days: i64) -> LocalDate {
    civil_from_days(days_from_civil(date.0, date.1, date.2) + days)
}

/// An inclusive range of days; `start` is never after `end`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateRange {
    pub start: LocalDate,
    pub end: LocalDate,
}

impl DateRange {
    /// The range between two days in either order.
    pub fn new(a: LocalDate, b: LocalDate) -> Self {
        Self {
            start: a.min(b),
            end: a.max(b),
        }
    }

    /// Number of days, both ends included.
    pub fn days(self) -> i64 {
        days_from_civil(self.end.0, self.end.1, self.end.2)
            - days_from_civil(self.start.0, self.start.1, self.start.2)
            + 1
    }

    pub fn contains(self, d: LocalDate) -> bool {
        self.start <= d && d <= self.end
    }
}

/// Ready-made ranges relative to today; weeks start on Monday.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateRangePreset {
    Today,
    Yesterday,
    ThisWeek,
    LastWeek,
    ThisMonth,
    LastMonth,
    /// The last `n` days ending today.
    LastDays(u32),
    ThisYear,
    LastYear,
    /// A fixed range with its own label.
    Custom(String, DateRange),
}

impl DateRangePreset {
    /// The presets DateRangeField offers by default.
    pub fn defaults() -> Vec<Self> {
        vec![
            Self::Today,
            Self::Yesterday,
            Self::ThisWeek,
            Self::LastWeek,
            Self::ThisMonth,
            Self::LastMonth,
            Self::LastDays(7),
            Self::LastDays(30),
            Self::ThisYear,
        ]
    }

    /// The range for `today`.
    pub fn resolve(&self, today: LocalDate) -> DateRange {
        let (y, m, _) = today;
        let monday = add_days(today, -i64::from(weekday(today.0, today.1, today.2)));
        let month_start = (y, m, 1);
        let previous_month = if m == 1 { (y - 1, 12) } else { (y, m - 1) };
        match self {
            Self::Today => DateRange::new(today, today),
            Self::Yesterday => {
                let d = add_days(today, -1);
                DateRange::new(d, d)
            }
            Self::ThisWeek => DateRange::new(monday, add_days(monday, 6)),
            Self::LastWeek => DateRange::new(add_days(monday, -7), add_days(monday, -1)),
            Self::ThisMonth => DateRange::new(month_start, (y, m, days_in_month(y, m))),
            Self::LastMonth => DateRange::new(
                (previous_month.0, previous_month.1, 1),
                (
                    previous_month.0,
                    previous_month.1,
                    days_in_month(previous_month.0, previous_month.1),
                ),
            ),
            Self::LastDays(n) => DateRange::new(add_days(today, 1 - i64::from((*n).max(1))), today),
            Self::ThisYear => DateRange::new((y, 1, 1), (y, 12, 31)),
            Self::LastYear => DateRange::new((y - 1, 1, 1), (y - 1, 12, 31)),
            Self::Custom(_, r) => *r,
        }
    }
}

/// `12 Jul – 5 Nov 2026`, `3 – 10 Aug 2026`, `12 Jul 2025 – 5 Nov 2026`, or `5 Nov 2026` for one
/// day, with
/// `short_months` January first.
pub fn format_date_range(r: DateRange, short_months: &[String]) -> String {
    let name = |m: u32| {
        short_months
            .get(usize::try_from(m).unwrap_or(1).saturating_sub(1))
            .cloned()
            .unwrap_or_else(|| format!("{m:02}"))
    };
    let (s, e) = (r.start, r.end);
    if s == e {
        return format!("{} {} {}", e.2, name(e.1), e.0);
    }
    if s.0 == e.0 && s.1 == e.1 {
        format!("{} – {} {} {}", s.2, e.2, name(e.1), e.0)
    } else if s.0 == e.0 {
        format!("{} {} – {} {} {}", s.2, name(s.1), e.2, name(e.1), e.0)
    } else {
        format!(
            "{} {} {} – {} {} {}",
            s.2,
            name(s.1),
            s.0,
            e.2,
            name(e.1),
            e.0
        )
    }
}

/// One day as `YYYY-MM-DD`, `D.M[.YYYY]` or `D Mon [YYYY]`; the year defaults to `year`.
fn parse_day(text: &str, months: &[Vec<String>], year: i32) -> Option<LocalDate> {
    let t = text.trim();
    let date = |y: i32, m: u32, d: u32| (LocalDateTime::new(y, m, d, 0)).map(|_| (y, m, d));
    let words: Vec<&str> = t.split_whitespace().collect();
    if words.len() >= 2 {
        let day: u32 = words[0].trim_end_matches('.').parse().ok()?;
        let word = words[1].trim_end_matches(['.', ',']).to_lowercase();
        let month = month_from_word(&word, months)?;
        let y = match words.get(2) {
            Some(y) => y.parse().ok()?,
            None => year,
        };
        return (words.len() <= 3).then(|| date(y, month, day)).flatten();
    }
    let parts: Vec<&str> = t.split(['-', '.', '/']).collect();
    match parts.as_slice() {
        [y, m, d] if y.len() == 4 => date(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?),
        [d, m, y] => date(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?),
        [d, m] => date(year, m.parse().ok()?, d.parse().ok()?),
        _ => None,
    }
}

/// The month whose name in any list shares its first three letters with `word`, so
/// inflected forms such as `липня` match `Липень`.
fn month_from_word(word: &str, months: &[Vec<String>]) -> Option<u32> {
    let stem = |w: &str| w.to_lowercase().chars().take(3).collect::<String>();
    let key = stem(word);
    if key.chars().count() < 3 {
        return None;
    }
    months.iter().find_map(|list| {
        list.iter()
            .position(|name| stem(name) == key)
            .and_then(|i| u32::try_from(i + 1).ok())
    })
}

/// A range from text: two days joined by `–`, `—`, ` - `, `..` or ` to `, or one day. Missing
/// years default to `year` for the end and to the end's year for the start (a year earlier
/// when that would put the start after the end). `months` holds month-name lists, January
/// first, matched by prefix.
pub fn parse_date_range(text: &str, months: &[Vec<String>], year: i32) -> Option<DateRange> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    let separators = [" – ", "–", " — ", "—", " - ", "..", " to "];
    let (a, b) = separators
        .iter()
        .find_map(|s| t.split_once(s))
        .unwrap_or((t, t));
    let end = parse_day(b, months, year)?;
    // A bare day number before the separator takes the month and year of the end.
    if let Ok(day) = a.trim().parse::<u32>() {
        return LocalDateTime::new(end.0, end.1, day, 0)
            .map(|_| DateRange::new((end.0, end.1, day), end));
    }
    let start = parse_day(a, months, end.0)?;
    let explicit_start_year = a.split(|c: char| !c.is_ascii_digit()).any(|p| p.len() == 4);
    let start = if start > end && !explicit_start_year {
        parse_day(a, months, end.0 - 1)?
    } else {
        start
    };
    Some(DateRange::new(start, end))
}

/// Unit sizes in milliseconds: day, hour, minute, second, millisecond.
pub const DURATION_UNITS_MS: [i64; 5] = [86_400_000, 3_600_000, 60_000, 1_000, 1];

/// Splits the magnitude of `ms` into the units marked in `shown` (day, hour, minute, second,
/// millisecond); the largest shown unit takes everything above it and anything below the
/// smallest shown unit is dropped.
pub fn split_duration(ms: i64, shown: [bool; 5]) -> [i64; 5] {
    let mut rest = ms.unsigned_abs();
    let mut out = [0_i64; 5];
    for (i, size) in DURATION_UNITS_MS.iter().enumerate() {
        if shown[i] {
            let size = size.unsigned_abs();
            out[i] = i64::try_from(rest / size).unwrap_or(i64::MAX);
            rest %= size;
        }
    }
    out
}

/// The total of `parts` in milliseconds with the given sign.
pub fn join_duration(parts: [i64; 5], negative: bool) -> i64 {
    let total = parts
        .iter()
        .zip(DURATION_UNITS_MS)
        .fold(0_i64, |acc, (p, size)| {
            acc.saturating_add(p.saturating_mul(size))
        });
    if negative { -total } else { total }
}
