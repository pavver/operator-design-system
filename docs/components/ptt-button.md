# PttButton

Tactical push-to-talk control for voice communication, telemetry links, and mission dispatch.
It supports press-and-hold audio transmission, upward drag gesture to lock hands-free mode, and Operator state borders.

Status: **draft** on web.

## Use

Use `PttButton` in tactical radios, voice intercom channels, operator dispatch terminals, and team coordination views.
Do not use `PttButton` for regular text messaging actions; use [ActionButton](action-button.md) or [MediaRecordButton](media-record-button.md) instead.
When locked, single-tap releases the lock and terminates transmission immediately.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry; round exception `50%` circular when `circular` prop is enabled). |
| Standard border | `0.25 DU` standard border width. |
| Emphasis border | `0.5 DU` active/talking border stroke (`--op-ptt-button-border-width`). |
| Default size | `22.5 DU` square dimension (`--op-ptt-button-size`). |

## Behavior

* Pointer down initiates transmission immediately and marks the control as talking.
* Dragging upward beyond the lock threshold locks the transmission into hands-free mode.
* Releasing the pointer when unlocked terminates transmission.
* Single-tapping when locked releases the lock and concludes transmission.
* The button reports changes through the `on_state_change` callback with `(talking, locked)`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `talking` | `bool` | `false` | True when audio transmission is actively taking place. |
| `locked` | `bool` | `false` | True when hands-free lock is engaged. |
| `state` | `Option<ControlState>` | `None` | Operator control state override. |
| `disabled` | `bool` | `false` | Disables user interaction. |
| `circular` | `bool` | `false` | Circular round shape exception. |
| `size` | `f32` | `22.5` | Button size in Design Units. |
| `border_width` | `f32` | `0.5` | Border stroke width in Design Units. |
| `label` | `string` | none | Custom label text override. |
| `on_state_change` | `Callback<(bool, bool)>` | none | Event reporting `(is_talking, is_locked)`. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `size` | `--op-ptt-button-size` | `22.5` | DU |
| `border_width` | `--op-ptt-button-border-width` | `0.5` | DU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-ptt-button-bg` | `var(--op-color-surface)` | Rest background color. |
| `--op-ptt-button-border` | `var(--op-color-border)` | Rest border color. |
| `--op-ptt-button-color` | `var(--op-color-text)` | Rest icon and label color. |
| `--op-ptt-button-active-bg` | accent tint | Active transmitting background. |
| `--op-ptt-button-active-border` | `var(--op-color-accent)` | Active transmitting border. |
| `--op-ptt-button-locked-bg` | success tint | Locked hands-free background. |
| `--op-ptt-button-locked-border` | `var(--op-color-success)` | Locked hands-free border. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `ptt-button.idle` | `"PUSH TO TALK"` | Resting label. |
| `ptt-button.talking` | `"TALKING"` | Active transmitting label. |
| `ptt-button.locked` | `"HANDS-FREE"` | Locked transmission label. |
| `ptt-button.hint` | `"Slide up to lock"` | Hint displayed while holding. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `ptt-button.mic` | `icons::ARROW_UPWARD` | Transmit direction icon. |
| `ptt-button.lock` | `icons::LOCK` | Hands-free lock icon. |

## Accessibility

* Renders as an HTML `<button>` element with `aria-pressed` mirroring the transmitting state.
* Accessible status announcements reflect idle, active, and locked states via `aria-label`.
* Keyboard focus is visibly marked with high-contrast outline.
