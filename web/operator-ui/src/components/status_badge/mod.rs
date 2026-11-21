//! Status badge: PresenceStatus and StatusBadge.

use crate::components::primitives::ProgressIndicator;
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
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
    /// Size in DU (`--op-status-badge-size`).
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    /// Optional caption rendered to the right of the badge.
    #[prop(optional, into)]
    text: MaybeProp<String>,
    /// Caption slot next to the badge.
    #[prop(optional, into)]
    text_slot: Slot,
    /// Gap between badge and caption in DU (`--op-status-badge-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Caption font size in TU (`--op-status-badge-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    /// Caption line height in TU (`--op-status-badge-line-height`).
    #[prop(optional, into)]
    line_height: MaybeProp<f32>,
    /// Accessible name override.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let size_val = move || size.get().unwrap_or(dim::PAD_CARD);
    let online_str = ui_text(None::<String>.into(), "status-badge.online", "Online");
    let offline_str = ui_text(None::<String>.into(), "status-badge.offline", "Offline");
    let alarm_str = ui_text(None::<String>.into(), "status-badge.alarm", "Alarm");
    let pending_str = ui_text(None::<String>.into(), "status-badge.pending", "Pending");
    let error_str = ui_text(None::<String>.into(), "status-badge.error", "Error");
    let typing_str = ui_text(None::<String>.into(), "status-badge.typing", "Typing");
    let rec_str = ui_text(
        None::<String>.into(),
        "status-badge.recording-audio",
        "Recording audio",
    );
    let send_str = ui_text(
        None::<String>.into(),
        "status-badge.sending-file",
        "Sending file",
    );

    let accessible_name = Signal::derive(move || {
        if let Some(name) = aria_label.get() {
            name
        } else {
            match status.get() {
                PresenceStatus::Online => online_str.get(),
                PresenceStatus::Offline => offline_str.get(),
                PresenceStatus::Alarm => alarm_str.get(),
                PresenceStatus::Pending => pending_str.get(),
                PresenceStatus::Error => error_str.get(),
                PresenceStatus::Typing => typing_str.get(),
                PresenceStatus::RecordingAudio => rec_str.get(),
                PresenceStatus::SendingFile => send_str.get(),
            }
        }
    });

    let badge = move || {
        match status.get() {
        PresenceStatus::Pending => view! {
            <ProgressIndicator size=size_val() stroke=0.3 color="var(--op-status-badge-activity, var(--op-status-activity))"/>
        }
        .into_any(),
        PresenceStatus::Typing => view! {
            <span class="op-badge-typing">
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
                <svg class="op-badge-voice" viewBox="0 0 100 100">
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
                <svg class="op-badge-file" viewBox="0 0 100 100">
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
            view! { <span class="op-badge-led" data-kind=kind></span> }.into_any()
        }
    }
    };

    let text_content = move || {
        if text_slot.is_set() {
            view! { <span class="op-badge-text">{text_slot.run()}</span> }.into_any()
        } else if let Some(t) = text.get() {
            view! { <span class="op-badge-text">{t}</span> }.into_any()
        } else {
            ().into_any()
        }
    };

    view! {
        <span
            class=root_class("op-badge", class)
            role="status"
            aria-label=move || accessible_name.get()
            style=root_style(
                vec![
                    Metric::du("--op-status-badge-size", size),
                    Metric::du("--op-status-badge-gap", gap),
                    Metric::tu("--op-status-badge-font-size", font_size),
                    Metric::tu("--op-status-badge-line-height", line_height),
                ],
                style,
            )
        >
            {badge}
            {text_content}
        </span>
    }
}
