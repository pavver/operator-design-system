//! Placement rules of [`fit_layer`]: preferences hold while the layer fits, else they bend.
use super::*;

const VW: f64 = 1000.0;
const VH: f64 = 800.0;

/// A 100×40 trigger at the given top-left corner.
fn trigger(left: f64, top: f64) -> Bounds {
    Bounds {
        left,
        top,
        width: 100.0,
        height: 40.0,
    }
}

fn fit(
    anchor: Bounds,
    w: f64,
    h: f64,
    side: LayerSide,
    align: LayerAlign,
) -> (FloatPos, FloatLimit) {
    fit_layer(anchor, w, h, VW, VH, 0.0, side, align)
}

#[test]
fn below_centered_by_default() {
    let (pos, limit) = fit(
        trigger(450.0, 300.0),
        200.0,
        100.0,
        LayerSide::default(),
        LayerAlign::default(),
    );
    assert_eq!(
        pos,
        FloatPos {
            left: 400.0,
            top: 340.0
        }
    );
    assert_eq!(limit, FloatLimit::default());
}

#[test]
fn vertical_alignments_stick_out_right_and_left() {
    let a = trigger(450.0, 300.0);
    assert_eq!(
        fit(a, 200.0, 100.0, LayerSide::Above, LayerAlign::Start).0,
        FloatPos {
            left: 450.0,
            top: 200.0
        }
    );
    assert_eq!(
        fit(a, 200.0, 100.0, LayerSide::Above, LayerAlign::End).0,
        FloatPos {
            left: 350.0,
            top: 200.0
        }
    );
}

#[test]
fn side_alignments_stick_out_down_and_up() {
    let a = trigger(450.0, 300.0);
    assert_eq!(
        fit(a, 200.0, 100.0, LayerSide::Right, LayerAlign::Start).0,
        FloatPos {
            left: 550.0,
            top: 300.0
        }
    );
    assert_eq!(
        fit(a, 200.0, 100.0, LayerSide::Right, LayerAlign::Center).0,
        FloatPos {
            left: 550.0,
            top: 270.0
        }
    );
    assert_eq!(
        fit(a, 200.0, 100.0, LayerSide::Left, LayerAlign::End).0,
        FloatPos {
            left: 250.0,
            top: 240.0
        }
    );
}

#[test]
fn flips_to_the_opposite_side_without_room() {
    // Near the bottom edge: below does not fit, above does.
    assert_eq!(
        fit(
            trigger(450.0, 740.0),
            200.0,
            100.0,
            LayerSide::Below,
            LayerAlign::Center
        )
        .0
        .top,
        640.0
    );
    // Near the right edge: right does not fit, left does.
    assert_eq!(
        fit(
            trigger(850.0, 300.0),
            200.0,
            100.0,
            LayerSide::Right,
            LayerAlign::Center
        )
        .0
        .left,
        650.0
    );
}

#[test]
fn side_placement_falls_back_below_when_neither_side_fits() {
    let (pos, _) = fit(
        trigger(450.0, 300.0),
        500.0,
        100.0,
        LayerSide::Left,
        LayerAlign::Center,
    );
    assert_eq!(
        pos,
        FloatPos {
            left: 250.0,
            top: 340.0
        }
    );
}

#[test]
fn alignment_bends_at_the_viewport_edge() {
    let (pos, _) = fit(
        trigger(0.0, 300.0),
        200.0,
        100.0,
        LayerSide::Below,
        LayerAlign::End,
    );
    assert_eq!(pos.left, 0.0);
}

#[test]
fn too_tall_takes_the_roomier_side_and_scrolls() {
    let (pos, limit) = fit(
        trigger(450.0, 500.0),
        200.0,
        900.0,
        LayerSide::Below,
        LayerAlign::Center,
    );
    assert_eq!(pos.top, 0.0);
    assert_eq!(limit.max_height, Some(500.0));
}

#[test]
fn too_wide_is_limited_to_the_viewport() {
    let (pos, limit) = fit(
        trigger(450.0, 300.0),
        1200.0,
        100.0,
        LayerSide::Below,
        LayerAlign::Start,
    );
    assert_eq!(pos.left, 0.0);
    assert_eq!(limit.max_width, Some(VW));
}
