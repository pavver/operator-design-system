# MediaRecordButton

Tactical media recording button that captures audio voice notes and video notes.
It features hold-to-record, slide-left-to-cancel, and slide-up-to-lock gestures.

It ships in the media extension: the `media-record-button` feature is in the `media` and `extensions` groups, not in `full`.

Status: **draft** on web.

## Use

Use `MediaRecordButton` alongside chat input bars and messaging dialogs.
A quick tap toggles between audio voice note and video note capture modes.
Pressing and holding begins recording; sliding left cancels without saving, while sliding up locks recording into hands-free mode.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` standard (supports optional circular exception). |
| Standard border | `0.25 DU` container outline border. |
| Default size | `12 DU` (square control; `--op-media-record-button-size`). |

## Behavior

* Tap without holding switches between voice audio and round video note mode.
* Pressing and holding past 200 ms triggers `on_record_start`.
* Sliding left cancels active recording and triggers `on_record_cancel`.
* Sliding up locks recording into hands-free mode and triggers `on_record_locked`.
* Releasing after hold completes recording and triggers `on_record_stop`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `mode` | `MediaRecordMode` | `Voice` | Recording mode (`Voice` or `Video`). |
| `recording` | `bool` | `false` | Whether recording is active. |
| `locked` | `bool` | `false` | Hands-free locked state. |
| `disabled` | `bool` | `false` | Disables interaction. |
| `circular` | `bool` | `false` | Optional round shape exception. |
| `size` | `f32` | `12.0` | Button size in Design Units. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |
| `on_mode_change` | `Callback<MediaRecordMode>` | none | Event reporting mode toggle. |
| `on_record_start` | `Callback<MediaRecordMode>` | none | Event when recording begins. |
| `on_record_stop` | `Callback<()>` | none | Event when recording finishes. |
| `on_record_cancel` | `Callback<()>` | none | Event when recording is cancelled. |
| `on_record_locked` | `Callback<()>` | none | Event when recording is locked. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `size` | `--op-media-record-button-size` | `12.0` | DU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-media-record-button-bg` | `var(--op-surface)` | Resting background color. |
| `--op-media-record-button-border` | `var(--op-border)` | Resting border color. |
| `--op-media-record-button-color` | `var(--op-text-primary)` | Resting icon color. |
| `--op-media-record-button-active-border` | `var(--op-accent)` | Active recording border. |
| `--op-media-record-button-locked-border` | `var(--op-success)` | Locked hands-free border. |
| `--op-media-record-button-canceling-border` | `var(--op-error)` | Cancelling drag border. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `media-record.audio` | `"Voice note"` | Accessible voice mode label. |
| `media-record.video` | `"Video note"` | Accessible video mode label. |
| `media-record.locked-label` | `"Locked recording (click to stop)"` | Accessible locked state label. |
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
