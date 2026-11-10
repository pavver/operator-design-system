//! ThemeToggle: animated day/night switch.
//!
//! Based on the Day/Night Toggle by Jhey Tompkins (https://codepen.io/jh3y/pen/LYgjpYZ), MIT License.
//! The cloud outlines are taken from the original; see NOTICE.md.

use crate::core::{ControlState, Press, du, spawn_ripple};
use leptos::prelude::*;
use operator_ui_tokens::DU_POINTS;
use std::sync::atomic::{AtomicU32, Ordering};

const CLOUD_BACK: &str = "M335 147.5c0 27.89-22.61 50.5-50.5 50.5a50.78 50.78 0 0 1-9.29-.853c-2.478 12.606-10.595 23.188-21.615 29.011C245.699 243.749 228.03 256 207.5 256a50.433 50.433 0 0 1-16.034-2.599A41.811 41.811 0 0 1 166 262a41.798 41.798 0 0 1-22.893-6.782A42.21 42.21 0 0 1 135 256a41.82 41.82 0 0 1-19.115-4.592A41.84 41.84 0 0 1 88 262c-1.827 0-3.626-.117-5.391-.343C74.911 270.448 63.604 276 51 276c-23.196 0-42-18.804-42-42s18.804-42 42-42c1.827 0 3.626.117 5.391.343C64.089 183.552 75.396 178 88 178a41.819 41.819 0 0 1 19.115 4.592C114.532 176.002 124.298 172 135 172a41.798 41.798 0 0 1 22.893 6.782 42.066 42.066 0 0 1 7.239-.773C174.137 164.159 189.749 155 207.5 155c.601 0 1.199.01 1.794.031A41.813 41.813 0 0 1 234 147h.002c.269-27.66 22.774-50 50.498-50 27.89 0 50.5 22.61 50.5 50.5Z";
const CLOUD_FRONT: &str = "M328 167.5c0 15.214-7.994 28.56-20.01 36.068.007.31.01.621.01.932 0 23.472-19.028 42.5-42.5 42.5-3.789 0-7.461-.496-10.957-1.426C249.671 263.676 233.141 277 213.5 277a42.77 42.77 0 0 1-7.702-.696C198.089 284.141 187.362 289 175.5 289a42.338 42.338 0 0 1-27.864-10.408A42.411 42.411 0 0 1 133.5 281c-4.36 0-8.566-.656-12.526-1.876C113.252 287.066 102.452 292 90.5 292a42.388 42.388 0 0 1-15.8-3.034A42.316 42.316 0 0 1 48.5 298C25.028 298 6 278.972 6 255.5S25.028 213 48.5 213a42.388 42.388 0 0 1 15.8 3.034A42.316 42.316 0 0 1 90.5 207c4.36 0 8.566.656 12.526 1.876C110.748 200.934 121.548 196 133.5 196a42.338 42.338 0 0 1 27.864 10.408A42.411 42.411 0 0 1 175.5 204c2.63 0 5.204.239 7.702.696C190.911 196.859 201.638 192 213.5 192c3.789 0 7.461.496 10.957 1.426 2.824-10.491 9.562-19.377 18.553-24.994-.007-.31-.01-.621-.01-.932 0-23.472 19.028-42.5 42.5-42.5s42.5 19.028 42.5 42.5Z";

/// Stars as (x ratio, y ratio, radius in DU).
const STARS: [(f32, f32, f32); 7] = [
    (0.16, 0.22, 0.55),
    (0.28, 0.35, 0.375),
    (0.14, 0.68, 0.45),
    (0.24, 0.82, 0.35),
    (0.38, 0.25, 0.3),
    (0.42, 0.70, 0.4),
    (0.50, 0.38, 0.5),
];

/// Thumb inset from the track edge, in DU.
const THUMB_PAD: f32 = 0.625;

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

/// Four-point sparkle centered at `(x, y)`.
fn star_path(x: f32, y: f32, r: f32) -> String {
    format!(
        "M{x} {t}Q{x} {y} {rr} {y}Q{x} {y} {x} {b}Q{x} {y} {l} {y}Q{x} {y} {x} {t}Z",
        t = y - r,
        b = y + r,
        l = x - r,
        rr = x + r
    )
}

/// Pill-shaped day/night switch with a sun that turns into a moon.
///
/// A click calls `on_toggle` with the opposite value immediately; theme changes are local and never wait.
#[component]
pub fn ThemeToggle(
    /// `true` for night (dark), `false` for day (light).
    #[prop(into)]
    dark: Signal<bool>,
    #[prop(optional, into)] on_toggle: Option<Callback<bool>>,
    /// Width in DU.
    #[prop(default = 17.0)]
    width: f32,
    /// Height in DU.
    #[prop(default = 7.5)]
    height: f32,
    #[prop(default = "Dark mode".into(), into)] label: String,
) -> impl IntoView {
    let uid = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let track_clip = format!("op-tt-track-{uid}");

    let press = Press::new();
    let interactive = on_toggle.is_some();
    let current = Signal::derive(move || {
        if press.pressed.get() {
            ControlState::Pressed
        } else {
            ControlState::Idle
        }
    });

    // Geometry in SVG user units (1 unit = 1 logical point).
    let w = width * DU_POINTS;
    let h = height * DU_POINTS;
    let pad = THUMB_PAD * DU_POINTS;
    let d = h - 2.0 * pad;
    let r = d / 2.0;
    let travel = w - d - 2.0 * pad;
    let cloud_scale = w / 290.0;
    let cloud_y = h / cloud_scale - 228.0;

    let stars = STARS
        .iter()
        .map(|&(x, y, rad)| {
            view! { <path class="op-tt-star" d=star_path(w * x, h * y, rad * DU_POINTS)></path> }
        })
        .collect_view();

    view! {
        <button
            type="button"
            role="switch"
            class="op-sb op-rippled op-theme-toggle"
            aria-label=label
            aria-checked=move || dark.get().to_string()
            aria-disabled=(!interactive).then_some("true")
            style:width=du(width)
            style:height=du(height)
            style:--sb-rx=format!("{}px", h / 2.0)
            style:--tt-travel=format!("{travel}px")
            style:--tt-cloud-drop=format!("{}px", 160.0)
            data-dark=move || dark.get().then_some("")
            data-state=move || current.get().as_str()
            on:pointerdown=move |ev| {
                if interactive {
                    press.down();
                    spawn_ripple(&ev);
                }
            }
            on:pointerup=move |_| press.up()
            on:pointerleave=move |_| press.up()
            on:pointercancel=move |_| press.up()
            on:click=move |_| {
                if let Some(cb) = on_toggle {
                    cb.run(!dark.get_untracked());
                }
            }
        >
            <svg class="op-tt-svg" viewBox=format!("0 0 {w} {h}") aria-hidden="true">
                <defs>
                    <clipPath id=track_clip.clone()>
                        <rect width=w height=h rx=h / 2.0></rect>
                    </clipPath>
                </defs>
                <rect class="op-tt-sky" width=w height=h rx=h / 2.0></rect>
                <g clip-path=format!("url(#{track_clip})")>
                    <g class="op-tt-stars">{stars}</g>
                    <g transform=format!("scale({cloud_scale}) translate(0 {cloud_y})")>
                        <g class="op-tt-clouds">
                            <path class="op-tt-cloud-back" d=CLOUD_BACK transform="translate(0 -10)"></path>
                            <path class="op-tt-cloud" d=CLOUD_FRONT></path>
                        </g>
                    </g>
                    <g transform=format!("translate({pad} {pad})")>
                        <g class="op-tt-thumb">
                            <g class="op-tt-body">
                                <circle class="op-tt-halo" cx=r cy=r r=r * 2.1 opacity="0.12"></circle>
                                <circle class="op-tt-halo" cx=r cy=r r=r * 1.55 opacity="0.22"></circle>
                                <circle class="op-tt-orb" cx=r cy=r r=r></circle>
                                <g class="op-tt-craters">
                                    <circle cx=d * 0.42 cy=d * 0.26 r=r * 0.22></circle>
                                    <circle cx=d * 0.68 cy=d * 0.62 r=r * 0.25></circle>
                                    <circle cx=d * 0.28 cy=d * 0.52 r=r * 0.35></circle>
                                </g>
                            </g>
                        </g>
                    </g>
                </g>
            </svg>
        </button>
    }
}
