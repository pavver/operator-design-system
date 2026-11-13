//! Examples for the reviewed core components.

use crate::kit::{Cell, Example, STATES, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn action_button() -> AnyView {
    let b = backend();
    let enabled = RwSignal::new(true);
    view! {
        <Example title="Interactive" note="Async action with a pending running border, a 500 ms result icon flash and a 300 ms tail. Clicks are blocked while pending.">
            <ActionButton text="Restart stream" on_click=b.action("restart stream") enabled=enabled/>
            <ActionButton text="Sync" icon=icons::REFRESH on_click=b.action("sync presets") enabled=enabled/>
            <ActionButton text="Save" icon=icons::CHECK keep_flash_icon_on_success=true on_click=b.action("save settings") enabled=enabled/>
            <ActionButton text="Delete camera" dangerous=true on_click=b.action("delete camera") enabled=enabled/>
            <ActionButton text="No pending border" show_pending_border=false on_click=b.action("silent request") enabled=enabled/>
            <AsyncCheckbox
                checked=enabled
                text=Signal::stored("Enabled".to_string())
                on_change=AsyncAction::new(move |v: bool| async move {
                    enabled.set(v);
                    ActionResult::Success
                })
            />
        </Example>
        <Example title="Static states">
            {STATES.into_iter().map(|(name, s)| view! { <Cell label=name><ActionButton text="Apply" state=s/></Cell> }).collect_view()}
            <Cell label="Disabled"><ActionButton text="Apply" enabled=false/></Cell>
            <Cell label="Danger"><ActionButton text="Delete" dangerous=true/></Cell>
            <Cell label="Danger pressed"><ActionButton text="Delete" dangerous=true state=ControlState::Pressed/></Cell>
        </Example>
        <Example title="Long text">
            <div class="demo-narrow">
                <ActionButton text="Request a full device configuration export" on_click=b.action("export config") style="width: 100%"/>
            </div>
        </Example>
        <Example title="Link mode" note="With href the button renders a real link: crawlers follow it, and middle-click, Ctrl+click, and the context menu work. Only Idle, Pressed, and Disabled apply.">
            <ActionButton text="Open icon button" href="#/components/icon-button"/>
            <ActionButton text="Source code" icon=icons::OPEN_IN_NEW href="https://github.com/pavver/operator-design-system" new_tab=true/>
            <ActionButton text="Disabled link" href="#/components/icon-button" enabled=false/>
        </Example>
    }
    .into_any()
}

pub fn icon_button() -> AnyView {
    let b = backend();
    let clicks = RwSignal::new(0u32);
    view! {
        <Example title="Interactive">
            <IconButton
                icon=icons::SETTINGS
                label="Settings"
                on_click=Callback::new(move |()| {
                    clicks.update(|c| *c += 1);
                    b.log(format!("[UI] settings clicked ({})", clicks.get_untracked()));
                })
            />
            <IconButton icon=icons::REFRESH label="Refresh" on_async_click=b.action("refresh")/>
            <IconButton icon=icons::DELETE label="Delete" on_async_click=b.action("delete item")/>
            <IconButton icon=icons::VIDEOCAM label="Camera" size=10.0 icon_size=5.0 on_async_click=b.action("open camera")/>
            <IconButton icon=icons::MORE_VERT label="More" size=6.0 icon_size=4.0 on_click=Callback::new(|()| {})/>
        </Example>
        <Example title="Static states">
            {STATES.into_iter().map(|(name, s)| view! { <Cell label=name><IconButton icon=icons::SEARCH label="Search" state=s/></Cell> }).collect_view()}
            <Cell label="Disabled"><IconButton icon=icons::SEARCH label="Search" enabled=false/></Cell>
            <Cell label="Loading"><IconButton icon=icons::SEARCH label="Search" loading=true/></Cell>
        </Example>
        <Example title="Link mode" note="With href the button renders a real link styled as an icon button.">
            <IconButton icon=icons::OPEN_IN_NEW label="Source code" href="https://github.com/pavver/operator-design-system" new_tab=true/>
            <IconButton icon=icons::SETTINGS label="Action button page" href="#/components/action-button"/>
            <IconButton icon=icons::SETTINGS label="Disabled link" href="#/components/action-button" enabled=false/>
        </Example>
    }
    .into_any()
}

fn async_switch(label: &'static str, value: RwSignal<bool>) -> impl IntoView {
    let b = backend();
    view! {
        <AsyncSwitch
            checked=value
            label=label
            on_toggle=AsyncAction::new(move |v: bool| async move {
                let r = b.call(format!("{label}={v}")).await;
                if r.is_success() {
                    value.set(v);
                }
                r
            })
        />
    }
}

pub fn switch() -> AnyView {
    view! {
        <Example title="Interactive" note="The value commits only after success; the thumb parks in the center with a spinner while pending.">
            <Cell label="Night mode">{async_switch("night_mode", RwSignal::new(true))}</Cell>
            <Cell label="IR light">{async_switch("ir_light", RwSignal::new(false))}</Cell>
        </Example>
        <Example title="Static states">
            <Cell label="On"><AsyncSwitch checked=true/></Cell>
            <Cell label="Off"><AsyncSwitch checked=false/></Cell>
            <Cell label="Pressed"><AsyncSwitch checked=true state=ControlState::Pressed/></Cell>
            <Cell label="Pending"><AsyncSwitch checked=true state=ControlState::Pending/></Cell>
            <Cell label="Success"><AsyncSwitch checked=true state=ControlState::Success/></Cell>
            <Cell label="Error"><AsyncSwitch checked=false state=ControlState::Error/></Cell>
            <Cell label="Disabled on"><AsyncSwitch checked=true enabled=false/></Cell>
            <Cell label="Disabled off"><AsyncSwitch checked=false enabled=false/></Cell>
        </Example>
    }
    .into_any()
}

pub fn checkbox() -> AnyView {
    let b = backend();
    let autofocus = RwSignal::new(true);
    view! {
        <Example title="Interactive">
            <AsyncCheckbox
                checked=autofocus
                text=Signal::stored("Autofocus".to_string())
                on_change=AsyncAction::new(move |v: bool| async move {
                    let r = b.call(format!("autofocus={v}")).await;
                    if r.is_success() {
                        autofocus.set(v);
                    }
                    r
                })
            />
        </Example>
        <Example title="Static states">
            <Cell label="Unchecked"><AsyncCheckbox checked=false/></Cell>
            <Cell label="Checked"><AsyncCheckbox checked=true/></Cell>
            <Cell label="Pressed"><AsyncCheckbox checked=false state=ControlState::Pressed/></Cell>
            <Cell label="Pending"><AsyncCheckbox checked=false state=ControlState::Pending/></Cell>
            <Cell label="Success"><AsyncCheckbox checked=true state=ControlState::Success/></Cell>
            <Cell label="Error"><AsyncCheckbox checked=false state=ControlState::Error/></Cell>
            <Cell label="Disabled"><AsyncCheckbox checked=true enabled=false/></Cell>
        </Example>
    }
    .into_any()
}

pub fn checkbox_group() -> AnyView {
    let b = backend();
    let selected = RwSignal::new(vec!["north".to_string()]);
    let items = vec![
        GroupItem::new("north", "North gate"),
        GroupItem::new("south", "South gate"),
        GroupItem::new("yard", "Yard perimeter"),
    ];
    view! {
        <Example title="Armed zones" note="Each tile runs its own request; several can be pending at once.">
            <div class="demo-narrow">
                <CheckboxGroup
                    items=items
                    selected=selected
                    on_change=AsyncAction::new(move |(id, on): (String, bool)| async move {
                        let r = b.call(format!("zone {id}={on}")).await;
                        if r.is_success() {
                            selected.update(|s| {
                                s.retain(|x| *x != id);
                                if on {
                                    s.push(id);
                                }
                            });
                        }
                        r
                    })
                />
            </div>
        </Example>
        {[("Small icons", GroupIconSize::Small), ("Medium icons", GroupIconSize::Medium), ("Large icons", GroupIconSize::Large)]
            .into_iter()
            .map(|(title, size)| {
                let icon_selected = RwSignal::new(vec!["camera".to_string()]);
                view! {
                    <Example title=title>
                        <div class="demo-narrow">
                            <CheckboxGroup
                                icon_size=size
                                items=vec![
                                    GroupItem::new("camera", "Camera alerts").with_icon(icons::VIDEOCAM),
                                    GroupItem::new("search", "Search results").with_icon(icons::SEARCH),
                                    GroupItem::new("settings", "Settings changes").with_icon(icons::SETTINGS),
                                ]
                                selected=icon_selected
                                on_change=AsyncAction::new(move |(id, on): (String, bool)| async move {
                                    let r = b.call(format!("notify {id}={on}")).await;
                                    if r.is_success() {
                                        icon_selected.update(|s| {
                                            s.retain(|x| *x != id);
                                            if on {
                                                s.push(id);
                                            }
                                        });
                                    }
                                    r
                                })
                            />
                        </div>
                    </Example>
                }
            })
            .collect_view()}
    }
    .into_any()
}

pub fn text_field() -> AnyView {
    let b = backend();
    let name = RwSignal::new("Gate-North".to_string());
    let saved = RwSignal::new("Gate-North".to_string());
    view! {
        <Example title="Interactive" note="Edit the first value to reveal the async SAVE button. It hides after a successful save." block=true>
            <div class="demo-stack demo-narrow">
                <TextField
                    value=name
                    initial_value=saved
                    placeholder="Device name"
                    label="Device name"
                    on_save=AsyncAction::new(move |v: String| async move {
                        let r = b.call(format!("rename to {v}")).await;
                        if r.is_success() {
                            saved.set(v);
                        }
                        r
                    })
                />
                <TextField value=RwSignal::new(String::new()) placeholder="rtsp://host:554/stream" label="Stream URL"/>
                <TextField value=RwSignal::new("hunter2".to_string()) placeholder="Password" password=true label="Password"/>
                <TextField value=RwSignal::new("Read-only value".to_string()) placeholder="" enabled=false/>
            </div>
        </Example>
        <Example title="Static states">
            {[("Focused", ControlState::Focused), ("Pending", ControlState::Pending), ("Success", ControlState::Success), ("Error", ControlState::Error)]
                .into_iter()
                .map(|(n, s)| view! {
                    <Cell label=n>
                        <div class="demo-field-cell"><TextField value=RwSignal::new("value".to_string()) placeholder="" state=s/></div>
                    </Cell>
                })
                .collect_view()}
        </Example>
    }
    .into_any()
}

pub fn slider() -> AnyView {
    let b = backend();
    view! {
        <Example title="Interactive" note="Drag or click the track; arrow keys work when focused. The commit after release runs async; on error the value rolls back." block=true>
            <div class="demo-stack demo-narrow">
                <div class="demo-labelled">
                    <span class="op-t-caption demo-muted">"Continuous 0..1"</span>
                    <Slider value=RwSignal::new(0.64f32) label="Brightness"/>
                </div>
                <div class="demo-labelled">
                    <span class="op-t-caption demo-muted">"Stepped 0..10, async commit"</span>
                    <Slider value=RwSignal::new(4.0f32) max=10.0 step=1.0 label="Zoom" on_commit=move |v: f32| b.call(format!("zoom={v}"))/>
                </div>
            </div>
        </Example>
        <Example title="Static states">
            {[("Pending", ControlState::Pending), ("Success", ControlState::Success), ("Error", ControlState::Error)]
                .into_iter()
                .map(|(n, s)| view! {
                    <Cell label=n><div class="demo-slider-cell"><Slider value=RwSignal::new(0.4) state=s/></div></Cell>
                })
                .collect_view()}
            <Cell label="Disabled">
                <div class="demo-slider-cell"><Slider value=RwSignal::new(5.0) max=10.0 step=1.0 enabled=false/></div>
            </Cell>
        </Example>
    }
    .into_any()
}

pub fn stepper() -> AnyView {
    let b = backend();
    view! {
        <Example title="Interactive" note="Stays interactive while pending for continuous input.">
            <Cell label="Async (PTZ preset)">
                <Stepper value=RwSignal::new(3) min=0 max=9 on_async_change=AsyncAction::new(move |v: i32| b.call(format!("ptz preset={v}")))/>
            </Cell>
            <Cell label="Sync, 0..10"><Stepper value=RwSignal::new(5) min=0 max=10/></Cell>
        </Example>
        <Example title="Static states">
            <Cell label="Pending"><Stepper value=RwSignal::new(2) state=ControlState::Pending/></Cell>
            <Cell label="Success"><Stepper value=RwSignal::new(2) state=ControlState::Success/></Cell>
            <Cell label="Error"><Stepper value=RwSignal::new(2) state=ControlState::Error/></Cell>
            <Cell label="At min"><Stepper value=RwSignal::new(0) min=0/></Cell>
            <Cell label="Disabled"><Stepper value=RwSignal::new(2) enabled=false/></Cell>
        </Example>
    }
    .into_any()
}

pub fn segmented() -> AnyView {
    let b = backend();
    view! {
        <Example title="Variants" note="The async variant shows a spinner in the target segment and slides back on error." block=true>
            <div class="demo-stack demo-narrow">
                <SegmentedControl items=vec![SegmentItem::text("Auto"), SegmentItem::text("Day"), SegmentItem::text("Night")] selected=RwSignal::new(1usize)/>
                <SegmentedControl
                    items=vec![SegmentItem::item("Live", icons::VIDEOCAM), SegmentItem::item("Search", icons::SEARCH), SegmentItem::icon(icons::SETTINGS)]
                    selected=RwSignal::new(0usize)
                />
                <SegmentedControl
                    items=vec![SegmentItem::text("SD"), SegmentItem::text("HD"), SegmentItem::text("4K")]
                    selected=RwSignal::new(0usize)
                    on_select=AsyncAction::new(move |i: usize| b.call(format!("stream quality={i}")))
                />
                <SegmentedControl items=vec![SegmentItem::text("Auto"), SegmentItem::text("Day"), SegmentItem::text("Night")] selected=RwSignal::new(0) enabled=false/>
            </div>
        </Example>
    }
    .into_any()
}

pub fn dropdown() -> AnyView {
    let b = backend();
    let zones: Vec<String> = (1..=24).map(|i| format!("Zone {i:02}")).collect();
    view! {
        <Example title="Variants" block=true>
            <div class="demo-grid-2">
                <DropdownMenu
                    label="Codec"
                    options=vec!["H.264".into(), "H.265".into(), "MJPEG".into()]
                    selected=RwSignal::new("H.265".to_string())
                    on_select=Callback::new(move |v: String| b.log(format!("[UI] codec={v}")))
                />
                <DropdownMenu label="Zone (scrolls)" options=zones selected=RwSignal::new("Zone 07".to_string())/>
                <DropdownMenu label="Pending" options=vec!["One".into()] selected=RwSignal::new("Loading…".to_string()) state=ControlState::Pending/>
                <DropdownMenu label="Disabled" options=vec!["One".into()] selected=RwSignal::new("Unavailable".to_string()) enabled=false/>
            </div>
        </Example>
    }
    .into_any()
}

pub fn list_item() -> AnyView {
    let b = backend();
    let alerts = RwSignal::new(true);
    let selected = RwSignal::new(false);
    view! {
        <Example title="Rows" block=true>
            <div class="demo-list">
                <ListItem title="Stream quality" trailing=ViewFn::from(|| view! { <span class="op-t-body demo-muted">"HD"</span> })/>
                <ListItem
                    title="Motion alerts"
                    subtitle="Push a notification when motion is detected in an armed zone"
                    trailing=ViewFn::from(move || {
                        view! {
                            <AsyncSwitch
                                checked=alerts
                                on_toggle=AsyncAction::new(move |v: bool| async move {
                                    let r = b.call(format!("alerts={v}")).await;
                                    if r.is_success() {
                                        alerts.set(v);
                                    }
                                    r
                                })
                            />
                        }
                    })
                />
                <ListItem
                    title="Gate camera"
                    subtitle="Online · 12 ms · H.265"
                    leading=ViewFn::from(|| view! { <Avatar id="cam-3" name="Gate" kind=AvatarKind::ThreadedGroup status=PresenceStatus::Online/> })
                    trailing=ViewFn::from(|| view! { <Icon icon=icons::CHEVRON_RIGHT size=dim::ICON_SMALL/> })
                    selected=selected
                    on_click=Callback::new(move |()| selected.update(|s| *s = !*s))
                />
                <ListItem
                    title="Olena Kovalenko"
                    subtitle="Typing…"
                    leading=ViewFn::from(|| view! { <Avatar id="olena" name="Olena" status=PresenceStatus::Typing/> })
                    on_click=Callback::new(move |()| b.log("[UI] open chat"))
                />
                <ListItem title="Firmware update" subtitle="Not available for this model" enabled=false/>
            </div>
        </Example>
    }
    .into_any()
}

pub fn status_badge() -> AnyView {
    view! {
        <Example title="All statuses at 4 DU">
            {PresenceStatus::ALL.into_iter().map(|s| view! { <Cell label=s.name()><StatusBadge status=s size=4.0/></Cell> }).collect_view()}
        </Example>
        <Example title="With text">
            <StatusBadge status=PresenceStatus::Online text=Signal::stored("Online".to_string())/>
            <StatusBadge status=PresenceStatus::Alarm text=Signal::stored("Motion alarm".to_string())/>
            <StatusBadge status=PresenceStatus::Offline text=Signal::stored("Offline 3 h".to_string())/>
            <StatusBadge status=PresenceStatus::SendingFile text=Signal::stored("Uploading clip".to_string())/>
        </Example>
    }
    .into_any()
}

pub fn avatar() -> AnyView {
    view! {
        <Example title="Kinds" note="Color is derived from the id with the same hash on every platform.">
            <Cell label="Direct"><Avatar id="andrii" name="Andrii" kind=AvatarKind::Direct/></Cell>
            <Cell label="Threaded group"><Avatar id="ops-room" name="Ops" kind=AvatarKind::ThreadedGroup/></Cell>
            <Cell label="Channel"><Avatar id="news" name="News" kind=AvatarKind::Channel/></Cell>
            <Cell label="Simple group"><Avatar id="family" name="Family" kind=AvatarKind::SimpleGroup/></Cell>
        </Example>
        <Example title="Sizes and status">
            <Cell label="6 DU"><Avatar id="a1" name="Ihor" size=6.0 status=PresenceStatus::Online/></Cell>
            <Cell label="8 DU"><Avatar id="a2" name="Marta" size=8.0 status=PresenceStatus::Alarm/></Cell>
            <Cell label="10 DU"><Avatar id="a3" name="Taras" size=10.0 status=PresenceStatus::RecordingAudio/></Cell>
            <Cell label="14 DU"><Avatar id="a4" name="Zoia" size=14.0 status=PresenceStatus::SendingFile/></Cell>
            <Cell label="Loading"><Avatar id="a5" name="Bot" size=10.0 image_url="debug_loading"/></Cell>
        </Example>
    }
    .into_any()
}

pub fn header() -> AnyView {
    let b = backend();
    view! {
        <Example title="Variants" block=true>
            <div class="demo-stack">
                <Header title="Settings" on_back=Callback::new(move |()| b.log("[UI] back"))/>
                <Header
                    title="Gate camera"
                    subtitle="online · 12 ms"
                    on_back=Callback::new(move |()| b.log("[UI] back"))
                    avatar=HeaderAvatar { id: "cam-3".into(), name: "Gate".into(), group: false, status: Some(PresenceStatus::Online) }
                    actions=ViewFn::from(move || {
                        view! {
                            <IconButton icon=icons::VIDEOCAM label="Live" on_click=Callback::new(move |()| b.log("[UI] live"))/>
                            <IconButton icon=icons::MORE_VERT label="More" on_click=Callback::new(move |()| b.log("[UI] more"))/>
                        }
                    })
                />
            </div>
        </Example>
    }
    .into_any()
}

pub fn confirm_dialog() -> AnyView {
    let b = backend();
    let delete_open = RwSignal::new(false);
    let reboot_open = RwSignal::new(false);
    let opener = move |s: RwSignal<bool>| {
        AsyncAction::new(move |()| async move {
            s.set(true);
            ActionResult::Success
        })
    };
    view! {
        <Example title="Destructive and neutral" note="Confirm runs async: success closes after 500 ms, error pulses and unlocks the dialog.">
            <ActionButton text="Delete recording…" dangerous=true on_click=opener(delete_open)/>
            <ActionButton text="Reboot device…" on_click=opener(reboot_open)/>
            <ConfirmDialog
                open=delete_open
                title="Delete recording?"
                message="The clip from 14:02 will be removed from the device and cannot be restored."
                on_confirm=b.action("delete recording")
            />
            <ConfirmDialog
                open=reboot_open
                title="Reboot device?"
                message="The stream will be unavailable for about 40 seconds."
                confirm_text="Reboot"
                destructive=false
                on_confirm=b.action("reboot device")
            />
        </Example>
    }
    .into_any()
}

/// xterm output that exercises the SGR parser.
const ANSI_SAMPLE: &[&str] = &[
    "\x1b[1mcargo build\x1b[0m --release",
    "   \x1b[1;32mCompiling\x1b[0m operator-ui v0.6.0",
    "\x1b[1;33mwarning\x1b[0m\x1b[1m: unused variable: `frame`\x1b[0m",
    "\x1b[1;31merror[E0308]\x1b[0m\x1b[1m: mismatched types\x1b[0m",
    "16 colors: \x1b[30;47m 0 \x1b[0m\x1b[31m 1 \x1b[32m 2 \x1b[33m 3 \x1b[34m 4 \x1b[35m 5 \x1b[36m 6 \x1b[37m 7 \x1b[0m",
    "bright:    \x1b[90m 8 \x1b[91m 9 \x1b[92m 10 \x1b[93m 11 \x1b[94m 12 \x1b[95m 13 \x1b[96m 14 \x1b[97m 15 \x1b[0m",
    "256 colors: \x1b[38;5;208m208\x1b[0m \x1b[38;5;45m45\x1b[0m \x1b[48;5;236m 236 \x1b[0m  truecolor: \x1b[38;2;240;165;58mamber\x1b[0m",
    "\x1b[1mbold\x1b[0m \x1b[2mdim\x1b[0m \x1b[3mitalic\x1b[0m \x1b[4munderline\x1b[0m \x1b[9mstrike\x1b[0m \x1b[7minverse\x1b[0m",
    "download  10%\rdownload  55%\rdownload 100% \x1b[32mdone\x1b[0m",
];

pub fn console() -> AnyView {
    let b = backend();
    let sample = ANSI_SAMPLE
        .iter()
        .map(|l| (*l).to_owned())
        .collect::<Vec<_>>();
    view! {
        <Example title="Event log of this site" note="Every simulated request on these pages is logged here. It auto-scrolls to the newest line." block=true>
            <ConsoleTerminal logs=b.logs/>
        </Example>
        <Example title="xterm escape sequences" note="Colors and attributes come from SGR sequences; a carriage return rewrites the line." block=true>
            <ConsoleTerminal logs=sample height=50.0/>
        </Example>
    }
    .into_any()
}

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

pub fn theme_toggle() -> AnyView {
    let local = RwSignal::new(false);
    view! {
        <Example title="Interactive" note="The switch in the top bar of this site is the same component.">
            <Cell label="Default"><ThemeToggle dark=local on_toggle=Callback::new(move |v: bool| local.set(v))/></Cell>
            <Cell label="Large, 24 × 10 DU">
                <ThemeToggle dark=local on_toggle=Callback::new(move |v: bool| local.set(v)) width=24.0 height=10.0/>
            </Cell>
        </Example>
        <Example title="Static">
            <Cell label="Day"><ThemeToggle dark=false/></Cell>
            <Cell label="Night"><ThemeToggle dark=true/></Cell>
        </Example>
    }
    .into_any()
}

pub fn primitives() -> AnyView {
    view! {
        <Example title="Icon and ProgressIndicator">
            <Cell label="Icon 4 DU"><Icon icon=icons::SETTINGS size=4.0/></Cell>
            <Cell label="Icon 6 DU"><Icon icon=icons::SETTINGS/></Cell>
            <Cell label="Spinner 6 DU"><ProgressIndicator/></Cell>
            <Cell label="Spinner pending"><ProgressIndicator size=4.0 stroke=dim::SPINNER_STROKE color="var(--op-pending)"/></Cell>
        </Example>
        <Example title="Card and Divider" block=true>
            <div class="demo-grid-2">
                <Card>
                    <div class="op-t-row-title">"Surface card"</div>
                    <div class="op-t-body-small demo-muted">"Primary container background."</div>
                </Card>
                <Card alt=true>
                    <div class="op-t-row-title">"Alt card"</div>
                    <div class="op-t-body-small demo-muted">"Sunken background for displays."</div>
                </Card>
            </div>
            <Divider/>
        </Example>
    }
    .into_any()
}
