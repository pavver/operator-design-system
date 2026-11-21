//! Live examples for header.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn header() -> AnyView {
    let b = backend();
    view! {
        <Example title="Variants" block=true>
            <div class="demo-stack">
                <Header title="Settings" on_back=Callback::new(move |()| b.log("[UI] back"))/>
                <Header
                    title="Gate camera"
                    subtitle="online · 12 ms"
                    on_back=Callback::new(move |()| b.log("[UI] back"))
                    avatar=HeaderAvatar { id: "cam-3".into(), name: "Gate".into(), group: false, status: Some(PresenceStatus::Online) }
                    actions=ViewFn::from(move || {
                        view! {
                            <IconButton icon=icons::VIDEOCAM label="Live" on_click=Callback::new(move |()| b.log("[UI] live"))/>
                            <IconButton icon=icons::MORE_VERT label="More" on_click=Callback::new(move |()| b.log("[UI] more"))/>
                        }
                    })
                />
            </div>
        </Example>
        <Example title="Customization" note="Custom metrics, colors via CSS variables, translated strings and replaceable icons." block=true>
            <div class="demo-stack">
                <Header
                    title="Custom Header"
                    subtitle="Custom height and back label"
                    height=16.0
                    back_label="Go back"
                    on_back=Callback::new(move |()| b.log("[UI] custom back"))
                    actions=Slot::from(move || {
                        view! {
                            <IconButton icon=icons::REFRESH label="Refresh" on_click=Callback::new(move |()| b.log("[UI] refresh"))/>
                        }
                    })
                />
            </div>
        </Example>
    }
    .into_any()
}
