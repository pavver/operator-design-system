//! Linear progress bar: LinearProgress and ProgressFormat.

use crate::core::{ControlState, Metric, Slot, du, root_class, root_style, ui_text};
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
    /// Bar thickness in DU (`--op-linear-progress-thickness`).
    #[prop(optional, into)]
    thickness: MaybeProp<f32>,
    /// Accessible name; defaults to `linear-progress.label` ("Progress").
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Gap between track and value label in DU (`--op-linear-progress-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Minimum width of the value label in DU (`--op-linear-progress-value-min-width`).
    #[prop(optional, into)]
    value_min_width: MaybeProp<f32>,
    /// Custom value label slot replacing the formatted text.
    #[prop(optional, into)]
    value_slot: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_label = ui_text(label, "linear-progress.label", "Progress");
    let thickness_val = move || thickness.get().unwrap_or(1.0);
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
    let labelled = show_value && (value.is_some() || value_slot.is_set());
    let inside = labelled && thickness_val() >= PROGRESS_INSIDE_MIN_DU;
    let percent = move || ratio().map(|r| format!("{}%", r * 100.0));

    let render_val = Callback::new(move |()| {
        if value_slot.is_set() {
            value_slot.run()
        } else {
            text().map(|t| t.into_any())
        }
    });

    view! {
        <div
            class=root_class("op-progress", class)
            style=root_style(
                vec![
                    Metric::du("--op-linear-progress-thickness", thickness),
                    Metric::du("--op-linear-progress-gap", gap),
                    Metric::du("--op-linear-progress-value-min-width", value_min_width),
                ],
                style,
            )
        >
            <div
                class="op-progress-track"
                role="progressbar"
                aria-label=move || resolved_label.get()
                aria-valuemin="0"
                aria-valuemax=move || count_text(max_value())
                aria-valuenow=move || value.map(|v| count_text(v.get()))
                aria-valuetext=text
                data-tone=tone
                data-indeterminate=value.is_none().then_some("")
                style:height=move || du(thickness_val())
            >
                <div class="op-progress-fill" style:width=percent></div>
                {move || {
                    inside.then(|| {
                        let clip = move || {
                            ratio().map(|r| format!("inset(0 {}% 0 0)", (1.0 - r) * 100.0))
                        };
                        view! {
                            <span class="op-progress-inner op-t-caption" aria-hidden="true">{render_val.run(())}</span>
                            <span class="op-progress-inner op-progress-inner-on op-t-caption" aria-hidden="true" style:clip-path=clip>
                                {render_val.run(())}
                            </span>
                        }
                    })
                }}
            </div>
            {move || {
                (labelled && !inside).then(|| {
                    view! { <span class="op-progress-value op-t-caption">{render_val.run(())}</span> }
                })
            }}
        </div>
    }
}
