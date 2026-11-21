//! Live examples for CircularProgress.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn circular_progress() -> AnyView {
    let value = RwSignal::new(65.0f32);

    view! {
        <Example title="Determinate with center value" note="Shows percentage completion inside the ring." block=true>
            <div class="demo-stack demo-narrow" style="display:flex;gap:calc(var(--op-du) * 4);align-items:center;">
                <CircularProgress value=value show_value=true />
                <CircularProgress value=value show_value=true format=CircularProgressFormat::Count max=100.0 />
            </div>
        </Example>
        <Example title="Indeterminate and state tones" note="Spinning mode with success and error indicators." block=true>
            <div class="demo-stack demo-narrow" style="display:flex;gap:calc(var(--op-du) * 4);align-items:center;">
                <CircularProgress />
                <CircularProgress value=100.0 state=ControlState::Success show_value=true />
                <CircularProgress value=42.0 state=ControlState::Error show_value=true />
            </div>
        </Example>
        <Example title="Custom size and thickness" note="Custom dimensions in Design Units." block=true>
            <div class="demo-stack demo-narrow" style="display:flex;gap:calc(var(--op-du) * 4);align-items:center;">
                <CircularProgress value=value size=16.0 thickness=1.5 show_value=true />
                <CircularProgress value=value size=6.0 thickness=0.5 />
            </div>
        </Example>
    }
    .into_any()
}
