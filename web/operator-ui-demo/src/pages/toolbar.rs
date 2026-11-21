//! Live examples for toolbar.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn toolbar() -> AnyView {
    let b = backend();
    let query = RwSignal::new(String::new());
    let log =
        move |what: &'static str| Callback::new(move |()| b.log(format!("[UI] toolbar: {what}")));
    let items = move || {
        vec![
            ToolbarItem::new("refresh", "Refresh", icons::REFRESH)
                .on_async(b.action("refresh recordings")),
            ToolbarItem::new("filter", "Filter", icons::FILTER_LIST).on_click(log("filter")),
            ToolbarItem::new("download", "Download", icons::DOWNLOAD)
                .on_async(b.action("download")),
            ToolbarItem::new("copy", "Copy link", icons::CONTENT_COPY)
                .on_click(log("copy link"))
                .separated(),
            ToolbarItem::new("edit", "Rename", icons::EDIT).on_click(log("rename")),
            ToolbarItem::new("settings", "Settings", icons::SETTINGS)
                .on_click(log("settings"))
                .disabled(),
            ToolbarItem::new("select", "Select all", icons::CHECK)
                .on_click(log("select all"))
                .overflow(),
            ToolbarItem::new("delete", "Delete selected", icons::DELETE)
                .on_async(b.action("delete selected"))
                .danger()
                .separated()
                .overflow(),
        ]
    };
    view! {
        <Example
            title="Command bar"
            note="Arrow keys move focus between the controls. Drag the bottom-right corner to narrow the bar: commands that do not fit move into the overflow menu."
            block=true
        >
            <div class="demo-resize">
                <Toolbar
                    label="Recordings"
                    title="Recordings"
                    items=items()
                    end=ViewFn::from(move || view! {
                        <div style:width=du(45.0)>
                            <SearchField value=query placeholder="Filter"/>
                        </div>
                    })
                />
            </div>
        </Example>
        <Example title="Dense" block=true>
            <div class="demo-resize">
                <Toolbar
                    dense=true
                    items=vec![
                        ToolbarItem::new("add", "Add", icons::ADD).on_click(log("add")),
                        ToolbarItem::new("remove", "Remove", icons::REMOVE).on_click(log("remove")),
                        ToolbarItem::new("copy", "Copy", icons::CONTENT_COPY).on_click(log("copy")).separated(),
                    ]
                />
            </div>
        </Example>
        <Example
            title="Customization"
            note="Custom metrics, start/end slots, and translated strings."
            block=true
        >
            <div class="demo-resize">
                <Toolbar
                    label=crate::i18n::t("Toolbar")
                    items=vec![
                        ToolbarItem::new("refresh", "Refresh", icons::REFRESH),
                        ToolbarItem::new("settings", "Settings", icons::SETTINGS),
                    ]
                    height=12.0
                    gap=2.0
                    padding_x=2.0
                    start=Slot::from(|| view! {
                        <span
                            class="op-t-caption demo-muted"
                            style="padding-right: calc(var(--op-du) * 2);"
                        >
                            "Tools"
                        </span>
                    })
                    more_label=crate::i18n::t("More commands")
                />
            </div>
        </Example>
    }
    .into_any()
}
