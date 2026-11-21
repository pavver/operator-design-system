//! Compact number badges: Counter, WithCounter, CounterTone.

use crate::core::{Metric, root_class, root_style, ui_text};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("counter.css");

/// Color of a [`Counter`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CounterTone {
    /// Unread or new items that need attention.
    #[default]
    Accent,
    /// Plain totals.
    Neutral,
    /// Failures or alarms.
    Error,
}

/// Compact number badge; values above `max` show as `max+`.
#[component]
pub fn Counter(
    #[prop(into)] count: Signal<u32>,
    #[prop(default = 99)] max: u32,
    #[prop(optional)] tone: CounterTone,
    /// Keeps the badge visible at zero.
    #[prop(optional)]
    show_zero: bool,
    /// Accessible name; defaults to "`n` items".
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// Minimum badge width in DU (`--op-counter-min-width`).
    #[prop(optional, into)]
    min_width: MaybeProp<f32>,
    /// Badge height in DU (`--op-counter-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Horizontal badge padding in DU (`--op-counter-padding-x`).
    #[prop(optional, into)]
    padding_x: MaybeProp<f32>,
    /// Font size in TU (`--op-counter-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let tone = match tone {
        CounterTone::Accent => "accent",
        CounterTone::Neutral => "neutral",
        CounterTone::Error => "error",
    };
    let items_text = ui_text(None::<String>.into(), "counter.items", "items");
    let accessible_name = Signal::derive(move || {
        aria_label
            .get()
            .unwrap_or_else(|| format!("{} {}", count.get(), items_text.get()))
    });
    let text = move || {
        let c = count.get();
        if c > max {
            format!("{max}+")
        } else {
            c.to_string()
        }
    };
    let visible = move || show_zero || count.get() > 0;
    view! {
        <Show when=visible>
            <span
                class=root_class("op-counter", class)
                style=root_style(
                    vec![
                        Metric::du("--op-counter-min-width", min_width),
                        Metric::du("--op-counter-height", height),
                        Metric::du("--op-counter-padding-x", padding_x),
                        Metric::tu("--op-counter-font-size", font_size),
                    ],
                    style,
                )
                data-tone=tone
                aria-label=move || accessible_name.get()
            >
                {text}
            </span>
        </Show>
    }
}

/// Places a [`Counter`] on the top end corner of its child, for example an icon button.
#[component]
pub fn WithCounter(
    #[prop(into)] count: Signal<u32>,
    #[prop(default = 99)] max: u32,
    #[prop(optional)] tone: CounterTone,
    #[prop(optional)] show_zero: bool,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    /// Badge offset from corner in DU (`--op-counter-offset`).
    #[prop(optional, into)]
    offset: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            class=root_class("op-with-counter", class)
            style=root_style(vec![Metric::du("--op-counter-offset", offset)], style)
        >
            {children()}
            <span class="op-with-counter-badge">
                <Counter count=count max=max tone=tone show_zero=show_zero aria_label=aria_label/>
            </span>
        </span>
    }
}
