//! Linear progress bar: LinearProgress and ProgressFormat.

use crate::core::{ControlState, du};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("linear_progress.css");

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
