//! Feedback: Toast, Banner, LinearProgress, Skeleton, EmptyState.

use crate::basic::Icon;
use crate::core::{ActionResult, ControlState, du, spawn_ripple};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};
use send_wrapper::SendWrapper;
use std::time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Wrapped text lines at which a toast switches to the header layout.
const TALL_TOAST_LINES: f64 = 4.0;

/// Meaning of a message; maps to a signal color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Tone {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl Tone {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Short human-readable name, shown in the header of a tall toast.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Success => "Success",
            Self::Warning => "Warning",
            Self::Error => "Error",
        }
    }

    pub const fn icon(self) -> IconData {
        match self {
            Self::Info => icons::INFO,
            Self::Success => icons::CHECK_CIRCLE,
            Self::Warning => icons::WARNING,
            Self::Error => icons::ERROR,
        }
    }
}

// ---------------------------------------------------------------------------
// Toast
// ---------------------------------------------------------------------------

/// Where the toast stack sits in the viewport.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ToastPosition {
    TopStart,
    TopCenter,
    TopEnd,
    BottomStart,
    #[default]
    BottomCenter,
    BottomEnd,
}

impl ToastPosition {
    pub const ALL: [Self; 6] = [
        Self::TopStart,
        Self::TopCenter,
        Self::TopEnd,
        Self::BottomStart,
        Self::BottomCenter,
        Self::BottomEnd,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopStart => "top-start",
            Self::TopCenter => "top-center",
            Self::TopEnd => "top-end",
            Self::BottomStart => "bottom-start",
            Self::BottomCenter => "bottom-center",
            Self::BottomEnd => "bottom-end",
        }
    }

    pub const fn is_top(self) -> bool {
        matches!(self, Self::TopStart | Self::TopCenter | Self::TopEnd)
    }
}

/// Maximum number of toasts on screen; older ones are dropped first.
pub const TOAST_LIMIT: usize = 3;

/// Content of one toast.
#[derive(Clone)]
pub struct ToastSpec {
    pub tone: Tone,
    pub text: String,
    /// Optional single action, for example `("Retry", callback)`.
    pub action: Option<(String, Callback<()>)>,
    /// Visible time in milliseconds; `None` uses the tone default.
    pub duration_ms: Option<u32>,
}

impl ToastSpec {
    pub fn new(tone: Tone, text: impl Into<String>) -> Self {
        Self {
            tone,
            text: text.into(),
            action: None,
            duration_ms: None,
        }
    }

    pub fn with_action(mut self, label: impl Into<String>, on: Callback<()>) -> Self {
        self.action = Some((label.into(), on));
        self
    }
}

#[derive(Clone)]
struct ToastEntry {
    id: u64,
    spec: ToastSpec,
    hovered: RwSignal<bool>,
    expired: RwSignal<bool>,
    leaving: RwSignal<bool>,
}

/// Handle for showing toasts; provided by [`ToastHost`].
#[derive(Clone, Copy)]
pub struct Toaster {
    items: RwSignal<Vec<ToastEntry>>,
    next: StoredValue<u64>,
    position: RwSignal<ToastPosition>,
}

impl Toaster {
    /// Moves the toast stack; toasts already on screen move with it.
    pub fn set_position(&self, position: ToastPosition) {
        self.position.set(position);
    }

    /// Current position of the toast stack.
    pub fn position(&self) -> ToastPosition {
        self.position.get()
    }

    /// Shows a toast and returns its id.
    pub fn show(&self, spec: ToastSpec) -> u64 {
        self.next.update_value(|n| *n += 1);
        let id = self.next.get_value();
        let duration = spec.duration_ms.unwrap_or(if spec.tone == Tone::Error {
            motion::TOAST_ERROR_DURATION_MS
        } else {
            motion::TOAST_DURATION_MS
        });
        let entry = ToastEntry {
            id,
            spec,
            hovered: RwSignal::new(false),
            expired: RwSignal::new(false),
            leaving: RwSignal::new(false),
        };
        let (hovered, expired) = (entry.hovered, entry.expired);
        self.items.update(|v| {
            v.push(entry);
            let extra = v.len().saturating_sub(TOAST_LIMIT);
            v.drain(..extra);
        });
        let this = *self;
        set_timeout(
            move || {
                if hovered.try_get_untracked().unwrap_or(false) {
                    let _ = expired.try_set(true);
                } else {
                    this.dismiss(id);
                }
            },
            Duration::from_millis(u64::from(duration)),
        );
        id
    }

    /// Shorthand for a toast without an action.
    pub fn message(&self, tone: Tone, text: impl Into<String>) -> u64 {
        self.show(ToastSpec::new(tone, text))
    }

    /// Shows an error toast for a failed result; does nothing on success.
    pub fn report(&self, result: &ActionResult, what: &str) {
        if let ActionResult::Error(reason) = result {
            let text = match reason {
                Some(r) => format!("{what} failed: {r}"),
                None => format!("{what} failed"),
            };
            self.message(Tone::Error, text);
        }
    }

    /// Closes a toast with its exit animation.
    pub fn dismiss(&self, id: u64) {
        let items = self.items;
        let found = items.try_with_untracked(|v| v.iter().find(|e| e.id == id).map(|e| e.leaving));
        let Some(Some(leaving)) = found else {
            return;
        };
        let _ = leaving.try_set(true);
        set_timeout(
            move || {
                let _ = items.try_update(|v| v.retain(|e| e.id != id));
            },
            Duration::from_millis(u64::from(motion::DIALOG_ENTER_MS)),
        );
    }
}

/// Returns the [`Toaster`] provided by the nearest [`ToastHost`].
pub fn use_toaster() -> Option<Toaster> {
    use_context::<Toaster>()
}

/// Provides a [`Toaster`] to its children and renders the toast stack.
#[component]
pub fn ToastHost(
    /// Initial place of the stack; change it later with [`Toaster::set_position`].
    #[prop(optional)]
    position: ToastPosition,
    children: Children,
) -> impl IntoView {
    let toaster = Toaster {
        items: RwSignal::new(Vec::new()),
        next: StoredValue::new(0),
        position: RwSignal::new(position),
    };
    provide_context(toaster);
    view! {
        {children()}
        <div
            class="op-toast-stack"
            role="region"
            aria-label="Notifications"
            data-position=move || toaster.position.get().as_str()
        >
            <For each=move || toaster.items.get() key=|e| e.id let:entry>
                <ToastView entry=entry toaster=toaster/>
            </For>
        </div>
    }
}

#[component]
fn ToastView(entry: ToastEntry, toaster: Toaster) -> impl IntoView {
    let ToastEntry {
        id,
        spec,
        hovered,
        expired,
        leaving,
    } = entry;
    let tone = spec.tone;
    let role = if tone == Tone::Error {
        "alert"
    } else {
        "status"
    };

    // Count wrapped text lines; 4 or more switch the toast to the header layout.
    // The header layout gives the text more width, so it switches back only at 2 lines or fewer.
    let text_ref = NodeRef::<leptos::html::Span>::new();
    let tall = RwSignal::new(false);
    let measure = move || {
        let Some(el) = text_ref.get_untracked() else {
            return;
        };
        let Some(style) = web_sys::window().and_then(|w| w.get_computed_style(&el).ok().flatten())
        else {
            return;
        };
        let px = |name: &str| {
            style
                .get_property_value(name)
                .ok()
                .and_then(|v| v.trim_end_matches("px").parse::<f64>().ok())
                .unwrap_or(0.0)
        };
        let line = px("line-height");
        if line <= 0.0 {
            return;
        }
        let content = f64::from(el.scroll_height()) - px("padding-top") - px("padding-bottom");
        let lines = (content / line).round();
        if lines >= TALL_TOAST_LINES {
            tall.set(true);
        } else if lines <= TALL_TOAST_LINES - 2.0 {
            tall.set(false);
        }
    };
    Effect::new(move |_| {
        let Some(el) = text_ref.get() else {
            return;
        };
        let callback = Closure::<dyn FnMut()>::new(measure);
        let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) else {
            return;
        };
        observer.observe(&el);
        let keep = SendWrapper::new((observer, callback));
        on_cleanup(move || keep.0.disconnect());
    });

    view! {
        <div
            class="op-toast"
            role=role
            data-tone=tone.as_str()
            data-tall=move || tall.get().then_some("")
            data-leaving=move || leaving.get().then_some("")
            on:pointerenter=move |_| hovered.set(true)
            on:pointerleave=move |_| {
                hovered.set(false);
                if expired.get_untracked() {
                    toaster.dismiss(id);
                }
            }
        >
            <span class="op-toast-icon"><Icon icon=tone.icon() size=8.0/></span>
            <span class="op-toast-label" aria-hidden="true">{tone.label()}</span>
            <span node_ref=text_ref class="op-toast-text">{spec.text}</span>
            {spec
                .action
                .map(|(label, cb)| {
                    view! {
                        <button
                            type="button"
                            class="op-rippled op-toast-action"
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| {
                                cb.run(());
                                toaster.dismiss(id);
                            }
                        >
                            {label}
                        </button>
                    }
                })}
            <button
                type="button"
                class="op-rippled op-toast-close"
                aria-label="Dismiss"
                on:pointerdown=move |ev| spawn_ripple(&ev)
                on:click=move |_| toaster.dismiss(id)
            >
                <Icon icon=icons::CLOSE size=dim::ICON_SMALL/>
            </button>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Banner
// ---------------------------------------------------------------------------

/// Persistent inline message about the state of a screen or section.
#[component]
pub fn Banner(
    #[prop(optional)] tone: Tone,
    #[prop(optional, into)] title: Option<String>,
    #[prop(into)] text: Signal<String>,
    /// Buttons placed under the text.
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    /// Shows a close button that calls this handler.
    #[prop(optional, into)]
    on_dismiss: Option<Callback<()>>,
) -> impl IntoView {
    let role = if matches!(tone, Tone::Error | Tone::Warning) {
        "alert"
    } else {
        "status"
    };
    view! {
        <div class="op-banner" role=role data-tone=tone.as_str()>
            <span class="op-banner-icon"><Icon icon=tone.icon() size=8.0/></span>
            <div class="op-banner-body">
                {title.map(|t| view! { <div class="op-banner-title">{t}</div> })}
                <div class="op-banner-text">{move || text.get()}</div>
                {actions.map(|a| view! { <div class="op-banner-actions">{a.run()}</div> })}
            </div>
            {on_dismiss
                .map(|cb| {
                    view! {
                        <button
                            type="button"
                            class="op-rippled op-banner-close"
                            aria-label="Dismiss"
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| cb.run(())
                        >
                            <Icon icon=icons::CLOSE size=dim::ICON_SMALL/>
                        </button>
                    }
                })}
        </div>
    }
}

// ---------------------------------------------------------------------------
// LinearProgress
// ---------------------------------------------------------------------------

/// How [`LinearProgress`] writes its value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ProgressFormat {
    /// `54%`.
    #[default]
    Percent,
    /// `543 / 1000`, using `value` and `max`.
    Count,
}

/// Bar thickness in DU from which the value label moves inside the bar.
pub const PROGRESS_INSIDE_MIN_DU: f32 = 5.0;

/// Formats a count without trailing decimals for whole numbers.
fn count_text(v: f32) -> String {
    if (v - v.round()).abs() < 0.005 {
        format!("{v:.0}")
    } else {
        format!("{v:.2}")
    }
}

/// Horizontal progress bar; indeterminate when `value` is not given.
#[component]
pub fn LinearProgress(
    /// Progress from `0.0` to `max`.
    #[prop(optional, into)]
    value: Option<Signal<f32>>,
    /// Value of a finished operation, for example a byte or item count.
    #[prop(default = 1.0.into(), into)]
    max: Signal<f32>,
    /// `Success` or `Error` recolors the bar for the final result.
    #[prop(optional, into)]
    state: Signal<Option<ControlState>>,
    /// Shows the value; inside the bar when it is at least [`PROGRESS_INSIDE_MIN_DU`] thick.
    #[prop(optional)]
    show_value: bool,
    /// Percent or `value / max`.
    #[prop(optional)]
    format: ProgressFormat,
    /// Bar thickness in DU.
    #[prop(default = 1.0)]
    thickness: f32,
    #[prop(default = "Progress".into(), into)] label: String,
) -> impl IntoView {
    let max_value = move || {
        let m = max.get();
        if m > 0.0 { m } else { 1.0 }
    };
    let ratio = move || value.map(|v| (v.get() / max_value()).clamp(0.0, 1.0));
    let text = move || {
        let v = value?.get();
        Some(match format {
            ProgressFormat::Percent => format!("{:.0}%", (v / max_value()).clamp(0.0, 1.0) * 100.0),
            ProgressFormat::Count => {
                format!(
                    "{} / {}",
                    count_text(v.clamp(0.0, max_value())),
                    count_text(max_value())
                )
            }
        })
    };
    let tone = move || match state.get() {
        Some(ControlState::Success) => "success",
        Some(ControlState::Error) => "error",
        _ => "pending",
    };
    let labelled = show_value && value.is_some();
    let inside = labelled && thickness >= PROGRESS_INSIDE_MIN_DU;
    let percent = move || ratio().map(|r| format!("{}%", r * 100.0));
    view! {
        <div class="op-progress">
            <div
                class="op-progress-track"
                role="progressbar"
                aria-label=label
                aria-valuemin="0"
                aria-valuemax=move || count_text(max_value())
                aria-valuenow=move || value.map(|v| count_text(v.get()))
                aria-valuetext=text
                data-tone=tone
                data-indeterminate=value.is_none().then_some("")
                style:height=du(thickness)
            >
                <div class="op-progress-fill" style:width=percent></div>
                {inside
                    .then(|| {
                        // Two copies of the label: one over the track, one over the fill,
                        // clipped to the filled part so the text keeps contrast on both.
                        let clip = move || {
                            ratio().map(|r| format!("inset(0 {}% 0 0)", (1.0 - r) * 100.0))
                        };
                        view! {
                            <span class="op-progress-inner op-t-caption" aria-hidden="true">{text}</span>
                            <span class="op-progress-inner op-progress-inner-on op-t-caption" aria-hidden="true" style:clip-path=clip>
                                {text}
                            </span>
                        }
                    })}
            </div>
            {(labelled && !inside)
                .then(|| view! { <span class="op-progress-value op-t-caption">{text}</span> })}
        </div>
    }
}

// ---------------------------------------------------------------------------
// Skeleton
// ---------------------------------------------------------------------------

/// Placeholder that holds the place of content while it loads.
#[component]
pub fn Skeleton(
    /// Width in DU; full width when omitted.
    #[prop(optional)]
    width: Option<f32>,
    /// Height in DU; one text line when omitted.
    #[prop(optional)]
    height: Option<f32>,
    /// Number of text lines; the last line is shorter.
    #[prop(default = 1)]
    lines: u32,
    /// Circle for avatar placeholders.
    #[prop(optional)]
    circle: bool,
) -> impl IntoView {
    let count = lines.max(1);
    let bars = (0..count)
        .map(|i| {
            let last = count > 1 && i + 1 == count;
            view! {
                <span
                    class="op-skeleton"
                    data-circle=circle.then_some("")
                    data-text=height.is_none().then_some("")
                    style:width=if last { Some("60%".to_string()) } else { width.map(du) }
                    style:height=height.map(du)
                ></span>
            }
        })
        .collect_view();
    view! {
        <span class="op-skeleton-group" aria-busy="true" aria-label="Loading">
            {bars}
        </span>
    }
}

// ---------------------------------------------------------------------------
// EmptyState
// ---------------------------------------------------------------------------

/// Message shown in place of a list or view that has no content.
#[component]
pub fn EmptyState(
    #[prop(into)] title: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(default = icons::INBOX)] icon: IconData,
    /// Buttons under the text.
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    /// Smaller variant for panels and table bodies.
    #[prop(optional)]
    compact: bool,
) -> impl IntoView {
    let icon_size = if compact { 8.0 } else { 12.0 };
    view! {
        <div class="op-empty" data-compact=compact.then_some("")>
            <span class="op-empty-icon"><Icon icon=icon size=icon_size/></span>
            <div class="op-empty-title">{title}</div>
            {description.map(|d| view! { <div class="op-empty-text">{d}</div> })}
            {actions.map(|a| view! { <div class="op-empty-actions">{a.run()}</div> })}
        </div>
    }
}
