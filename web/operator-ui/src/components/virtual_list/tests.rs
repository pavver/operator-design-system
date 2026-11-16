use super::*;

#[test]
fn window_covers_the_viewport_with_overscan() {
    // 100 rows of 40 px, viewport 200 px scrolled to row 10.
    assert_eq!(visible_window(100, 40.0, 400.0, 200.0, 2), (8, 18));
}

#[test]
fn window_is_clamped_at_both_ends() {
    assert_eq!(visible_window(100, 40.0, 0.0, 200.0, 6), (0, 12));
    assert_eq!(visible_window(10, 40.0, 10_000.0, 200.0, 6), (10, 10));
    assert_eq!(visible_window(10, 40.0, 320.0, 200.0, 6), (2, 10));
}

#[test]
fn empty_lists_and_zero_rows_render_nothing() {
    assert_eq!(visible_window(0, 40.0, 0.0, 200.0, 6), (0, 0));
    assert_eq!(visible_window(10, 0.0, 0.0, 200.0, 6), (0, 0));
}

#[test]
fn window_size_does_not_depend_on_list_length() {
    let (start, end) = visible_window(20_000, 32.0, 320_000.0, 400.0, 6);
    assert!(end - start <= 26, "{start}..{end}");
}

#[test]
fn reveal_scrolls_only_when_the_row_is_outside() {
    assert_eq!(reveal_offset(5, 40.0, 0.0, 400.0, false), None);
    assert_eq!(reveal_offset(20, 40.0, 0.0, 400.0, false), Some(440.0));
    assert_eq!(reveal_offset(2, 40.0, 400.0, 400.0, false), Some(80.0));
}

#[test]
fn reveal_centers_on_request() {
    assert_eq!(reveal_offset(20, 40.0, 0.0, 400.0, true), Some(620.0));
    assert_eq!(reveal_offset(1, 40.0, 0.0, 400.0, true), Some(0.0));
}
