//! ThemeToggle: animated day/night switch.
//!
//! Based on the Day/Night Toggle by Jhey Tompkins (https://codepen.io/jh3y/pen/LYgjpYZ), MIT License.
//! The cloud outlines are taken from the original; see NOTICE.md.

use crate::core::{
    ControlState, Metric, Press, Slot, root_class, root_style, spawn_ripple, ui_text,
};
use leptos::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

pub(crate) const CSS: &str = include_str!("theme_toggle.css");

mod geometry;
mod scene;
static NEXT_ID: AtomicU32 = AtomicU32::new(0);

/// Pill-shaped day/night switch with a sun that turns into a moon.
///
/// A click calls `on_toggle` with the opposite value immediately; theme changes are local and never wait.
#[component]
pub fn ThemeToggle(
    /// `true` for night (dark), `false` for day (light).
    #[prop(into)]
    dark: Signal<bool>,
    #[prop(optional, into)] on_toggle: Option<Callback<bool>>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] thumb_inset: MaybeProp<f32>,
    #[prop(optional, into)] star_scale: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] content: Slot,
) -> impl IntoView {
    let label = ui_text(label, "theme-toggle.label", "Dark mode");

    let root_metrics = vec![
        Metric::du("--op-theme-toggle-width", width),
        Metric::du("--op-theme-toggle-height", height),
        Metric::du("--op-theme-toggle-thumb-inset", thumb_inset),
        Metric::du("--op-theme-toggle-star-scale", star_scale),
    ];

    let root = NodeRef::<leptos::html::Button>::new();
    let probe = NodeRef::<leptos::html::Span>::new();
    let geometry = geometry::observe(root, probe);
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

    view! {
        <button
            node_ref=root
            type="button"
            role="switch"
            class=root_class("op-sb op-rippled op-theme-toggle", class)
            style=root_style(root_metrics, style)
            aria-label=move || label.get()
            aria-checked=move || dark.get().to_string()
            aria-disabled=(!interactive).then_some("true")
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
            <span node_ref=probe class="op-tt-probe" aria-hidden="true"></span>
            {move || if content.is_set() { content.run().into_any() } else {
                let (w, h, pad, star_scale) = geometry.get();
                scene::draw(w, h, pad, star_scale, &track_clip)
            }}
        </button>
    }
}
