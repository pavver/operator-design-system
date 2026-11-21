# Avatar

Image or initial of a person, group, or channel, with an optional status badge.

Status: **reviewed** on web.

## Shape

The shape tells what the avatar represents.
Rounded shapes here are an allowed exception to the rectangular geometry.

| Kind | Shape |
| :--- | :--- |
| `Direct` (a person) | Circle. |
| `ThreadedGroup` | Square, `0` radius. |
| `Channel` | Rounded square, radius `15%` of the size. |
| `SimpleGroup` | Rounded square, radius `35%` of the size. |

## Geometry

| Property | Value |
| :--- | :--- |
| Size | `avatar_header` (`10 DU`) by default; `avatar_small` (`6 DU`) and `avatar_default` (`8 DU`) for compact lists. |
| Outline | `0.25 DU` in `border`, always present. |
| Initial | First letter of the name, uppercase, bold, `45%` of the size. |
| Loading spinner | `80%` of the size, `pending`, centered around the initial. |
| Status badge | Bottom-right corner, overlapping the outline. |

## Color

Avatars without an image use a color derived from their id, so the same entity has the same color everywhere.

1. Compute the Java `String.hashCode` of the id: for each UTF-16 code unit, `h = h * 31 + unit` with 32-bit wrapping arithmetic.
2. Pick a hue from `[190, 210, 250, 290, 330, 160, 35]` at index `abs(h) mod 7`.
3. Convert from HSV with saturation `0.45` and value `0.75` in dark mode, or `0.65` and `0.8` in light mode.

The initial uses this color; the background is the same color at 15%.

## Behavior

* An image covers the whole shape with center cropping.
* While the image loads, the initial and the spinner are shown.

## API

| Property | Type | Default |
| :--- | :--- | :--- |
| `id` | string | required |
| `name` | string | required |
| `kind` | `AvatarKind` | `Direct` |
| `size` | DU | `10` |
| `status` | `PresenceStatus` | none |
| `image_url` | string | none |
| `badge` | slot | none |
| `font_size` | TU | computed from size |
| `border_width` | DU | `0.25` |
| `badge_offset` | DU | `0.25` |
| `badge_size` | DU | computed from size |
| `aria_label` | string | `name` |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-avatar-size` | `avatar_header` (`10 DU`) |
| `font_size` | TU | `--op-avatar-font-size` | `18 TU` at 10 DU (`size * 1.8`) |
| `border_width` | DU | `--op-avatar-border-width` | `border` (`0.25 DU`) |
| `badge_offset` | DU | `--op-avatar-badge-offset` | `border` (`0.25 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-avatar-background` | `color-mix(in srgb, var(--op-avatar-color) 15%, transparent)` |
| `--op-avatar-text` | `var(--op-avatar-color)` |
| `--op-avatar-border` | `border` |

| Slot | Place |
| :--- | :--- |
| `badge` | Replaces the built-in status badge at the bottom end corner. |

`class` and `style` apply to the root.
