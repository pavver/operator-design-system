//! Live examples for banner.

use crate::i18n::l;
use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn banner() -> AnyView {
    let b = backend();
    let visible = RwSignal::new(true);
    let custom_visible = RwSignal::new(true);
    view! {
        <Example title="Tones" block=true>
            <div class="demo-stack">
                <Banner tone=Tone::Info text="Recordings older than 30 days are removed automatically."/>
                <Banner tone=Tone::Success title="Connected" text="All 12 cameras are online."/>
                <Banner
                    tone=Tone::Warning
                    title="Storage almost full"
                    text="92% of the recorder disk is used. New recordings will overwrite the oldest ones."
                    actions=ViewFn::from(move || view! { <ActionButton text="Manage storage" on_click=b.action("open storage")/> })
                />
                <Show when=move || visible.get()>
                    <Banner
                        tone=Tone::Error
                        title="Server unreachable"
                        text="Changes are saved locally and will be sent when the connection returns."
                        on_dismiss=Callback::new(move |()| visible.set(false))
                    />
                </Show>
            </div>
        </Example>
        <Example title="Customization" block=true>
            <ActionButton text=l("Show message", "Показати повідомлення") on_click=crate::kit::instant(move || custom_visible.set(true))/>
            <Show when=move || custom_visible.get()>
            <Banner tone=Tone::Info text=l("The device is connected.", "Пристрій під’єднано.") padding=5.0 gap=4.0 icon_size=6.0
                dismiss_label=l("Dismiss information", "Закрити інформацію") on_dismiss=Callback::new(move |()| custom_visible.set(false))
                header=move || view! { <strong>{l("Connection", "З’єднання")}</strong> }
                actions=move || view! { <ActionButton text=l("Check connection", "Перевірити з’єднання") on_click=b.action("check connection")/> }
                footer=move || view! { <small>{l("Updated just now", "Щойно оновлено")}</small> }
                style="--op-banner-background: var(--op-surface-alt); --op-banner-tone: var(--op-accent);"
            />
            </Show>
        </Example>
    }
    .into_any()
}
