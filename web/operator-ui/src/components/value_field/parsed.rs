//! ParsedField: a text field that commits a typed value.

use crate::components::field::{Field, FieldMetrics, FieldVariant};
use crate::core::{ControlState, Slot, sleep_ms, ui_text};
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Text field over a typed value.
///
/// Typing edits the text only; Enter or leaving the field parses it. A parsed value is stored in
/// `value` and the text is re-formatted; invalid text rolls back to the formatted current value and
/// the field shows the error state for a moment. Escape rolls back without committing.
#[component]
pub fn ParsedField<T>(
    value: RwSignal<T>,
    /// Text for a value.
    format: Callback<T, String>,
    /// Value for a text, or `None` when the text is invalid.
    parse: Callback<String, Option<T>>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    /// Called after every committed change.
    #[prop(default = None)]
    on_commit: Option<Callback<T>>,
    /// Arrow Up/Down: the value one step up (`1`) or down (`-1`) from the current one.
    #[prop(default = None)]
    on_step: Option<Callback<(T, i32), T>>,
    /// `inputmode` hint for virtual keyboards, for example `decimal` or `numeric`.
    #[prop(optional, into)]
    input_mode: Option<&'static str>,
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

    /// Views after the input inside the field (a unit, a button).
    #[prop(default = None)]
    trailing: Option<ChildrenFn>,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let text = RwSignal::new(format.run(value.get_untracked()));
    let focused = RwSignal::new(false);
    let flash = RwSignal::new(None::<ControlState>);
    let current = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if let Some(s) = flash.get().or_else(|| state.get()) {
            s
        } else if focused.get() {
            ControlState::Focused
        } else {
            ControlState::Idle
        }
    });
    let placeholder = ui_text(placeholder, "parsed-field.placeholder", "");
    let name = ui_text(aria_label, "parsed-field.label", "Value");
    let accessible_name = Signal::derive(move || label.get().unwrap_or_else(|| name.get()));
    let blocked = move || {
        matches!(
            current.get(),
            ControlState::Disabled | ControlState::Pending
        )
    };

    // External changes of the value show up unless the user is typing.
    Effect::new(move |_| {
        let v = value.get();
        if !focused.get_untracked() {
            text.set(format.run(v));
        }
    });

    let reject = move || {
        text.set(format.run(value.get_untracked()));
        flash.set(Some(ControlState::Error));
        spawn_local(async move {
            sleep_ms(1000).await;
            flash.set(None);
        });
    };
    let set = move |v: T| {
        text.set(format.run(v.clone()));
        if v != value.get_untracked() {
            value.set(v.clone());
            if let Some(cb) = on_commit {
                cb.run(v);
            }
        }
    };
    let commit = move || {
        if blocked() {
            return;
        }
        let t = text.get_untracked();
        if t == format.run(value.get_untracked()) {
            return;
        }
        match parse.run(t) {
            Some(v) => set(v),
            None => reject(),
        }
    };
    let step = move |delta: i32| {
        let Some(step) = on_step else { return };
        let base = parse
            .run(text.get_untracked())
            .unwrap_or_else(|| value.get_untracked());
        set(step.run((base, delta)));
    };
    let has_trailing = trailing.is_some();
    let append = Slot::from(move || view! { {append.run()} {trailing.as_ref().map(|t| t())} });

    view! {
        <Field state=current append=append trailing=has_trailing pending_run=true box_class="op-value-field"
            label=label hint=hint error=error variant=variant
            prepend=prepend before=before after=after label_slot=label_slot hint_slot=hint_slot
            metrics=FieldMetrics { height, padding_x, slot_size, gap, font_size }
            class=class style=style
        >
            <input
                class="op-field-input"
                type="text"
                inputmode=input_mode
                aria-label=move || accessible_name.get()
                aria-invalid=move || (error.get().is_some() || flash.get() == Some(ControlState::Error)).then_some("true")
                placeholder=move || placeholder.get()
                prop:value=move || text.get()
                disabled=move || current.get() == ControlState::Disabled
                readonly=move || current.get() == ControlState::Pending
                on:input=move |ev| text.set(event_target_value(&ev))
                on:focus=move |_| focused.set(true)
                on:blur=move |_| {
                    focused.set(false);
                    commit();
                }
                on:keydown=move |ev| { if blocked() { return; } match ev.key().as_str() {
                    "Enter" => {
                        ev.prevent_default();
                        commit();
                    }
                    "Escape" => text.set(format.run(value.get_untracked())),
                    "ArrowUp" if on_step.is_some() => {
                        ev.prevent_default();
                        step(1);
                    }
                    "ArrowDown" if on_step.is_some() => {
                        ev.prevent_default();
                        step(-1);
                    }
                    _ => {}
                }}
            />
        </Field>
    }
}
