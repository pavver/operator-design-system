# MarqueeText

Single-line text for bounded containers.
Text that does not fit scrolls slowly instead of being cut, and the full text is available in a tooltip.

Status: **draft**, new on web.

## Use

* Names, titles, and device labels in rows, headers, and toolbars whose width is limited.
* Places where an ellipsis would hide the part of the text that tells items apart.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Takes the parent's width; never wider than the parent. |
| Height | One line of the applied type style. |
| Padding | `0 DU`; the parent provides insets. |
| Loop gap | `6 DU` between the end of the text and its repeated start. |
| Tooltip | `caption` style, `surface` background, `0.25 DU` `border` outline, padding `1 DU` × `2 DU`. |
| Tooltip width | Up to `75 DU`; at most 6 lines, then an ellipsis. |
| Tooltip offset | `1.5 DU` from the text. |

## Behavior

### Scrolling

1. Text that fits never moves.
2. Overflowing text stays still for `2000 ms` so the start can be read.
3. It then scrolls right to left at `7.5 DU` per second, linear, until the repeated copy reaches the start position.
4. It pauses for `1200 ms` and repeats.
5. A loop limit stops the text at the start position after the last loop.

Changing the text or the container width restarts the cycle from the stationary phase.
With the reduced-motion preference, the text does not scroll and ends with an ellipsis.

### Tooltip

* With a mouse, the tooltip opens on hover and closes when the pointer leaves.
* On touch, a `500 ms` long press opens it; it closes `3500 ms` after the finger lifts.
* A tap on the text while the tooltip is open, or a press anywhere else, closes it.
* It appears above the text, centered, and flips below when there is no room above.
* It is clamped horizontally to the viewport.
* It opens for fitting text too, so the behavior is predictable.

### Click

A click calls `on_click`, unless it ends a long press or closes the tooltip.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `text` | string | required | The text. |
| `class` | string | none | Typography class, for example `op-t-row-title`. |
| `initial_delay_ms` | integer | `2000` | Stationary time before scrolling. |
| `delay_ms` | integer | `1200` | Pause between loops. |
| `velocity` | DU per second | `7.5` | Scroll speed. |
| `spacing` | DU | `6` | Loop gap. |
| `iterations` | integer | unlimited | Number of loops. |
| `tooltip` | bool | `true` | Enables the tooltip. |
| `tooltip_text` | string | `text` | Tooltip content. |
| `on_click` | callback | none | Click handler. |

## Accessibility

* Assistive technology reads the text once; the repeated copy is hidden.
* The tooltip has the `tooltip` role.
