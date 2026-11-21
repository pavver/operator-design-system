//! Live examples for PttButton (push-to-talk).

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn ptt_button() -> AnyView {
    let is_talking_1 = RwSignal::new(false);
    let is_locked_1 = RwSignal::new(false);

    let is_talking_2 = RwSignal::new(false);
    let is_locked_2 = RwSignal::new(false);

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 120);">
            <Example
                title="Interactive Push-To-Talk"
                note="Hold to speak, slide up to lock hands-free, tap when locked to terminate transmission."
                block=true
            >
                <div style="display:flex;align-items:center;justify-content:center;padding:calc(var(--op-du) * 4);width:100%;">
                    <PttButton
                        talking=is_talking_1
                        locked=is_locked_1
                        on_state_change=Callback::new(move |(talking, locked)| {
                            is_talking_1.set(talking);
                            is_locked_1.set(locked);
                        })
                    />
                </div>
            </Example>

            <Example
                title="Circular Geometry & States"
                note="Circular round exception, success lock, and disabled state."
                block=true
            >
                <div style="display:flex;gap:calc(var(--op-du) * 4);align-items:center;justify-content:center;padding:calc(var(--op-du) * 4);flex-wrap:wrap;">
                    <PttButton
                        circular=true
                        talking=is_talking_2
                        locked=is_locked_2
                        on_state_change=Callback::new(move |(talking, locked)| {
                            is_talking_2.set(talking);
                            is_locked_2.set(locked);
                        })
                    />

                    <PttButton
                        locked=true
                        label="LOCKED"
                    />

                    <PttButton
                        disabled=true
                        label="OFFLINE"
                    />
                </div>
            </Example>
        </div>
    }.into_any()
}
