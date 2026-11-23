//! Live examples for action-stack.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn action_stack() -> AnyView {
    let b = backend();
    // Wider than the trigger, so the alignment is visible.
    let long_actions = move || {
        vec![
            ActionStackItem::new(
                "Create a note for the current camera",
                b.action("create note"),
            )
            .icon(icons::ADD),
            ActionStackItem::new("Refresh every connected stream", b.action("refresh"))
                .icon(icons::REFRESH),
        ]
    };
    view! {
        <Example title="Interactive" block=true>
            <ActionStack actions=vec![
                ActionStackItem::new("Create note", b.action("create note")).icon(icons::ADD),
                ActionStackItem::new("Refresh", b.action("refresh")).icon(icons::REFRESH),
            ]/>
        </Example>
        <Example title="Long action list" block=true>
            <ActionStack label=l("Long action list", "Довгий список дій") actions=(1..=16)
                .map(|n| ActionStackItem::new(format!("Workspace action number {n}"), b.action("workspace action")))
                .collect()/>
        </Example>
        <Example title="Side and alignment" block=true>
            <div class="demo-row">
                <ActionStack label=l("Above, to the right", "Зверху, праворуч") side=LayerSide::Above align=LayerAlign::Start actions=long_actions()/>
                <ActionStack label=l("Below, to the left", "Знизу, ліворуч") align=LayerAlign::End actions=long_actions()/>
                <ActionStack label=l("Right, centered", "Праворуч, по центру") side=LayerSide::Right actions=long_actions()/>
                <ActionStack label=l("Left, upwards", "Ліворуч, угору") side=LayerSide::Left align=LayerAlign::End actions=long_actions()/>
            </div>
        </Example>
        <Example title="States" block=true>
            <ActionStack enabled=false actions=vec![]/>
        </Example>
        <Example title="Customization" block=true>
            <ActionStack label=l("Add item", "Додати елемент") size=12.0 gap=3.0 side=LayerSide::Above actions=vec![
                ActionStackItem::new("Create", b.action("create")),
            ]/>
        </Example>
    }
    .into_any()
}
