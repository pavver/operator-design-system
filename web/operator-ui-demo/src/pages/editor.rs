//! Live examples for editor.
use crate::i18n::l;
use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn editor() -> AnyView {
    let value = RwSignal::new("<p>Edit <strong>rich text</strong> here.</p>".to_string());
    view! {
        <Example title="Interactive" block=true>
            <Editor value=value/>
            <pre>{move || value.get()}</pre>
        </Example>
        <Example title="States" block=true>
            <Editor value=RwSignal::new("<p>Read only</p>".to_string()) readonly=true/>
            <Editor value=RwSignal::new(String::new()) state=Some(ControlState::Pending)/>
        </Example>
        <Example title="Customization" block=true>
            <Editor value=RwSignal::new(String::new()) label=l("Description", "Опис") min_height=28.0 padding=4.0 gap=2.0 font_size=16.0 line_height=24.0 toolbar=vec![EditorCommand::Bold, EditorCommand::Italic, EditorCommand::BulletList, EditorCommand::Undo, EditorCommand::Redo]/>
        </Example>
    }.into_any()
}
