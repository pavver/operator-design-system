//! Live examples for banner.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn banner() -> AnyView {
    let b = backend();
    let visible = RwSignal::new(true);
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
    }
    .into_any()
}
