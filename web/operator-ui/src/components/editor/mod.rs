//! Browser rich text editing with an accessible toolbar and constrained HTML output.

use crate::components::action_button::ActionButton;
use crate::components::field::{Field, FieldMetrics};
use crate::components::icon_button::IconButton;
use crate::components::text_field::TextField;
use crate::core::{ActionResult, ControlState, Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::icons;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

mod html;
pub use html::{safe_link, sanitize_html};

pub(crate) const CSS: &str = include_str!("editor.css");

/// Toolbar commands; HTML structure is kept to the documented allowlist.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditorCommand {
    Bold,
    Italic,
    Underline,
    Strike,
    BulletList,
    OrderedList,
    Paragraph,
    Heading1,
    Heading2,
    Link,
    Undo,
    Redo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandGroup {
    Inline,
    Lists,
    Headings,
    Link,
    History,
}

impl EditorCommand {
    fn metadata(self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Self::Bold => ("editor.bold", "Bold", "bold", ""),
            Self::Italic => ("editor.italic", "Italic", "italic", ""),
            Self::Underline => ("editor.underline", "Underline", "underline", ""),
            Self::Strike => ("editor.strike", "Strikethrough", "strikeThrough", ""),
            Self::BulletList => ("editor.bullets", "Bullet list", "insertUnorderedList", ""),
            Self::OrderedList => ("editor.numbers", "Numbered list", "insertOrderedList", ""),
            Self::Paragraph => ("editor.paragraph", "Paragraph", "formatBlock", "p"),
            Self::Heading1 => ("editor.heading1", "Heading 1", "formatBlock", "h1"),
            Self::Heading2 => ("editor.heading2", "Heading 2", "formatBlock", "h2"),
            Self::Link => ("editor.link", "Link", "createLink", ""),
            Self::Undo => ("editor.undo", "Undo", "undo", ""),
            Self::Redo => ("editor.redo", "Redo", "redo", ""),
        }
    }

    fn glyph(self) -> &'static str {
        match self {
            Self::Bold => "B",
            Self::Italic => "I",
            Self::Underline => "U",
            Self::Strike => "S",
            Self::BulletList => "•",
            Self::OrderedList => "1.",
            Self::Paragraph => "P",
            Self::Heading1 => "H1",
            Self::Heading2 => "H2",
            Self::Link => "Link",
            Self::Undo => "↶",
            Self::Redo => "↷",
        }
    }

    fn group(self) -> CommandGroup {
        match self {
            Self::Bold | Self::Italic | Self::Underline | Self::Strike => CommandGroup::Inline,
            Self::BulletList | Self::OrderedList => CommandGroup::Lists,
            Self::Paragraph | Self::Heading1 | Self::Heading2 => CommandGroup::Headings,
            Self::Link => CommandGroup::Link,
            Self::Undo | Self::Redo => CommandGroup::History,
        }
    }
}

#[component]
pub fn Editor(
    value: RwSignal<String>,
    #[prop(optional)] toolbar: Option<Vec<EditorCommand>>,
    #[prop(optional, into)] on_change: Option<Callback<String>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional)] readonly: bool,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] link_label: MaybeProp<String>,
    #[prop(optional, into)] apply_label: MaybeProp<String>,
    #[prop(optional, into)] cancel_label: MaybeProp<String>,
    #[prop(optional, into)] invalid_link_label: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] min_height: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let editor = NodeRef::<leptos::html::Div>::new();
    let focused = RwSignal::new(false);
    let link_open = RwSignal::new(false);
    let link = RwSignal::new(String::new());
    let link_error = RwSignal::new(false);
    let range = StoredValue::new(None::<SendWrapper<web_sys::Range>>);
    let toolbar = toolbar.unwrap_or_else(|| {
        vec![
            EditorCommand::Bold,
            EditorCommand::Italic,
            EditorCommand::Underline,
            EditorCommand::Strike,
            EditorCommand::BulletList,
            EditorCommand::OrderedList,
            EditorCommand::Paragraph,
            EditorCommand::Heading1,
            EditorCommand::Heading2,
            EditorCommand::Link,
            EditorCommand::Undo,
            EditorCommand::Redo,
        ]
    });
    let label = ui_text(label, "editor.label", "Rich text");
    let placeholder = ui_text(placeholder, "editor.placeholder", "Write something");
    let link_label = ui_text(link_label, "editor.url", "Link URL");
    let apply = ui_text(apply_label, "editor.apply", "Apply link");
    let cancel = ui_text(cancel_label, "editor.cancel", "Cancel");
    let invalid_link = ui_text(
        invalid_link_label,
        "editor.invalid-link",
        "Enter an HTTP, HTTPS, mail or relative URL",
    );
    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else {
            state.get().unwrap_or_else(|| {
                if focused.get() {
                    ControlState::Focused
                } else {
                    ControlState::Idle
                }
            })
        }
    });
    let available = move || {
        !readonly
            && !matches!(
                current.get(),
                ControlState::Pending | ControlState::Disabled
            )
    };
    let version = RwSignal::new(0u64);
    let emit = move || {
        if let Some(el) = editor.get_untracked() {
            let html = sanitize_html(&el.inner_html());
            value.set(html.clone());
            if let Some(cb) = on_change {
                cb.run(html);
            }
            version.update(|n| *n += 1);
        }
    };
    let execute = move |command: &str, arg: &str| {
        if !available() {
            return;
        }
        if let Some(el) = editor.get_untracked() {
            let _ = el.focus();
        }
        if let Ok(doc) = document().dyn_into::<web_sys::HtmlDocument>() {
            let _ = doc.exec_command_with_show_ui_and_value(command, false, arg);
        }
        emit();
    };
    Effect::new(move |_| {
        let html = sanitize_html(&value.get());
        if let Some(el) = editor.get()
            && el.inner_html() != html
        {
            el.set_inner_html(&html);
        }
    });
    let classes = Signal::derive(root_class("op-editor", class));
    let styles = Signal::derive(root_style(
        vec![
            Metric::du("--op-editor-min-height", min_height),
            Metric::du("--op-editor-max-height", max_height),
            Metric::du("--op-editor-padding", padding),
            Metric::du("--op-editor-gap", gap),
            Metric::tu("--op-editor-font-size", font_size),
            Metric::tu("--op-editor-line-height", line_height),
        ],
        style,
    ));

    let mut grouped_items = Vec::new();
    let mut prev_group: Option<CommandGroup> = None;
    for command in toolbar {
        let group = command.group();
        if let Some(prev) = prev_group
            && prev != group
        {
            grouped_items.push(
                view! { <span class="op-editor-sep" role="separator" aria-orientation="vertical"/> }
                    .into_any(),
            );
        }
        prev_group = Some(group);

        let (key, default_name, cmd, arg) = command.metadata();
        let name = ui_text(None::<String>.into(), key, default_name);
        let glyph = command.glyph();
        let is_active = move || {
            version.track();
            document()
                .dyn_into::<web_sys::HtmlDocument>()
                .ok()
                .and_then(|doc| doc.query_command_state(cmd).ok())
                .unwrap_or(false)
        };
        let state = Signal::derive(move || {
            if is_active() {
                Some(ControlState::Selected)
            } else {
                None
            }
        });
        let available_sig = Signal::derive(available);
        let content =
            Slot::from(move || view! { <span class="op-editor-glyph">{glyph}</span> }.into_any());
        let on_click = Callback::new(move |()| {
            if !available() {
                return;
            }
            if command == EditorCommand::Link {
                if let Ok(Some(selection)) = window().get_selection()
                    && selection.range_count() > 0
                    && let Ok(selected) = selection.get_range_at(0)
                {
                    range.set_value(Some(SendWrapper::new(selected)));
                }
                link_open.set(true);
                link_error.set(false);
            } else {
                execute(cmd, arg);
            }
        });
        grouped_items.push(
            view! {
                <IconButton
                    icon=icons::CHECK
                    label=Signal::derive(move || name.get())
                    state=state
                    enabled=available_sig
                    content=content
                    size=8.0
                    on_click=on_click
                    class="op-editor-btn"
                />
            }
            .into_any(),
        );
    }

    view! {
        <Field
            state=current
            label=label
            class=classes
            style=styles
            box_class="op-editor-box"
            pending_run=true
            metrics=FieldMetrics::default()
        >
            <div class="op-editor-body">
                {header.run()}
                <div
                    class="op-editor-toolbar"
                    role="toolbar"
                    aria-label=move || label.get()
                    on:mousedown=|ev| ev.prevent_default()
                >
                    {grouped_items}
                    {actions.run()}
                </div>
                <Show when=move || link_open.get()>
                    <div
                        class="op-editor-link"
                        role="group"
                        aria-label=move || link_label.get()
                    >
                        <TextField
                            value=link
                            label=link_label
                            placeholder="https://"
                            error=Signal::derive(move || link_error.get().then(|| invalid_link.get()))
                            enabled=Signal::derive(available)
                            class="op-editor-link-field"
                        />
                        <div class="op-editor-link-actions">
                            <ActionButton
                                text=apply
                                enabled=Signal::derive(available)
                                on_click=move |()| async move {
                                    if !available() {
                                        return ActionResult::Success;
                                    }
                                    let url = link.get_untracked();
                                    if !safe_link(&url) {
                                        link_error.set(true);
                                        return ActionResult::Error(None);
                                    }
                                    if let Ok(Some(selection)) = window().get_selection()
                                        && let Some(range) = range.get_value()
                                    {
                                        let _ = selection.remove_all_ranges();
                                        let _ = selection.add_range(&range);
                                    }
                                    execute("createLink", &url);
                                    link_open.set(false);
                                    link.set(String::new());
                                    ActionResult::Success
                                }
                            />
                            <ActionButton
                                text=cancel
                                enabled=Signal::derive(available)
                                on_click=move |()| async move {
                                    if available() {
                                        link_open.set(false);
                                        link_error.set(false);
                                        if let Some(el) = editor.get_untracked() {
                                            let _ = el.focus();
                                        }
                                    }
                                    ActionResult::Success
                                }
                            />
                        </div>
                    </div>
                </Show>
                <div
                    node_ref=editor
                    class="op-editor-content"
                    role="textbox"
                    aria-label=move || label.get()
                    aria-multiline="true"
                    aria-readonly=readonly.to_string()
                    aria-disabled=move || (!enabled.get()).to_string()
                    aria-busy=move || (current.get() == ControlState::Pending).to_string()
                    contenteditable=move || available().to_string()
                    tabindex=move || if enabled.get() { 0 } else { -1 }
                    data-placeholder=move || placeholder.get()
                    on:focus=move |_| focused.set(true)
                    on:blur=move |_| focused.set(false)
                    on:input=move |_| {
                        if available() {
                            emit();
                        }
                    }
                    on:keyup=move |_| version.update(|n| *n += 1)
                    on:mouseup=move |_| version.update(|n| *n += 1)
                    on:paste=move |ev| {
                        ev.prevent_default();
                        if !available() {
                            return;
                        }
                        if let Some(data) = ev.clipboard_data() {
                            let html = data.get_data("text/html").unwrap_or_default();
                            let html = if html.is_empty() {
                                let Ok(el) = document().create_element("div") else {
                                    return;
                                };
                                el.set_text_content(
                                    Some(&data.get_data("text/plain").unwrap_or_default()),
                                );
                                el.inner_html()
                            } else {
                                sanitize_html(&html)
                            };
                            execute("insertHTML", &html);
                        }
                    }
                    on:drop=|ev| ev.prevent_default()
                ></div>
                {footer.run()}
            </div>
        </Field>
    }
}
