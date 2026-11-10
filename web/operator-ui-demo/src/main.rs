//! Operator UI gallery: every web component with its static states and an interactive sandbox.

use leptos::prelude::*;
use operator_ui::prelude::*;
use operator_ui::tokens::{Palette, Rgba, Theme as TokenTheme};

// ---------------------------------------------------------------------------
// Fake backend shared by every interactive example
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Success,
    Error,
    Alternate,
}

#[derive(Clone, Copy)]
struct Backend {
    latency_ms: RwSignal<u32>,
    outcome: RwSignal<Outcome>,
    flip: StoredValue<bool>,
    logs: RwSignal<Vec<String>>,
}

fn timestamp() -> String {
    let d = js_sys::Date::new_0();
    format!(
        "{:02}:{:02}:{:02}",
        d.get_hours(),
        d.get_minutes(),
        d.get_seconds()
    )
}

impl Backend {
    fn new() -> Self {
        Self {
            latency_ms: RwSignal::new(1200),
            outcome: RwSignal::new(Outcome::Alternate),
            flip: StoredValue::new(false),
            logs: RwSignal::new(vec![format!(
                "{} [UI] Operator UI gallery ready",
                timestamp()
            )]),
        }
    }

    fn log(self, line: impl Into<String>) {
        let line = format!("{} {}", timestamp(), line.into());
        self.logs.update(|l| {
            l.push(line);
            if l.len() > 200 {
                l.remove(0);
            }
        });
    }

    async fn call(self, what: String) -> ActionResult {
        self.log(format!("[TX] {what}"));
        sleep_ms(self.latency_ms.get_untracked()).await;
        let ok = match self.outcome.get_untracked() {
            Outcome::Success => true,
            Outcome::Error => false,
            Outcome::Alternate => {
                self.flip.update_value(|f| *f = !*f);
                self.flip.get_value()
            }
        };
        if ok {
            self.log(format!("[SUCCESS] {what}"));
            ActionResult::Success
        } else {
            self.log(format!("[ERROR] {what}: timeout"));
            ActionResult::Error(Some("timeout".into()))
        }
    }

    fn action(self, what: &'static str) -> AsyncAction {
        AsyncAction::new(move |()| self.call(what.to_owned()))
    }
}

fn backend() -> Backend {
    expect_context::<Backend>()
}

// ---------------------------------------------------------------------------
// Layout helpers
// ---------------------------------------------------------------------------

#[component]
fn Section(
    id: &'static str,
    title: &'static str,
    #[prop(into)] note: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="demo-section" id=id>
            <header class="demo-section-head">
                <h2 class="op-t-screen-title">{title}</h2>
                <p class="demo-note op-t-body-small">{note}</p>
            </header>
            {children()}
        </section>
    }
}

#[component]
fn Row(#[prop(optional, into)] label: Option<&'static str>, children: Children) -> impl IntoView {
    view! {
        <div class="demo-row-wrap">
            {label.map(|l| view! { <div class="demo-row-label op-t-caption">{l}</div> })}
            <div class="demo-row">{children()}</div>
        </div>
    }
}

#[component]
fn Cell(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="demo-cell">
            <div class="demo-cell-body">{children()}</div>
            <div class="demo-cell-label op-t-caption">{label}</div>
        </div>
    }
}

const STATES: [(&str, ControlState); 7] = [
    ("Idle", ControlState::Idle),
    ("Pressed", ControlState::Pressed),
    ("Pending", ControlState::Pending),
    ("Success", ControlState::Success),
    ("Error", ControlState::Error),
    ("Selected", ControlState::Selected),
    ("Focused", ControlState::Focused),
];

// ---------------------------------------------------------------------------
// Sections
// ---------------------------------------------------------------------------

fn hex(c: Rgba) -> String {
    if c.a == 255 {
        format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b)
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", c.r, c.g, c.b, c.a)
    }
}

fn palette_entries(p: &Palette) -> Vec<(&'static str, Rgba)> {
    vec![
        ("background", p.background),
        ("surface", p.surface),
        ("surface_alt", p.surface_alt),
        ("border", p.border),
        ("border_strong", p.border_strong),
        ("divider", p.divider),
        ("text_primary", p.text_primary),
        ("text_secondary", p.text_secondary),
        ("text_disabled", p.text_disabled),
        ("icon", p.icon),
        ("accent", p.accent),
        ("accent_muted", p.accent_muted),
        ("on_accent", p.on_accent),
        ("pending", p.pending),
        ("success", p.success),
        ("warning", p.warning),
        ("error", p.error),
        ("info", p.info),
        ("pressed", p.pressed),
        ("selection", p.selection),
        ("disabled_surface", p.disabled_surface),
        ("disabled_border", p.disabled_border),
    ]
}

fn tokens_section() -> impl IntoView {
    let theme = OperatorTheme::current();
    let swatches = move || {
        let palette = theme.theme.get().palette(theme.dark.get());
        palette_entries(palette)
            .into_iter()
            .map(|(name, c)| {
                view! {
                    <div class="demo-swatch">
                        <span class="demo-swatch-chip" style:background=hex(c)></span>
                        <span class="demo-swatch-text op-t-caption">
                            <span>{name}</span>
                            <span class="demo-muted">{hex(c)}</span>
                        </span>
                    </div>
                }
            })
            .collect_view()
    };
    let type_rows = [
        ("app_title", "op-t-app-title", typography::APP_TITLE),
        (
            "screen_title",
            "op-t-screen-title",
            typography::SCREEN_TITLE,
        ),
        (
            "section_title",
            "op-t-section-title",
            typography::SECTION_TITLE,
        ),
        ("row_title", "op-t-row-title", typography::ROW_TITLE),
        ("body", "op-t-body", typography::BODY),
        ("body_small", "op-t-body-small", typography::BODY_SMALL),
        ("caption", "op-t-caption", typography::CAPTION),
        ("console", "op-t-console", typography::CONSOLE),
    ]
    .into_iter()
    .map(|(name, class, t)| {
        view! {
            <div class="demo-type-row">
                <span class="demo-muted op-t-caption">
                    {format!("{name} · {} / {} TU · {}", t.size, t.line, t.weight)}
                </span>
                <span class=class>"Gate camera 03 · North entrance · 0x4F2A"</span>
            </div>
        }
    })
    .collect_view();
    let scale = [
        0.25f32, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 6.0, 8.0, 10.0, 12.0,
    ]
    .into_iter()
    .map(|v| {
        view! {
            <div class="demo-du">
                <span class="demo-du-bar" style:width=du(v)></span>
                <span class="op-t-caption demo-muted">{format!("{v} DU")}</span>
            </div>
        }
    })
    .collect_view();
    view! {
        <Section
            id="tokens"
            title="Tokens"
            note="Generated from tokens/tokens.toml. 1 DU = 4 logical px; 1 TU = 1 scalable point (0.0625rem)."
        >
            <div class="demo-swatches">{swatches}</div>
            <div class="demo-types">{type_rows}</div>
            <div class="demo-du-scale">{scale}</div>
        </Section>
    }
}

use operator_ui::tokens::typography;

fn action_button_section() -> impl IntoView {
    let b = backend();
    let enabled = RwSignal::new(true);
    view! {
        <Section
            id="action-button"
            title="ActionButton"
            note="Async action with pending running border, 500 ms result icon flash and 300 ms tail. Clicks are blocked while pending."
        >
            <Row label="Interactive">
                <ActionButton text="Restart stream" on_click=b.action("restart stream") enabled=enabled/>
                <ActionButton text="Sync" icon=icons::REFRESH on_click=b.action("sync presets") enabled=enabled/>
                <ActionButton
                    text="Save"
                    icon=icons::CHECK
                    keep_flash_icon_on_success=true
                    on_click=b.action("save settings")
                    enabled=enabled
                />
                <ActionButton text="Delete camera" dangerous=true on_click=b.action("delete camera") enabled=enabled/>
                <ActionButton
                    text="No pending border"
                    show_pending_border=false
                    on_click=b.action("silent request")
                    enabled=enabled
                />
            </Row>
            <Row label="Controls">
                <AsyncCheckbox
                    checked=enabled
                    text=Signal::stored("Enabled".to_string())
                    on_change=AsyncAction::new(move |v: bool| async move {
                        enabled.set(v);
                        ActionResult::Success
                    })
                />
            </Row>
            <Row label="Static states">
                {STATES
                    .into_iter()
                    .map(|(name, s)| view! { <Cell label=name><ActionButton text="Apply" state=s/></Cell> })
                    .collect_view()}
                <Cell label="Disabled">
                    <ActionButton text="Apply" enabled=false/>
                </Cell>
                <Cell label="Danger">
                    <ActionButton text="Delete" dangerous=true/>
                </Cell>
                <Cell label="Danger pressed">
                    <ActionButton text="Delete" dangerous=true state=ControlState::Pressed/>
                </Cell>
            </Row>
            <Row label="Long text">
                <div class="demo-narrow">
                    <ActionButton
                        text="Request a full device configuration export"
                        on_click=b.action("export config")
                        style="width: 100%"
                    />
                </div>
            </Row>
        </Section>
    }
}

fn icon_button_section() -> impl IntoView {
    let b = backend();
    let clicks = RwSignal::new(0u32);
    view! {
        <Section id="icon-button" title="IconButton" note="Square 8 DU button; sync or async click, external state and loading flag.">
            <Row label="Interactive">
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
            </Row>
            <Row label="Static states">
                {STATES
                    .into_iter()
                    .map(|(name, s)| {
                        view! {
                            <Cell label=name>
                                <IconButton icon=icons::SEARCH label="Search" state=s/>
                            </Cell>
                        }
                    })
                    .collect_view()}
                <Cell label="Disabled">
                    <IconButton icon=icons::SEARCH label="Search" enabled=false/>
                </Cell>
                <Cell label="Loading">
                    <IconButton icon=icons::SEARCH label="Search" loading=true/>
                </Cell>
            </Row>
        </Section>
    }
}

fn switch_section() -> impl IntoView {
    let b = backend();
    let night = RwSignal::new(true);
    let ir = RwSignal::new(false);
    view! {
        <Section
            id="switch"
            title="AsyncSwitch"
            note="The value commits only after success; the thumb parks in the center with a spinner while pending."
        >
            <Row label="Interactive">
                <Cell label="Night mode">
                    <AsyncSwitch
                        checked=night
                        label="Night mode"
                        on_toggle=AsyncAction::new(move |v: bool| async move {
                            let r = b.call(format!("night_mode={v}")).await;
                            if r.is_success() {
                                night.set(v);
                            }
                            r
                        })
                    />
                </Cell>
                <Cell label="IR light">
                    <AsyncSwitch
                        checked=ir
                        label="IR light"
                        on_toggle=AsyncAction::new(move |v: bool| async move {
                            let r = b.call(format!("ir_light={v}")).await;
                            if r.is_success() {
                                ir.set(v);
                            }
                            r
                        })
                    />
                </Cell>
            </Row>
            <Row label="Static states">
                <Cell label="On"><AsyncSwitch checked=true/></Cell>
                <Cell label="Off"><AsyncSwitch checked=false/></Cell>
                <Cell label="Pressed"><AsyncSwitch checked=true state=ControlState::Pressed/></Cell>
                <Cell label="Pending"><AsyncSwitch checked=true state=ControlState::Pending/></Cell>
                <Cell label="Success"><AsyncSwitch checked=true state=ControlState::Success/></Cell>
                <Cell label="Error"><AsyncSwitch checked=false state=ControlState::Error/></Cell>
                <Cell label="Disabled on"><AsyncSwitch checked=true enabled=false/></Cell>
                <Cell label="Disabled off"><AsyncSwitch checked=false enabled=false/></Cell>
            </Row>
        </Section>
    }
}

fn checkbox_section() -> impl IntoView {
    let b = backend();
    let autofocus = RwSignal::new(true);
    let selected = RwSignal::new(vec!["north".to_string()]);
    let items = vec![
        GroupItem::new("north", "North gate"),
        GroupItem::new("south", "South gate"),
        GroupItem::new("yard", "Yard perimeter"),
    ];
    view! {
        <Section
            id="checkbox"
            title="AsyncCheckbox · CheckboxGroup"
            note="Checked boxes use the Selected border. The checkmark shows only for a true value."
        >
            <Row label="Interactive">
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
            </Row>
            <Row label="Static states">
                <Cell label="Unchecked"><AsyncCheckbox checked=false/></Cell>
                <Cell label="Checked"><AsyncCheckbox checked=true/></Cell>
                <Cell label="Pressed"><AsyncCheckbox checked=false state=ControlState::Pressed/></Cell>
                <Cell label="Pending"><AsyncCheckbox checked=false state=ControlState::Pending/></Cell>
                <Cell label="Success"><AsyncCheckbox checked=true state=ControlState::Success/></Cell>
                <Cell label="Error"><AsyncCheckbox checked=false state=ControlState::Error/></Cell>
                <Cell label="Disabled"><AsyncCheckbox checked=true enabled=false/></Cell>
            </Row>
            <Row label="CheckboxGroup">
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
            </Row>
        </Section>
    }
}

fn text_field_section() -> impl IntoView {
    let b = backend();
    let name = RwSignal::new("Gate-North".to_string());
    let saved = RwSignal::new("Gate-North".to_string());
    let host = RwSignal::new(String::new());
    let secret = RwSignal::new("hunter2".to_string());
    view! {
        <Section
            id="text-field"
            title="TextField"
            note="Edit the value to reveal the async SAVE button. It hides after a successful save."
        >
            <div class="demo-stack">
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
                <TextField value=host placeholder="rtsp://host:554/stream" label="Stream URL"/>
                <TextField value=secret placeholder="Password" password=true label="Password"/>
                <TextField value=RwSignal::new("Read-only value".to_string()) placeholder="" enabled=false/>
            </div>
            <Row label="Static states">
                {[("Focused", ControlState::Focused), ("Pending", ControlState::Pending), ("Success", ControlState::Success), ("Error", ControlState::Error)]
                    .into_iter()
                    .map(|(n, s)| {
                        view! {
                            <Cell label=n>
                                <div class="demo-field-cell">
                                    <TextField value=RwSignal::new("value".to_string()) placeholder="" state=s/>
                                </div>
                            </Cell>
                        }
                    })
                    .collect_view()}
            </Row>
        </Section>
    }
}

fn slider_section() -> impl IntoView {
    let b = backend();
    let brightness = RwSignal::new(0.64f32);
    let preset = RwSignal::new(4.0f32);
    view! {
        <Section
            id="slider"
            title="Slider"
            note="Drag or click the track; arrow keys work when focused. The commit after release runs async and shows its state in the thumb; on error the value rolls back."
        >
            <div class="demo-stack">
                <div class="demo-labelled">
                    <span class="op-t-caption demo-muted">"Continuous 0..1"</span>
                    <Slider value=brightness label="Brightness"/>
                </div>
                <div class="demo-labelled">
                    <span class="op-t-caption demo-muted">"Stepped 0..10, async commit"</span>
                    <Slider
                        value=preset
                        max=10.0
                        step=1.0
                        label="Zoom"
                        on_commit=move |v: f32| b.call(format!("zoom={v}"))
                    />
                </div>
            </div>
            <Row label="Static states">
                {[("Pending", ControlState::Pending), ("Success", ControlState::Success), ("Error", ControlState::Error)]
                    .into_iter()
                    .map(|(n, s)| {
                        view! {
                            <Cell label=n>
                                <div class="demo-slider-cell">
                                    <Slider value=RwSignal::new(0.4) state=s/>
                                </div>
                            </Cell>
                        }
                    })
                    .collect_view()}
                <Cell label="Disabled">
                    <div class="demo-slider-cell">
                        <Slider value=RwSignal::new(5.0) max=10.0 step=1.0 enabled=false/>
                    </div>
                </Cell>
            </Row>
        </Section>
    }
}

fn stepper_section() -> impl IntoView {
    let b = backend();
    let preset = RwSignal::new(3);
    let local = RwSignal::new(5);
    view! {
        <Section
            id="stepper"
            title="Stepper"
            note="Stays interactive while pending for continuous input; the pressed side shows the spinner and result."
        >
            <Row label="Interactive">
                <Cell label="Async (PTZ preset)">
                    <Stepper
                        value=preset
                        min=0
                        max=9
                        on_async_change=AsyncAction::new(move |v: i32| b.call(format!("ptz preset={v}")))
                    />
                </Cell>
                <Cell label="Sync, 0..10">
                    <Stepper value=local min=0 max=10/>
                </Cell>
            </Row>
            <Row label="Static states">
                <Cell label="Pending"><Stepper value=RwSignal::new(2) state=ControlState::Pending/></Cell>
                <Cell label="Success"><Stepper value=RwSignal::new(2) state=ControlState::Success/></Cell>
                <Cell label="Error"><Stepper value=RwSignal::new(2) state=ControlState::Error/></Cell>
                <Cell label="At min"><Stepper value=RwSignal::new(0) min=0/></Cell>
                <Cell label="Disabled"><Stepper value=RwSignal::new(2) enabled=false/></Cell>
            </Row>
        </Section>
    }
}

fn segmented_section() -> impl IntoView {
    let b = backend();
    let profile = RwSignal::new(1usize);
    let view_mode = RwSignal::new(0usize);
    let quality = RwSignal::new(0usize);
    view! {
        <Section
            id="segmented"
            title="SegmentedControl"
            note="Sliding indicator (150 ms linear). The async variant shows a spinner in the target segment and rolls back on error."
        >
            <div class="demo-stack demo-narrow">
                <SegmentedControl
                    items=vec![SegmentItem::text("Auto"), SegmentItem::text("Day"), SegmentItem::text("Night")]
                    selected=profile
                />
                <SegmentedControl
                    items=vec![
                        SegmentItem::item("Live", icons::VIDEOCAM),
                        SegmentItem::item("Search", icons::SEARCH),
                        SegmentItem::icon(icons::SETTINGS),
                    ]
                    selected=view_mode
                />
                <SegmentedControl
                    items=vec![SegmentItem::text("SD"), SegmentItem::text("HD"), SegmentItem::text("4K")]
                    selected=quality
                    on_select=AsyncAction::new(move |i: usize| b.call(format!("stream quality={i}")))
                />
                <SegmentedControl
                    items=vec![SegmentItem::text("Auto"), SegmentItem::text("Day"), SegmentItem::text("Night")]
                    selected=RwSignal::new(0)
                    enabled=false
                />
            </div>
        </Section>
    }
}

fn dropdown_section() -> impl IntoView {
    let b = backend();
    let codec = RwSignal::new("H.265".to_string());
    let zone = RwSignal::new("Zone 07".to_string());
    let zones: Vec<String> = (1..=24).map(|i| format!("Zone {i:02}")).collect();
    view! {
        <Section id="dropdown" title="DropdownMenu" note="Custom popup with a 50 DU max height and accent border.">
            <div class="demo-grid-2">
                <DropdownMenu
                    label="Codec"
                    options=vec!["H.264".into(), "H.265".into(), "MJPEG".into()]
                    selected=codec
                    on_select=Callback::new(move |v: String| b.log(format!("[UI] codec={v}")))
                />
                <DropdownMenu label="Zone (scrolls)" options=zones selected=zone/>
                <DropdownMenu
                    label="Pending"
                    options=vec!["One".into()]
                    selected=RwSignal::new("Loading…".to_string())
                    state=ControlState::Pending
                />
                <DropdownMenu
                    label="Disabled"
                    options=vec!["One".into()]
                    selected=RwSignal::new("Unavailable".to_string())
                    enabled=false
                />
            </div>
        </Section>
    }
}

fn list_section() -> impl IntoView {
    let b = backend();
    let alerts = RwSignal::new(true);
    let selected = RwSignal::new(false);
    view! {
        <Section id="list-item" title="ListItem" note="Row model: 10 DU standard, 13 DU with subtitle, 14 DU with leading content.">
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
                    leading=ViewFn::from(|| {
                        view! { <Avatar id="cam-3" name="Gate" kind=AvatarKind::ThreadedGroup status=PresenceStatus::Online/> }
                    })
                    trailing=ViewFn::from(|| view! { <Icon icon=icons::CHEVRON_RIGHT size=dim::ICON_SMALL/> })
                    selected=selected
                    on_click=Callback::new(move |()| selected.update(|s| *s = !*s))
                />
                <ListItem
                    title="Olena Kovalenko"
                    subtitle="Typing…"
                    leading=ViewFn::from(|| {
                        view! { <Avatar id="olena" name="Olena" status=PresenceStatus::Typing/> }
                    })
                    on_click=Callback::new(move |()| b.log("[UI] open chat"))
                />
                <ListItem title="Firmware update" subtitle="Not available for this model" enabled=false/>
            </div>
        </Section>
    }
}

fn badge_section() -> impl IntoView {
    let badges = PresenceStatus::ALL
        .into_iter()
        .map(|s| {
            view! {
                <Cell label=s.name()>
                    <StatusBadge status=s size=4.0/>
                </Cell>
            }
        })
        .collect_view();
    view! {
        <Section id="status-badge" title="StatusBadge" note="LED states plus animated activity variants; shown at 4 DU and default 3 DU.">
            <Row label="4 DU">{badges}</Row>
            <Row label="With text">
                <StatusBadge status=PresenceStatus::Online text=Signal::stored("Online".to_string())/>
                <StatusBadge status=PresenceStatus::Alarm text=Signal::stored("Motion alarm".to_string())/>
                <StatusBadge status=PresenceStatus::Offline text=Signal::stored("Offline 3 h".to_string())/>
                <StatusBadge status=PresenceStatus::SendingFile text=Signal::stored("Uploading clip".to_string())/>
            </Row>
        </Section>
    }
}

fn avatar_section() -> impl IntoView {
    view! {
        <Section
            id="avatar"
            title="Avatar"
            note="Color is derived from the id with the same hash as Android, so both platforms pick the same hue."
        >
            <Row label="Kinds">
                <Cell label="Direct"><Avatar id="andrii" name="Andrii" kind=AvatarKind::Direct/></Cell>
                <Cell label="Threaded group"><Avatar id="ops-room" name="Ops" kind=AvatarKind::ThreadedGroup/></Cell>
                <Cell label="Channel"><Avatar id="news" name="News" kind=AvatarKind::Channel/></Cell>
                <Cell label="Simple group"><Avatar id="family" name="Family" kind=AvatarKind::SimpleGroup/></Cell>
            </Row>
            <Row label="Sizes and status">
                <Cell label="6 DU"><Avatar id="a1" name="Ihor" size=6.0 status=PresenceStatus::Online/></Cell>
                <Cell label="8 DU"><Avatar id="a2" name="Marta" size=8.0 status=PresenceStatus::Alarm/></Cell>
                <Cell label="10 DU"><Avatar id="a3" name="Taras" size=10.0 status=PresenceStatus::RecordingAudio/></Cell>
                <Cell label="14 DU"><Avatar id="a4" name="Zoia" size=14.0 status=PresenceStatus::SendingFile/></Cell>
                <Cell label="Loading"><Avatar id="a5" name="Bot" size=10.0 image_url="debug_loading"/></Cell>
            </Row>
        </Section>
    }
}

fn header_section() -> impl IntoView {
    let b = backend();
    view! {
        <Section id="header" title="Header" note="13.5 DU app bar with a 1 unit bottom border.">
            <div class="demo-stack">
                <Header title="Settings" on_back=Callback::new(move |()| b.log("[UI] back"))/>
                <Header
                    title="Gate camera"
                    subtitle="online · 12 ms"
                    on_back=Callback::new(move |()| b.log("[UI] back"))
                    avatar=HeaderAvatar {
                        id: "cam-3".into(),
                        name: "Gate".into(),
                        group: false,
                        status: Some(PresenceStatus::Online),
                    }
                    actions=ViewFn::from(move || {
                        view! {
                            <IconButton icon=icons::VIDEOCAM label="Live" on_click=Callback::new(move |()| b.log("[UI] live"))/>
                            <IconButton icon=icons::MORE_VERT label="More" on_click=Callback::new(move |()| b.log("[UI] more"))/>
                        }
                    })
                />
            </div>
        </Section>
    }
}

fn dialog_section() -> impl IntoView {
    let b = backend();
    let delete_open = RwSignal::new(false);
    let reboot_open = RwSignal::new(false);
    view! {
        <Section
            id="dialog"
            title="ConfirmDialog"
            note="Confirm runs async: success closes after 500 ms, error pulses then unlocks the dialog."
        >
            <Row>
                <ActionButton
                    text="Delete recording…"
                    dangerous=true
                    on_click=AsyncAction::new(move |()| async move {
                        delete_open.set(true);
                        ActionResult::Success
                    })
                />
                <ActionButton
                    text="Reboot device…"
                    on_click=AsyncAction::new(move |()| async move {
                        reboot_open.set(true);
                        ActionResult::Success
                    })
                />
            </Row>
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
        </Section>
    }
}

fn card_section() -> impl IntoView {
    view! {
        <Section id="card" title="Card · Divider" note="Rectangular container, 3 DU padding, surface or surface_alt background.">
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
        </Section>
    }
}

// ---------------------------------------------------------------------------
// App shell
// ---------------------------------------------------------------------------

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = "https://github.com/pavver/operator-design-system";

const NAV: [(&str, &str); 17] = [
    ("tokens", "Tokens"),
    ("action-button", "ActionButton"),
    ("icon-button", "IconButton"),
    ("switch", "AsyncSwitch"),
    ("checkbox", "Checkbox"),
    ("text-field", "TextField"),
    ("slider", "Slider"),
    ("stepper", "Stepper"),
    ("segmented", "Segmented"),
    ("dropdown", "Dropdown"),
    ("list-item", "ListItem"),
    ("status-badge", "StatusBadge"),
    ("avatar", "Avatar"),
    ("header", "Header"),
    ("dialog", "ConfirmDialog"),
    ("card", "Card"),
    ("console", "Console"),
];

fn controls(dark: RwSignal<bool>, b: Backend) -> impl IntoView {
    let mode_index = RwSignal::new(0usize);
    let outcome_index = RwSignal::new(2usize);
    let latency = RwSignal::new("1200 ms".to_string());
    Effect::new(move |_| dark.set(mode_index.get() == 0));
    Effect::new(move |_| {
        b.outcome.set(match outcome_index.get() {
            0 => Outcome::Success,
            1 => Outcome::Error,
            _ => Outcome::Alternate,
        })
    });
    Effect::new(move |_| {
        let ms = latency
            .get()
            .trim_end_matches(" ms")
            .parse()
            .unwrap_or(1200);
        b.latency_ms.set(ms);
    });
    view! {
        <div class="demo-controls">
            <div class="demo-labelled">
                <span class="op-t-caption demo-muted">"Mode"</span>
                <SegmentedControl
                    items=vec![SegmentItem::item("Dark", icons::DARK_MODE), SegmentItem::item("Light", icons::LIGHT_MODE)]
                    selected=mode_index
                />
            </div>
            <div class="demo-labelled">
                <span class="op-t-caption demo-muted">"Next result"</span>
                <SegmentedControl
                    items=vec![SegmentItem::text("Success"), SegmentItem::text("Error"), SegmentItem::text("Alternate")]
                    selected=outcome_index
                />
            </div>
            <DropdownMenu
                label="Latency"
                options=vec!["150 ms".into(), "600 ms".into(), "1200 ms".into(), "3000 ms".into(), "8000 ms".into()]
                selected=latency
            />
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let theme = RwSignal::new(TokenTheme::Operator);
    let dark = RwSignal::new(true);
    let b = Backend::new();
    provide_context(b);

    view! {
        <ThemeRoot theme=theme dark=dark class="demo-root">
            <div class="demo-layout">
                <aside class="demo-aside">
                    <div class="demo-brand">
                        <span class="op-t-app-title">"Operator UI"</span>
                        <span class="op-t-caption demo-muted">{format!("v{VERSION} · web · leptos")}</span>
                        <span class="demo-links op-t-caption">
                            <a href=REPO_URL>"GitHub"</a>
                            <a href=format!("{REPO_URL}/tree/main/docs")>"Docs"</a>
                            <a href=format!("{REPO_URL}/releases")>"Releases"</a>
                        </span>
                    </div>
                    {controls(dark, b)}
                    <nav class="demo-nav op-t-body-small">
                        {NAV.into_iter().map(|(id, name)| view! { <a href=format!("#{id}")>{name}</a> }).collect_view()}
                    </nav>
                </aside>
                <main class="demo-main">
                    {tokens_section()}
                    {action_button_section()}
                    {icon_button_section()}
                    {switch_section()}
                    {checkbox_section()}
                    {text_field_section()}
                    {slider_section()}
                    {stepper_section()}
                    {segmented_section()}
                    {dropdown_section()}
                    {list_section()}
                    {badge_section()}
                    {avatar_section()}
                    {header_section()}
                    {dialog_section()}
                    {card_section()}
                    <Section id="console" title="ConsoleTerminal" note="Event log of this page; auto-scrolls to the newest line.">
                        <ConsoleTerminal logs=b.logs/>
                    </Section>
                </main>
            </div>
        </ThemeRoot>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
