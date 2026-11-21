//! Live examples for Video.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn video() -> AnyView {
    let src = RwSignal::new("about:blank".to_string());

    view! {
        <Example
            title="Responsive video embed"
            note="Maintains 16:9 widescreen ratio with visible offline placeholder."
            block=true
        >
            <div
                class="demo-stack demo-narrow"
                style="max-width:calc(var(--op-du) * 80);"
            >
                <Video
                    src=src
                    title="Demo playback video"
                    ratio=1.7778
                />
            </div>
        </Example>
    }
    .into_any()
}
