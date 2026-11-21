//! Live examples for list item.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn list_item() -> AnyView {
    let b = backend();
    let alerts = RwSignal::new(true);
    let selected = RwSignal::new(false);
    view! {
        <Example title="Rows" block=true>
            <div class="demo-list">
                <ListItem title="Stream quality" trailing=ViewFn::from(|| view! { <span class="op-t-body demo-muted">"HD"</span> })/>
                <ListItem
                    title="Motion alerts"
                    subtitle="Push a notification when motion is detected in an armed zone"
                    trailing=ViewFn::from(move || {
                        view! {
                            <AsyncSwitch
                                checked=alerts
                                on_toggle=AsyncAction::new(move |v: bool| async move {
                                    let r = b.call(format!("alerts={v}")).await;
                                    if r.is_success() {
                                        alerts.set(v);
                                    }
                                    r
                                })
                            />
                        }
                    })
                />
                <ListItem
                    title="Gate camera"
                    subtitle="Online · 12 ms · H.265"
                    leading=ViewFn::from(|| view! { <Avatar id="cam-3" name="Gate" kind=AvatarKind::ThreadedGroup status=PresenceStatus::Online/> })
                    trailing=ViewFn::from(|| view! { <Icon icon=icons::CHEVRON_RIGHT size=dim::ICON_SMALL/> })
                    selected=selected
                    on_click=Callback::new(move |()| selected.update(|s| *s = !*s))
                />
                <ListItem
                    title="Olena Kovalenko"
                    subtitle="Typing…"
                    leading=ViewFn::from(|| view! { <Avatar id="olena" name="Olena" status=PresenceStatus::Typing/> })
                    on_click=Callback::new(move |()| b.log("[UI] open chat"))
                />
                <ListItem title="Firmware update" subtitle="Not available for this model" enabled=false/>
            </div>
        </Example>

        <Example title="Customization" note="Metric props, a content slot and application strings." block=true>
            <ThemeRoot strings=Callback::new(|key| (key == "demo.list-item.title").then(|| "Display preferences".to_string()))>
                <ListItem
                    title=ui_text(MaybeProp::default(), "demo.list-item.title", "Settings")
                    padding_x=5.0 padding_y=3.0 min_height=14.0
                    leading=|| view! { <Icon icon=icons::SETTINGS/> }
                    content=|| { let title = ui_text(MaybeProp::default(), "demo.list-item.title", "Settings"); view! { <strong>{move || title.get()}</strong> } }
                />
            </ThemeRoot>
        </Example>
    }
    .into_any()
}
