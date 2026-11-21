//! CodeField: a multi-line code input with highlighting, diagnostics and completion.
//!
//! Project-specific (`code-field` feature): the language lives in the application, which passes a
//! highlighter and a completion source.

mod caret;
mod text;

pub use text::{
    CodeSpan, CodeToken, Segment, byte_to_utf16, first_message, segments, splice, utf16_to_byte,
};

use crate::components::combobox::{
    ComboList, ComboOption, filter_rows, option_at, option_count, row_of, step,
};
use crate::components::dropdown_menu::{FieldListHandle, field_popup_label, field_popup_style};
use crate::components::field::{Field, FieldMetrics, FieldVariant};
use crate::components::hover_card::HoverCard;
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::core::{
    ControlState, Metric, ScopedSlot, Slot, field_state, root_class, root_style, ui_text,
};
use leptos::prelude::*;
use std::ops::Range;
use wasm_bindgen::JsCast;

pub(crate) const CSS: &str = include_str!("code_field.css");

/// Completion offered at the caret: the byte range it replaces and the choices.
#[derive(Clone, Debug, PartialEq)]
pub struct CodeCompletion {
    pub range: Range<usize>,
    pub options: Vec<ComboOption>,
}

/// Multi-line code input that grows with its text.
///
/// `highlight` colours the text and marks diagnostics (the first message shows under the field).
/// `complete` is asked on every edit and on Ctrl+Space with the text and the caret byte offset; a
/// result opens a list at the caret. Arrows move in it, Enter or Tab inserts, Escape closes. An
/// insert goes through the browser's edit history, so Ctrl+Z undoes it. Leaving the field or
/// Ctrl+Enter commits.
#[component]
pub fn CodeField(
    value: RwSignal<String>,
    #[prop(optional)] highlight: Option<Callback<String, Vec<CodeSpan>>>,
    #[prop(optional)] complete: Option<Callback<(String, usize), Option<CodeCompletion>>>,
    #[prop(optional)] on_commit: Option<Callback<String>>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Height of the empty field in lines.
    #[prop(default = 2)]
    min_lines: u32,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] slot_size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] caption_font_size: MaybeProp<f32>,
    #[prop(optional, into)] caption_line_height: MaybeProp<f32>,
    #[prop(optional, into)] completion_min_width: MaybeProp<f32>,
    #[prop(optional, into)] completion_max_width: MaybeProp<f32>,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] option_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] option_gap: MaybeProp<f32>,
    #[prop(optional, into)] option_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] detail_font_size: MaybeProp<f32>,
    #[prop(optional, into)] detail_line_height: MaybeProp<f32>,
    #[prop(optional, into)] section_padding_y: MaybeProp<f32>,
    #[prop(optional, into)] letter_spacing: MaybeProp<f32>,
    #[prop(optional, into)] gutter_width: MaybeProp<f32>,
    #[prop(optional, into)] gutter_gap: MaybeProp<f32>,
    #[prop(optional, into)] prepend: Slot,
    #[prop(optional, into)] append: Slot,
    #[prop(optional, into)] before: Slot,
    #[prop(optional, into)] after: Slot,
    #[prop(optional, into)] label_slot: Slot,
    #[prop(optional, into)] hint_slot: Slot,
    #[prop(optional, into)] diagnostics_hover: Slot,
    #[prop(optional, into)] gutter: Slot,
    #[prop(optional)] completion_item: Option<ScopedSlot<ComboOption>>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] variant: Signal<FieldVariant>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] completion_label: MaybeProp<String>,
    #[prop(optional, into)] diagnostics_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    /// Tab width as a number of spaces.
    #[prop(default = 2)]
    tab_size: u32,
) -> impl IntoView {
    let input = NodeRef::<leptos::html::Textarea>::new();
    let focused = RwSignal::new(false);
    let committed = StoredValue::new(value.get_untracked());
    let state = field_state(enabled, state, focused);
    let blocked = move || matches!(state.get(), ControlState::Pending | ControlState::Disabled);
    let placeholder = ui_text(placeholder, "code-field.placeholder", "");
    let name = ui_text(aria_label, "code-field.label", "Code");
    let accessible_name = Signal::derive(move || label.get().unwrap_or_else(|| name.get()));
    let completion_label = ui_text(completion_label, "code-field.completions", "Completions");
    let diagnostics_label = ui_text(diagnostics_label, "code-field.diagnostics", "Diagnostics");
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
        Metric::tu("--op-field-caption-font-size", caption_font_size),
        Metric::tu("--op-field-caption-line-height", caption_line_height),
        Metric::du("--op-field-completion-min-width", completion_min_width),
        Metric::du("--op-field-completion-max-width", completion_max_width),
        Metric::du("--op-field-row-height", row_height),
        Metric::du("--op-field-max-height", max_height),
        Metric::du("--op-field-option-padding-x", option_padding_x),
        Metric::du("--op-field-option-gap", option_gap),
        Metric::du("--op-field-option-icon-size", option_icon_size),
        Metric::tu("--op-field-detail-font-size", detail_font_size),
        Metric::tu("--op-field-detail-line-height", detail_line_height),
        Metric::du("--op-field-section-padding-y", section_padding_y),
        Metric::tu("--op-field-letter-spacing", letter_spacing),
        Metric::du("--op-field-gutter-width", gutter_width),
        Metric::du("--op-field-gutter-gap", gutter_gap),
    ]);
    let custom_style = Signal::derive(root_style(metrics, style));
    let host = NodeRef::<leptos::html::Div>::new();
    let spans = Memo::new(move |_| highlight.map(|h| h.run(value.get())).unwrap_or_default());
    let message = Signal::derive(move || spans.with(|s| first_message(s)));

    // Completion state.
    let open = RwSignal::new(false);
    let anchor = RwSignal::new(PopoverAnchor::Point { x: 0.0, y: 0.0 });
    let completion = RwSignal::new(None::<CodeCompletion>);
    let active = RwSignal::new(0usize);
    let list = FieldListHandle::new();
    let popup_style = field_popup_style(host, open, custom_style);
    let popup_ref = NodeRef::<leptos::html::Div>::new();
    field_popup_label(popup_ref, completion_label);
    Effect::new(move |_| {
        if blocked() {
            open.set(false);
        }
    });
    let rows = Memo::new(move |_| {
        completion.with(|c| {
            c.as_ref()
                .map(|c| filter_rows(&c.options, ""))
                .unwrap_or_default()
        })
    });
    let skip_next = StoredValue::new(false);

    let caret = move || -> Option<(web_sys::HtmlTextAreaElement, u32)> {
        let el = input.get_untracked()?;
        let el: web_sys::HtmlTextAreaElement =
            el.unchecked_ref::<web_sys::HtmlTextAreaElement>().clone();
        let pos = el.selection_end().ok().flatten()?;
        Some((el, pos))
    };
    let ask = move || {
        if blocked() {
            return;
        }
        let Some(source) = complete else { return };
        let Some((el, pos)) = caret() else { return };
        let text = value.get_untracked();
        let byte = utf16_to_byte(&text, pos as usize);
        match source.run((text, byte)).filter(|c| !c.options.is_empty()) {
            Some(c) => {
                if let Some((x, y, w, h)) = caret::caret_rect(&el, pos) {
                    anchor.set(PopoverAnchor::Rect {
                        left: x,
                        top: y,
                        width: w,
                        height: h,
                    });
                }
                completion.set(Some(c));
                active.set(0);
                open.set(true);
            }
            None => open.set(false),
        }
    };
    let accept = Callback::new(move |option: ComboOption| {
        if blocked() {
            return;
        }
        let Some(c) = completion.get_untracked() else {
            return;
        };
        let Some((el, _)) = caret() else { return };
        let text = value.get_untracked();
        let start = u32::try_from(byte_to_utf16(&text, c.range.start)).unwrap_or(0);
        let end = u32::try_from(byte_to_utf16(&text, c.range.end)).unwrap_or(start);
        let _ = el.set_selection_range(start, end);
        open.set(false);
        skip_next.set_value(true);
        // insertText keeps the browser undo stack; fall back to a plain splice.
        let inserted = document()
            .dyn_into::<web_sys::HtmlDocument>()
            .ok()
            .and_then(|d| {
                d.exec_command_with_show_ui_and_value("insertText", false, &option.value)
                    .ok()
            })
            .unwrap_or(false);
        if !inserted {
            let (next, caret_byte) = splice(&text, c.range.clone(), &option.value);
            value.set(next.clone());
            el.set_value(&next);
            let at = u32::try_from(byte_to_utf16(&next, caret_byte)).unwrap_or(0);
            let _ = el.set_selection_range(at, at);
        }
    });
    let commit = move || {
        let v = value.get_untracked();
        if committed.with_value(|c| *c != v) {
            committed.set_value(v.clone());
            if let Some(cb) = on_commit {
                cb.run(v);
            }
        }
    };
    let reveal = move || {
        if let Some(row) = rows.with_untracked(|r| row_of(r, active.get_untracked())) {
            list.reveal(row, false);
        }
    };

    view! {
        <div node_ref=host class=root_class("op-code-field", class) style=move || custom_style.get()>
            <Field state=state label=label hint=hint error=error variant=variant
                prepend=prepend append=append before=before after=after label_slot=label_slot hint_slot=hint_slot
                box_class="op-code-box" pending_run=true>
            <div class="op-code-editor">
                {gutter.is_set().then(|| view! { <div class="op-code-gutter">{gutter.run()}</div> })}
            <div class="op-code-stack" style=format!("--op-field-min-lines:{min_lines};--op-field-tab-size:{tab_size}")>
                <pre class="op-code-layer" aria-hidden="true">
                    {move || {
                        value.with(|v| spans.with(|s| segments(v, s)))
                            .into_iter()
                            .map(|seg| view! { <span class=seg.classes.join(" ")>{seg.text}</span> })
                            .collect_view()
                    }}
                    // A trailing newline needs a character after it to take up a line.
                    "\u{200b}"
                </pre>
                <textarea
                    node_ref=input
                    class="op-code-input"
                    spellcheck="false"
                    autocomplete="off"
                    aria-label=move || accessible_name.get()
                    aria-invalid=move || (error.get().is_some() || message.with(|m| m.as_ref().is_some_and(|(t, _)| *t == CodeToken::Error))).then_some("true")
                    placeholder=move || placeholder.get()
                    prop:value=move || value.get()
                    disabled=blocked
                    on:input=move |e| {
                        if blocked() { return; }
                        value.set(event_target_value(&e));
                        if skip_next.get_value() {
                            skip_next.set_value(false);
                        } else {
                            ask();
                        }
                    }
                    on:focus=move |_| focused.set(true)
                    on:blur=move |_| {
                        focused.set(false);
                        open.set(false);
                        commit();
                    }
                    on:keydown=move |e| {
                        if blocked() { return; }
                        let n = rows.with_untracked(|r| option_count(r));
                        let key = e.key();
                        if open.get_untracked() {
                            match key.as_str() {
                                "ArrowDown" => {
                                    e.prevent_default();
                                    active.update(|a| *a = step(*a, n, 1));
                                    reveal();
                                    return;
                                }
                                "ArrowUp" => {
                                    e.prevent_default();
                                    active.update(|a| *a = step(*a, n, -1));
                                    reveal();
                                    return;
                                }
                                "Enter" | "Tab" if !e.ctrl_key() => {
                                    e.prevent_default();
                                    if let Some(o) = rows.with_untracked(|r| option_at(r, active.get_untracked())) {
                                        accept.run(o);
                                    }
                                    return;
                                }
                                "Escape" => {
                                    e.prevent_default();
                                    e.stop_propagation();
                                    open.set(false);
                                    return;
                                }
                                _ => {}
                            }
                        }
                        if key == " " && e.ctrl_key() {
                            e.prevent_default();
                            ask();
                        } else if key == "Enter" && (e.ctrl_key() || e.meta_key()) {
                            e.prevent_default();
                            commit();
                        }
                    }
                ></textarea>
            </div>
            </div>
            </Field>
            {move || message.get().map(|(t, m)| {
                let message = view! { <span>{m}</span> };
                let content = if diagnostics_hover.is_set() {
                    let content = diagnostics_hover.clone();
                    view! { <HoverCard card=ViewFn::from(move || content.run()) aria_label=diagnostics_label>
                        <span tabindex="0" class="op-code-diagnostic-trigger">{message}</span>
                    </HoverCard> }.into_any()
                } else { message.into_any() };
                view! { <div class="op-code-message" data-kind=if t == CodeToken::Error { "error" } else { "warning" }>{content}</div> }
            })}
            <Popover
                open=open
                anchor=anchor
                placement=Placement::BelowStart
                restore_focus=false
                class="op-dropdown-menu op-code-completion"
                style=popup_style
                node_ref=popup_ref
                role="listbox"
            >
                <ComboList
                    rows=rows
                    handle=list
                    active=active
                    selected=Signal::stored(String::new())
                    highlight=Signal::stored(String::new())
                    empty=Signal::stored(None::<String>)
                    on_pick=accept
                    option=completion_item
                />
            </Popover>
        </div>
    }
}

#[cfg(test)]
mod tests;
