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

mod colors {
    use super::super::color_math::*;

    fn near(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn hex_forms_parse_with_alpha() {
        let c = parse_rgba("#11223380").map(|c| c.bytes());
        assert_eq!(c, Some([0x11, 0x22, 0x33, 0x80]));
        assert_eq!(
            parse_rgba("#abcd").map(|c| c.bytes()),
            Some([0xaa, 0xbb, 0xcc, 0xdd])
        );
        assert_eq!(parse_rgba("#12345"), None);
        assert_eq!(parse_rgba("red"), None);
    }

    #[test]
    fn css_functions_parse() {
        assert_eq!(
            parse_rgba("rgb(255, 0, 128)").map(|c| c.hex(true)),
            Some("#ff0080".into())
        );
        assert_eq!(
            parse_rgba("rgba(0 0 0 / 50%)").map(|c| c.hex(true)),
            Some("#00000080".into())
        );
        assert_eq!(
            parse_rgba("hsl(120, 100%, 50%)").map(|c| c.hex(false)),
            Some("#00ff00".into())
        );
        assert_eq!(
            parse_rgba("hsv(240, 100, 100)").map(|c| c.hex(false)),
            Some("#0000ff".into())
        );
        assert_eq!(parse_rgba("rgb(300, 0, 0)"), None);
    }

    #[test]
    fn hex_drops_opaque_alpha() {
        let c = Rgba::from_bytes(1, 2, 3, 255);
        assert_eq!(c.hex(true), "#010203");
        assert_eq!(Rgba::from_bytes(1, 2, 3, 0).hex(true), "#01020300");
        assert_eq!(Rgba::from_bytes(1, 2, 3, 0).hex(false), "#010203");
    }

    #[test]
    fn models_round_trip() {
        for hex in [
            "#2ec4b6", "#e63946", "#000000", "#ffffff", "#808080", "#123456",
        ] {
            let c = parse_rgba(hex).unwrap_or(Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            });
            assert_eq!(hsv_to_rgb(rgb_to_hsv(c), 1.0).hex(false), hex);
            assert_eq!(hsl_to_rgb(rgb_to_hsl(c), 1.0).hex(false), hex);
        }
        let hsl = rgb_to_hsl(Rgba::from_bytes(46, 196, 182, 255));
        assert!(near(hsl.h, 174.4) || (hsl.h - 174.4).abs() < 0.5);
    }

    #[test]
    fn greys_keep_the_previous_hue() {
        let previous = Hsv {
            h: 200.0,
            s: 0.7,
            v: 0.9,
        };
        let grey = hsv_keeping(Rgba::from_bytes(80, 80, 80, 255), previous);
        assert!(near(grey.h, 200.0) && near(grey.s, 0.0));
        let black = hsv_keeping(Rgba::from_bytes(0, 0, 0, 255), previous);
        assert!(near(black.h, 200.0) && near(black.s, 0.7));
    }

    #[test]
    fn temperatures_round_trip() {
        assert_eq!(
            parse_rgba("6600K").map(|c| c.hex(false)),
            Some("#ffffff".into())
        );
        for k in [2200.0, 2700.0, 4000.0, 5000.0, 6000.0] {
            let back = rgb_to_kelvin(kelvin_to_rgb(k), 1500.0, 6500.0);
            assert!((back - k).abs() < 50.0, "{k} -> {back}");
        }
        assert!(kelvin_to_rgb(2000.0).b < kelvin_to_rgb(5000.0).b);
    }
}

#[test]
fn dial_maps_angles_to_hours_and_minutes() {
    assert!((dial_angle(1.0, 0.0) - 90.0).abs() < 1e-9);
    assert!((dial_angle(0.0, 1.0) - 180.0).abs() < 1e-9);
    assert_eq!(dial_hour(0.0, false, false, false), 12);
    assert_eq!(dial_hour(0.0, true, false, false), 0);
    assert_eq!(dial_hour(90.0, false, false, false), 3);
    assert_eq!(dial_hour(90.0, true, false, false), 15);
    assert_eq!(dial_hour(359.0, true, false, false), 0);
    assert_eq!(dial_hour(0.0, false, true, false), 0);
    assert_eq!(dial_hour(0.0, false, true, true), 12);
    assert_eq!(dial_hour(210.0, false, true, true), 19);
    assert_eq!(dial_minute(90.0, 1), 15);
    assert_eq!(dial_minute(357.0, 1), 0);
    assert_eq!(dial_minute(100.0, 5), 15);
    assert_eq!(dial_minute(350.0, 15), 0);
}
