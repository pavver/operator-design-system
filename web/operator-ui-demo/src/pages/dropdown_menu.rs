//! Live examples for dropdown menu.

use crate::i18n::l;
use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn dropdown() -> AnyView {
    let b = backend();
    let zones: Vec<String> = (1..=24).map(|i| format!("Zone {i:02}")).collect();
    view! {
        <Example title="Variants" block=true>
            <div class="demo-grid-2">
                <DropdownMenu
                    label="Codec"
                    options=vec!["H.264".into(), "H.265".into(), "MJPEG".into()]
                    selected=RwSignal::new("H.265".to_string())
                    on_select=Callback::new(move |v: String| b.log(format!("[UI] codec={v}")))
                />
                <DropdownMenu label="Zone (scrolls)" options=zones selected=RwSignal::new("Zone 07".to_string())/>
                <DropdownMenu label="Pending" options=vec!["One".into()] selected=RwSignal::new("Loading…".to_string()) state=ControlState::Pending/>
                <DropdownMenu label="Disabled" options=vec!["One".into()] selected=RwSignal::new("Unavailable".to_string()) enabled=false/>
            </div>
        </Example>
        <Example title="Customization" block=true>
            <DropdownMenu options=vec!["Low".into(), "Medium".into(), "High".into()] selected=RwSignal::new("Medium".to_string())
                label=l("Quality", "Якість") aria_label=l("Select quality", "Виберіть якість")
                height=12.0 padding_x=4.0 row_height=10.0 max_height=30.0
                prepend=|| view! { <Icon icon=icons::INFO/> }
                option=Callback::new(|item: String| view! { <strong>{item}</strong> }.into_any())
                footer=|| view! { <span>{l("Select one option", "Виберіть один варіант")}</span> }/>
        </Example>
    }
    .into_any()
}
