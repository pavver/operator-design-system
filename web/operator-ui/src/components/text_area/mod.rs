//! Multi-line text input: TextArea.

use crate::components::field::{Field, FieldMetrics, FieldVariant};
use crate::components::scrollbar::{ScrollAxis, Scrollbar};
use crate::components::tooltip::Tooltip;
use crate::core::{ControlState, Metric, Slot, field_state, root_class, root_style, ui_text};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

pub(crate) const CSS: &str = include_str!("text_area.css");

/// Multi-line text input that grows with its content between `min_rows` and `max_rows`.
///
/// The user can also drag the corner grip to set the height between `min_rows` and
/// `resize_max_rows`; a double-click on the grip returns to content-driven height.
#[component]
pub fn TextArea(
    value: RwSignal<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] variant: Signal<FieldVariant>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] prepend: Slot,
    #[prop(optional, into)] append: Slot,
    #[prop(optional, into)] before: Slot,
    #[prop(optional, into)] after: Slot,
    #[prop(optional, into)] label_slot: Slot,
    #[prop(optional, into)] hint_slot: Slot,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] slot_size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] grip_size: MaybeProp<f32>,
    #[prop(optional, into)] caption_font_size: MaybeProp<f32>,
    #[prop(optional, into)] caption_line_height: MaybeProp<f32>,
    #[prop(optional, into)] counter_gap: MaybeProp<f32>,
    #[prop(optional, into)] resize_label: MaybeProp<String>,
    /// Counter template; `{count}` and `{max}` are replaced with the character counts.
    #[prop(optional, into)]
    counter_text: MaybeProp<String>,
    #[prop(default = 3)] min_rows: u32,
    #[prop(default = 8)] max_rows: u32,
    /// Shows the corner grip for manual resizing.
    #[prop(default = true)]
    resizable: bool,
    /// Largest height, in lines, reachable by manual resizing.
    #[prop(default = 20)]
    resize_max_rows: u32,
    /// Hard character limit with a counter under the field.
    #[prop(optional)]
    max_length: Option<usize>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
) -> impl IntoView {
    let area = NodeRef::<leptos::html::Textarea>::new();
    let focused = RwSignal::new(false);
    let current = field_state(enabled, state, focused);
    let blocked = move || {
        matches!(
            current.get(),
            ControlState::Pending | ControlState::Disabled
        )
    };
    let placeholder = ui_text(placeholder, "text-area.placeholder", "");
    let name = ui_text(aria_label, "text-area.label", "Text");
    let accessible_name = Signal::derive(move || label.get().unwrap_or_else(|| name.get()));
    let resize_label = ui_text(
        resize_label,
        "text-area.resize",
        "Drag to resize, double-click to fit the text",
    );
    let counter_text = ui_text(counter_text, "text-area.counter", "{count} / {max}");
    let max_rows = max_rows.max(min_rows);
    let resize_max_rows = resize_max_rows.max(max_rows);
    // Height set by the grip in CSS pixels; `None` means content-driven height.
    let manual = RwSignal::new(None::<f64>);
    let resizing = RwSignal::new(false);
    // Pointer y and textarea height at the start of a grip drag.
    let drag_from = StoredValue::new((0.0f64, 0.0f64));

    let fit = move || {
        let Some(el) = area.try_get_untracked().flatten() else {
            return;
        };
        let style = web_sys::HtmlElement::style(&el);
        if let Some(h) = manual.get_untracked() {
            let _ = style.set_property("height", &format!("{h}px"));
            return;
        }
        let _ = style.set_property("height", "auto");
        let _ = style.set_property("height", &format!("{}px", el.scroll_height()));
    };
    Effect::new(move |_| {
        value.track();
        manual.track();
        request_animation_frame(fit);
    });

    // Height limits of manual resizing, measured from the rendered line height and padding.
    let limits = move || -> Option<(f64, f64)> {
        let el = area.try_get_untracked().flatten()?;
        let style = web_sys::window()?.get_computed_style(&el).ok().flatten()?;
        let px = |name: &str| {
            style
                .get_property_value(name)
                .ok()
                .and_then(|v| v.trim_end_matches("px").parse::<f64>().ok())
                .unwrap_or(0.0)
        };
        let line = px("line-height");
        let pad = px("padding-top") + px("padding-bottom");
        Some((
            line * f64::from(min_rows) + pad,
            line * f64::from(resize_max_rows) + pad,
        ))
    };
    let on_grip_down = move |ev: web_sys::PointerEvent| {
        if ev.button() != 0 || blocked() {
            return;
        }
        ev.prevent_default();
        let Some(el) = area.try_get_untracked().flatten() else {
            return;
        };
        if let Some(t) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let _ = t.set_pointer_capture(ev.pointer_id());
        }
        drag_from.set_value((
            f64::from(ev.client_y()),
            el.get_bounding_client_rect().height(),
        ));
        resizing.set(true);
    };
    let on_grip_move = move |ev: web_sys::PointerEvent| {
        if blocked() || !resizing.get_untracked() {
            return;
        }
        let Some((min, max)) = limits() else {
            return;
        };
        let (start_y, start_h) = drag_from.get_value();
        let h = (start_h + f64::from(ev.client_y()) - start_y).clamp(min, max);
        manual.set(Some(h.round()));
    };
    let on_grip_up = move |_| resizing.set(false);

    let count = move || value.with(|v| v.chars().count());
    let counter_tone = move || {
        max_length.map(|max| {
            if count() >= max {
                "error"
            } else if count() * 10 >= max * 9 {
                "warning"
            } else {
                "normal"
            }
        })
    };

    let mut metrics = FieldMetrics {
        height,
        padding_x,
        slot_size,
        gap,
        font_size,
    }
    .vars();
    metrics.extend([
        Metric::du("--op-field-padding-y", padding_y),
        Metric::tu("--op-field-line-height", line_height),
        Metric::du("--op-field-grip-size", grip_size),
        Metric::du("--op-field-counter-gap", counter_gap),
        Metric::tu("--op-field-caption-font-size", caption_font_size),
        Metric::tu("--op-field-caption-line-height", caption_line_height),
    ]);
    let custom_style = root_style(metrics, style);
    view! {
        <div class=root_class("op-textarea-wrap", class)
            style=move || format!("--ta-min-rows:{min_rows};--ta-max-rows:{max_rows};--ta-resize-max-rows:{resize_max_rows};{}", custom_style().unwrap_or_default())
            data-resizing=move || resizing.get().then_some("")
        >
            <Field state=current label=label hint=hint error=error variant=variant
                prepend=prepend append=append before=before after=after label_slot=label_slot hint_slot=hint_slot
                box_class="op-textarea" pending_run=true
            >
                <textarea
                    node_ref=area
                    class="op-textarea-input op-scroll-native-off"
                    rows=min_rows
                    data-manual=move || manual.get().is_some().then_some("")
                    aria-invalid=move || error.get().is_some().then_some("true")
                    aria-label=move || accessible_name.get()
                    placeholder=move || placeholder.get()
                    maxlength=max_length.map(|m| m.to_string())
                    prop:value=move || value.get()
                    disabled=blocked
                    on:input=move |ev| value.set(event_target_value(&ev))
                    on:focus=move |_| focused.set(true)
                    on:blur=move |_| focused.set(false)
                ></textarea>
                <Scrollbar target=area axis=ScrollAxis::Vertical corner=Signal::stored(resizable)/>
                {resizable
                    .then(|| {
                        view! {
                            <span class="op-textarea-grip-slot">
                            <Tooltip text=resize_label>
                            <span
                                class="op-textarea-grip"
                                aria-hidden="true"
                                on:pointerdown=on_grip_down
                                on:pointermove=on_grip_move
                                on:pointerup=on_grip_up
                                on:pointercancel=on_grip_up
                                on:lostpointercapture=on_grip_up
                                on:dblclick=move |_| { if !blocked() { manual.set(None); } }
                            ></span>
                            </Tooltip>
                            </span>
                        }
                    })}
            </Field>
            {max_length
                .map(|max| {
                    view! {
                        <div class="op-textarea-counter op-t-caption" data-tone=counter_tone>
                            {move || counter_text.get().replace("{count}", &count().to_string()).replace("{max}", &max.to_string())}
                        </div>
                    }
                })}
        </div>
    }
}
