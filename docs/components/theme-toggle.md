# ThemeToggle

Day/night switch for light and dark mode.
A sun slides across a sky with clouds and turns into a moon among stars.

Status: **draft**, new on web.

It is based on the [Day/Night Toggle](https://codepen.io/jh3y/pen/LYgjpYZ) by Jhey Tompkins, MIT License.
The cloud outlines, sky, sun, and moon colors, and the easing come from the original.
The bear animation and idle twinkling of the original are left out; nothing moves while the toggle is idle.

## Geometry

The pill track and the round thumb are an allowed exception to the rectangular geometry.

| Property | Value |
| :--- | :--- |
| Size | `17 DU` × `7.5 DU` by default. |
| Thumb | Circle with a diameter of the height minus `1.25 DU`, inset `0.625 DU`. |
| Outline | State border with the spinner stroke (`0.375 DU`); base color `border` at 50%. |
| Spinner | `65%` of the thumb diameter, `pending`. |

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

Disabled replaces the sky with `disabled_surface`.

## Motion

* All parts move over `450 ms` with `cubic-bezier(0.4, -0.2, 0.4, 1.2)`, a slight overshoot.
* The sky color changes over `400 ms`.
* Day to night: the thumb slides right, the moon slides over the sun from the right, the halos fade, the clouds sink and fade, and seven stars grow from `30%` to full size.
* Night to day reverses every movement.

## Behavior

* Without an async handler, a click calls `on_toggle` with the opposite value immediately.
* With an async handler:
  1. The thumb slides to the requested side and is replaced by a spinner; the outline runs `pending`; clicks are ignored.
  2. On **success**, `on_toggle` is called, and the outline pulses `success` for `500 ms`.
  3. On **error**, the thumb slides back with the same motion, and the outline pulses `error` for `600 ms`.
  4. The result then fades out over `500 ms`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `dark` | bool | required | `true` for night. |
| `on_toggle` | callback `(bool)` | none | Commits the new value. |
| `on_toggle_async` | async `(bool) -> ActionResult` | none | Persists the value before `on_toggle`. |
| `enabled` | bool | `true` | Interactivity. |
| `state` | `ControlState?` | none | External state override. |
| `width` | DU | `17` | Width. |
| `height` | DU | `7.5` | Height. |
| `label` | string | `"Dark mode"` | Accessible name. |

## Accessibility

The control has the `switch` role and is checked in night mode.
