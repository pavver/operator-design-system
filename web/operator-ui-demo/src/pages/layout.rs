//! Examples for layout components.

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
                    end=ViewFn::from(move || view! { <div style:width=du(45.0)><SearchField value=query placeholder="Filter"/></div> })
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
    }
    .into_any()
}

pub fn split_pane() -> AnyView {
    let ratio = RwSignal::new(0.35f32);
    let list = ViewFn::from(|| {
        view! {
            <div class="demo-pane">
                {["Gate North", "Gate South", "Yard East", "Parking A"]
                    .into_iter()
                    .map(|c| view! { <ListItem title=c/> })
                    .collect_view()}
            </div>
        }
    });
    let detail = ViewFn::from(|| {
        view! {
            <div class="demo-pane demo-sheet-pad">
                <PropertyList>
                    <Property label="Camera" value="Gate North"/>
                    <Property label="Status" value="Online"/>
                </PropertyList>
            </div>
        }
    });
    let (list2, detail2) = (list.clone(), detail.clone());
    view! {
        <Example title="Horizontal" note="Drag the divider, use the arrow keys when it is focused, or double-click to reset." block=true>
            <div class="demo-split-box">
                <SplitPane start=list end=detail ratio=ratio/>
            </div>
            <span class="op-t-caption demo-muted">{move || format!("First pane: {:.0}%", ratio.get() * 100.0)}</span>
        </Example>
        <Example title="Vertical" block=true>
            <div class="demo-split-box demo-split-tall">
                <SplitPane start=detail2 end=list2 vertical=true/>
            </div>
        </Example>
    }
    .into_any()
}

pub fn scroll_area() -> AnyView {
    let rows = (1..=30)
        .map(|i| view! { <ListItem title=format!("Event {i:02}") subtitle="Motion detected · Gate North"/> })
        .collect_view();
    view! {
        <Example title="Stretch overscroll" note="Scroll to an edge and keep pulling with touch or the mouse wheel: the content stretches and springs back." block=true>
            <div class="demo-grid-2">
                <div class="demo-list">
                    <ScrollArea height=70.0 label="Events">{rows}</ScrollArea>
                </div>
                <div class="demo-list">
                    <ScrollArea height=70.0 label="Short list">
                        <ListItem title="Only two rows"/>
                        <ListItem title="Still stretches when pulled"/>
                    </ScrollArea>
                </div>
            </div>
        </Example>
    }
    .into_any()
}

pub fn scrollbar() -> AnyView {
    let rows = (1..=40)
        .map(|i| view! { <ListItem title=format!("Camera {i:02}") subtitle="Online · 1080p · H.264"/> })
        .collect_view();
    let wide = (1..=60)
        .map(|i| {
            format!(
                "{i:03} 2026-11-11T12:{:02}:00Z gate-north camera=cam-{i:02} event=motion score=0.{i:02} zone=07 bbox=[120,64,480,360] codec=h264 bitrate=4096k",
                i % 60
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    view! {
        <Example title="Vertical" note="Hover the right edge to widen the bar; drag the thumb or press the track to page." block=true>
            <div class="demo-list">
                <ScrollView height=70.0 focusable=true>{rows}</ScrollView>
            </div>
        </Example>
        <Example title="Both axes" note="Each bar stops short of the shared corner." block=true>
            <div class="demo-list">
                <ScrollView axis=ScrollAxis::Both height=50.0 focusable=true>
                    <pre class="demo-wide">{wide}</pre>
                </ScrollView>
            </div>
        </Example>
    }
    .into_any()
}
