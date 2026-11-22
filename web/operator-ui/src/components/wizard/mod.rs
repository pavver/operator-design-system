//! Multi-step wizard flow with step header, validation, and async finish state.

use crate::components::action_button::ActionButton;
use crate::components::primitives::Icon;
use crate::core::{AsyncAction, Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

pub(crate) const CSS: &str = include_str!("wizard.css");

/// Definition of one step in a [`Wizard`].
#[derive(Clone, Debug, PartialEq)]
pub struct WizardStep {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<IconData>,
}

impl WizardStep {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: None,
            icon: None,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Multi-step flow controller with step header, validation, and asynchronous finish.
#[component]
pub fn Wizard(
    steps: Vec<WizardStep>,
    current: RwSignal<usize>,
    #[prop(optional)] on_validate_step: Option<Callback<usize, bool>>,
    #[prop(optional, into)] on_finish: Option<AsyncAction>,
    #[prop(optional, into)] next_label: MaybeProp<String>,
    #[prop(optional, into)] back_label: MaybeProp<String>,
    #[prop(optional, into)] finish_label: MaybeProp<String>,
    /// Custom navigation footer replacing the default Next / Back / Finish row.
    #[prop(optional, into)]
    navigation: Slot,
    /// Gap between step indicator badge and labels in DU (`--op-wizard-header-gap`).
    #[prop(optional, into)]
    header_gap: MaybeProp<f32>,
    /// Step indicator square size in DU (`--op-wizard-step-size`).
    #[prop(optional, into)]
    step_size: MaybeProp<f32>,
    /// Outer padding in DU (`--op-wizard-padding`).
    #[prop(optional, into)]
    padding: MaybeProp<f32>,
    /// Vertical gap between sections in DU (`--op-wizard-content-gap`).
    #[prop(optional, into)]
    content_gap: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let next_str = ui_text(next_label, "wizard.next", "Next");
    let back_str = ui_text(back_label, "wizard.back", "Back");
    let finish_str = ui_text(finish_label, "wizard.finish", "Finish");

    let total_steps = steps.len();
    let has_custom_nav = navigation.is_set();

    let metrics = vec![
        Metric::du("--op-wizard-header-gap", header_gap),
        Metric::du("--op-wizard-step-size", step_size),
        Metric::du("--op-wizard-padding", padding),
        Metric::du("--op-wizard-content-gap", content_gap),
    ];
    let class_fn = root_class("op-wizard", class);
    let style_fn = root_style(metrics, style);

    let handle_next = move |_| {
        let curr = current.get();
        if curr + 1 < total_steps {
            if let Some(validator) = on_validate_step
                && !validator.run(curr)
            {
                return;
            }
            current.set(curr + 1);
        }
    };

    let handle_back = move |_| {
        let curr = current.get();
        if curr > 0 {
            current.set(curr - 1);
        }
    };

    view! {
        <div class=class_fn style=style_fn>
            <nav class="op-wizard-header" aria-label="Steps">
                {steps.into_iter().enumerate().map(|(idx, step)| {
                    let is_active = Signal::derive(move || current.get() == idx);
                    let is_done = Signal::derive(move || current.get() > idx);
                    let can_click = Signal::derive(move || current.get() > idx);

                    let on_step_click = move |_| {
                        if current.get() > idx {
                            current.set(idx);
                        }
                    };

                    view! {
                        {if idx > 0 {
                            Some(view! {
                                <div
                                    class="op-wizard-connector"
                                    data-done=move || is_done.get().then_some("")
                                />
                            })
                        } else {
                            None
                        }}
                        <button
                            type="button"
                            class="op-wizard-step-item"
                            data-active=move || is_active.get().then_some("")
                            data-done=move || is_done.get().then_some("")
                            disabled=move || !can_click.get()
                            on:click=on_step_click
                            aria-current=move || is_active.get().then_some("step")
                        >
                            <span class="op-wizard-step-badge">
                                {move || {
                                    if is_done.get() {
                                        view! {
                                            <Icon icon=icons::CHECK size=16.0 />
                                        }.into_any()
                                    } else if let Some(icon) = step.icon {
                                        view! {
                                            <Icon icon=icon size=16.0 />
                                        }.into_any()
                                    } else {
                                        view! {
                                            {format!("{}", idx + 1)}
                                        }.into_any()
                                    }
                                }}
                            </span>
                            <span class="op-wizard-step-labels">
                                <span class="op-wizard-step-title">{step.title}</span>
                                {step.subtitle.map(|sub| view! {
                                    <span class="op-wizard-step-subtitle">{sub}</span>
                                })}
                            </span>
                        </button>
                    }
                }).collect_view()}
            </nav>

            <div class="op-wizard-body">
                {children()}
            </div>

            {if has_custom_nav {
                view! {
                    <div class="op-wizard-nav">
                        {navigation.run()}
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="op-wizard-nav">
                        <div>
                            {move || (current.get() > 0).then(|| view! {
                                <ActionButton
                                    text=back_str.get()
                                    on_click=AsyncAction::new(move |()| {
                                        handle_back(());
                                        Box::pin(async { crate::core::ActionResult::Success })
                                    })
                                />
                            })}
                        </div>
                        <div>
                            {move || {
                                let curr = current.get();
                                if curr + 1 < total_steps {
                                    view! {
                                        <ActionButton
                                            text=next_str.get()
                                            on_click=AsyncAction::new(move |()| {
                                                handle_next(());
                                                Box::pin(async { crate::core::ActionResult::Success })
                                            })
                                        />
                                    }.into_any()
                                } else if let Some(finish_action) = on_finish.clone() {
                                    view! {
                                        <ActionButton
                                            text=finish_str.get()
                                            on_click=finish_action
                                        />
                                    }.into_any()
                                } else {
                                    view! {
                                        <ActionButton
                                            text=finish_str.get()
                                        />
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
