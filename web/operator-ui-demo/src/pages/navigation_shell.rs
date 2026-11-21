//! Live examples for navigation shell.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

fn demo_tree() -> Vec<TreeNode> {
    use operator_ui::tokens::icons;
    (1..=20)
        .map(|area| {
            TreeNode::new(format!("area{area}"), format!("Area {area}"))
                .icon(icons::INBOX)
                .detail("100")
                .children(
                    (1..=100)
                        .map(|i| {
                            TreeNode::new(
                                format!("area{area}.sensor{i}"),
                                format!("sensor.area{area}_{i:03}"),
                            )
                            .detail("°C")
                        })
                        .collect(),
                )
        })
        .collect()
}

pub fn navigation_shell() -> AnyView {
    use operator_ui::tokens::icons;
    let section = RwSignal::new("events".to_string());
    let rail = vec![
        RailItem::new("events", "Events", icons::INFO).color("#f2b134"),
        RailItem::new("control", "Control", icons::FILTER_LIST).color("#e8743b"),
        RailItem::new("time", "Time", icons::REFRESH).color("#5b7cfa"),
        RailItem::new("data", "Data", icons::INBOX),
        RailItem::new("debug", "Debug", icons::SETTINGS),
    ];
    let collapsed = RwSignal::new(false);
    let expanded = RwSignal::new(std::collections::HashSet::from(["area1".to_string()]));
    let selected = RwSignal::new(None::<String>);
    let nodes = Signal::stored(demo_tree());
    view! {
        <Example title="Rail, side panel and tree" note="2 000 tree rows, virtualized. Arrows navigate the tree; the chevron collapses the panel." block=true>
            <div class="demo-shell">
                <NavigationRail items=rail selected=section/>
                <SidePanel title=Signal::derive(move || format!("Section: {}", section.get())) collapsed=collapsed>
                    <Tree nodes=nodes expanded=expanded selected=selected height=Some(70.0) label="Entities"/>
                </SidePanel>
                <div class="demo-shell-main op-t-caption demo-muted">
                    {move || format!("Selected: {}", selected.get().unwrap_or_default())}
                </div>
            </div>
        </Example>

        <Example title="Customization" note="Tree row metrics, a scoped node label and an application accessible name." block=true>
            <ThemeRoot strings=Callback::new(|key| (key == "tree.label").then(|| "Browse devices".to_string()))>
                <Tree nodes=vec![TreeNode::new("custom", "Devices").children(vec![TreeNode::new("camera", "Camera")])]
                    expanded=RwSignal::new(std::collections::HashSet::from(["custom".to_string()]))
                    selected=RwSignal::new(None::<String>) height=Some(30.0)
                    row_height=12.0 indent=6.0 gap=2.0
                    option=Callback::new(|row: TreeRow| view! { <strong>{row.label}</strong> }.into_any())
                    footer=|| view! { <span class="op-t-caption">"Application footer"</span> }/>
            </ThemeRoot>
        </Example>
    }
    .into_any()
}
