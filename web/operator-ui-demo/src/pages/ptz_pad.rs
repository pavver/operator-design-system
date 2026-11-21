//! Live examples for PtzPad.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn ptz_pad() -> AnyView {
    let enabled = RwSignal::new(true);
    let last_direction = RwSignal::new("None (Idle)".to_string());
    let last_speed = RwSignal::new(0.0f32);
    let zoom_level = RwSignal::new(1.0f32);

    let on_move = Callback::new(move |(dir, speed): (PtzDirection, f32)| {
        last_direction.set(format!("{:?}", dir));
        last_speed.set(speed);
    });

    let on_zoom = Callback::new(move |z: PtzZoomDirection| match z {
        PtzZoomDirection::In => zoom_level.update(|lvl| *lvl = (*lvl * 1.2).min(10.0)),
        PtzZoomDirection::Out => zoom_level.update(|lvl| *lvl = (*lvl / 1.2).max(1.0)),
    });

    let on_async_command = AsyncAction::new(move |_cmd: PtzCommand| async move {
        sleep_ms(250).await;
        ActionResult::Success
    });

    view! {
        <Example
            title="Interactive PTZ Dpad"
            note="Drag the joystick thumb or use keyboard arrow keys. Supports + and - for zoom."
        >
            <div style="display:flex;align-items:center;gap:calc(var(--op-du)*6);flex-wrap:wrap;">
                <PtzPad
                    enabled=enabled
                    on_move=on_move
                    on_zoom=on_zoom
                    on_async_command=on_async_command
                />

                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du)*1.5);min-width:calc(var(--op-du)*40);">
                    <div style="font-size:calc(var(--op-tu)*12);color:var(--op-text-secondary);text-transform:uppercase;letter-spacing:0.05em;">
                        "Telemetry Monitor"
                    </div>
                    <div style="padding:calc(var(--op-du)*2);background:var(--op-surface-subtle);border:calc(var(--op-du)*0.25) solid var(--op-border);">
                        <div>"Direction: " <strong>{move || last_direction.get()}</strong></div>
                        <div>"Speed: " <strong>{move || format!("{:.2}", last_speed.get())}</strong></div>
                        <div>"Zoom Level: " <strong>{move || format!("{:.1}x", zoom_level.get())}</strong></div>
                    </div>

                    <AsyncCheckbox
                        checked=enabled
                        text=Signal::stored("Controller Enabled".to_string())
                        on_change=AsyncAction::new(move |v: bool| async move {
                            enabled.set(v);
                            ActionResult::Success
                        })
                    />
                </div>
            </div>
        </Example>

        <Example
            title="Custom Pad Dimensions"
            note="Configurable size metric props without diagonal arrows."
        >
            <PtzPad
                size=30.0
                thumb_size=10.0
                show_diagonals=false
                show_zoom=false
            />
        </Example>
    }
    .into_any()
}
