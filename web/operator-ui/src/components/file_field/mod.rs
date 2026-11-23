//! File picker and drop target on the shared Field shell.
use crate::components::{
    chip::Chip,
    field::{Field, FieldMetrics, FieldVariant},
    primitives::Icon,
};
use crate::core::{
    ControlState, Metric, ScopedSlot, Slot, root_class, root_style, ui_icon, ui_text,
};
use leptos::prelude::*;
use operator_ui_tokens::dim;
use operator_ui_tokens::{Icon as IconData, icons};
use send_wrapper::SendWrapper;
use std::sync::atomic::{AtomicU64, Ordering};
pub(crate) const CSS: &str = include_str!("file_field.css");

/// A browser File with stable identity, safe to keep in Leptos signals.
#[derive(Clone, Debug)]
pub struct PickedFile {
    pub id: u64,
    pub file: SendWrapper<web_sys::File>,
}
impl PickedFile {
    pub fn new(file: web_sys::File) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self {
            id: NEXT.fetch_add(1, Ordering::Relaxed),
            file: SendWrapper::new(file),
        }
    }
    pub fn name(&self) -> String {
        self.file.name()
    }
    pub fn size(&self) -> f64 {
        self.file.size()
    }
}
impl PartialEq for PickedFile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
/// Tests the HTML accept syntax for extensions, exact MIME types and MIME wildcards.
pub fn accepts_file(name: &str, mime: &str, accept: &str) -> bool {
    let name = name.to_lowercase();
    let mime = mime.to_lowercase();
    accept.trim().is_empty()
        || accept.split(',').any(|rule| {
            let rule = rule.trim().to_lowercase();
            if rule.starts_with('.') {
                name.ends_with(&rule)
            } else if let Some(prefix) = rule.strip_suffix("/*") {
                mime.starts_with(&format!("{prefix}/"))
            } else {
                mime == rule
            }
        })
}
#[component]
pub fn FileField(
    value: RwSignal<Vec<PickedFile>>,
    #[prop(default = true)] multiple: bool,
    #[prop(default = true)] append_files: bool,
    #[prop(optional, into)] accept: MaybeProp<String>,
    #[prop(optional)] max_files: Option<usize>,
    #[prop(optional)] max_size: Option<f64>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional, into)] on_change: Option<Callback<Vec<PickedFile>>>,
    #[prop(optional, into)] on_rejected: Option<Callback<Vec<String>>>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] pick_label: MaybeProp<String>,
    #[prop(optional)] pick_icon: Option<IconData>,
    #[prop(optional, into)] variant: Signal<FieldVariant>,
    #[prop(optional, into)] prepend: Slot,
    #[prop(optional, into)] append: Slot,
    #[prop(optional, into)] before: Slot,
    #[prop(optional, into)] after: Slot,
    #[prop(optional, into)] label_slot: Slot,
    #[prop(optional, into)] hint_slot: Slot,
    #[prop(optional)] file: Option<ScopedSlot<PickedFile>>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] slot_size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] chip_gap: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let input = NodeRef::<leptos::html::Input>::new();
    let focused = RwSignal::new(false);
    let dragging = RwSignal::new(false);
    let pick = ui_text(pick_label, "file-field.pick", "Choose files");
    let name = ui_text(label, "file-field.label", "Files");
    let icon = ui_icon(pick_icon, "file-field.pick", icons::ADD);
    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else {
            state.get().unwrap_or_else(|| {
                if focused.get() || dragging.get() {
                    ControlState::Focused
                } else {
                    ControlState::Idle
                }
            })
        }
    });
    let available = Signal::derive(move || {
        !matches!(
            current.get(),
            ControlState::Pending | ControlState::Disabled
        )
    });
    let add = Callback::new(move |files: Vec<web_sys::File>| {
        if !available.get_untracked() {
            return;
        }
        let mut selected = if append_files && multiple {
            value.get_untracked()
        } else {
            Vec::new()
        };
        let mut rejected = Vec::new();
        for f in files {
            if !accepts_file(
                &f.name(),
                &f.type_(),
                &accept.get_untracked().unwrap_or_default(),
            ) || max_size.is_some_and(|limit| f.size() > limit)
                || selected.len()
                    >= if multiple {
                        max_files.unwrap_or(usize::MAX)
                    } else {
                        1
                    }
            {
                rejected.push(f.name());
                continue;
            }
            selected.push(PickedFile::new(f));
        }
        value.set(selected.clone());
        if let Some(cb) = on_change {
            cb.run(selected);
        }
        if !rejected.is_empty()
            && let Some(cb) = on_rejected
        {
            cb.run(rejected);
        }
    });
    let classes = Signal::derive(root_class("op-file-field", class));
    let styles = Signal::derive(root_style(
        vec![Metric::du("--op-file-field-chip-gap", chip_gap)],
        style,
    ));
    view! {
        <Field state=current label=name hint=hint error=error variant=variant prepend=prepend append=append before=before after=after label_slot=label_slot hint_slot=hint_slot box_class="op-file-field-box" pending_run=true
            metrics=FieldMetrics{height,padding_x,slot_size,gap,font_size} class=classes style=styles>
            <div class="op-file-field-control" data-dragging=move || dragging.get().then_some("")
                on:dragover=move |ev| {ev.prevent_default();if available.get_untracked() {dragging.set(true);}}
                on:dragleave=move |_| dragging.set(false)
                on:drop=move |ev| {ev.prevent_default();dragging.set(false);if let Some(files)=ev.data_transfer().and_then(|d|d.files()) {add.run((0..files.length()).filter_map(|i|files.item(i)).collect());}}>
                <input node_ref=input class="op-file-field-input" type="file" multiple=multiple accept=move || accept.get() disabled=move || !available.get() aria-label=move || name.get()
                    on:focus=move |_| focused.set(true) on:blur=move |_| focused.set(false)
                    on:change=move |_| {if let Some(el)=input.get_untracked() {if let Some(files)=el.files() {add.run((0..files.length()).filter_map(|i|files.item(i)).collect());}el.set_value("");}}/>
                <button type="button" class="op-file-field-pick" disabled=move || !available.get() on:click=move |_| {if available.get_untracked() && let Some(el)=input.get_untracked() {el.click();}}
                    on:focus=move |_| focused.set(true) on:blur=move |_| focused.set(false)>
                    {move || view! {<Icon icon=icon.get() size=dim::ICON_SMALL/>}}{move || pick.get()}
                </button>
                <div class="op-file-field-chips">
                    <For each=move || value.get() key=|f| f.id children=move |f| {let id=f.id;
                        if let Some(slot)=file {slot.run(f)} else {view! {<Chip text=f.name() enabled=available on_remove=move |()| {if available.get_untracked() {value.update(|v|v.retain(|f|f.id!=id));if let Some(cb)=on_change {cb.run(value.get_untracked());}}}/>} .into_any()}
                    }/>
                </div>
            </div>
        </Field>
    }
}
#[cfg(test)]
mod tests {
    use super::accepts_file;
    #[test]
    fn accept_matches_extensions_and_mime_without_accepting_other_files() {
        assert!(accepts_file("PHOTO.PNG", "image/png", ".png,text/plain"));
        assert!(accepts_file("photo.png", "image/png", "image/*"));
        assert!(!accepts_file("photo.png", "image/png", "image/jpeg"));
        assert!(!accepts_file(
            "photo.png.exe",
            "application/octet-stream",
            ".png"
        ));
    }
}
