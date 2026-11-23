//! Live examples for fab.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn fab() -> AnyView {
    let b = backend();
    // Wider than the trigger, so the alignment is visible.
    let long_actions = move || {
        vec![
            FabAction::new(
                "Create a note for the current camera",
                b.action("create note"),
            )
            .icon(icons::ADD),
            FabAction::new("Refresh every connected stream", b.action("refresh"))
                .icon(icons::REFRESH),
        ]
    };
    view! {
        <Example title="Interactive" block=true>
            <Fab actions=vec![
                FabAction::new("Create note", b.action("create note")).icon(icons::ADD),
                FabAction::new("Refresh", b.action("refresh")).icon(icons::REFRESH),
            ]/>
        </Example>
        <Example title="Long action list" block=true>
            <Fab label=l("Long action list", "Довгий список дій") actions=(1..=16)
                .map(|n| FabAction::new(format!("Workspace action number {n}"), b.action("workspace action")))
                .collect()/>
        </Example>
        <Example title="Side and alignment" block=true>
            <div class="demo-row">
                <Fab label=l("Above, to the right", "Зверху, праворуч") side=LayerSide::Above align=LayerAlign::Start actions=long_actions()/>
                <Fab label=l("Below, to the left", "Знизу, ліворуч") align=LayerAlign::End actions=long_actions()/>
                <Fab label=l("Right, centered", "Праворуч, по центру") side=LayerSide::Right actions=long_actions()/>
                <Fab label=l("Left, upwards", "Ліворуч, угору") side=LayerSide::Left align=LayerAlign::End actions=long_actions()/>
            </div>
        </Example>
        <Example title="States" block=true>
            <Fab enabled=false actions=vec![]/>
        </Example>
        <Example title="Customization" block=true>
            <Fab label=l("Quick actions", "Швидкі дії") size=12.0 gap=3.0 side=LayerSide::Above actions=vec![
                FabAction::new("Create", b.action("create")),
            ]/>
        </Example>
    }
    .into_any()
}
