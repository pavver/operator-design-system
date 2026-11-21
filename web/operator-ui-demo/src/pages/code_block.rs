//! Live examples for code block.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

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

        <Example title="Customization" note="Code metrics, a header slot and an application copy string." block=true>
            <ThemeRoot strings=Callback::new(|key| (key == "code-block.copy").then(|| "Copy snippet".to_string()))>
                <CodeBlock code=SAMPLE language="toml" padding=5.0 font_size=15.0 line_height=23.0
                    header=|| view! { <strong>"Camera configuration"</strong> }/>
            </ThemeRoot>
        </Example>
    }
    .into_any()
}
