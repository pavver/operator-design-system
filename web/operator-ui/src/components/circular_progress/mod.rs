//! Circular progress indicator: CircularProgress.

use crate::core::{ControlState, Metric, Slot, du, root_class, root_style, ui_text};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("circular_progress.css");

/// How [`CircularProgress`] formats its numeric center label.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CircularProgressFormat {
    /// Percentage e.g. `75%`.
    #[default]
    Percent,
    /// Absolute value and maximum e.g. `75 / 100`.
    Count,
}

/// Circular progress ring; indeterminate when `value` is omitted.
#[component]
pub fn CircularProgress(
    /// Progress value from `0.0` to `max`. When `None`, displays an indeterminate spinner.
    #[prop(optional, into)]
    value: Option<Signal<f32>>,
    /// Maximum progress value; defaults to `100.0`.
    #[prop(default = 100.0.into(), into)]
    max: Signal<f32>,
    /// External control state tinting the ring: `Pending`, `Success`, `Error`.
    #[prop(optional, into)]
    state: Signal<Option<ControlState>>,
    /// Shows the numeric progress value in the center.
    #[prop(optional)]
    show_value: bool,
    /// Format of the center value.
    #[prop(optional)]
    format: CircularProgressFormat,
    /// Diameter of the ring in DU (`--op-circular-progress-size`); defaults to `10 DU`.
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    /// Stroke thickness of the ring in DU (`--op-circular-progress-thickness`); defaults to `0.75 DU`.
    #[prop(optional, into)]
    thickness: MaybeProp<f32>,
    /// Font size for the center label in TU (`--op-circular-progress-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    /// Accessible name; defaults to `"Progress"`.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Custom slot rendered in the center of the ring.
    #[prop(optional, into)]
    center_slot: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_label = ui_text(label, "circular-progress.label", "Progress");
    let size_val = move || size.get().unwrap_or(10.0);
    let thickness_val = move || thickness.get().unwrap_or(0.75);

    let max_val = move || {
        let m = max.get();
        if m > 0.0 { m } else { 100.0 }
    };
    let ratio = move || value.map(|v| (v.get() / max_val()).clamp(0.0, 1.0));

    let text = move || {
        let v = value?.get();
        let m = max_val();
        Some(match format {
            CircularProgressFormat::Percent => format!("{:.0}%", (v / m).clamp(0.0, 1.0) * 100.0),
            CircularProgressFormat::Count => {
                if (v - v.round()).abs() < 0.005 && (m - m.round()).abs() < 0.005 {
                    format!("{:.0} / {:.0}", v.clamp(0.0, m), m)
                } else {
                    format!("{:.1} / {:.1}", v.clamp(0.0, m), m)
                }
            }
        })
    };

    let tone = move || match state.get() {
        Some(ControlState::Success) => "success",
        Some(ControlState::Error) => "error",
        _ => "pending",
    };

    let base_style = root_style(
        vec![
            Metric::du("--op-circular-progress-size", size),
            Metric::du("--op-circular-progress-thickness", thickness),
            Metric::tu("--op-circular-progress-font-size", font_size),
        ],
        style,
    );

    let full_style = move || {
        let rest = base_style().unwrap_or_default();
        let s = du(size_val());
        Some(format!("width:{s};height:{s};{rest}"))
    };

    let stroke_pct = move || (thickness_val() / size_val() * 100.0).clamp(2.0, 40.0);
    let r = move || (50.0 - stroke_pct() / 2.0).max(1.0);
    let c = move || 2.0 * std::f32::consts::PI * r();

    let offset = move || {
        let frac = ratio().unwrap_or(0.0);
        c() * (1.0 - frac)
    };

    let is_determinate = value.is_some();

    view! {
        <div
            class=root_class("op-circular-progress", class)
            role="progressbar"
            aria-label=move || resolved_label.get()
            aria-valuenow=move || value.map(|v| v.get())
            aria-valuemin=0.0
            aria-valuemax=max_val
            data-indeterminate=(!is_determinate).then_some("")
            data-tone=tone
            style=full_style
        >
            <svg
                class="op-circular-progress-svg"
                viewBox="0 0 100 100"
            >
                <circle
                    class="op-circular-progress-track"
                    cx="50"
                    cy="50"
                    r=r
                    stroke-width=stroke_pct
                />
                <circle
                    class="op-circular-progress-circle"
                    cx="50"
                    cy="50"
                    r=r
                    stroke-width=stroke_pct
                    stroke-dasharray=move || format!("{:.2}", c())
                    stroke-dashoffset=move || if is_determinate { format!("{:.2}", offset()) } else { format!("{:.2}", c() * 0.25) }
                />
            </svg>
            {(show_value || center_slot.is_set()).then(|| view! {
                <div class="op-circular-progress-center">
                    {if center_slot.is_set() {
                        center_slot.run()
                    } else {
                        text().map(|t| t.into_any())
                    }}
                </div>
            })}
        </div>
    }
}
