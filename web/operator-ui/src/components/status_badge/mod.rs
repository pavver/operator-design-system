//! Status badge: PresenceStatus and StatusBadge.

use crate::components::primitives::ProgressIndicator;
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::dim;

pub(crate) const CSS: &str = include_str!("status_badge.css");

/// Presence and activity status shown by [`StatusBadge`] and [`crate::components::avatar::Avatar`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum PresenceStatus {
    Online = 0,
    Offline = 1,
    Alarm = 2,
    Pending = 3,
    Error = 4,
    Typing = 5,
    RecordingAudio = 6,
    SendingFile = 7,
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

    pub const fn led_kind(self) -> Option<&'static str> {
        match self {
            Self::Online => Some("online"),
            Self::Offline => Some("offline"),
            Self::Alarm => Some("alarm"),
            Self::Error => Some("error"),
            _ => None,
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

    let labels = [
        online_str,
        offline_str,
        alarm_str,
        pending_str,
        error_str,
        typing_str,
        rec_str,
        send_str,
    ];

    let accessible_name = Signal::derive(move || {
        resolve_badge_accessible_name(status.get(), aria_label.get(), &labels)
    });

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
            <StatusBadgeGlyph status=status size_val=Signal::derive(size_val) />
            <StatusBadgeText text=text text_slot=text_slot />
        </span>
    }
}

pub fn resolve_badge_accessible_name(
    status: PresenceStatus,
    aria_label: Option<String>,
    labels: &[Signal<String>; 8],
) -> String {
    if let Some(name) = aria_label {
        name
    } else {
        labels[status as usize].get()
    }
}

fn recording_audio_svg() -> impl IntoView {
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
}

fn sending_file_svg() -> impl IntoView {
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
}

#[component]
fn StatusBadgeGlyph(status: Signal<PresenceStatus>, size_val: Signal<f32>) -> impl IntoView {
    move || {
        match status.get() {
        PresenceStatus::Pending => view! {
            <ProgressIndicator size=size_val.get() stroke=0.3 color="var(--op-status-badge-activity, var(--op-status-activity))"/>
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
        PresenceStatus::RecordingAudio => recording_audio_svg().into_any(),
        PresenceStatus::SendingFile => sending_file_svg().into_any(),
        other => {
            let kind = other.led_kind().unwrap_or("fallback");
            view! { <span class="op-badge-led" data-kind=kind></span> }.into_any()
        }
    }
    }
}

#[component]
fn StatusBadgeText(text: MaybeProp<String>, text_slot: Slot) -> impl IntoView {
    move || {
        if text_slot.is_set() {
            view! { <span class="op-badge-text">{text_slot.run()}</span> }.into_any()
        } else if let Some(t) = text.get() {
            view! { <span class="op-badge-text">{t}</span> }.into_any()
        } else {
            ().into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presence_status_led_kind() {
        assert_eq!(PresenceStatus::Online.led_kind(), Some("online"));
        assert_eq!(PresenceStatus::Offline.led_kind(), Some("offline"));
        assert_eq!(PresenceStatus::Alarm.led_kind(), Some("alarm"));
        assert_eq!(PresenceStatus::Error.led_kind(), Some("error"));
        assert_eq!(PresenceStatus::Pending.led_kind(), None);
        assert_eq!(PresenceStatus::Typing.led_kind(), None);
    }

    #[test]
    fn test_resolve_badge_accessible_name() {
        let labels = [
            Signal::derive(|| "Online".to_string()),
            Signal::derive(|| "Offline".to_string()),
            Signal::derive(|| "Alarm".to_string()),
            Signal::derive(|| "Pending".to_string()),
            Signal::derive(|| "Error".to_string()),
            Signal::derive(|| "Typing".to_string()),
            Signal::derive(|| "Recording".to_string()),
            Signal::derive(|| "Sending".to_string()),
        ];
        assert_eq!(
            resolve_badge_accessible_name(PresenceStatus::Online, None, &labels),
            "Online"
        );
        assert_eq!(
            resolve_badge_accessible_name(PresenceStatus::Alarm, Some("Override".into()), &labels),
            "Override"
        );
    }
}
