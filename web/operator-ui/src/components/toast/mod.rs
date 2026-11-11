//! Toast notifications: ToastHost, use_toaster, and toast types.

use crate::components::primitives::Icon;
use crate::core::{ActionResult, spawn_ripple};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};
use send_wrapper::SendWrapper;
use std::time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(crate) const CSS: &str = include_str!("toast.css");

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
