# ThemeToggle

Day/night switch for light and dark mode.
A sun slides across a sky with clouds and turns into a moon among stars while it moves.

Status: **reviewed** on web.

It is based on the [Day/Night Toggle](https://codepen.io/jh3y/pen/LYgjpYZ) by Jhey Tompkins, MIT License.
The cloud outlines, sky, sun, and moon colors, and the easing come from the original.
The bear animation and idle twinkling of the original are left out; nothing moves while the toggle is idle.
The original moon that slides over the sun is replaced by a single body that changes color.

## Geometry

The pill track and the round thumb are an allowed exception to the rectangular geometry.

| Property | Value |
| :--- | :--- |
| Size | `17 DU` × `7.5 DU` by default. |
| Thumb | Circle with a diameter of the height minus `1.25 DU`, inset `0.625 DU`. |
| Outline | `0.25 DU`, `border` at 50%; `border_strong` while pressed. |

## Colors

The illustration uses dedicated tokens that are identical in both modes:

| Token | Value | Part |
| :--- | :--- | :--- |
| `toggle_day_sky` | `#3893C8` | Day sky. |
| `toggle_night_sky` | `#1E2333` | Night sky. |
| `toggle_sun` | `#F7CA31` | Sun. |
| `toggle_moon` | `#CAD5DE` | Moon. |
| `toggle_crater` | `#9EB0C2` | Moon craters. |
| `toggle_cloud` | `#FFFFFF` | Front clouds, 95% opacity. |
| `toggle_cloud_back` | `#D9D9D9` | Back clouds, 45% opacity. |
| `toggle_star` | `#FFFFFF` | Stars and sun halos. |

## Motion

* All parts move over `450 ms` with `cubic-bezier(0.4, -0.2, 0.4, 1.2)`, a slight overshoot.
* The sky color changes over `400 ms`.
* The sun and the moon are one body.
  While it slides, its color changes from `toggle_sun` to `toggle_moon` and the craters fade in, both over `450 ms` with the smooth curve.
* Day to night: the body slides right and becomes the moon, the halos fade, the clouds sink and fade, and seven stars grow from `30%` to full size.
* Night to day reverses every movement.

## Behavior

* A click calls `on_toggle` with the opposite value immediately.
* The toggle has no async, disabled, or result states.
  A theme is a local preference: it must apply at once, and persisting it is the caller's background task.
  Where the theme cannot be changed, for example while it follows the system setting, hide the toggle instead of disabling it.
* Press feedback: ripple clipped to the pill and a `border_strong` outline.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `dark` | bool | required | `true` for night. |
| `on_toggle` | callback `(bool)` | none | Called with the new value; without it the toggle is static. |
| `width` | DU | `17` | Width. |
| `height` | DU | `7.5` | Height. |
| `label` | string | `"Dark mode"` | Accessible name. |

## Accessibility

The control has the `switch` role and is checked in night mode.
