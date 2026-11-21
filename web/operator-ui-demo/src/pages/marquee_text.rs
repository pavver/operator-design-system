//! Live examples for marquee text.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn marquee() -> AnyView {
    let long = "Gate camera 03 · North entrance · perimeter zone B · infrared enabled · 0x4F2A";
    view! {
        <Example title="Behavior" note="Overflowing text waits 2 s, scrolls at 7.5 DU/s and pauses 1.2 s between loops. Hover or long-press shows the full text." block=true>
            <div class="demo-stack demo-narrow">
                <div class="demo-labelled">
                    <span class="op-t-caption demo-muted">"Overflowing, row title"</span>
                    <div class="demo-marquee-box"><MarqueeText text=long class="op-t-row-title"/></div>
                </div>
                <div class="demo-labelled">
                    <span class="op-t-caption demo-muted">"Fits, stays still"</span>
                    <div class="demo-marquee-box"><MarqueeText text="Gate camera 03" class="op-t-row-title"/></div>
                </div>
                <div class="demo-labelled">
                    <span class="op-t-caption demo-muted">"Two loops, custom tooltip"</span>
                    <div class="demo-marquee-box">
                        <MarqueeText text=long class="op-t-body" iterations=2 tooltip_text="Camera 03 at the north entrance"/>
                    </div>
                </div>
            </div>
        </Example>
    }
    .into_any()
}
