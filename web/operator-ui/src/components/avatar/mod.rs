//! Avatar: Avatar, AvatarKind, and avatar_color.

use crate::components::primitives::ProgressIndicator;
use crate::components::status_badge::{PresenceStatus, StatusBadge};
use crate::core::theme::OperatorTheme;
use crate::core::{du, tu};
use leptos::prelude::*;
use operator_ui_tokens::dim;

pub(crate) const CSS: &str = include_str!("avatar.css");

/// Avatar outline shape by chat kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AvatarKind {
    /// Circle (direct person-to-person chat).
    #[default]
    Direct,
    /// Square (threaded group).
    ThreadedGroup,
    /// 15% corner radius (channel).
    Channel,
    /// 35% corner radius (simple group).
    SimpleGroup,
}

impl AvatarKind {
    fn radius(self) -> &'static str {
        match self {
            Self::Direct => "50%",
            Self::ThreadedGroup => "0",
            Self::Channel => "15%",
            Self::SimpleGroup => "35%",
        }
    }
}

/// Java `String.hashCode` (UTF-16, wrapping) so every platform picks the same avatar color.
fn java_hash(s: &str) -> i32 {
    s.encode_utf16()
        .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(i32::from(c)))
}

fn hsv(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match (h / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let to = |f: f32| ((f + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    (to(r), to(g), to(b))
}

/// Deterministic avatar color for an id (same algorithm as the Android app).
pub fn avatar_color(id: &str, light: bool) -> (u8, u8, u8) {
    const HUES: [f32; 7] = [190.0, 210.0, 250.0, 290.0, 330.0, 160.0, 35.0];
    let hue = HUES[(java_hash(id).unsigned_abs() as usize) % HUES.len()];
    if light {
        hsv(hue, 0.65, 0.8)
    } else {
        hsv(hue, 0.45, 0.75)
    }
}

/// Letter avatar with deterministic color, optional image and status badge.
#[component]
pub fn Avatar(
    #[prop(into)] id: String,
    #[prop(into)] name: String,
    #[prop(optional)] kind: AvatarKind,
    /// Size in DU.
    #[prop(default = dim::AVATAR_HEADER)]
    size: f32,
    #[prop(optional, into)] status: Option<Signal<PresenceStatus>>,
    /// Image URL; the special value `debug_loading` shows a persistent loading spinner.
    #[prop(optional, into)]
    image_url: Option<String>,
) -> impl IntoView {
    let theme = OperatorTheme::current();
    let letter = name
        .chars()
        .next()
        .map(|c| c.to_uppercase().collect::<String>())
        .unwrap_or_else(|| "?".into());
    let color = move || {
        let (r, g, b) = avatar_color(&id, !theme.dark.get());
        format!("rgb({r} {g} {b})")
    };
    let loaded = RwSignal::new(false);
    let badge_size = (size * 0.32).max(2.5);
    let image = image_url.map(|url| {
        if url == "debug_loading" {
            view! {
                <span class="op-avatar-layer op-avatar-center">
                    <ProgressIndicator size=size * 0.8 stroke=dim::SPINNER_STROKE color="var(--op-pending)"/>
                </span>
            }
            .into_any()
        } else {
            view! {
                <Show when=move || !loaded.get()>
                    <span class="op-avatar-layer op-avatar-center">
                        <ProgressIndicator size=size * 0.8 stroke=dim::SPINNER_STROKE color="var(--op-pending)"/>
                    </span>
                </Show>
                <img class="op-avatar-layer op-avatar-img" src=url alt="" on:load=move |_| loaded.set(true)/>
            }
            .into_any()
        }
    });
    view! {
        <span
            class="op-avatar"
            role="img"
            aria-label=name.clone()
            style:width=du(size)
            style:height=du(size)
            style=("--op-avatar-radius", kind.radius())
            style=("--op-avatar-color", color)
        >
            <span class="op-avatar-layer op-avatar-letter" style:font-size=tu(size * operator_ui_tokens::DU_POINTS * 0.45)>
                {letter}
            </span>
            {image}
            <span class="op-avatar-layer op-avatar-outline"></span>
            {status
                .map(|s| {
                    view! {
                        <span class="op-avatar-badge">
                            <StatusBadge status=s size=badge_size/>
                        </span>
                    }
                })}
        </span>
    }
}
