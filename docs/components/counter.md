# Counter

Compact number badge for unread messages, alarms, or totals.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Size | `4.5 DU` tall, at least `4.5 DU` wide. |
| Padding | `1 DU` horizontal. |
| Type | `caption` size, bold, tabular figures. |
| Corners | `0 DU`. |
| On a control | Top end corner, offset `1 DU` outward. |

## Tones

| Tone | Look | Use |
| :--- | :--- | :--- |
| Accent | `accent` background, `on_accent` text. | New or unread items. |
| Neutral | `surface_alt` background, `border` outline, `text_secondary` text. | Plain totals. |
| Error | `error` background, `on_accent` text. | Failures and alarms. |

## Behavior

* Values above `max` (default 99) show as `99+`.
* At zero the counter is hidden unless `show_zero` is set.

## API

| Component | Properties |
| :--- | :--- |
| `Counter` | `count`, `max` (`99`), `tone` (Accent), `show_zero` (`false`). |
| `WithCounter` | `count`, `max`, `tone`, `children`: the control that carries the counter. |

## Accessibility

The counter is named "`n` items" with the exact number, even when it shows `99+`.
