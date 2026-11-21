# MediaRecordButton

Tactical media recording button that reports capture intents for voice audio and circular video notes.
It supports click mode toggling, hold-to-record, slide-left-to-cancel, slide-up-to-lock, and active duration timer feedback.

Status: **draft** on web.

## Use

Use `MediaRecordButton` in chat inputs, dispatch panels, and multimedia communication feeds.
The button reports recording intents (`on_record_start`, `on_record_stop`, `on_record_cancel`, `on_record_locked`) to the application.
The host application is responsible for hardware microphone/camera streaming and audio file creation.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry; round exception `50%` circular when `circular` prop is enabled). |
| Standard border | `0.25 DU` standard resting border. |
| Emphasis border | `0.5 DU` border during active recording or locked states. |
| Default size | `12 DU` (48px square control; `--op-media-record-button-size`). |

## Behavior

* Short click toggles recording mode between Voice and Video note.
* Holding down for 200ms triggers active recording start.
* Sliding left by more than 15 DU triggers cancelation and discards the recording.
* Sliding up by more than 15 DU locks the recording into hands-free mode.
* When locked, a single click finalizes and submits the recording.
* When released without locking or canceling, the recording completes normally.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `mode` | `MediaRecordMode` | `Voice` | Current media capture mode (`Voice` or `Video`). |
| `recording` | `bool` | `false` | True when media recording is currently active. |
| `locked` | `bool` | `false` | True when recording is locked in hands-free mode. |
| `disabled` | `bool` | `false` | Disables user interaction. |
| `circular` | `bool` | `false` | Circular round shape exception. |
| `size` | `f32` | `12.0` | Size in Design Units. |
| `on_mode_change` | `Callback<MediaRecordMode>` | none | Triggered on mode toggle. |
| `on_record_start` | `Callback<MediaRecordMode>` | none | Triggered when hold threshold begins recording. |
| `on_record_stop` | `Callback<()>` | none | Triggered when recording completes. |
| `on_record_cancel` | `Callback<()>` | none | Triggered when slide-left cancels recording. |
| `on_record_locked` | `Callback<()>` | none | Triggered when slide-up locks hands-free recording. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `size` | `--op-media-record-button-size` | `12.0` | DU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-media-record-button-bg` | `var(--op-color-surface)` | Resting background color. |
| `--op-media-record-button-border` | `var(--op-color-border)` | Resting border color. |
| `--op-media-record-button-active-bg` | accent tint | Active recording background. |
| `--op-media-record-button-active-border` | `var(--op-color-accent)` | Active recording border. |
| `--op-media-record-button-locked-bg` | success tint | Locked hands-free background. |
| `--op-media-record-button-locked-border` | `var(--op-color-success)` | Locked hands-free border. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `media-record.audio` | `"Voice note"` | Accessible voice mode label. |
| `media-record.video` | `"Video note"` | Accessible video mode label. |
| `media-record.cancel-hint` | `"< Slide to cancel"` | Slide left cancel hint. |
| `media-record.lock-hint` | `"^ Slide to lock"` | Slide up lock hint. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `media-record.voice` | `icons::ARROW_UPWARD` | Audio voice note indicator. |
| `media-record.video` | `icons::VIDEOCAM` | Video note indicator. |
| `media-record.lock` | `icons::LOCK` | Hands-free lock indicator. |

## Accessibility

* Accessible labels update based on active mode and locked status.
* Live recording duration timer is marked with high-visibility pulsing dot.
