//! Live examples for linear progress.

use crate::kit::Example;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui::prelude::*;

pub fn linear_progress() -> AnyView {
    let value = RwSignal::new(0.0f32);
    let state = RwSignal::new(None::<ControlState>);
    let running = RwSignal::new(false);
    let start = AsyncAction::new(move |()| async move {
        if running.get_untracked() {
            return ActionResult::Success;
        }
        running.set(true);
        state.set(None);
        value.set(0.0);
        for i in 1..=20u8 {
            sleep_ms(120).await;
            value.set(f32::from(i) / 20.0);
        }
        state.set(Some(ControlState::Success));
        running.set(false);
        spawn_local(async move {
            sleep_ms(1500).await;
            state.set(None);
        });
        ActionResult::Success
    });
    view! {
        <Example title="Determinate" block=true>
            <div class="demo-stack demo-narrow">
                <LinearProgress value=value state=state show_value=true label="Firmware upload"/>
                <div><ActionButton text="Upload firmware" on_click=start/></div>
            </div>
        </Example>
        <Example title="Indeterminate and results" block=true>
            <div class="demo-stack demo-narrow">
                <LinearProgress label="Connecting"/>
                <LinearProgress value=1.0 state=ControlState::Success show_value=true/>
                <LinearProgress value=0.42 state=ControlState::Error show_value=true/>
            </div>
        </Example>
        <Example title="Thickness and value inside" note="From 5 DU the value moves inside the bar; the text keeps contrast over both parts." block=true>
            <div class="demo-stack demo-narrow">
                <LinearProgress value=value thickness=2.0 show_value=true/>
                <LinearProgress value=value thickness=5.0 show_value=true/>
                <LinearProgress value=value thickness=8.0 state=state show_value=true/>
            </div>
        </Example>
        <Example title="Count instead of percent" block=true>
            <div class="demo-stack demo-narrow">
                <LinearProgress value=Signal::derive(move || value.get() * 1000.0) max=1000.0 format=ProgressFormat::Count show_value=true label="Files"/>
                <LinearProgress value=543.0 max=1000.0 format=ProgressFormat::Count thickness=6.0 show_value=true label="Files"/>
            </div>
        </Example>
        <Example title="Customization" note="Custom thickness, gap, and value slot." block=true>
            <div class="demo-stack demo-narrow">
                <LinearProgress
                    value=0.75
                    thickness=4.0
                    gap=4.0
                    show_value=true
                    label="Custom progress bar"
                    value_slot=Slot::from(|| view! { <span style="color:var(--op-accent);font-weight:bold;">"3/4 Complete"</span> })
                />
            </div>
        </Example>
    }
    .into_any()
}
