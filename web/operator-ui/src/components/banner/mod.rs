//! Persistent inline message: Banner.

use crate::components::primitives::Icon;
pub use crate::components::toast::Tone;
use crate::core::spawn_ripple;
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};

pub(crate) const CSS: &str = include_str!("banner.css");

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
