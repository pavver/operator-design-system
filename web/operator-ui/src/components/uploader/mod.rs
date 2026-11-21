//! File queue with application-provided upload futures and per-file progress.

use crate::components::{
    action_button::ActionButton,
    file_field::{FileField, PickedFile},
    linear_progress::LinearProgress,
};
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, Metric, PendingRun, ScopedSlot, Slot,
    root_class, root_style, ui_text, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::BTreeMap;

pub(crate) const CSS: &str = include_str!("uploader.css");

/// The application reports a fraction in 0..1 while the returned future is running.
#[derive(Clone)]
pub struct UploadRequest {
    pub file: PickedFile,
    pub progress: Callback<f32>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum UploadStatus {
    #[default]
    Queued,
    Pending(f32),
    Success,
    Error(Option<String>),
}

#[component]
pub fn Uploader(
    #[prop(into)] on_upload: AsyncAction<UploadRequest>,
    #[prop(optional)] files: Option<RwSignal<Vec<PickedFile>>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] accept: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] upload_label: MaybeProp<String>,
    #[prop(optional, into)] retry_label: MaybeProp<String>,
    #[prop(optional, into)] remove_label: MaybeProp<String>,
    #[prop(optional, into)] empty_label: MaybeProp<String>,
    #[prop(optional, into)] on_change: Option<Callback<Vec<PickedFile>>>,
    #[prop(optional, into)] on_complete: Option<Callback<(u64, ActionResult)>>,
    #[prop(optional)] row: Option<ScopedSlot<(PickedFile, UploadStatus)>>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let files = files.unwrap_or_else(|| RwSignal::new(Vec::new()));
    let statuses = RwSignal::new(BTreeMap::<u64, UploadStatus>::new());
    let label = ui_text(label, "uploader.label", "Upload files");
    let upload = ui_text(upload_label, "uploader.upload", "Upload");
    let retry = ui_text(retry_label, "uploader.retry", "Retry");
    let remove = ui_text(remove_label, "uploader.remove", "Remove");
    let empty = ui_text(empty_label, "uploader.empty", "No files selected");
    let on_upload = StoredValue::new(on_upload);
    let lifetime = JobToken::new();
    let life_id = lifetime.next();

    Effect::new(move |_| {
        let ids = files.with(|v| v.iter().map(|f| f.id).collect::<Vec<_>>());
        statuses.update(|s| s.retain(|id, _| ids.contains(id)));
    });

    view! {
        <section
            class=root_class("op-uploader", class)
            style=root_style(
                vec![
                    Metric::du("--op-uploader-gap", gap),
                    Metric::du("--op-uploader-padding", padding),
                ],
                style,
            )
            aria-label=move || label.get()
        >
            {header.run()}
            <FileField
                value=files
                enabled=enabled
                accept=accept
                label=label
                on_change=move |v| {
                    if let Some(cb) = on_change {
                        cb.run(v);
                    }
                }
            />
            {actions.run()}
            <Show when=move || files.with(Vec::is_empty)>
                <p>{move || empty.get()}</p>
            </Show>
            <ul class="op-uploader-list">
                <For
                    each=move || files.get()
                    key=|f| f.id
                    children=move |file| {
                        let id = file.id;
                        let name = file.name();
                        let file = StoredValue::new(file);
                        let status = Signal::derive(move || {
                            statuses.with(|s| s.get(&id).cloned().unwrap_or_default())
                        });
                        let pending = move || matches!(status.get(), UploadStatus::Pending(_));
                        let current = Signal::derive(move || match status.get() {
                            UploadStatus::Queued => ControlState::Idle,
                            UploadStatus::Pending(_) => ControlState::Pending,
                            UploadStatus::Success => ControlState::Success,
                            UploadStatus::Error(_) => ControlState::Error,
                        });
                        let visual = use_state_visual(current);
                        let attempt = JobToken::new();
                        let start = move |()| {
                            if !enabled.get_untracked()
                                || pending()
                                || status.get_untracked() == UploadStatus::Success
                            {
                                return;
                            }
                            let token = attempt.next();
                            statuses.update(|s| {
                                s.insert(id, UploadStatus::Pending(0.0));
                            });
                            let progress = Callback::new(move |v: f32| {
                                if lifetime.is_current(life_id)
                                    && attempt.is_current(token)
                                    && v.is_finite()
                                {
                                    statuses.update(|s| {
                                        if let Some(UploadStatus::Pending(p)) = s.get_mut(&id) {
                                            *p = v.clamp(0.0, 1.0);
                                        }
                                    });
                                }
                            });
                            let action = on_upload.get_value();
                            let request = UploadRequest {
                                file: file.get_value(),
                                progress,
                            };
                            spawn_local(async move {
                                let result = action.call(request).await;
                                if !lifetime.is_current(life_id) || !attempt.is_current(token) {
                                    return;
                                }
                                statuses.update(|s| {
                                    if let Some(status) = s.get_mut(&id) {
                                        *status = if result.is_success() {
                                            UploadStatus::Success
                                        } else {
                                            let ActionResult::Error(message) = result.clone() else {
                                                return;
                                            };
                                            UploadStatus::Error(message)
                                        };
                                    }
                                });
                                if let Some(cb) = on_complete {
                                    cb.run((id, result));
                                }
                            });
                        };
                        view! {
                            <li
                                class="op-uploader-row op-sb"
                                data-state=move || visual.state.get()
                                data-result=move || visual.result.get()
                                data-fading=move || visual.fading.get().then_some("")
                                aria-busy=move || pending().to_string()
                            >
                                {if let Some(row) = row {
                                    view! { {move || row.run((file.get_value(), status.get()))} }
                                        .into_any()
                                } else {
                                    view! { <span>{name}</span> }.into_any()
                                }}
                                <LinearProgress
                                    value=Signal::derive(move || match status.get() {
                                        UploadStatus::Pending(v) => v,
                                        UploadStatus::Success => 1.0,
                                        _ => 0.0,
                                    })
                                    state=Signal::derive(move || Some(current.get()))
                                    label=label
                                />
                                <span role="status">
                                    {move || match status.get() {
                                        UploadStatus::Error(message) => message,
                                        _ => None,
                                    }}
                                </span>
                                <ActionButton
                                    text=Signal::derive(move || {
                                        if matches!(status.get(), UploadStatus::Error(_)) {
                                            retry.get()
                                        } else {
                                            upload.get()
                                        }
                                    })
                                    enabled=Signal::derive(move || {
                                        enabled.get() && !pending()
                                            && status.get() != UploadStatus::Success
                                    })
                                    on_click=move |()| async move {
                                        start(());
                                        ActionResult::Success
                                    }
                                />
                                <ActionButton
                                    text=remove
                                    enabled=Signal::derive(move || enabled.get() && !pending())
                                    on_click=move |()| async move {
                                        if enabled.get_untracked() && !pending() {
                                            attempt.next();
                                            files.update(|v| v.retain(|f| f.id != id));
                                            if let Some(cb) = on_change {
                                                cb.run(files.get_untracked());
                                            }
                                        }
                                        ActionResult::Success
                                    }
                                />
                                <PendingRun/>
                            </li>
                        }
                    }
                />
            </ul>
            {footer.run()}
        </section>
    }
}
