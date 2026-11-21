//! Live examples for LauncherTile.

use crate::kit::{Cell, Example, STATES, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn launcher_tile() -> AnyView {
    let b = backend();
    let enabled = RwSignal::new(true);
    let selected = RwSignal::new(false);

    view! {
        <Example
            title="Application Launchpad Grid"
            note="Square 24 DU tiles for quick actions and application launching."
        >
            <div style="display:flex;flex-wrap:wrap;gap:calc(var(--op-du)*2);">
                <LauncherTile
                    label="Cameras"
                    icon=icons::VIDEOCAM
                    on_async_click=b.action("open cameras")
                    enabled=enabled
                />
                <LauncherTile
                    label="Settings"
                    icon=icons::SETTINGS
                    on_async_click=b.action("open settings")
                    enabled=enabled
                />
                <LauncherTile
                    label="Terminal"
                    icon=icons::EDIT
                    on_async_click=b.action("open terminal")
                    enabled=enabled
                />
                <LauncherTile
                    label="Refresh Telemetry"
                    icon=icons::REFRESH
                    on_async_click=b.action("refresh telemetry")
                    enabled=enabled
                />
            </div>
            <div style="margin-top:calc(var(--op-du)*2);">
                <AsyncCheckbox
                    checked=enabled
                    text=Signal::stored("Enabled".to_string())
                    on_change=AsyncAction::new(move |v: bool| async move {
                        enabled.set(v);
                        ActionResult::Success
                    })
                />
            </div>
        </Example>

        <Example
            title="Selectable and Customization"
            note="Configurable metric sizes and notification badge slot."
        >
            <div style="display:flex;flex-wrap:wrap;gap:calc(var(--op-du)*2);">
                <LauncherTile
                    label="Inbox"
                    icon=icons::INBOX
                    selected=selected
                    on_click=Callback::new(move |()| selected.update(|s| *s = !*s))
                    badge_slot=|| view! {
                        <span style="background:var(--op-accent);color:#fff;font-size:calc(var(--op-tu)*10);font-weight:700;padding:calc(var(--op-du)*0.5) calc(var(--op-du)*1.5);">
                            "4"
                        </span>
                    }
                />
                <LauncherTile
                    label="Compact Tile"
                    icon=icons::SEARCH
                    size=18.0
                    icon_size=6.0
                    on_async_click=b.action("compact tile clicked")
                />
            </div>
        </Example>

        <Example title="Static States">
            <div style="display:flex;flex-wrap:wrap;gap:calc(var(--op-du)*2);">
                {STATES.into_iter().map(|(name, s)| view! {
                    <Cell label=name>
                        <LauncherTile label="Tile" icon=icons::SETTINGS state=s />
                    </Cell>
                }).collect_view()}
                <Cell label="Disabled">
                    <LauncherTile label="Tile" icon=icons::SETTINGS enabled=false />
                </Cell>
            </div>
        </Example>

        <Example title="Navigation Link Mode" note="Renders as a semantic anchor link with href.">
            <div style="display:flex;gap:calc(var(--op-du)*2);">
                <LauncherTile
                    label="Docs"
                    icon=icons::OPEN_IN_NEW
                    href=Signal::stored("#/platforms/web".to_string())
                />
            </div>
        </Example>
    }
    .into_any()
}
