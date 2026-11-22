//! Live examples for parallax.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

// Offline SVG data URI for parallax background
const PARALLAX_SVG: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='800' height='600'><defs><linearGradient id='g' x1='0' y1='0' x2='1' y2='1'><stop offset='0%' stop-color='%23101827'/><stop offset='50%' stop-color='%231f293d'/><stop offset='100%' stop-color='%230f172a'/></linearGradient></defs><rect width='100%' height='100%' fill='url(%23g)'/><circle cx='200' cy='150' r='100' fill='%231e3a8a' opacity='0.4'/><circle cx='600' cy='450' r='150' fill='%233b82f6' opacity='0.2'/></svg>";

pub fn parallax() -> AnyView {
    view! {
        <Example title="Parallax Banner" note="Background image moves at fractional scroll speed; respects prefers-reduced-motion." block=true>
            <div style="width: 100%; max-width: 600px; height: 350px; overflow-y: auto; border: var(--op-dim-border) solid var(--op-border);">
                <div style="height: 150px; padding: 16px; background: var(--op-surface);">
                    <span style="font-size: var(--op-type-caption-size); color: var(--op-text-secondary);">
                        "Scroll down to see the parallax effect..."
                    </span>
                </div>
                <Parallax src=PARALLAX_SVG height=50.0 speed=0.4>
                    <div style="color: #fff; text-shadow: 0 2px 4px rgba(0,0,0,0.8); text-align: center;">
                        <span style="font-weight: 700; font-size: var(--op-type-section-title-size); display: block;">
                            "Deep Space Exploration"
                        </span>
                        <span style="font-size: var(--op-type-caption-size); opacity: 0.9;">
                            "Autonomous observation array"
                        </span>
                    </div>
                </Parallax>
                <div style="height: 400px; padding: 16px; background: var(--op-surface);">
                    <p style="color: var(--op-text-secondary); font-size: var(--op-type-caption-size);">
                        "Additional page content following the parallax hero unit."
                    </p>
                </div>
            </div>
        </Example>
    }
    .into_any()
}
