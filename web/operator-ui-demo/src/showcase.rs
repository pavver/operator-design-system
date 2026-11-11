//! Showcase: one device panel built from real components, in dark and light mode side by side.
//! Opened at `#/showcase`; used for the README screenshot.

use leptos::prelude::*;
use operator_ui::prelude::*;

const ESC: &str = "\u{1b}";

fn log_lines() -> Vec<String> {
    let dim = |t: &str| format!("{ESC}[2m{t}{ESC}[0m");
    vec![
        format!("{} ptz   preset=03 pan=-12.5 tilt=4.0", dim("14:02:11")),
        format!(
            "{} {ESC}[36msync  pending  rtt=184ms{ESC}[0m",
            dim("14:02:12")
        ),
        format!(
            "{} {ESC}[32msync  ok       ack=0x4F2A{ESC}[0m",
            dim("14:02:12")
        ),
        format!(
            "{} {ESC}[31mstream timeout  retry 1/3{ESC}[0m",
            dim("14:02:19")
        ),
    ]
}

#[component]
fn Section(#[prop(into)] title: String) -> impl IntoView {
    view! { <div class="sc-section op-t-caption">{title}</div> }
}

#[component]
fn Panel() -> impl IntoView {
    let night = RwSignal::new(true);
    let autofocus = RwSignal::new(true);
    let brightness = RwSignal::new(0.64_f32);
    let preset = RwSignal::new(3);
    let profile = RwSignal::new(1_usize);
    let name = RwSignal::new("Gate-North".to_string());
    let logs = Signal::derive(log_lines);
    let pct = move || format!("{:.0}%", brightness.get() * 100.0);

    view! {
        <div class="sc-panel">
            <Header
                title="Camera 3 · Gate"
                subtitle="ID 0x4F2A · 12 ms"
                on_back=Callback::new(|()| {})
                actions=ViewFn::from(|| view! { <StatusBadge status=PresenceStatus::Online/> })
            />
            <Section title="Image"/>
            <div class="sc-list">
                <ListItem
                    title="Night mode"
                    subtitle="IR light after sunset"
                    trailing=ViewFn::from(move || view! { <AsyncSwitch checked=night/> })
                />
                <ListItem
                    title="Autofocus"
                    trailing=ViewFn::from(move || view! { <AsyncCheckbox checked=autofocus/> })
                />
                <ListItem
                    title="Brightness"
                    trailing=ViewFn::from(move || view! {
                        <div class="sc-slider">
                            <span class="op-t-caption sc-muted">{pct}</span>
                            <Slider value=brightness/>
                        </div>
                    })
                />
                <ListItem
                    title="PTZ preset"
                    trailing=ViewFn::from(move || view! { <Stepper value=preset min=1 max=16 width=25.0/> })
                />
                <ListItem
                    title="Profile"
                    trailing=ViewFn::from(move || view! {
                        <div class="sc-seg">
                            <SegmentedControl
                                items=vec![SegmentItem::text("Auto"), SegmentItem::text("Day"), SegmentItem::text("Night")]
                                selected=profile
                            />
                        </div>
                    })
                />
            </div>
            <div class="sc-body">
                <div class="sc-buttons">
                    <ActionButton text="Restart" icon=icons::REFRESH/>
                    <ActionButton text="Syncing…" state=ControlState::Pending/>
                    <ActionButton text="Saved" state=ControlState::Success/>
                    <ActionButton text="Timeout" state=ControlState::Error/>
                    <ActionButton text="Delete" dangerous=true/>
                    <ActionButton text="Unavailable" enabled=false/>
                </div>
                <TextField value=name placeholder="Device name" label="Device name"/>
            </div>
            <Section title="Log"/>
            <div class="sc-body">
                <ConsoleTerminal logs=logs height=26.0/>
            </div>
        </div>
    }
}

/// Dark and light copies of the panel.
#[component]
pub fn Showcase() -> impl IntoView {
    view! {
        <div class="sc-root">
            <ThemeRoot dark=RwSignal::new(true) class="sc-theme"><Panel/></ThemeRoot>
            <ThemeRoot dark=RwSignal::new(false) class="sc-theme"><Panel/></ThemeRoot>
        </div>
    }
}
