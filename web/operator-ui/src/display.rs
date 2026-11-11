//! ListItem, StatusBadge, Avatar, Header and ConsoleTerminal.

use crate::basic::{Icon, ProgressIndicator};
use crate::core::{Press, du, spawn_ripple, tu};
use crate::scroll::ScrollView;
use crate::theme::OperatorTheme;
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};

// ---------------------------------------------------------------------------
// ListItem
// ---------------------------------------------------------------------------

/// Base list row with optional leading and trailing slots and a bottom divider.
#[component]
pub fn ListItem(
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] subtitle: Option<Signal<String>>,
    #[prop(optional, into)] leading: Option<ViewFn>,
    #[prop(optional, into)] trailing: Option<ViewFn>,
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(default = 1)] title_max_lines: u32,
    #[prop(default = 2)] subtitle_max_lines: u32,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let press = Press::new();
    let clickable = on_click.is_some();
    let min_height = if leading.is_some() {
        dim::ROW_AVATAR
    } else if subtitle.is_some() {
        dim::ROW_TWO_LINE
    } else {
        dim::ROW_STANDARD
    };
    let has_subtitle = subtitle.is_some();
    let centered = subtitle.is_none() && leading.is_none();
    let bg = move || {
        if !enabled.get() {
            "disabled"
        } else if press.pressed.get() {
            "pressed"
        } else if selected.get() {
            "selected"
        } else {
            "idle"
        }
    };
    let active = move || clickable && enabled.get();

    view! {
        <div class="op-list-item">
            <div
                class="op-list-row"
                class:op-rippled=clickable
                class:op-list-centered=centered
                role=clickable.then_some("button")
                tabindex=move || active().then_some("0")
                style:min-height=du(min_height)
                data-bg=bg
                on:pointerdown=move |ev| {
                    if active() {
                        press.down();
                        spawn_ripple(&ev);
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
                on:click=move |_| {
                    if active()
                        && let Some(cb) = on_click
                    {
                        cb.run(());
                    }
                }
                on:keydown=move |ev| {
                    if active() && (ev.key() == "Enter" || ev.key() == " ") {
                        ev.prevent_default();
                        if let Some(cb) = on_click {
                            cb.run(());
                        }
                    }
                }
            >
                {leading.map(|l| view! { <div class="op-list-leading">{l.run()}</div> })}
                <div class="op-list-text">
                    <div
                        class="op-list-title"
                        data-disabled=move || (!enabled.get()).then_some("")
                        style=format!("-webkit-line-clamp: {title_max_lines}")
                    >
                        {move || title.get()}
                    </div>
                    {subtitle
                        .map(|s| {
                            view! {
                                <div class="op-list-subtitle" style=format!("-webkit-line-clamp: {subtitle_max_lines}")>
                                    {move || s.get()}
                                </div>
                            }
                        })}
                </div>
                {trailing
                    .map(|t| {
                        view! {
                            <div class="op-list-trailing" class:op-list-trailing-top=has_subtitle>
                                {t.run()}
                            </div>
                        }
                    })}
            </div>
            <div class="op-divider"></div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// StatusBadge
// ---------------------------------------------------------------------------

/// Presence and activity status shown by [`StatusBadge`] and [`Avatar`].
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

// ---------------------------------------------------------------------------
// Avatar
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Header
// ---------------------------------------------------------------------------

/// Avatar shown in a [`Header`].
#[derive(Clone, Debug)]
pub struct HeaderAvatar {
    pub id: String,
    pub name: String,
    pub group: bool,
    pub status: Option<PresenceStatus>,
}

/// Top app bar with back button, optional avatar, title, subtitle and actions.
#[component]
pub fn Header(
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] subtitle: Option<Signal<String>>,
    #[prop(optional, into)] on_back: Option<Callback<()>>,
    #[prop(optional)] avatar: Option<HeaderAvatar>,
    #[prop(optional, into)] on_title_click: Option<Callback<()>>,
    #[prop(optional, into)] actions: Option<ViewFn>,
    /// Height in DU.
    #[prop(default = 13.5)]
    height: f32,
) -> impl IntoView {
    let has_subtitle = subtitle.is_some();
    view! {
        <header class="op-header" style:height=du(height)>
            {match on_back {
                Some(cb) => view! {
                    <button
                        type="button"
                        class="op-rippled op-header-back"
                        aria-label="Back"
                        on:pointerdown=move |ev| spawn_ripple(&ev)
                        on:click=move |_| cb.run(())
                    >
                        <Icon icon=icons::ARROW_BACK size=dim::ICON_MEDIUM/>
                    </button>
                    <span class="op-header-gap-back"></span>
                }
                .into_any(),
                None => view! { <span class="op-header-start"></span> }.into_any(),
            }}
            {avatar
                .map(|a| {
                    view! {
                        <Avatar
                            id=a.id
                            name=a.name
                            kind=if a.group { AvatarKind::SimpleGroup } else { AvatarKind::Direct }
                            size=9.0
                            status=a.status.map(Signal::from).unwrap_or_else(|| Signal::stored(PresenceStatus::Offline))
                        />
                        <span class="op-header-gap-avatar"></span>
                    }
                })}
            <div
                class="op-header-titles"
                class:op-header-titles-click=on_title_click.is_some()
                on:click=move |_| {
                    if let Some(cb) = on_title_click {
                        cb.run(());
                    }
                }
            >
                <div class="op-header-title" class:op-header-title-small=has_subtitle>
                    {move || title.get()}
                </div>
                {subtitle.map(|s| view! { <div class="op-header-subtitle">{move || s.get()}</div> })}
            </div>
            {match actions {
                Some(a) => view! { <div class="op-header-actions">{a.run()}</div> }.into_any(),
                None => view! { <span class="op-header-start"></span> }.into_any(),
            }}
        </header>
    }
}

// ---------------------------------------------------------------------------
// ConsoleTerminal
// ---------------------------------------------------------------------------

fn log_tone(line: &str) -> &'static str {
    if line.contains("[CRITICAL]") || line.contains("[ERROR]") {
        "error"
    } else if line.contains("[WARNING]") {
        "warning"
    } else if line.contains("[DEBUG]") {
        "debug"
    } else if line.contains("[SUCCESS]") {
        "success"
    } else if ["[API]", "[UI]", "[TX]", "[RX]"]
        .iter()
        .any(|t| line.contains(t))
    {
        "io"
    } else {
        "plain"
    }
}

/// Embedded log terminal that auto-scrolls to the newest line.
#[component]
pub fn ConsoleTerminal(
    #[prop(into)] logs: Signal<Vec<String>>,
    /// Height in DU.
    #[prop(default = 45.0)]
    height: f32,
) -> impl IntoView {
    let node = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        let count = logs.with(Vec::len);
        if let Some(el) = node.get()
            && count > 0
        {
            el.set_scroll_top(el.scroll_height());
        }
    });
    view! {
        <div class="op-sb op-terminal" data-state="idle" role="log" style:height=du(height)>
            <ScrollView node_ref=node viewport_class="op-terminal-log" focusable=true>
            {move || {
                logs.get()
                    .into_iter()
                    .map(|line| {
                        let tone = log_tone(&line);
                        view! { <div class="op-terminal-line" data-tone=tone>{line}</div> }
                    })
                    .collect_view()
            }}
            </ScrollView>
        </div>
    }
}
