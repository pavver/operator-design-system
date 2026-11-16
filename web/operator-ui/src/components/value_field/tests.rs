use super::parse::*;

#[test]
fn numbers_accept_commas_and_groups() {
    assert_eq!(parse_number("1,5"), Some(1.5));
    assert_eq!(parse_number(" 12 000 "), Some(12_000.0));
    assert_eq!(parse_number("-3e2"), Some(-300.0));
    assert_eq!(parse_number("abc"), None);
    assert_eq!(parse_number(""), None);
    assert_eq!(parse_number("inf"), None);
}

#[test]
fn numbers_format_without_trailing_zeros() {
    assert_eq!(format_number(1.5, 2), "1.5");
    assert_eq!(format_number(2.0, 2), "2");
    assert_eq!(format_number(1.005, 0), "1");
    assert_eq!(format_number(-0.0001, 2), "0");
}

#[test]
fn steps_snap_to_the_grid_and_clamp() {
    assert_eq!(step_number(1.0, 1, 0.5, None, None), 1.5);
    assert_eq!(step_number(1.2, 1, 0.5, None, None), 1.5);
    assert_eq!(step_number(1.2, -1, 0.5, None, None), 1.0);
    assert_eq!(step_number(9.5, 1, 1.0, None, Some(10.0)), 10.0);
    assert_eq!(step_number(0.0, -1, 1.0, Some(0.0), None), 0.0);
}

#[test]
fn durations_round_trip() {
    for ms in [
        0,
        1,
        999,
        1_000,
        90_000,
        3_600_000,
        5_400_500,
        -86_400_000 - 1_000,
    ] {
        assert_eq!(
            parse_duration(&format_duration(ms), 1_000),
            Some(ms),
            "{ms}"
        );
    }
    assert_eq!(format_duration(5_400_500), "1h 30m 500ms");
    assert_eq!(format_duration(-61_000), "-1m 1s");
}

#[test]
fn durations_accept_many_forms() {
    assert_eq!(parse_duration("1.5h", 1_000), Some(5_400_000));
    assert_eq!(parse_duration("1h30m", 1_000), Some(5_400_000));
    assert_eq!(parse_duration("90", 1_000), Some(90_000));
    assert_eq!(parse_duration("90", 60_000), Some(5_400_000));
    assert_eq!(parse_duration("1:30", 1_000), Some(90_000));
    assert_eq!(parse_duration("1:02:03", 1_000), Some(3_723_000));
    assert_eq!(parse_duration("- 2 min", 1_000), Some(-120_000));
    assert_eq!(parse_duration("2 weeks", 1_000), None);
    assert_eq!(parse_duration("1:75", 1_000), None);
    assert_eq!(parse_duration("-", 1_000), None);
}

#[test]
fn times_parse_and_format() {
    assert_eq!(parse_time("7:05"), Some(7 * 3600 + 5 * 60));
    assert_eq!(parse_time("0730"), Some(7 * 3600 + 30 * 60));
    assert_eq!(parse_time("23:59:59"), Some(86_399));
    assert_eq!(parse_time("24:00"), None);
    assert_eq!(parse_time("9"), Some(9 * 3600));
    assert_eq!(format_time(7 * 3600 + 5 * 60, false), "07:05");
    assert_eq!(format_time(61, false), "00:01:01");
    assert_eq!(format_time(0, true), "00:00:00");
}

#[test]
fn calendar_math() {
    assert_eq!(days_from_civil(1970, 1, 1), 0);
    assert_eq!(days_from_civil(2000, 3, 1), 11_017);
    assert_eq!(weekday(2026, 11, 17), 1); // Tuesday
    assert_eq!(weekday(2024, 2, 29), 3); // Thursday
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(days_in_month(2100, 2), 28);
    assert_eq!(days_in_month(2000, 2), 29);
}

#[test]
fn date_times_parse_and_format() {
    let v = parse_date_time("2026-11-17 07:30").expect("valid");
    assert_eq!(
        v,
        LocalDateTime::new(2026, 11, 17, 7 * 3600 + 30 * 60).expect("valid")
    );
    assert_eq!(format_date_time(v, false), "2026-11-17 07:30");
    assert_eq!(
        parse_date_time("2026-11-17T07:30:15").map(|v| v.second_of_day),
        Some(27_015)
    );
    assert_eq!(parse_date_time("2026-02-29"), None);
    assert_eq!(
        parse_date_time("2024-02-29").map(|v| v.second_of_day),
        Some(0)
    );
    assert_eq!(parse_date_time("17.11.2026"), None);
}

#[test]
fn colors_normalize_to_six_digits() {
    assert_eq!(parse_color("#ABC"), Some("#aabbcc".into()));
    assert_eq!(parse_color("12ab9F"), Some("#12ab9f".into()));
    assert_eq!(parse_color("#12345"), None);
    assert_eq!(parse_color("red"), None);
}
