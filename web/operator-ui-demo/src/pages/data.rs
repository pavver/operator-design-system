//! Examples for data display components.

use crate::kit::{Cell, Example, backend, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

fn camera_rows() -> Vec<TableRow> {
    [
        ("c1", "Gate North", "Online", "12", "H.265", "4K"),
        ("c2", "Gate South", "Online", "18", "H.265", "1080p"),
        ("c3", "Yard East", "Offline", "—", "H.264", "1080p"),
        ("c4", "Parking A", "Online", "40", "H.264", "720p"),
        ("c5", "Warehouse dock", "Alarm", "9", "H.265", "4K"),
        ("c6", "Server room", "Online", "3", "MJPEG", "720p"),
        ("c7", "Reception", "Online", "22", "H.265", "1080p"),
        ("c8", "Roof", "Offline", "—", "H.264", "4K"),
    ]
    .into_iter()
    .map(|(id, a, b, c, d, e)| TableRow::new(id, [a, b, c, d, e]))
    .collect()
}

pub fn data_table() -> AnyView {
    let b = backend();
    let rows = RwSignal::new(camera_rows());
    let selected = RwSignal::new(Some("c5".to_string()));
    let loading = RwSignal::new(false);
    let columns = || {
        vec![
            Column::new("Camera").sortable(),
            Column::new("Status").sortable().width(24.0),
            Column::new("RTT, ms").numeric().sortable().width(22.0),
            Column::new("Codec").width(20.0),
            Column::new("Resolution").width(24.0),
        ]
    };
    let no_rows: Vec<TableRow> = Vec::new();
    let reload = AsyncAction::new(move |()| async move {
        loading.set(true);
        let r = b.call("reload cameras".into()).await;
        loading.set(false);
        r
    });
    view! {
        <Example title="Sortable, selectable" note="Click a header to sort ascending, descending, then back to the original order." block=true>
            <DataTable
                columns=columns()
                rows=rows
                selected=selected
                loading=loading
                max_height=70.0
                label="Cameras"
                on_row_click=Callback::new(move |id: String| b.log(format!("[UI] row {id}")))
            />
            <div class="demo-inline">
                <ActionButton text="Reload" icon=icons::REFRESH on_click=reload/>
                <ActionButton text="Clear rows" on_click=instant(move || rows.set(Vec::new()))/>
                <ActionButton text="Restore rows" on_click=instant(move || rows.set(camera_rows()))/>
            </div>
        </Example>
        <Example title="Dense, empty" block=true>
            <DataTable
                columns=columns()
                rows=no_rows
                dense=true
                empty=ViewFn::from(|| view! { <EmptyState title="No cameras" description="Add a camera to see it here." compact=true/> })
            />
        </Example>
    }
    .into_any()
}

pub fn counter() -> AnyView {
    let unread = RwSignal::new(3i32);
    let count = Signal::derive(move || u32::try_from(unread.get()).unwrap_or(0));
    view! {
        <Example title="Tones and overflow">
            <Cell label="Accent"><Counter count=3u32/></Cell>
            <Cell label="Neutral"><Counter count=27u32 tone=CounterTone::Neutral/></Cell>
            <Cell label="Error"><Counter count=1u32 tone=CounterTone::Error/></Cell>
            <Cell label="Over max"><Counter count=1250u32/></Cell>
            <Cell label="Zero shown"><Counter count=0u32 show_zero=true tone=CounterTone::Neutral/></Cell>
        </Example>
        <Example title="On an icon button" note="Click the inbox to mark everything read.">
            <WithCounter count=count>
                <IconButton icon=icons::INBOX label="Inbox" on_click=Callback::new(move |()| unread.set(0))/>
            </WithCounter>
            <Stepper value=unread min=0 max=150/>
        </Example>
    }
    .into_any()
}

pub fn property_list() -> AnyView {
    view! {
        <Example title="Device details" note="Narrow containers stack the label above the value." block=true>
            <div class="demo-grid-2">
                <PropertyList>
                    <Property label="Model" value="DS-2CD2143G2-I"/>
                    <Property label="Firmware" value="V5.7.3 build 220112"/>
                    <Property label="Address" value="192.168.1.40" copyable=true/>
                    <Property label="Serial" value="DS2CD2143-20220112AAWRJ3" copyable=true/>
                    <Property label="Status">
                        <StatusBadge status=PresenceStatus::Online text=Signal::stored("Online".to_string())/>
                    </Property>
                </PropertyList>
                <div style:max-width=du(55.0)>
                    <PropertyList>
                        <Property label="Uptime" value="12 d 4 h"/>
                        <Property label="Storage" value="1.2 / 2 TB"/>
                    </PropertyList>
                </div>
            </div>
        </Example>
    }
    .into_any()
}

pub fn accordion() -> AnyView {
    view! {
        <Example title="Accordion" note="Only one section is open at a time." block=true>
            <div class="demo-narrow">
                <Accordion>
                    <Collapsible title="Network" subtitle="DHCP · 192.168.1.40" initially_open=true>
                        <PropertyList>
                            <Property label="Mode" value="DHCP"/>
                            <Property label="Gateway" value="192.168.1.1"/>
                        </PropertyList>
                    </Collapsible>
                    <Collapsible title="Video" subtitle="H.265 · 4K · 25 fps">
                        <p class="demo-p">"Main stream settings."</p>
                    </Collapsible>
                    <Collapsible title="Alarms" trailing=ViewFn::from(|| view! { <Counter count=2u32 tone=CounterTone::Error/> })>
                        <p class="demo-p">"Two zones report motion."</p>
                    </Collapsible>
                </Accordion>
            </div>
        </Example>
        <Example title="Standalone collapsible" block=true>
            <div class="demo-narrow">
                <Collapsible title="Advanced options">
                    <AsyncCheckbox checked=false text=Signal::stored("Verbose logging".to_string())/>
                </Collapsible>
            </div>
        </Example>
    }
    .into_any()
}

const SAMPLE: &str = r#"[camera.gate_north]
host = "192.168.1.40"
codec = "h265"
resolution = "3840x2160"
night_mode = "auto"   # auto | day | night
"#;

const RUST_SAMPLE: &str = r##"/// Reconnects a camera after a settings change.
#[derive(Debug, Clone)]
pub struct Camera<'a> {
    host: &'a str,
    retries: u8,
}

impl Camera<'_> {
    pub async fn reconnect(&self) -> Result<(), Error> {
        const MAX_WAIT_MS: u32 = 1_500;
        for attempt in 0..self.retries {
            log::info!("attempt {attempt} to {}", self.host);
            if probe(self.host, MAX_WAIT_MS).await.is_ok() {
                return Ok(());
            }
        }
        Err(Error::Timeout) // gave up
    }
}"##;

const KOTLIN_SAMPLE: &str = r#"@Composable
fun CameraRow(camera: Camera, onRetry: () -> Unit) {
    val label = if (camera.online) "Online" else "Offline"
    OperatorListItem(title = camera.name, subtitle = label)
}"#;

const JSON_SAMPLE: &str = r#"{
  "id": "gate_north",
  "online": true,
  "bitrate": 4.5e6,
  "zones": [1, 2, 3],
  "error": null
}"#;

const YAML_SAMPLE: &str = r#"# Deployment for the gate cameras
defaults: &defaults
  codec: h265
  night_mode: auto
cameras:
  - name: gate_north
    url: "rtsp://192.168.1.40:554/main"
    settings: *defaults
enabled: yes"#;

const SHELL_SAMPLE: &str = r#"#!/usr/bin/env bash
set -euo pipefail
for cam in gate_north gate_south; do
  if ! ping -c 1 "$cam" > /dev/null; then
    echo "offline: ${cam}" >&2  # report and continue
  fi
done"#;

pub fn code_block() -> AnyView {
    let log = (1..=14)
        .map(|i| {
            format!("12:00:{i:02} [RX] frame {i} keyframe=false size=48213 bytes from gate_north")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let short = log.lines().take(3).collect::<Vec<_>>().join("\n");
    view! {
        <Example title="With language and line numbers" block=true>
            <CodeBlock code=SAMPLE language="toml" line_numbers=true/>
        </Example>
        <Example title="Rust" block=true>
            <CodeBlock code=RUST_SAMPLE language="rust" line_numbers=true/>
        </Example>
        <Example title="Kotlin" block=true>
            <CodeBlock code=KOTLIN_SAMPLE language="kotlin"/>
        </Example>
        <Example title="JSON and YAML" block=true>
            <CodeBlock code=JSON_SAMPLE language="json"/>
            <CodeBlock code=YAML_SAMPLE language="yaml"/>
        </Example>
        <Example title="Shell" block=true>
            <CodeBlock code=SHELL_SAMPLE language="bash"/>
        </Example>
        <Example title="Long output, scrolling" note="An unknown language is shown without highlighting." block=true>
            <CodeBlock code=log.clone() language="log" max_height=40.0/>
        </Example>
        <Example title="Wrapped, no header" block=true>
            <CodeBlock code=short wrap=true copy=false/>
        </Example>
    }
    .into_any()
}

pub fn kbd() -> AnyView {
    view! {
        <Example title="Keys and shortcuts">
            <Kbd keys="Esc"/>
            <Kbd keys="Ctrl+K"/>
            <Kbd keys="Ctrl+Shift+P"/>
            <Kbd keys="Ctrl++"/>
        </Example>
        <Example title="Wide keys and arrows" note="Modifier and editing keys are wider; arrow names print as arrows.">
            <Kbd keys="Shift+Enter"/>
            <Kbd keys="Tab"/>
            <Kbd keys="Ctrl+Space"/>
            <Kbd keys="Up"/>
            <Kbd keys="Left"/>
            <Kbd keys="Down"/>
            <Kbd keys="Right"/>
        </Example>
        <Example title="In text">
            <span>"Press " <Kbd keys="F2"/> " to rename the selected camera."</span>
        </Example>
    }
    .into_any()
}
