use super::*;

#[test]
fn test_track_run_attr() {
    assert_eq!(track_run_attr(0), None);
    assert_eq!(track_run_attr(1), Some("a"));
    assert_eq!(track_run_attr(2), Some("b"));
    assert_eq!(track_run_attr(3), Some("a"));
}

#[test]
fn test_animation_duration_ms() {
    // velocity 7.5 DU/s = 7.5 * 4 = 30.0 px/s
    // distance 150.0 px => 150 / 30 = 5.0 s = 5000ms
    assert_eq!(animation_duration_ms(150.0, 7.5), "5000ms");
}
