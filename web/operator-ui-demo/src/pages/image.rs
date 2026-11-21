//! Live examples for Image.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn image() -> AnyView {
    let valid_src = RwSignal::new("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='400' height='225' viewBox='0 0 400 225'><rect width='400' height='225' fill='%232563eb'/><text x='200' y='115' font-family='sans-serif' font-size='20' fill='%23ffffff' text-anchor='middle'>Operator UI Asset</text></svg>".to_string());
    let broken_src = RwSignal::new("https://invalid.domain.example/nonexistent.png".to_string());

    view! {
        <Example title="Standard image with 16:9 ratio" note="Maintains 16:9 proportion and renders caption overlay." block=true>
            <div class="demo-stack demo-narrow" style="max-width:calc(var(--op-du) * 80);">
                <Image
                    src=valid_src
                    alt="Sample valid image"
                    ratio=1.7778
                    caption="Primary Display Telemetry"
                />
            </div>
        </Example>
        <Example title="Broken source with error fallback" note="Shows error icon and message when download fails." block=true>
            <div class="demo-stack demo-narrow" style="max-width:calc(var(--op-du) * 80);">
                <Image
                    src=broken_src
                    alt="Broken image link"
                    ratio=1.7778
                />
            </div>
        </Example>
    }
    .into_any()
}
