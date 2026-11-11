//! Status badge: PresenceStatus and StatusBadge.

use crate::components::primitives::ProgressIndicator;
use crate::core::du;
use leptos::prelude::*;
use operator_ui_tokens::dim;

pub(crate) const CSS: &str = include_str!("status_badge.css");

/// Presence and activity status shown by [`StatusBadge`] and [`crate::components::avatar::Avatar`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PresenceStatus {
    Online,
    Offline,
    Alarm,
    Pending,
    Error,
    Typing,
    RecordingAudio,
    SendingFile,
}

impl PresenceStatus {
    pub const ALL: [Self; 8] = [
        Self::Online,
        Self::Offline,
        Self::Alarm,
        Self::Pending,
        Self::Error,
        Self::Typing,
        Self::RecordingAudio,
        Self::SendingFile,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Online => "Online",
            Self::Offline => "Offline",
            Self::Alarm => "Alarm",
            Self::Pending => "Pending",
            Self::Error => "Error",
            Self::Typing => "Typing",
            Self::RecordingAudio => "RecordingAudio",
            Self::SendingFile => "SendingFile",
        }
    }
}

fn arc_path(cx: f32, cy: f32, r: f32, from_deg: f32, to_deg: f32) -> String {
    let (a, b) = (from_deg.to_radians(), to_deg.to_radians());
    format!(
        "M{:.2} {:.2} A{r:.2} {r:.2} 0 0 1 {:.2} {:.2}",
        cx + r * a.cos(),
        cy + r * a.sin(),
        cx + r * b.cos(),
        cy + r * b.sin()
    )
}

/// LED-style status indicator with animated activity variants.
#[component]
pub fn StatusBadge(
    #[prop(into)] status: Signal<PresenceStatus>,
    /// Size in DU.
    #[prop(default = dim::PAD_CARD)]
    size: f32,
    /// Optional caption rendered to the right of the badge.
    #[prop(optional, into)]
    text: Option<Signal<String>>,
) -> impl IntoView {
    let badge = move || match status.get() {
        PresenceStatus::Pending => view! {
            <ProgressIndicator size=size stroke=0.3 color="var(--op-status-activity)"/>
        }
        .into_any(),
        PresenceStatus::Typing => view! {
            <span class="op-badge-typing" style:width=du(size) style:height=du(size)>
                <i></i>
                <i></i>
                <i></i>
            </span>
        }
        .into_any(),
        PresenceStatus::RecordingAudio => {
            let arcs = (0..3)
                .map(|i| {
                    let r = 28.0 + i as f32 * 23.5;
                    view! { <path class=format!("op-wave op-wave-{i}") d=arc_path(18.0, 50.0, r, -32.0, 32.0)></path> }
                })
                .collect_view();
            view! {
                <svg class="op-badge-voice" viewBox="0 0 100 100" style:width=du(size) style:height=du(size)>
                    <circle class="op-wave-src" cx="18" cy="50" r="11"></circle>
                    {arcs}
                </svg>
            }
            .into_any()
        }
        PresenceStatus::SendingFile => {
            let chevrons = (0..3)
                .map(|i| {
                    let tip = 28.0 + i as f32 * 30.0;
                    let start = tip - 15.0;
                    view! {
                        <path
                            class=format!("op-chev op-chev-{i}")
                            d=format!("M{start} 26 L{tip} 50 L{start} 74")
                        ></path>
                    }
                })
                .collect_view();
            view! {
                <svg class="op-badge-file" viewBox="0 0 100 100" style:width=du(size) style:height=du(size)>
                    {chevrons}
                </svg>
            }
            .into_any()
        }
        other => {
            let kind = match other {
                PresenceStatus::Online => "online",
                PresenceStatus::Offline => "offline",
                PresenceStatus::Alarm => "alarm",
                PresenceStatus::Error => "error",
                _ => "fallback",
            };
            view! { <span class="op-badge-led" data-kind=kind style:width=du(size) style:height=du(size)></span> }
                .into_any()
        }
    };
    view! {
        <span class="op-badge" role="status" aria-label=move || status.get().name()>
            {badge}
            {text.map(|t| view! { <span class="op-badge-text">{move || t.get()}</span> })}
        </span>
    }
}
