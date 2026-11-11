//! Single-line text field: TextField.

use crate::components::action_button::ActionButton;
use crate::components::primitives::Icon;
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, PendingRun, sleep_ms, spawn_ripple,
    use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons};

pub(crate) const CSS: &str = include_str!("text_field.css");

/// Single-line text field; shows an async SAVE button once the value differs from `initial_value`.
#[component]
pub fn TextField(
    value: RwSignal<String>,
    #[prop(into)] placeholder: Signal<String>,
    /// Baseline used to detect modifications; defaults to the value at creation.
    #[prop(optional, into)]
    initial_value: Option<Signal<String>>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(default = "SAVE".into(), into)] save_text: Signal<String>,
    #[prop(optional)] save_icon: Option<IconData>,
    #[prop(optional, into)] on_save: Option<AsyncAction<String>>,
    #[prop(optional)] password: bool,
    /// Shows the reveal toggle for password fields.
    #[prop(default = true)]
    show_reveal: bool,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let revealed = RwSignal::new(false);
    let has_reveal = password && show_reveal;
    let initial = initial_value.unwrap_or_else(|| Signal::stored(value.get_untracked()));
    let focused = RwSignal::new(false);
    let local = RwSignal::new(ControlState::Idle);
    let save_visible = RwSignal::new(false);
    let job = JobToken::new();
    let has_save = on_save.is_some();

    Effect::new(move |_| {
        let modified = value.get() != initial.get();
        if modified && has_save {
            save_visible.set(true);
        } else if !modified && local.get_untracked() == ControlState::Idle {
            save_visible.set(false);
        }
    });

    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if let Some(s) = state.get() {
            s
        } else if local.get() != ControlState::Idle {
            local.get()
        } else if focused.get() {
            ControlState::Focused
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(current);

    let save_action = on_save.map(|action| {
        AsyncAction::new(move |()| {
            let action = action.clone();
            async move {
                let id = job.next();
                local.set(ControlState::Pending);
                let result: ActionResult = action.call(value.get_untracked()).await;
                if !job.is_current(id) {
                    return result;
                }
                local.set(result.clone().into());
                let success = result.is_success();
                spawn_local(async move {
                    if success {
                        sleep_ms(700).await;
                        if !job.is_current(id) {
                            return;
                        }
                        save_visible.set(false);
                        sleep_ms(100).await;
                    } else {
                        sleep_ms(1000).await;
                    }
                    if job.is_current(id) {
                        local.set(ControlState::Idle);
                    }
                });
                result
            }
        })
    });

    view! {
        <div
            class="op-sb op-field"
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
            data-trailing=move || (has_reveal || save_visible.get()).then_some("")
        >
            <input
                class="op-field-input"
                type=move || if password && !revealed.get() { "password" } else { "text" }
                aria-label=label
                placeholder=move || placeholder.get()
                prop:value=move || value.get()
                disabled=move || !enabled.get()
                on:input=move |ev| value.set(event_target_value(&ev))
                on:focus=move |_| focused.set(true)
                on:blur=move |_| focused.set(false)
            />
            {has_reveal
                .then(|| {
                    view! {
                        <button
                            type="button"
                            class="op-rippled op-field-reveal"
                            aria-label=move || if revealed.get() { "Hide password" } else { "Show password" }
                            aria-pressed=move || revealed.get().to_string()
                            disabled=move || !enabled.get()
                            on:mousedown=|ev| ev.prevent_default()
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| revealed.update(|r| *r = !*r)
                        >
                            {move || {
                                let icon = if revealed.get() { icons::VISIBILITY_OFF } else { icons::VISIBILITY };
                                view! { <Icon icon=icon size=dim::ICON_SMALL/> }
                            }}
                        </button>
                    }
                })}
            {move || {
                (save_visible.get())
                    .then(|| {
                        save_action
                            .clone()
                            .map(|action| {
                                view! {
                                    // Keeps input focus (and the focused border) while pressing SAVE.
                                    <span class="op-field-save-slot" on:mousedown=|ev| ev.prevent_default()>
                                    <ActionButton
                                        text=save_text
                                        icon=save_icon.unwrap_or(icons::CHECK)
                                        show_pending_border=false
                                        keep_flash_icon_on_success=true
                                        enabled=enabled
                                        on_click=action
                                        class="op-field-save"
                                    />
                                    </span>
                                }
                            })
                    })
            }}
            <PendingRun/>
        </div>
    }
}
