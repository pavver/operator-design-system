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
}

impl Backend {
    fn new() -> Self {
        Self {
            latency_ms: RwSignal::new(1200),
            outcome: RwSignal::new(Outcome::Alternate),
            flip: StoredValue::new(false),
        }
    }

    async fn call(self, what: String) -> ActionResult {
        sleep_ms(self.latency_ms.get_untracked()).await;
        let ok = match self.outcome.get_untracked() {
            Outcome::Success => true,
            Outcome::Error => false,
            Outcome::Alternate => {
                self.flip.update_value(|f| *f = !*f);
                self.flip.get_value()
            }
        };
        let _ = what;
        if ok {
            ActionResult::Success
        } else {
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
                <span class=class>"Камера 3 · Ворота — Gate camera 0x4F2A"</span>
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

fn action_button_section() -> impl IntoView {
    let b = backend();
    let enabled_index = RwSignal::new(0usize);
    let enabled = Signal::derive(move || enabled_index.get() == 0);
    view! {
        <Section
            id="action-button"
            title="ActionButton"
            note="Dependency of TextField (draft, not yet reviewed). Async action with pending running border, 500 ms result icon flash and 300 ms tail. Clicks are blocked while pending."
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
                <div class="demo-narrow">
                    <SegmentedControl
                        items=vec![SegmentItem::text("Enabled"), SegmentItem::text("Disabled")]
                        selected=enabled_index
                    />
                </div>
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

// ---------------------------------------------------------------------------
// App shell
// ---------------------------------------------------------------------------

const NAV: [(&str, &str); 5] = [
    ("tokens", "Tokens"),
    ("text-field", "TextField"),
    ("slider", "Slider"),
    ("segmented", "SegmentedControl"),
    ("action-button", "ActionButton"),
];

const LATENCIES: [u32; 4] = [150, 600, 1200, 3000];

fn controls(dark: RwSignal<bool>, b: Backend) -> impl IntoView {
    let mode_index = RwSignal::new(0usize);
    let outcome_index = RwSignal::new(2usize);
    let latency_index = RwSignal::new(2usize);
    Effect::new(move |_| dark.set(mode_index.get() == 0));
    Effect::new(move |_| {
        b.outcome.set(match outcome_index.get() {
            0 => Outcome::Success,
            1 => Outcome::Error,
            _ => Outcome::Alternate,
        })
    });
    Effect::new(move |_| {
        let i = latency_index.get().min(LATENCIES.len() - 1);
        b.latency_ms.set(LATENCIES[i]);
    });
    view! {
        <div class="demo-controls">
            <div class="demo-labelled">
                <span class="op-t-caption demo-muted">"Mode"</span>
                <SegmentedControl
                    items=vec![SegmentItem::item("Dark", icons::DARK_MODE), SegmentItem::item("Light", icons::WB_SUNNY)]
                    selected=mode_index
                />
            </div>
            <div class="demo-labelled">
                <span class="op-t-caption demo-muted">"Next result"</span>
                <SegmentedControl
                    items=vec![SegmentItem::text("Success"), SegmentItem::text("Error"), SegmentItem::text("Alt")]
                    selected=outcome_index
                />
            </div>
            <div class="demo-labelled">
                <span class="op-t-caption demo-muted">"Latency, ms"</span>
                <SegmentedControl
                    items=LATENCIES.iter().map(|ms| SegmentItem::text(ms.to_string())).collect()
                    selected=latency_index
                />
            </div>
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
                        <span class="op-t-caption demo-muted">"web · leptos"</span>
                    </div>
                    {controls(dark, b)}
                    <nav class="demo-nav op-t-body-small">
                        {NAV.into_iter().map(|(id, name)| view! { <a href=format!("#{id}")>{name}</a> }).collect_view()}
                    </nav>
                </aside>
                <main class="demo-main">
                    {tokens_section()}
                    {text_field_section()}
                    {slider_section()}
                    {segmented_section()}
                    {action_button_section()}
                </main>
            </div>
        </ThemeRoot>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
