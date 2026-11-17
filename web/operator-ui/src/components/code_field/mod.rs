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
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::virtual_list::VirtualListHandle;
use crate::core::{ControlState, use_state_visual};
use leptos::prelude::*;
use operator_ui_tokens::dim;
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
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Height of the empty field in lines.
    #[prop(default = 2)]
    min_lines: u32,
) -> impl IntoView {
    let input = NodeRef::<leptos::html::Textarea>::new();
    let focused = RwSignal::new(false);
    let committed = StoredValue::new(value.get_untracked());
    let state = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if focused.get() {
            ControlState::Focused
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(state);
    let spans = Memo::new(move |_| highlight.map(|h| h.run(value.get())).unwrap_or_default());
    let message = Signal::derive(move || spans.with(|s| first_message(s)));

    // Completion state.
    let open = RwSignal::new(false);
    let anchor = RwSignal::new(PopoverAnchor::Point { x: 0.0, y: 0.0 });
    let completion = RwSignal::new(None::<CodeCompletion>);
    let active = RwSignal::new(0usize);
    let list = VirtualListHandle::new(dim::ROW_CONTROL);
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
        <div
            class="op-sb op-code-field"
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
        >
            <div class="op-code-stack" style=format!("--op-code-min-lines: {min_lines}")>
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
                    aria-label=label
                    aria-invalid=move || message.with(|m| m.as_ref().is_some_and(|(t, _)| *t == CodeToken::Error)).then_some("true")
                    placeholder=move || placeholder.get()
                    prop:value=move || value.get()
                    disabled=move || !enabled.get()
                    on:input=move |e| {
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
            {move || message.get().map(|(t, m)| view! {
                <div class="op-code-message op-t-caption" data-kind=if t == CodeToken::Error { "error" } else { "warning" }>{m}</div>
            })}
            <Popover
                open=open
                anchor=anchor
                placement=Placement::BelowStart
                restore_focus=false
                class="op-dropdown-menu op-code-completion"
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
                />
            </Popover>
        </div>
    }
}

#[cfg(test)]
mod tests;
