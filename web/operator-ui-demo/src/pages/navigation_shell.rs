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
        <Example title="Customization" note="Metric props, header/footer slots and custom labels customize NavigationRail and SidePanel." block=true>
            <div class="demo-shell">
                <NavigationRail
                    items=vec![
                        RailItem::new("events", crate::i18n::t("Events").get_untracked(), icons::INFO),
                        RailItem::new("settings", crate::i18n::t("Settings").get_untracked(), icons::SETTINGS),
                    ]
                    selected=RwSignal::new("events".to_string())
                    width=20.0
                    item_height=14.0
                    header=Slot::from(|| view! { <div class="op-t-caption demo-muted" style="padding: 4px; text-align: center;">"App"</div> })
                    footer=Slot::from(|| view! { <div class="op-t-caption demo-muted" style="padding: 4px; text-align: center;">"v1.0"</div> })
                    label=crate::i18n::t("Navigation")
                />
                <SidePanel
                    title=crate::i18n::t("Inspector")
                    collapsed=RwSignal::new(false)
                    width=60.0
                    header_height=12.0
                    collapse_label=crate::i18n::t("Collapse")
                    footer=Slot::from(|| view! { <div class="op-t-caption demo-muted" style="padding: 8px;">"Ready"</div> })
                >
                    <div style="padding: 8px;">"Panel content"</div>
                </SidePanel>
                <div class="demo-shell-main op-t-caption demo-muted">
                    "Main area"
                </div>
            </div>
        </Example>
    }
    .into_any()
}
