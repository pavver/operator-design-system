//! Compact number badges: Counter, WithCounter, CounterTone.

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
) -> impl IntoView {
    let tone = match tone {
        CounterTone::Accent => "accent",
        CounterTone::Neutral => "neutral",
        CounterTone::Error => "error",
    };
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
            <span class="op-counter" data-tone=tone aria-label=move || format!("{} items", count.get())>
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
    children: Children,
) -> impl IntoView {
    view! {
        <span class="op-with-counter">
            {children()}
            <span class="op-with-counter-badge">
                <Counter count=count max=max tone=tone/>
            </span>
        </span>
    }
}
