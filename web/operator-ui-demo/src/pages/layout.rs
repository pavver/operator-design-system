//! Examples for layout components.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn toolbar() -> AnyView {
    let b = backend();
    let query = RwSignal::new(String::new());
    view! {
        <Example title="Command bar" note="Arrow keys move focus between the controls. Drag the bottom-right corner to narrow the bar: it scrolls sideways instead of wrapping." block=true>
            <div class="demo-resize">
            <Toolbar label="Recordings">
                <ToolbarTitle text="Recordings"/>
                <ToolbarSeparator/>
                <IconButton icon=icons::REFRESH label="Refresh" on_async_click=b.action("refresh recordings")/>
                <IconButton icon=icons::FILTER_LIST label="Filter" on_click=Callback::new(|()| {})/>
                <IconButton icon=icons::DOWNLOAD label="Download" on_async_click=b.action("download")/>
                <ToolbarSpacer/>
                <div style:width=du(50.0)><SearchField value=query placeholder="Filter"/></div>
                <MenuButton
                    items=vec![MenuItem::new("select", "Select all"), MenuItem::new("delete", "Delete selected").danger().separated()]
                    on_select=Callback::new(move |id: String| b.log(format!("[UI] toolbar menu: {id}")))
                />
            </Toolbar>
            </div>
        </Example>
        <Example title="Dense" block=true>
            <Toolbar dense=true>
                <IconButton icon=icons::ADD label="Add" size=6.0 on_click=Callback::new(|()| {})/>
                <IconButton icon=icons::REMOVE label="Remove" size=6.0 on_click=Callback::new(|()| {})/>
                <ToolbarSeparator/>
                <IconButton icon=icons::CONTENT_COPY label="Copy" size=6.0 on_click=Callback::new(|()| {})/>
            </Toolbar>
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
