# LinearProgress

Horizontal bar for the progress of a long operation, such as an upload or a scan.

Status: **reviewed** on web.

## Use

* Determinate: the share of the work done is known.
* Indeterminate: work is running but its length is unknown.
* Inside a control, prefer the state border; the bar is for whole views, cards, and table headers.

## Geometry

| Property | Value |
| :--- | :--- |
| Thickness | `1 DU` by default; any DU value, for example `0.5 DU` in a table header or `6 DU` for a prominent bar. |
| Width | Full width of its container. |
| Track | Bar color at 20% over `surface_alt`. |
| Value label outside | Optional, `caption` style, `text_secondary`, at least `9 DU` wide, `2 DU` after the bar. |
| Value label inside | From a thickness of `5 DU` the label is centered inside the bar, `caption` style, bold. |
| Inside label colors | `text_primary` over the track and `on_accent` over the fill; the label is split exactly at the fill edge. |
| Corners | `0 DU`. |

## Value Format

* Percent: `54%`, the default.
* Count: `543 / 1000`, the value and the maximum; whole numbers have no decimals, others have two.
* Use a count when the unit matters to the operator, such as files, cameras, or megabytes.

## Colors

* Running: `pending`, because progress is work in progress.
* Finished: `success`.
* Failed: `error`; the bar stops where the failure happened.

## Motion

* Determinate width changes over `state_color` (`150 ms`).
* Indeterminate: a segment of 35% of the track sweeps from left to right once per `pending_lap` (`2000 ms`) with the smooth curve.
* With reduced motion the sweep runs at the reduced speed defined in [motion.md](../foundations/motion.md).

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | `0.0..max` | none | Progress; omitted means indeterminate. |
| `max` | number | `1.0` | Value of the finished operation. |
| `state` | control state | none | `Success` or `Error` recolors the bar. |
| `show_value` | bool | `false` | Shows the value, inside the bar from `5 DU` thickness. |
| `format` | `Percent` or `Count` | `Percent` | How the value is written. |
| `thickness` | DU | `1.0` | Bar thickness. |
| `label` | string | `"Progress"` | Accessible name. |

## Accessibility

The track has the `progressbar` role with `aria-valuenow` and `aria-valuemax` in the units of `value` when determinate.
`aria-valuetext` carries the label as shown, so the count format is announced as written.
The inside label copies are hidden from assistive technology.
