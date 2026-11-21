//! Responsive image with ratio, fit, skeleton loading and error handling: Image.

use crate::components::primitives::Icon;
use crate::components::skeleton::Skeleton;
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("image.css");

/// How the image is fitted into its container.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ImageFit {
    #[default]
    Cover,
    Contain,
    Fill,
    None,
    ScaleDown,
}

impl ImageFit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cover => "cover",
            Self::Contain => "contain",
            Self::Fill => "fill",
            Self::None => "none",
            Self::ScaleDown => "scale-down",
        }
    }
}

/// Image component with ratio, fit, loading placeholder, error fallback, and optional caption.
#[component]
pub fn Image(
    /// Image source URL.
    #[prop(into)]
    src: Signal<String>,
    /// Accessible image description.
    #[prop(optional, into)]
    alt: MaybeProp<String>,
    /// Aspect ratio (width / height).
    #[prop(optional, into)]
    ratio: MaybeProp<f32>,
    /// How the image fits its box.
    #[prop(default = ImageFit::Cover)]
    fit: ImageFit,
    /// Object position (e.g. `"center"`).
    #[prop(optional, into)]
    position: MaybeProp<String>,
    /// Browser loading strategy: `"lazy"` or `"eager"`.
    #[prop(optional, into)]
    loading: MaybeProp<String>,
    /// Caption text overlay.
    #[prop(optional, into)]
    caption: MaybeProp<String>,
    /// Custom caption slot.
    #[prop(optional, into)]
    caption_slot: Slot,
    /// Custom loading slot replacing the default skeleton.
    #[prop(optional, into)]
    loading_slot: Slot,
    /// Custom error slot replacing the default error state.
    #[prop(optional, into)]
    error_slot: Slot,
    /// Accessible error message; defaults to `"Failed to load image"`.
    #[prop(optional, into)]
    error_label: MaybeProp<String>,
    /// Width in DU (`--op-image-width`).
    #[prop(optional, into)]
    width: MaybeProp<f32>,
    /// Height in DU (`--op-image-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Callback when image successfully loads.
    #[prop(optional)]
    on_load: Option<Callback<()>>,
    /// Callback when image fails to load.
    #[prop(optional)]
    on_error: Option<Callback<()>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_alt = ui_text(alt, "image.alt", "");
    let resolved_error = ui_text(error_label, "image.error", "Failed to load image");
    let is_loading = RwSignal::new(true);
    let has_error = RwSignal::new(false);

    let base_style = root_style(
        vec![
            Metric::du("--op-image-width", width),
            Metric::du("--op-image-height", height),
        ],
        style,
    );
    let full_style = move || {
        let mut out = String::new();
        if let Some(r) = ratio.get() {
            out.push_str(&format!("aspect-ratio:{r};"));
        }
        if let Some(pos) = position.get() {
            out.push_str(&format!("--op-image-position:{pos};"));
        }
        if let Some(rest) = base_style() {
            out.push_str(&rest);
        }
        (!out.is_empty()).then_some(out)
    };

    let img_src = src;
    Effect::new(move |_| {
        let _ = img_src.get();
        is_loading.set(true);
        has_error.set(false);
    });

    view! {
        <div
            class=root_class("op-image", class)
            style=full_style
            data-loading=move || is_loading.get().then_some("")
            data-error=move || has_error.get().then_some("")
        >
            <img
                class="op-image-img"
                src=move || src.get()
                alt=move || resolved_alt.get()
                loading=move || loading.get().unwrap_or_else(|| "lazy".to_string())
                style=format!("object-fit:{};", fit.as_str())
                on:load=move |_| {
                    is_loading.set(false);
                    if let Some(cb) = on_load {
                        cb.run(());
                    }
                }
                on:error=move |_| {
                    is_loading.set(false);
                    has_error.set(true);
                    if let Some(cb) = on_error {
                        cb.run(());
                    }
                }
            />
            {move || is_loading.get().then(|| {
                if loading_slot.is_set() {
                    view! {
                        <div class="op-image-placeholder">
                            {loading_slot.run()}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="op-image-placeholder">
                            <Skeleton
                                lines=1
                                height=height.get().unwrap_or(20.0)
                            />
                        </div>
                    }.into_any()
                }
            })}
            {move || has_error.get().then(|| {
                if error_slot.is_set() {
                    view! {
                        <div class="op-image-error">
                            {error_slot.run()}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="op-image-error">
                            <Icon
                                icon=icons::ERROR
                                size=6.0
                            />
                            <span>{resolved_error.get()}</span>
                        </div>
                    }.into_any()
                }
            })}
            {move || (caption_slot.is_set() || caption.get().is_some()).then(|| view! {
                <div class="op-image-caption">
                    {if caption_slot.is_set() {
                        caption_slot.run()
                    } else {
                        caption.get().map(|c| c.into_any())
                    }}
                </div>
            })}
        </div>
    }
}
