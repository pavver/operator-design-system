//! HoverCard: rich floating card for details of an element, opened by hover, focus or a press.

use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::core::{JobToken, Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;
use std::time::Duration;

pub(crate) const CSS: &str = include_str!("hover_card.css");

#[allow(clippy::module_inception)]
pub mod hover_card {
    /// Hover time before the card opens, in milliseconds.
    pub const SHOW_DELAY_MS: u32 = operator_ui_tokens::motion::TOOLTIP_SHOW_DELAY_MS;
    /// Grace time after the pointer leaves the anchor and the card, in milliseconds.
    pub const HIDE_DELAY_MS: u32 = 200;
    /// Distance from the anchor, in DU.
    pub const GAP_DU: f32 = 1.5;
    /// Default card width, in DU.
    pub const WIDTH_DU: f32 = 80.0;
    /// Maximum card height before it scrolls, in DU.
    pub const MAX_HEIGHT_DU: f32 = 100.0;
}

/// Card with details of its anchor: a description, help, a preview.
///
/// A mouse opens it after hovering `delay_ms`; it stays open while the pointer is over the anchor or
/// the card, so links and buttons inside it can be used. A press on the anchor (touch, pen, or a
/// click) toggles it, and keyboard focus opens it. Escape and a press outside close it.
#[component]
pub fn HoverCard(
    /// Card content.
    #[prop(into)]
    card: ViewFn,
    #[prop(default = hover_card::SHOW_DELAY_MS)] delay_ms: u32,
    /// Card width in DU.
    #[prop(optional, into)]
    width: MaybeProp<f32>,
    #[prop(optional)] placement: Placement,
    /// Accessible name of the card.
    #[prop(optional, into)]
    label: Option<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    /// Opens the card on keyboard focus of the anchor.
    #[prop(default = true)]
    open_on_focus: bool,
    #[prop(optional, into)] shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] shadow_blur: MaybeProp<f32>,
    #[prop(optional, into)] viewport_margin: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] offset: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] max_width: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
    /// The anchor.
    children: Children,
) -> impl IntoView {
    let legacy_label = label.clone();
    let label = ui_text(
        Signal::derive(move || aria_label.get().or_else(|| legacy_label.clone())).into(),
        "hover-card.label",
        "",
    );
    let header = StoredValue::new(header);
    let actions = StoredValue::new(actions);
    let footer = StoredValue::new(footer);
    let layer_class = Signal::derive(root_class("op-hover-card", class));
    let layer_style = Signal::derive(root_style(
        vec![
            Metric::du("--op-hover-card-max-width", max_width),
            Metric::du("--op-hover-card-shadow-y", shadow_y),
            Metric::du("--op-hover-card-shadow-blur", shadow_blur),
            Metric::du("--op-hover-card-viewport-margin", viewport_margin),
            Metric::du("--op-hover-card-padding", padding),
            Metric::du("--op-hover-card-gap", gap),
            Metric::du("--op-hover-card-width", width),
            Metric::du("--op-hover-card-max-height", max_height),
            Metric::du("--op-hover-card-offset", offset),
            Metric::tu("--op-hover-card-font-size", font_size),
            Metric::tu("--op-hover-card-line-height", line_height),
        ],
        style,
    ));
    let host = NodeRef::<leptos::html::Span>::new();
    let open = RwSignal::new(false);
    let anchor = RwSignal::new(PopoverAnchor::Point { x: 0.0, y: 0.0 });
    let job = JobToken::new();

    let show = move || {
        job.next();
        if let Some(h) = host.try_get_untracked().flatten() {
            anchor.set(PopoverAnchor::of(&h));
        }
        open.set(true);
    };
    let show_later = move || {
        let id = job.next();
        set_timeout(
            move || {
                if job.is_current(id) {
                    show();
                }
            },
            Duration::from_millis(u64::from(delay_ms)),
        );
    };
    let hide_later = move || {
        let id = job.next();
        set_timeout(
            move || {
                if job.is_current(id) {
                    open.set(false);
                }
            },
            Duration::from_millis(u64::from(hover_card::HIDE_DELAY_MS)),
        );
    };
    // Entering the card cancels a pending hide.
    let keep = move || {
        job.next();
    };

    let card = StoredValue::new(card);
    view! {
        <span
            node_ref=host
            class="op-hover-card-anchor"
            aria-expanded=move || open.get().to_string()
            on:pointerenter=move |e| {
                if e.pointer_type() == "mouse" {
                    if open.get_untracked() { keep() } else { show_later() }
                }
            }
            on:pointerleave=move |e| {
                if e.pointer_type() == "mouse" {
                    hide_later();
                }
            }
            on:click=move |_| {
                if open.get_untracked() {
                    job.next();
                    open.set(false);
                } else {
                    show();
                }
            }
            on:focusin=move |_| {
                if open_on_focus {
                    show();
                }
            }
        >
            {children()}
        </span>
        <Popover dismiss_on_scroll=true
            open=open
            anchor=anchor
            placement=placement
            restore_focus=false
            class=layer_class
            style=layer_style
            role="dialog"
            aria_label=label
        >
            <div
                class="op-hover-card-body"
                on:pointerenter=move |_| keep()
                on:pointerleave=move |e| {
                    if e.pointer_type() == "mouse" {
                        hide_later();
                    }
                }
            >
                {header.with_value(Slot::run)}
                {card.with_value(ViewFn::run)}
                {actions.with_value(Slot::run)}
                {footer.with_value(Slot::run)}
            </div>
        </Popover>
    }
}
