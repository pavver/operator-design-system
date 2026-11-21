//! Live examples for action button.

use crate::kit::{Cell, Example, STATES, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn action_button() -> AnyView {
    let b = backend();
    let enabled = RwSignal::new(true);
    view! {
        <Example title="Interactive" note="Async action with a pending running border, a 500 ms result icon flash and a 300 ms tail. Clicks are blocked while pending.">
            <ActionButton text="Restart stream" on_click=b.action("restart stream") enabled=enabled/>
            <ActionButton text="Sync" icon=icons::REFRESH on_click=b.action("sync presets") enabled=enabled/>
            <ActionButton text="Save" icon=icons::CHECK keep_flash_icon_on_success=true on_click=b.action("save settings") enabled=enabled/>
            <ActionButton text="Delete camera" dangerous=true on_click=b.action("delete camera") enabled=enabled/>
            <ActionButton text="No pending border" show_pending_border=false on_click=b.action("silent request") enabled=enabled/>
            <AsyncCheckbox
                checked=enabled
                text=Signal::stored("Enabled".to_string())
                on_change=AsyncAction::new(move |v: bool| async move {
                    enabled.set(v);
                    ActionResult::Success
                })
            />
        </Example>
        <Example title="Static states">
            {STATES.into_iter().map(|(name, s)| view! { <Cell label=name><ActionButton text="Apply" state=s/></Cell> }).collect_view()}
            <Cell label="Disabled"><ActionButton text="Apply" enabled=false/></Cell>
            <Cell label="Danger"><ActionButton text="Delete" dangerous=true/></Cell>
            <Cell label="Danger pressed"><ActionButton text="Delete" dangerous=true state=ControlState::Pressed/></Cell>
        </Example>
        <Example title="Long text">
            <div class="demo-narrow">
                <ActionButton text="Request a full device configuration export" on_click=b.action("export config") style="width: 100%"/>
            </div>
        </Example>
        <Example title="Link mode" note="With href the button renders a real link: crawlers follow it, and middle-click, Ctrl+click, and the context menu work. Only Idle, Pressed, and Disabled apply.">
            <ActionButton text="Open icon button" href="#/components/icon-button"/>
            <ActionButton text="Source code" icon=icons::OPEN_IN_NEW href="https://github.com/pavver/operator-design-system" new_tab=true/>
            <ActionButton text="Disabled link" href="#/components/icon-button" enabled=false/>
        </Example>
    }
    .into_any()
}
