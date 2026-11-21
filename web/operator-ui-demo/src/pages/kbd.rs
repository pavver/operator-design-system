//! Live examples for kbd.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn kbd() -> AnyView {
    view! {
        <Example title="Keys and shortcuts">
            <Kbd keys="Esc"/>
            <Kbd keys="Ctrl+K"/>
            <Kbd keys="Ctrl+Shift+P"/>
            <Kbd keys="Ctrl++"/>
        </Example>
        <Example title="Wide keys and arrows" note="Modifier and editing keys are wider; arrow names print as arrows.">
            <Kbd keys="Shift+Enter"/>
            <Kbd keys="Tab"/>
            <Kbd keys="Ctrl+Space"/>
            <Kbd keys="Up"/>
            <Kbd keys="Left"/>
            <Kbd keys="Down"/>
            <Kbd keys="Right"/>
        </Example>
        <Example title="In text">
            <span>"Press " <Kbd keys="F2"/> " to rename the selected camera."</span>
        </Example>
        <Example title="Customization" note="Metric props and custom separator slot.">
            <Kbd
                keys="Ctrl+Alt+Del"
                height=8.0
                padding_x=2.0
                font_size=13.0
                separator=Slot::from(|| view! { <span style="color:var(--op-accent);margin:0 4px;">"•"</span> })
                aria_label="Control Alt Delete shortcut"
            />
        </Example>
    }
    .into_any()
}
