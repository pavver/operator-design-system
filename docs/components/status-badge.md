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
