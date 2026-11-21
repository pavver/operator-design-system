//! Live examples for icon button.

use crate::kit::{Cell, Example, STATES, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn icon_button() -> AnyView {
    let b = backend();
    let clicks = RwSignal::new(0u32);
    view! {
        <Example title="Interactive">
            <IconButton
                icon=icons::SETTINGS
                label="Settings"
                on_click=Callback::new(move |()| {
                    clicks.update(|c| *c += 1);
                    b.log(format!("[UI] settings clicked ({})", clicks.get_untracked()));
                })
            />
            <IconButton icon=icons::REFRESH label="Refresh" on_async_click=b.action("refresh")/>
            <IconButton icon=icons::DELETE label="Delete" on_async_click=b.action("delete item")/>
            <IconButton icon=icons::VIDEOCAM label="Camera" size=10.0 icon_size=5.0 on_async_click=b.action("open camera")/>
            <IconButton icon=icons::MORE_VERT label="More" size=6.0 icon_size=4.0 on_click=Callback::new(|()| {})/>
        </Example>
        <Example title="Static states">
            {STATES.into_iter().map(|(name, s)| view! { <Cell label=name><IconButton icon=icons::SEARCH label="Search" state=s/></Cell> }).collect_view()}
            <Cell label="Disabled"><IconButton icon=icons::SEARCH label="Search" enabled=false/></Cell>
            <Cell label="Loading"><IconButton icon=icons::SEARCH label="Search" loading=true/></Cell>
        </Example>
        <Example title="Link mode" note="With href the button renders a real link styled as an icon button.">
            <IconButton icon=icons::OPEN_IN_NEW label="Source code" href="https://github.com/pavver/operator-design-system" new_tab=true/>
            <IconButton icon=icons::SETTINGS label="Action button page" href="#/components/action-button"/>
            <IconButton icon=icons::SETTINGS label="Disabled link" href="#/components/action-button" enabled=false/>
        </Example>
    }
    .into_any()
}
