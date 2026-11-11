//! Examples for feedback components.

use crate::kit::{Cell, Example, backend, instant};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui::prelude::*;

pub fn tooltip() -> AnyView {
    view! {
        <Example title="On icon buttons" note="Hover for 500 ms, focus with Tab, or long-press on touch.">
            <Tooltip text="Refresh presets"><IconButton icon=icons::REFRESH label="Refresh" on_click=Callback::new(|()| {})/></Tooltip>
            <Tooltip text="Camera settings"><IconButton icon=icons::SETTINGS label="Settings" on_click=Callback::new(|()| {})/></Tooltip>
            <Tooltip text="Delete the selected recording from the device"><IconButton icon=icons::DELETE label="Delete" on_click=Callback::new(|()| {})/></Tooltip>
        </Example>
        <Example title="On text">
            <Tooltip text="Round-trip time to the device">
                <span class="docs-dotted" tabindex="0">"RTT 12 ms"</span>
            </Tooltip>
        </Example>
    }
    .into_any()
}

const LONG_ERROR: &str = "Push preset failed: camera 03 (Gate North, 192.168.1.40) rejected the request with HTTP 409 Conflict because another operator is editing the same PTZ preset. The previous preset is still active. Close the editor on the other workstation or wait until the lock expires in 5 minutes, then try again. Request id 7f3c9a12-4b8e-4d21-9a6f-2c1e5b7d8f90.";
const LONG_WARNING: &str = "Recording sync is behind by 47 minutes on 6 of 12 cameras. The storage node at the north site answers slowly and the upload queue holds 1.8 GB. Recent footage stays on the cameras until the queue drains.";

pub fn toast() -> AnyView {
    let b = backend();
    let toaster = use_toaster();
    let show = move |tone: Tone, text: &'static str| {
        Callback::new(move |()| {
            if let Some(t) = toaster {
                t.message(tone, text);
            }
        })
    };
    let undo = Callback::new(move |()| {
        if let Some(t) = toaster {
            t.show(
                ToastSpec::new(Tone::Info, "Recording moved to trash")
                    .with_action("Undo", Callback::new(move |()| b.log("[UI] undo delete"))),
            );
        }
    });
    let failing = AsyncAction::new(move |()| async move {
        let r = b.call("push preset".into()).await;
        if let Some(t) = toaster {
            t.report(&r, "Push preset");
            if r.is_success() {
                t.message(Tone::Success, "Preset pushed to 3 cameras");
            }
        }
        r
    });
    // The stack position is global; restore the default when leaving the page.
    on_cleanup(move || {
        if let Some(t) = toaster {
            t.set_position(ToastPosition::default());
        }
    });
    let positions = ToastPosition::ALL
        .into_iter()
        .map(|p| {
            let place = instant(move || {
                if let Some(t) = toaster {
                    t.set_position(p);
                    t.message(Tone::Info, format!("Toast at {}", p.as_str()));
                }
            });
            view! { <ActionButton text=p.as_str() on_click=place/> }
        })
        .collect_view();
    view! {
        <Example title="Position" note="Set on ToastHost or at runtime with set_position. Top stacks put the newest toast nearest the edge.">
            {positions}
        </Example>
        <Example title="Tones">
            <ActionButton text="Info" on_click=instant(move || show(Tone::Info, "Firmware 4.2 is available").run(()))/>
            <ActionButton text="Success" on_click=instant(move || show(Tone::Success, "Settings saved").run(()))/>
            <ActionButton text="Warning" on_click=instant(move || show(Tone::Warning, "Storage is 92% full").run(()))/>
            <ActionButton text="Error" on_click=instant(move || show(Tone::Error, "Camera 03 did not respond").run(()))/>
        </Example>
        <Example title="Long text" note="Checks how wrapped text, the icon and the buttons behave in a tall toast.">
            <ActionButton text="Long error" on_click=instant(move || show(Tone::Error, LONG_ERROR).run(()))/>
            <ActionButton text="Long with action" on_click=instant(move || {
                if let Some(t) = toaster {
                    t.show(
                        ToastSpec::new(Tone::Warning, LONG_WARNING)
                            .with_action("Retry", Callback::new(move |()| b.log("[UI] retry sync"))),
                    );
                }
            })/>
        </Example>
        <Example title="With an action">
            <ActionButton text="Delete recording" dangerous=true on_click=instant(move || undo.run(()))/>
        </Example>
        <Example title="Reporting an async result" note="The control shows its own result; the toast carries the error message the control cannot show.">
            <ActionButton text="Push preset" on_click=failing/>
        </Example>
    }
    .into_any()
}

pub fn banner() -> AnyView {
    let b = backend();
    let visible = RwSignal::new(true);
    view! {
        <Example title="Tones" block=true>
            <div class="demo-stack">
                <Banner tone=Tone::Info text="Recordings older than 30 days are removed automatically."/>
                <Banner tone=Tone::Success title="Connected" text="All 12 cameras are online."/>
                <Banner
                    tone=Tone::Warning
                    title="Storage almost full"
                    text="92% of the recorder disk is used. New recordings will overwrite the oldest ones."
                    actions=ViewFn::from(move || view! { <ActionButton text="Manage storage" on_click=b.action("open storage")/> })
                />
                <Show when=move || visible.get()>
                    <Banner
                        tone=Tone::Error
                        title="Server unreachable"
                        text="Changes are saved locally and will be sent when the connection returns."
                        on_dismiss=Callback::new(move |()| visible.set(false))
                    />
                </Show>
            </div>
        </Example>
    }
    .into_any()
}

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
    }
    .into_any()
}

pub fn skeleton() -> AnyView {
    let loaded = RwSignal::new(false);
    view! {
        <Example title="Shapes">
            <Cell label="Circle"><Skeleton circle=true width=10.0 height=10.0/></Cell>
            <Cell label="Block"><Skeleton width=30.0 height=17.0/></Cell>
            <Cell label="Text, 3 lines"><div style:width=du(50.0)><Skeleton lines=3/></div></Cell>
        </Example>
        <Example title="Loading a list row" block=true>
            <div class="demo-list">
                {move || {
                    if loaded.get() {
                        view! {
                            <ListItem
                                title="Gate camera"
                                subtitle="Online · 12 ms · H.265"
                                leading=ViewFn::from(|| view! { <Avatar id="cam-3" name="Gate" status=PresenceStatus::Online/> })
                            />
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="demo-skeleton-row">
                                <Skeleton circle=true width=8.0 height=8.0/>
                                <div class="demo-skeleton-text"><Skeleton lines=2/></div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
            <div>
                <ActionButton
                    text="Toggle loaded"
                    on_click=instant(move || loaded.update(|l| *l = !*l))
                />
            </div>
        </Example>
    }
    .into_any()
}

pub fn empty_state() -> AnyView {
    let b = backend();
    view! {
        <Example title="Full" block=true>
            <div class="demo-list">
                <EmptyState
                    title="No recordings yet"
                    description="Recordings appear here when motion is detected in an armed zone."
                    actions=ViewFn::from(move || view! { <ActionButton text="Arm zones" on_click=b.action("arm zones")/> })
                />
            </div>
        </Example>
        <Example title="Compact" block=true>
            <div class="demo-list">
                <EmptyState icon=icons::SEARCH title="No matches" description="Try a shorter query." compact=true/>
            </div>
        </Example>
    }
    .into_any()
}
