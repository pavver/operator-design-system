//! Live examples for fab.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn fab() -> AnyView {
    let b = backend();
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
        <Example title="States" block=true>
            <Fab enabled=false actions=vec![]/>
        </Example>
        <Example title="Customization" block=true>
            <Fab label=l("Quick actions", "Швидкі дії") size=12.0 gap=3.0 below=true actions=vec![
                FabAction::new("Create", b.action("create")),
            ]/>
        </Example>
    }
    .into_any()
}
