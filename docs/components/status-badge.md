# StatusBadge

Small round indicator of presence or activity, used alone or on an [Avatar](avatar.md).

Status: **reviewed** on web.

## Geometry

* Circle, `3 DU` diameter by default.
* A `0.25 DU` outline in `status_border` separates the badge from any background.
* Every animation stays inside the badge bounds; the badge never changes the layout.

## Variants

All colors come from the `status_*` tokens.

| Status | Look |
| :--- | :--- |
| `Online` | Solid `status_online`. |
| `Offline` | Solid `status_offline`. |
| `Alarm` | `status_alarm` flashing on and off with a `250 ms` half period. |
| `Error` | Solid `status_error`. |
| `Pending` | Spinner in `status_activity` with a `0.3 DU` stroke. |
| `Typing` | Three `status_activity` dots jumping in a wave with an `800 ms` period. |
| `RecordingAudio` | A source dot with three sound arcs pulsing left to right over `900 ms`. |
| `SendingFile` | Three chevrons pulsing left to right over `900 ms`. |

### RecordingAudio geometry

Values are relative to the badge size.

* Source dot at `18%` width and `50%` height, radius `11%`, pulsing between `90%` and `100%`.
* Three arcs with radii `28%`, `51.5%`, and `75%`, each pulsing between `90%` and `100%`.
* Each arc spans `64°` centered on the right direction (`-32°` to `+32°`).
* Stroke `9%`, pulsing between `85%` and `100%`, with round caps.
* Arc opacity pulses between `25%` and `100%`; neighboring arcs are offset by `0.18` of the cycle.
* Vertical equalizer bars are not used for this state.

### SendingFile geometry

* Three stationary chevrons made of two round-capped strokes each.
* Opacity and stroke width pulse with a smoothstep curve, from left to right.
* A generic spinner is not used for this state.

## API

| Property | Type | Default |
| :--- | :--- | :--- |
| `status` | `PresenceStatus` | required |
| `size` | DU | `3` |
| `text` | string | none; optional caption next to the badge |
| `text_slot` | slot | none; custom caption content |
| `gap` | DU | `2` |
| `font_size` | TU | `type_caption_size` |
| `line_height` | TU | `type_caption_line` |
| `aria_label` | string | status name |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-status-badge-size` | `pad_card` (`3 DU`) |
| `gap` | DU | `--op-status-badge-gap` | `pad_small` (`2 DU`) |
| `font_size` | TU | `--op-status-badge-font-size` | `type_caption_size` |
| `line_height` | TU | `--op-status-badge-line-height` | `type_caption_line` |

| Color variable | Default |
| :--- | :--- |
| `--op-status-badge-online` | `status_online` |
| `--op-status-badge-offline` | `status_offline` |
| `--op-status-badge-alarm` | `status_alarm` |
| `--op-status-badge-error` | `status_error` |
| `--op-status-badge-fallback` | `status_fallback` |
| `--op-status-badge-activity` | `status_activity` |
| `--op-status-badge-border` | `status_border` |
| `--op-status-badge-text` | `text_primary` |

| Slot | Place |
| :--- | :--- |
| `text_slot` | Caption next to the badge. |

`class` and `style` apply to the root.

### Strings

| Key | Default |
| :--- | :--- |
| `status-badge.online` | `Online` |
| `status-badge.offline` | `Offline` |
| `status-badge.alarm` | `Alarm` |
| `status-badge.pending` | `Pending` |
| `status-badge.error` | `Error` |
| `status-badge.typing` | `Typing` |
| `status-badge.recording-audio` | `Recording audio` |
| `status-badge.sending-file` | `Sending file` |
