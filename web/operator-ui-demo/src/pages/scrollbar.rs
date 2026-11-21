//! Live examples for scrollbar.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn scrollbar() -> AnyView {
    let rows = (1..=40)
        .map(|i| view! { <ListItem title=format!("Camera {i:02}") subtitle="Online · 1080p · H.264"/> })
        .collect_view();
    let wide = (1..=60)
        .map(|i| {
            format!(
                "{i:03} 2026-11-11T12:{:02}:00Z gate-north camera=cam-{i:02} event=motion score=0.{i:02} zone=07 bbox=[120,64,480,360] codec=h264 bitrate=4096k",
                i % 60
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let custom_rows = (1..=20)
        .map(|i| view! { <ListItem title=format!("Custom Row {i}")/> })
        .collect_view();
    view! {
        <Example title="Vertical" note="Hover the right edge to widen the bar; drag the thumb or press the track to page." block=true>
            <div class="demo-list">
                <ScrollView height=70.0 focusable=true>{rows}</ScrollView>
            </div>
        </Example>
        <Example title="Both axes" note="Each bar stops short of the shared corner." block=true>
            <div class="demo-list">
                <ScrollView axis=ScrollAxis::Both height=50.0 focusable=true>
                    <pre class="demo-wide">{wide}</pre>
                </ScrollView>
            </div>
        </Example>
        <Example title="Customization" note="Custom track size, thumb thickness, and header/footer slots." block=true>
            <div class="demo-list">
                <ScrollView
                    height=45.0
                    track_size=4.0
                    thumb_thickness=2.0
                    focusable=true
                    label="Custom scroll view"
                    header=Slot::from(move || view! { <div class="op-t-caption demo-muted demo-sheet-pad">"Top fixed slot"</div> })
                    footer=Slot::from(move || view! { <div class="op-t-caption demo-muted demo-sheet-pad">"Bottom fixed slot"</div> })
                >
                    {custom_rows}
                </ScrollView>
            </div>
        </Example>
    }
    .into_any()
}
