//! Toast notifications: ToastHost, use_toaster, and toast types.

use crate::components::primitives::Icon;
use crate::core::{
    ActionResult, Metric, Slot, root_class, root_style, spawn_ripple, ui_icon, ui_text,
};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons, motion};
use send_wrapper::SendWrapper;
use std::time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(crate) const CSS: &str = include_str!("toast.css");

mod types;
use types::ToastEntry;
pub use types::*;
mod visual;
pub use visual::{Toast, ToastProps};

/// Handle for showing toasts; provided by [`ToastHost`].
#[derive(Clone, Copy)]
pub struct Toaster {
    items: RwSignal<Vec<ToastEntry>>,
    next: StoredValue<u64>,
    position: RwSignal<ToastPosition>,
    failed: Signal<String>,
    failed_reason: Signal<String>,
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
                Some(r) => self
                    .failed_reason
                    .get_untracked()
                    .replace("{what}", what)
                    .replace("{reason}", r),
                None => self.failed.get_untracked().replace("{what}", what),
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
    #[prop(optional, into)] shift: MaybeProp<f32>,
    #[prop(optional, into)] viewport_margin: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] min_height: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] padding_start: MaybeProp<f32>,
    #[prop(optional, into)] shadow_blur: MaybeProp<f32>,
    #[prop(optional, into)] label_font_size: MaybeProp<f32>,
    #[prop(optional, into)] label_line_height: MaybeProp<f32>,
    #[prop(optional, into)] text_padding_end: MaybeProp<f32>,
    #[prop(optional, into)] button_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] close_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] letter_spacing: MaybeProp<f32>,
    #[prop(optional, into)] padding_end: MaybeProp<f32>,
    #[prop(optional, into)] shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] failed_text: MaybeProp<String>,
    #[prop(optional, into)] failed_reason_text: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] tone_label: MaybeProp<String>,
    #[prop(optional, into)] dismiss_label: MaybeProp<String>,
    #[prop(optional_no_strip, into)] icon: Option<IconData>,
    #[prop(optional_no_strip, into)] dismiss_icon: Option<IconData>,
    children: Children,
) -> impl IntoView {
    let label = ui_text(label, "toast.notifications", "Notifications");
    let failed = ui_text(failed_text, "toast.failed", "{what} failed");
    let failed_reason = ui_text(
        failed_reason_text,
        "toast.failed-reason",
        "{what} failed: {reason}",
    );
    let slots = StoredValue::new((header, actions, footer));
    let toaster = Toaster {
        items: RwSignal::new(Vec::new()),
        next: StoredValue::new(0),
        position: RwSignal::new(position),
        failed,
        failed_reason,
    };
    provide_context(toaster);
    view! {
        {children()}
        <div
            class=root_class("op-toast-stack", class)
            style=root_style(vec![Metric::du("--op-toast-shadow-y", shadow_y),Metric::du("--op-toast-padding-end", padding_end),Metric::du("--op-toast-shift", shift),Metric::du("--op-toast-viewport-margin", viewport_margin),Metric::du("--op-toast-gap", gap),Metric::du("--op-toast-width", width),Metric::du("--op-toast-min-height", min_height),Metric::du("--op-toast-padding-y", padding_y),Metric::du("--op-toast-padding-start", padding_start),Metric::du("--op-toast-shadow-blur", shadow_blur),Metric::tu("--op-toast-label-font-size", label_font_size),Metric::tu("--op-toast-label-line-height", label_line_height),Metric::du("--op-toast-text-padding-end", text_padding_end),Metric::du("--op-toast-button-size", button_size),Metric::tu("--op-toast-font-size", font_size),Metric::tu("--op-toast-line-height", line_height),Metric::du("--op-toast-icon-size", icon_size),Metric::du("--op-toast-close-icon-size", close_icon_size),Metric::tu("--op-toast-letter-spacing", letter_spacing)], style)
            role="region"
            aria-label=move || label.get()
            data-position=move || toaster.position.get().as_str()
        >
            <For each=move || toaster.items.get() key=|e| e.id let:entry>
                <ToastView entry=entry toaster=toaster slots=slots tone_label=tone_label dismiss_label=dismiss_label icon=icon dismiss_icon=dismiss_icon/>
            </For>
        </div>
    }
}

#[component]
fn ToastView(
    entry: ToastEntry,
    toaster: Toaster,
    slots: StoredValue<(Slot, Slot, Slot)>,
    tone_label: MaybeProp<String>,
    dismiss_label: MaybeProp<String>,
    icon: Option<IconData>,
    dismiss_icon: Option<IconData>,
) -> impl IntoView {
    let ToastEntry {
        id,
        spec,
        hovered,
        expired,
        leaving,
    } = entry;
    let (header, actions, footer) = slots.get_value();
    view! {
        <Toast spec=spec leaving=leaving header=header actions=actions footer=footer
            tone_label=tone_label dismiss_label=dismiss_label icon=icon dismiss_icon=dismiss_icon
            on_dismiss=Callback::new(move |()| toaster.dismiss(id))
            on_hover=Callback::new(move |is_hovered| {
                hovered.set(is_hovered);
                if !is_hovered && expired.get_untracked() { toaster.dismiss(id); }
            })/>
    }
}
