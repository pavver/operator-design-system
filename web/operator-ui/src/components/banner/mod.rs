//! Persistent inline message: Banner.

use crate::components::primitives::Icon;
pub use crate::components::toast::Tone;
use crate::core::{Metric, Slot, root_class, root_style, spawn_ripple, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

pub(crate) const CSS: &str = include_str!("banner.css");

/// Persistent inline message about the state of a screen or section.
#[component]
pub fn Banner(
    #[prop(optional)] tone: Tone,
    #[prop(optional, into)] title: Option<String>,
    #[prop(into)] text: Signal<String>,
    /// Buttons placed under the text.
    #[prop(optional, into)]
    actions: Slot,
    /// Shows a close button that calls this handler.
    #[prop(optional, into)]
    on_dismiss: Option<Callback<()>>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] body_gap: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] button_size: MaybeProp<f32>,
    #[prop(optional, into)] close_outset: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] close_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] dismiss_label: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional)] dismiss_icon: Option<IconData>,
) -> impl IntoView {
    let dismiss_label = ui_text(dismiss_label, "banner.dismiss", "Dismiss");
    let dismiss_icon = ui_icon(dismiss_icon, "banner.dismiss", icons::CLOSE);
    let icon = ui_icon(
        icon,
        match tone {
            Tone::Info => "banner.info",
            Tone::Success => "banner.success",
            Tone::Warning => "banner.warning",
            Tone::Error => "banner.error",
        },
        tone.icon(),
    );
    let role = if matches!(tone, Tone::Error | Tone::Warning) {
        "alert"
    } else {
        "status"
    };
    view! {
        <div class=root_class("op-banner", class) style=root_style(vec![Metric::du("--op-banner-width", width),Metric::du("--op-banner-gap", gap),Metric::du("--op-banner-padding", padding),Metric::du("--op-banner-body-gap", body_gap),Metric::du("--op-banner-icon-size", icon_size),Metric::tu("--op-banner-line-height", line_height),Metric::du("--op-banner-button-size", button_size),Metric::du("--op-banner-close-outset", close_outset),Metric::tu("--op-banner-font-size", font_size),Metric::du("--op-banner-close-icon-size", close_icon_size)], style) role=role data-tone=tone.as_str()>
            <span class="op-banner-icon">{move || view! { <Icon icon=icon.get()/> }}</span>
            <div class="op-banner-body">
                {if header.is_set() { header.run().into_any() } else { title.map(|t| view! { <div class="op-banner-title">{t}</div> }).into_any() }}
                <div class="op-banner-text">{move || text.get()}</div>
                {actions.is_set().then(|| view! { <div class="op-banner-actions">{actions.run()}</div> })}
                {footer.run()}
            </div>
            {on_dismiss
                .map(|cb| {
                    view! {
                        <button
                            type="button"
                            class="op-rippled op-banner-close"
                            aria-label=move || dismiss_label.get()
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| cb.run(())
                        >
                            {move || view! { <Icon icon=dismiss_icon.get()/> }}
                        </button>
                    }
                })}
        </div>
    }
}
