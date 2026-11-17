use super::route::{route_midpoint, route_path};
use super::*;

#[test]
fn zoom_keeps_the_point_under_the_cursor() {
    let c = Camera {
        x: 10.0,
        y: 20.0,
        zoom: 1.0,
    };
    let before = c.to_world(100.0, 50.0);
    let z = c.zoom_at(100.0, 50.0, 2.0, 0.25, 4.0);
    assert_eq!(z.zoom, 2.0);
    assert_eq!(z.to_world(100.0, 50.0), before);
    assert_eq!(c.zoom_at(0.0, 0.0, 100.0, 0.25, 2.0).zoom, 2.0);
}

#[test]
fn screen_and_world_round_trip() {
    let c = Camera {
        x: -30.0,
        y: 12.0,
        zoom: 0.5,
    };
    let (sx, sy) = c.to_screen(100.0, -40.0);
    assert_eq!(c.to_world(sx, sy), (100.0, -40.0));
}

#[test]
fn fit_centers_the_bounds() {
    let b = WorldRect::new(100.0, 100.0, 400.0, 200.0);
    let c = Camera::fit(b, 800.0, 600.0, 0.0, 0.25, 1.0);
    assert_eq!(c.zoom, 1.0);
    assert_eq!(c.to_screen(300.0, 200.0), (400.0, 300.0));
    let small = Camera::fit(
        WorldRect::new(0.0, 0.0, 8000.0, 100.0),
        800.0,
        600.0,
        0.0,
        0.25,
        1.0,
    );
    assert_eq!(small.zoom, 0.25);
}

#[test]
fn visible_area_and_centering() {
    let c = Camera {
        x: 0.0,
        y: 0.0,
        zoom: 2.0,
    };
    assert_eq!(
        c.visible(800.0, 600.0),
        WorldRect::new(0.0, 0.0, 400.0, 300.0)
    );
    let centered = c.centered_on(100.0, 100.0, 800.0, 600.0);
    assert_eq!(centered.to_screen(100.0, 100.0), (400.0, 300.0));
}

#[test]
fn bounds_union_all_rects() {
    let b = WorldRect::bounds([
        WorldRect::new(0.0, 0.0, 10.0, 10.0),
        WorldRect::new(-5.0, 20.0, 10.0, 10.0),
    ]);
    assert_eq!(b, Some(WorldRect::new(-5.0, 0.0, 15.0, 30.0)));
    assert_eq!(WorldRect::bounds([]), None);
}

#[test]
fn wire_paths_start_and_end_at_the_sockets() {
    let d = route_path((0.0, 0.0), &[], (100.0, 50.0));
    assert!(d.starts_with("M 0 0 C"), "{d}");
    assert!(d.ends_with("100 50"), "{d}");
    let through = route_path((0.0, 0.0), &[(50.0, 100.0)], (100.0, 0.0));
    assert_eq!(through.matches(" C ").count(), 2);
    let mid = route_midpoint((0.0, 0.0), &[], (100.0, 0.0));
    assert!((mid.0 - 50.0).abs() < 1.0 && mid.1.abs() < 1e-9, "{mid:?}");
}

#[test]
fn minimap_scale_fits_and_centers() {
    let (scale, ox, oy) = minimap_scale(WorldRect::new(0.0, 0.0, 1000.0, 500.0), 200.0, 200.0);
    assert_eq!(scale, 0.2);
    assert_eq!(ox, 0.0);
    assert_eq!(oy, -250.0);
}
