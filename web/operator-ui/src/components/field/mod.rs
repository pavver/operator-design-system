//! Field shell shared by every text-like input: Field.
//!
//! It draws the label, the state-bordered box with its slots, and the hint or error line, so an
//! application can wrap its own control and look like a native field.

use crate::core::{
    ControlState, Metric, PendingRun, Slot, StateVisual, root_class, root_style, use_state_visual,
};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("field.css");

/// Look of the field box.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldVariant {
    /// Surface background with the state outline (the default).
    #[default]
    Outlined,
    /// No background and no base outline until focused or in a result state; for fields inside
    /// table cells, property rows and node rows.
    Borderless,
}

impl FieldVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Outlined => "outlined",
            Self::Borderless => "borderless",
        }
    }
}

/// Field metrics in DU (TU for `font_size`); unset values follow the `--op-field-*` variables.
#[derive(Clone, Copy, Default)]
pub struct FieldMetrics {
    pub height: MaybeProp<f32>,
    pub padding_x: MaybeProp<f32>,
    pub slot_size: MaybeProp<f32>,
    pub gap: MaybeProp<f32>,
    pub font_size: MaybeProp<f32>,
}

impl FieldMetrics {
    pub(crate) fn vars(self) -> Vec<Metric> {
        vec![
            Metric::du("--op-field-height", self.height),
            Metric::du("--op-field-padding-x", self.padding_x),
            Metric::du("--op-field-slot-size", self.slot_size),
            Metric::du("--op-field-gap", self.gap),
            Metric::tu("--op-field-font-size", self.font_size),
        ]
    }
}

/// Field shell: optional label above, `before` | box (`prepend` | control | `append`) | `after`,
/// and a hint or error line below. The control itself is `children`.
#[component]
pub fn Field(
    /// The control placed inside the box, usually an `<input class="op-field-input">`.
    children: Children,
    /// State shown by the box outline (focus, results, disabled).
    #[prop(into)]
    state: Signal<ControlState>,
    /// Text label above the box.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Helper text below the box.
    #[prop(optional, into)]
    hint: MaybeProp<String>,
    /// Error text below the box; replaces the hint and uses the error tone.
    #[prop(optional, into)]
    error: MaybeProp<String>,
    #[prop(optional, into)] variant: Signal<FieldVariant>,
    /// Content inside the box before the control (icon, prefix).
    #[prop(optional, into)]
    prepend: Slot,
    /// Content inside the box after the control (unit, buttons).
    #[prop(optional, into)]
    append: Slot,
    /// Content outside the box, before it.
    #[prop(optional, into)]
    before: Slot,
    /// Content outside the box, after it.
    #[prop(optional, into)]
    after: Slot,
    /// Custom label content; replaces `label`.
    #[prop(optional, into)]
    label_slot: Slot,
    /// Custom content below the box; replaces `hint` and `error`.
    #[prop(optional, into)]
    hint_slot: Slot,
    /// Extra class of the box, next to `op-field` (component classes such as `op-search`).
    #[prop(optional)]
    box_class: &'static str,
    /// Whether the box ends with a full-height trailing button (no end padding).
    #[prop(optional, into)]
    trailing: Signal<bool>,
    /// Draws the pending run segment on the box outline.
    #[prop(optional)]
    pending_run: bool,
    /// ARIA role of the box (for example `"search"`).
    #[prop(optional)]
    role: Option<&'static str>,
    #[prop(optional)] metrics: FieldMetrics,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let visual: StateVisual = use_state_visual(state);
    let box_classes = if box_class.is_empty() {
        "op-sb op-field".to_string()
    } else {
        format!("op-sb op-field {box_class}")
    };
    let label_view = move || {
        if label_slot.is_set() {
            Some(view! { <div class="op-field-label">{label_slot.run()}</div> }.into_any())
        } else {
            label
                .get()
                .map(|text| view! { <div class="op-field-label">{text}</div> }.into_any())
        }
    };
    let bottom = move || {
        if hint_slot.is_set() {
            return Some(view! { <div class="op-field-bottom">{hint_slot.run()}</div> }.into_any());
        }
        if let Some(text) = error.get() {
            return Some(
                view! { <div class="op-field-bottom op-field-error" role="alert">{text}</div> }
                    .into_any(),
            );
        }
        hint.get().map(|text| {
            view! { <div class="op-field-bottom op-field-hint">{text}</div> }.into_any()
        })
    };
    view! {
        <div
            class=root_class("op-field-root", class)
            style=root_style(metrics.vars(), style)
            data-variant=move || variant.get().as_str()
        >
            {label_view}
            <div class="op-field-row">
                {before.is_set().then(|| view! { <div class="op-field-before">{before.run()}</div> })}
                <div
                    class=box_classes
                    role=role
                    data-state=move || visual.state.get()
                    data-result=move || visual.result.get()
                    data-fading=move || visual.fading.get().then_some("")
                    data-trailing=move || trailing.get().then_some("")
                    data-error=move || error.with(Option::is_some).then_some("")
                >
                    {prepend.is_set().then(|| view! { <span class="op-field-prepend">{prepend.run()}</span> })}
                    {children()}
                    {append.is_set().then(|| view! { <span class="op-field-append">{append.run()}</span> })}
                    {pending_run.then(|| view! { <PendingRun/> })}
                </div>
                {after.is_set().then(|| view! { <div class="op-field-after">{after.run()}</div> })}
            </div>
            {bottom}
        </div>
    }
}
