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
| `Counter` | `count`, `max` (`99`), `tone` (Accent), `show_zero` (`false`), `aria_label` (optional). |
| `WithCounter` | `count`, `max`, `tone`, `show_zero`, `aria_label`, `offset`, `children`: the control that carries the counter. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `min_width` | DU | `--op-counter-min-width` | `icon_medium` (`4.5 DU`) |
| `height` | DU | `--op-counter-height` | `icon_medium` (`4.5 DU`) |
| `padding_x` | DU | `--op-counter-padding-x` | `pad_tight` (`1 DU`) |
| `font_size` | TU | `--op-counter-font-size` | `type_caption_size` |
| `offset` | DU | `--op-counter-offset` | `pad_tight` (`1 DU`); on WithCounter |

| Color variable | Default |
| :--- | :--- |
| `--op-counter-background` | `accent` |
| `--op-counter-text` | `on_accent` |
| `--op-counter-neutral-background` | `surface_alt` |
| `--op-counter-neutral-text` | `text_secondary` |
| `--op-counter-neutral-border` | `border` |
| `--op-counter-error-background` | `error` |
| `--op-counter-error-text` | `on_accent` |

`class` and `style` apply to the root.

### Strings

| Key | Default |
| :--- | :--- |
| `counter.items` | `items` |

## Accessibility

The counter is named "`n` items" with the exact number, even when it shows `99+`.
