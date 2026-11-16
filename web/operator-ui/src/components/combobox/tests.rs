use super::*;

fn opts() -> Vec<ComboOption> {
    vec![
        ComboOption::new("light.kitchen")
            .label("Kitchen")
            .section("Lights"),
        ComboOption::new("light.hall")
            .label("Hall")
            .section("Lights"),
        ComboOption::new("sensor.temp")
            .label("Temperature")
            .detail("°C")
            .section("Sensors"),
        ComboOption::new("plain"),
    ]
}

#[test]
fn rows_group_consecutive_sections() {
    let rows = filter_rows(&opts(), "");
    let keys: Vec<String> = rows.iter().map(ComboRow::key).collect();
    assert_eq!(
        keys,
        [
            "\u{1}Lights",
            "light.kitchen",
            "light.hall",
            "\u{1}Sensors",
            "sensor.temp",
            "plain"
        ]
    );
    assert_eq!(option_count(&rows), 4);
}

#[test]
fn filtering_matches_label_or_value_and_drops_empty_sections() {
    let rows = filter_rows(&opts(), "SENSOR");
    assert_eq!(rows.len(), 2);
    assert_eq!(row_of(&rows, 0), Some(1));
    assert_eq!(
        option_at(&rows, 0).map(|o| o.value),
        Some("sensor.temp".into())
    );
    assert_eq!(filter_rows(&opts(), "hall").len(), 2);
    assert!(filter_rows(&opts(), "zzz").is_empty());
}

#[test]
fn step_wraps_and_survives_empty_lists() {
    assert_eq!(step(0, 3, -1), 2);
    assert_eq!(step(2, 3, 1), 0);
    assert_eq!(step(5, 0, 1), 0);
}

#[test]
fn display_and_exact_lookup_use_labels() {
    let o = opts();
    assert_eq!(display_text(&o, "light.hall"), "Hall");
    assert_eq!(display_text(&o, "unknown"), "unknown");
    assert_eq!(
        find_exact(&o, "kitchen").map(|o| o.value),
        Some("light.kitchen".into())
    );
    assert_eq!(
        find_exact(&o, "PLAIN").map(|o| o.value),
        Some("plain".into())
    );
    assert!(find_exact(&o, "kit").is_none());
}
