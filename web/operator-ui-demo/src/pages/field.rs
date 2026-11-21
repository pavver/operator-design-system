//! Live examples for the public Field shell.

use crate::i18n::l;
use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn field() -> AnyView {
    let focused = RwSignal::new(false);
    let custom_state = Signal::derive(move || {
        if focused.get() {
            ControlState::Focused
        } else {
            ControlState::Idle
        }
    });
    let device_label = l("Device name", "Назва пристрою");
    let address_label = l("Address", "Адреса");
    let inline_label = l("Inline value", "Вбудоване значення");
    let amount_label = l("Amount", "Сума");
    let level_label = l("Level", "Рівень");
    view! {
        <Example title="Label and hint" block=true>
            <Field state=ControlState::Idle label=device_label hint=l("Use a descriptive name", "Використайте зрозумілу назву")>
                <input class="op-field-input" aria-label=move || device_label.get() value="North gate"/>
            </Field>
        </Example>
        <Example title="Error" block=true>
            <Field state=ControlState::Idle label=address_label hint="This hint is replaced" error=l("Enter a valid address", "Введіть правильну адресу")>
                <input class="op-field-input" aria-label=move || address_label.get() aria-invalid="true" value="invalid"/>
            </Field>
        </Example>
        <Example title="Borderless" block=true>
            <Field state=ControlState::Idle variant=FieldVariant::Borderless label=inline_label>
                <input class="op-field-input" aria-label=move || inline_label.get() value="Editable table cell"/>
            </Field>
        </Example>
        <Example title="Slots" block=true>
            <Field state=ControlState::Idle label=amount_label
                prepend=|| view! { <span>"$"</span> }
                append=|| view! { <span>"USD"</span> }
                before=|| view! { <Icon icon=icons::INFO/> }
                after=|| view! { <span>{l("net", "нетто")}</span> }
                metrics=FieldMetrics { height: 12.0.into(), padding_x: 4.0.into(), ..FieldMetrics::default() }
            >
                <input class="op-field-input" inputmode="decimal" aria-label=move || amount_label.get() value="125.00"/>
            </Field>
        </Example>
        <Example title="Custom control" note="Field owns the shell; the caller owns the control and its state." block=true>
            <Field state=custom_state label=level_label hint=l("A native slider inside Field", "Звичайний повзунок усередині Field")>
                <input class="op-field-input" type="range" min="0" max="100" aria-label=move || level_label.get()
                    on:focus=move |_| focused.set(true) on:blur=move |_| focused.set(false)/>
            </Field>
        </Example>
    }.into_any()
}
