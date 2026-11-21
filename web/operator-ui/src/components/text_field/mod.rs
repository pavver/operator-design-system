//! Single-line text field: TextField.

use crate::components::action_button::ActionButton;
use crate::components::field::{Field, FieldMetrics, FieldVariant};
use crate::components::primitives::Icon;
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, Metric, Slot, root_style, sleep_ms,
    spawn_ripple, ui_icon, ui_text,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons};

pub(crate) const CSS: &str = include_str!("text_field.css");

/// Single-line text field; shows an async SAVE button once the value differs from `initial_value`.
#[component]
pub fn TextField(
    value: RwSignal<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    /// Baseline used to detect modifications; defaults to the value at creation.
    #[prop(optional, into)]
    initial_value: Option<Signal<String>>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] save_text: MaybeProp<String>,
    #[prop(optional)] save_icon: Option<IconData>,
    #[prop(optional, into)] on_save: Option<AsyncAction<String>>,
    #[prop(optional)] password: bool,
    /// Shows the reveal toggle for password fields.
    #[prop(default = true)]
    show_reveal: bool,
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
    #[prop(optional, into)] show_password_label: MaybeProp<String>,
    #[prop(optional, into)] hide_password_label: MaybeProp<String>,
    #[prop(optional)] reveal_icon: Option<IconData>,
    #[prop(optional)] conceal_icon: Option<IconData>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] save_min_width: MaybeProp<f32>,
    #[prop(optional, into)] save_max_width: MaybeProp<f32>,
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
    let placeholder = ui_text(placeholder, "text-field.placeholder", "");
    let name = ui_text(aria_label, "text-field.label", "Text");
    let accessible_name = Signal::derive(move || label.get().unwrap_or_else(|| name.get()));
    let save_text = ui_text(save_text, "text-field.save", "SAVE");
    let show_password_label = ui_text(
        show_password_label,
        "text-field.show-password",
        "Show password",
    );
    let hide_password_label = ui_text(
        hide_password_label,
        "text-field.hide-password",
        "Hide password",
    );
    let save_icon = ui_icon(save_icon, "text-field.save", icons::CHECK);
    let reveal_icon = ui_icon(reveal_icon, "text-field.reveal", icons::VISIBILITY);
    let conceal_icon = ui_icon(conceal_icon, "text-field.conceal", icons::VISIBILITY_OFF);
    let custom_style = Signal::derive(root_style(
        vec![
            Metric::du("--op-field-icon-size", icon_size),
            Metric::du("--op-field-save-min-width", save_min_width),
            Metric::du("--op-field-save-max-width", save_max_width),
        ],
        style,
    ));
    let blocked = move || {
        matches!(
            current.get(),
            ControlState::Disabled | ControlState::Pending
        )
    };

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

    // Enter runs the save action while SAVE is shown.
    let enter_action = save_action.clone();
    let on_enter = move |ev: leptos::ev::KeyboardEvent| {
        if blocked() || ev.key() != "Enter" || !save_visible.get_untracked() {
            return;
        }
        if let Some(action) = enter_action.clone() {
            ev.prevent_default();
            spawn_local(async move {
                let _ = action.call(()).await;
            });
        }
    };

    let trailing = Slot::from(move || {
        let save_action = save_action.clone();
        view! {
                {append.run()}
                {has_reveal
                    .then(|| {
                        view! {
                            <button
                                type="button"
                                class="op-rippled op-field-reveal"
                                aria-label=move || if revealed.get() { hide_password_label.get() } else { show_password_label.get() }
                                aria-pressed=move || revealed.get().to_string()
                                disabled=blocked
                                on:mousedown=|ev| ev.prevent_default()
                                on:pointerdown=move |ev| { if !blocked() { spawn_ripple(&ev); } }
                                on:click=move |_| revealed.update(|r| *r = !*r)
                            >
                                {move || {
                                    let icon = if revealed.get() { conceal_icon.get() } else { reveal_icon.get() };
                                    view! { <Icon icon=icon size=dim::ICON_SMALL attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-small));height:var(--op-field-icon-size, var(--op-dim-icon-small));"/> }
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
                                            icon=save_icon.get()
                                            show_pending_border=false
                                            keep_flash_icon_on_success=true
                                            enabled=Signal::derive(move || enabled.get() && !matches!(state.get(), Some(ControlState::Disabled | ControlState::Pending)))
                                            on_click=action
                                            class="op-field-save"
                                        />
                                        </span>
                                    }
                                })
                        })
                }}
        }
    });
    view! {
        <Field state=current append=trailing
            trailing=Signal::derive(move || has_reveal || save_visible.get())
            pending_run=true box_class="op-text-field"
            label=label hint=hint error=error variant=variant
            prepend=prepend before=before after=after label_slot=label_slot hint_slot=hint_slot
            metrics=FieldMetrics { height, padding_x, slot_size, gap, font_size }
            class=class style=custom_style
        >
            <input
                class="op-field-input"
                type=move || if password && !revealed.get() { "password" } else { "text" }
                aria-label=move || accessible_name.get()
                aria-invalid=move || error.get().is_some().then_some("true")
                placeholder=move || placeholder.get()
                prop:value=move || value.get()
                // Pending blocks typing without the disabled look.
                disabled=move || current.get() == ControlState::Disabled
                readonly=move || current.get() == ControlState::Pending
                on:input=move |ev| value.set(event_target_value(&ev))
                on:focus=move |_| focused.set(true)
                on:blur=move |_| focused.set(false)
                on:keydown=on_enter
            />
        </Field>
    }
}
