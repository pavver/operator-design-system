//! Avatar: Avatar, AvatarKind, and avatar_color.

use crate::components::primitives::ProgressIndicator;
use crate::components::status_badge::{PresenceStatus, StatusBadge};
use crate::core::theme::OperatorTheme;
use crate::core::{Metric, Slot, root_class, root_style};
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
    /// Size in DU (`--op-avatar-size`).
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    #[prop(optional, into)] status: Option<Signal<PresenceStatus>>,
    /// Image URL; the special value `debug_loading` shows a persistent loading spinner.
    #[prop(optional, into)]
    image_url: Option<String>,
    /// Custom status badge slot replacing the built-in badge.
    #[prop(optional, into)]
    badge: Slot,
    /// Font size in TU (`--op-avatar-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    /// Outline border width in DU (`--op-avatar-border-width`).
    #[prop(optional, into)]
    border_width: MaybeProp<f32>,
    /// Badge offset in DU (`--op-avatar-badge-offset`).
    #[prop(optional, into)]
    badge_offset: MaybeProp<f32>,
    /// Badge size in DU.
    #[prop(optional, into)]
    badge_size: MaybeProp<f32>,
    /// Accessible name; defaults to `name`.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
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
    let size_val = move || size.get().unwrap_or(dim::AVATAR_HEADER);
    let accessible_name = Signal::derive(move || aria_label.get().unwrap_or_else(|| name.clone()));

    let image = image_url.map(|url| {
        if url == "debug_loading" {
            view! {
                <span class="op-avatar-layer op-avatar-center">
                    {move || view! { <ProgressIndicator size=size_val() * 0.8 stroke=dim::SPINNER_STROKE color="var(--op-pending)"/> }}
                </span>
            }
            .into_any()
        } else {
            view! {
                <Show when=move || !loaded.get()>
                    <span class="op-avatar-layer op-avatar-center">
                        {move || view! { <ProgressIndicator size=size_val() * 0.8 stroke=dim::SPINNER_STROKE color="var(--op-pending)"/> }}
                    </span>
                </Show>
                <img class="op-avatar-layer op-avatar-img" src=url alt="" on:load=move |_| loaded.set(true)/>
            }
            .into_any()
        }
    });

    let rendered_badge = move || {
        if badge.is_set() {
            view! { <span class="op-avatar-badge">{badge.run()}</span> }.into_any()
        } else if let Some(s) = status {
            let b_size = Signal::derive(move || {
                badge_size
                    .get()
                    .unwrap_or_else(|| (size_val() * 0.32).max(2.5))
            });
            view! {
                <span class="op-avatar-badge">
                    <StatusBadge status=s size=b_size/>
                </span>
            }
            .into_any()
        } else {
            ().into_any()
        }
    };

    view! {
        <span
            class=root_class("op-avatar", class)
            role="img"
            aria-label=move || accessible_name.get()
            style=root_style(
                vec![
                    Metric::du("--op-avatar-size", size),
                    Metric::tu("--op-avatar-font-size", font_size),
                    Metric::du("--op-avatar-border-width", border_width),
                    Metric::du("--op-avatar-badge-offset", badge_offset),
                ],
                style,
            )
            style=("--op-avatar-radius", kind.radius())
            style=("--op-avatar-color", color)
        >
            <span class="op-avatar-layer op-avatar-letter">
                {letter}
            </span>
            {image}
            <span class="op-avatar-layer op-avatar-outline"></span>
            {rendered_badge}
        </span>
    }
}
