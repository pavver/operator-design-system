//! Form validation through registered FormField boundaries.
use crate::core::{
    ActionResult, AsyncAction, ControlState, JobToken, Metric, PendingRun, Slot, root_class,
    root_style, sleep_ms, ui_text, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::motion;
use std::sync::atomic::{AtomicU64, Ordering};
use wasm_bindgen::JsCast;
pub(crate) const CSS: &str = include_str!("form.css");
/// First rule returning a message wins; None means valid.
pub type ValidationRule = Callback<String, Option<String>>;
/// Applies rules in declaration order.
pub fn validate_rules(value: &str, rules: &[ValidationRule]) -> Option<String> {
    rules.iter().find_map(|rule| rule.run(value.to_string()))
}
#[derive(Clone, Copy)]
struct Registration {
    id: u64,
    validate: Callback<(), bool>,
    reset: Callback<()>,
    focus: Callback<()>,
}
/// Context for custom fields and programmatic validation/reset.
#[derive(Clone, Copy)]
pub struct FormContext {
    fields: RwSignal<Vec<Registration>>,
    pub pending: Signal<bool>,
}
impl FormContext {
    pub fn validate(self, focus_error: bool) -> bool {
        let fields = self.fields.get_untracked();
        let mut first = None;
        for field in fields {
            if !field.validate.run(()) && first.is_none() {
                first = Some(field);
            }
        }
        if focus_error && let Some(field) = first {
            field.focus.run(());
        }
        first.is_none()
    }
    pub fn reset(self) {
        if !self.pending.get_untracked() {
            for field in self.fields.get_untracked() {
                field.reset.run(());
            }
        }
    }
    /// Register a custom control and automatically remove it on owner cleanup.
    pub fn register(self, validate: Callback<(), bool>, reset: Callback<()>, focus: Callback<()>) {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        self.fields.update(|v| {
            v.push(Registration {
                id,
                validate,
                reset,
                focus,
            })
        });
        on_cleanup(move || {
            self.fields.try_update(|v| v.retain(|f| f.id != id));
        });
    }
}
#[component]
pub fn Form(
    children: Children,
    #[prop(optional, into)] on_submit: Option<AsyncAction>,
    #[prop(optional, into)] on_reset: Option<Callback<()>>,
    #[prop(optional, into)] on_validation: Option<Callback<bool>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] submit_label: MaybeProp<String>,
    #[prop(optional, into)] reset_label: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let local = RwSignal::new(ControlState::Idle);
    let pending = Signal::derive(move || local.get() == ControlState::Pending);
    let context = FormContext {
        fields: RwSignal::new(Vec::new()),
        pending,
    };
    provide_context(context);
    let visual = use_state_visual(Signal::derive(move || {
        if enabled.get() {
            local.get()
        } else {
            ControlState::Disabled
        }
    }));
    let error = RwSignal::new(None::<String>);
    let label = ui_text(label, "form.label", "Form");
    let submit = ui_text(submit_label, "form.submit", "Submit");
    let reset = ui_text(reset_label, "form.reset", "Reset");
    let action = StoredValue::new(on_submit);
    let job = JobToken::new();
    view! {
            <form class=root_class("op-form op-sb",class) style=root_style(vec![Metric::du("--op-form-gap",gap),Metric::du("--op-form-padding",padding)],style) novalidate=true aria-label=move || label.get() aria-busy=move || pending.get().to_string()
                data-state=move || visual.state.get() data-result=move || visual.result.get() data-fading=move || visual.fading.get().then_some("")
                on:submit=move |ev| {
                    ev.prevent_default(); if pending.get_untracked() || !enabled.get_untracked() {return;}
                    let valid=context.validate(true);if let Some(cb)=on_validation {cb.run(valid);}
    if !valid {return;}
                    error.set(None);let id=job.next();local.set(ControlState::Pending);let action=action.get_value();
                    spawn_local(async move {let result=if let Some(action)=action {action.call(()).await} else {ActionResult::Success};if !job.is_current(id) {return;}
                        if let ActionResult::Error(message)=&result {error.set(message.clone());}local.set(result.into());
                        sleep_ms(motion::RESULT_HOLD_MS).await;if job.is_current(id) {local.set(ControlState::Idle);}
                    });
                }
                on:reset=move |ev| {ev.prevent_default();if !pending.get_untracked() && enabled.get_untracked() {job.next();context.reset();error.set(None);local.set(ControlState::Idle);if let Some(cb)=on_reset {cb.run(());}}}>
                {header.run()}
                <fieldset disabled=move || pending.get() || !enabled.get()>
                    <div class="op-form-fields" inert=move || pending.get() || !enabled.get()>{children()}</div>
                    <div class="op-form-actions">{if actions.is_set() {actions.run()} else {Some(view! {<button type="submit">{move || submit.get()}</button><button type="reset">{move || reset.get()}</button>}.into_any())}}</div>
                </fieldset>
                <span role="status">{move || error.get()}</span>
                {footer.run()}<PendingRun/>
            </form>
        }
}
/// Wrap any string-valued field to register its validation, blur and reset behavior.
#[component]
pub fn FormField(
    value: RwSignal<String>,
    rules: Vec<ValidationRule>,
    children: Children,
    #[prop(optional)] error: Option<RwSignal<Option<String>>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let error = error.unwrap_or_else(|| RwSignal::new(None));
    let root = NodeRef::<leptos::html::Div>::new();
    let initial = StoredValue::new(value.get_untracked());
    let rules = StoredValue::new(rules);
    let validate = Callback::new(move |()| {
        let message = if enabled.get_untracked() {
            validate_rules(&value.get_untracked(), &rules.get_value())
        } else {
            None
        };
        let valid = message.is_none();
        error.set(message);
        valid
    });
    if let Some(context) = use_context::<FormContext>() {
        context.register(validate,Callback::new(move |()| {value.set(initial.get_value());error.set(None);}),Callback::new(move |()| {if let Some(el)=root.get_untracked() && let Ok(Some(el))=el.query_selector("input:not(:disabled),textarea:not(:disabled),select:not(:disabled),[tabindex='0']") && let Ok(el)=el.dyn_into::<web_sys::HtmlElement>() {let _=el.focus();}}));
    }
    let context = use_context::<FormContext>();
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let id = format!("op-form-error-{}", NEXT.fetch_add(1, Ordering::Relaxed));
    let error_id = StoredValue::new(id.clone());
    Effect::new(move |_| {
        let invalid = error.get().is_some();
        if let Some(el) = root.get()
            && let Ok(Some(input)) = el.query_selector("input,textarea,select,[contenteditable]")
        {
            let _ = input.set_attribute("aria-invalid", if invalid { "true" } else { "false" });
            let _ = input.set_attribute("aria-describedby", &error_id.get_value());
        }
    });
    view! {
        <div node_ref=root class=root_class("op-form-field",class) style=root_style(vec![Metric::du("--op-form-gap",gap)],style)
            on:focusout=move |ev| {if context.is_some_and(|c|c.pending.get_untracked()) {return;} let inside=ev.related_target().and_then(|t|t.dyn_into::<web_sys::Node>().ok()).is_some_and(|node|root.get_untracked().is_some_and(|el|el.contains(Some(&node))));if !inside {validate.run(());}}>
            {children()}<span id=id class="op-form-error" role="alert">{move || error.get()}</span>
        </div>
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stops_at_first_error_and_preserves_rule_order() {
        let owner = Owner::new();
        owner.with(|| {
            let rules = vec![
                Callback::new(|v: String| v.is_empty().then(|| "Required".to_string())),
                Callback::new(|v: String| (v.len() < 3).then(|| "Too short".to_string())),
            ];
            assert_eq!(validate_rules("", &rules), Some("Required".to_string()));
            assert_eq!(validate_rules("ab", &rules), Some("Too short".to_string()));
            assert_eq!(validate_rules("abc", &rules), None);
        });
    }
}
