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
| `label` | string | `theme-toggle.label` | Accessible name. |
| `thumb_inset` | reactive DU | unset | Overrides `--op-theme-toggle-thumb-inset`; see Customization. |
| `star_scale` | reactive DU | unset | Overrides `--op-theme-toggle-star-scale`; see Customization. |
| `class` | reactive string | none | Applied to the root element. |
| `style` | reactive string | none | Applied to the root element. |
| `content` | slot | none | Replaces the decorative scene; the switch retains its accessible name. |

## Accessibility

The control has the `switch` role and is checked in night mode.

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-theme-toggle-width` | `calc(var(--op-du) * 17)` |
| `height` | DU | `--op-theme-toggle-height` | `calc(var(--op-du) * 7.5)` |
| `thumb_inset` | DU | `--op-theme-toggle-thumb-inset` | `calc(var(--op-du) * 0.625)` |
| `star_scale` | DU | `--op-theme-toggle-star-scale` | `calc(var(--op-du) * 1)` |

The existing numeric size props keep their names and DU meaning and now also accept reactive values.

| Color variable | Default |
| :--- | :--- |
| `--op-theme-toggle-accent` | `var(--op-accent)` |
| `--op-theme-toggle-border` | `var(--op-border)` |
| `--op-theme-toggle-emphasis-border` | `var(--op-border-strong)` |
| `--op-theme-toggle-cloud` | `var(--op-toggle-cloud)` |
| `--op-theme-toggle-cloud-back` | `var(--op-toggle-cloud-back)` |
| `--op-theme-toggle-crater` | `var(--op-toggle-crater)` |
| `--op-theme-toggle-day-sky` | `var(--op-toggle-day-sky)` |
| `--op-theme-toggle-disabled-border` | `var(--op-disabled-border)` |
| `--op-theme-toggle-moon` | `var(--op-toggle-moon)` |
| `--op-theme-toggle-night-sky` | `var(--op-toggle-night-sky)` |
| `--op-theme-toggle-ripple` | `var(--op-ripple)` |
| `--op-theme-toggle-star` | `var(--op-toggle-star)` |
| `--op-theme-toggle-sun` | `var(--op-toggle-sun)` |
| Slot | Place |
| :--- | :--- |
| `content` | Replaces the decorative scene; the switch retains its accessible name. |

The decorative scene follows the resolved root dimensions, thumb inset and star scale, including values inherited through CSS.
`star_scale` is the DU scale of the star artwork; its default is `1 DU`.

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `label` | `theme-toggle.label` | `Dark mode` |

### Icons

No built-in semantic icons.
