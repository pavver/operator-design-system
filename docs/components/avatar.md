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
