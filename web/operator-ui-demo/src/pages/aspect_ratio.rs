//! Live examples for AspectRatio.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn aspect_ratio() -> AnyView {
    view! {
        <Example
            title="16:9 widescreen box"
            note="Maintains standard widescreen proportions."
            block=true
        >
            <div
                class="demo-stack demo-narrow"
                style="max-width:calc(var(--op-du) * 80);"
            >
                <AspectRatio ratio=1.7778>
                    <div style="background-color:var(--op-surface-raised);border:calc(var(--op-du) * 0.25) solid var(--op-border);display:flex;align-items:center;justify-content:center;">
                        "16 : 9 Ratio"
                    </div>
                </AspectRatio>
            </div>
        </Example>
        <Example
            title="1:1 square box"
            note="Square aspect ratio."
            block=true
        >
            <div
                class="demo-stack demo-narrow"
                style="max-width:calc(var(--op-du) * 40);"
            >
                <AspectRatio ratio=1.0>
                    <div style="background-color:var(--op-surface-subtle);border:calc(var(--op-du) * 0.25) solid var(--op-border);display:flex;align-items:center;justify-content:center;">
                        "1 : 1 Square"
                    </div>
                </AspectRatio>
            </div>
        </Example>
    }
    .into_any()
}
