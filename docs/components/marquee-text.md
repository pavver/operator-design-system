# MarqueeText

Single-line text for bounded containers.
Text that does not fit scrolls slowly instead of being cut, and the full text is available in a tooltip.

Status: **draft** on web.

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
| Tooltip | `caption` style, `surface_alt` background, the shared [Popover](popover.md) `0.25 DU` `accent` frame, padding `1 DU` × `2 DU`. |
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
* The full-text tooltip is a passive [Popover](popover.md) layer in the overlay host.

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

### Customization API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `tooltip_gap` | optional number (DU) | `1.5 DU` | Overrides `--op-marquee-text-tooltip-gap`. |
| `tooltip_max_width` | optional number (DU) | `75 DU` | Overrides `--op-marquee-text-tooltip-max-width`. |
| `tooltip_padding_x` | optional number (DU) | `gutter` | Overrides `--op-marquee-text-tooltip-padding-x`. |
| `tooltip_padding_y` | optional number (DU) | `pad_tight` | Overrides `--op-marquee-text-tooltip-padding-y`. |
| `tooltip_font_size` | optional number (TU) | `type_caption_size` | Overrides `--op-marquee-text-tooltip-font-size`. |
| `tooltip_line_height` | optional number (TU) | `type_caption_line` | Overrides `--op-marquee-text-tooltip-line-height`. |
| `font_size` | optional number (TU) | `inherit` | Overrides `--op-marquee-text-font-size`. |
| `line_height` | optional number (TU) | `inherit` | Overrides `--op-marquee-text-line-height`. |
| `spacing` | optional number (DU) | `6 DU` | Overrides `--op-marquee-text-spacing`. |
| `tooltip_shadow_y` | optional number (DU) | `2 DU` | Overrides `--op-marquee-text-tooltip-shadow-y`. |
| `tooltip_shadow_blur` | optional number (DU) | `6 DU` | Overrides `--op-marquee-text-tooltip-shadow-blur`. |
| `content` | slot | none | Replaces both text copies; only one copy is exposed to assistive technology. |
| `tooltip_slot` | slot | none | Replaces tooltip content; tooltip_text remains the default. |
| `class` | optional string | none | Added to the root classes. |
| `style` | optional string | none | Appended after root metric declarations. |

## Accessibility

* Assistive technology reads the text once; the repeated copy is hidden.
* The tooltip has the `tooltip` role.

## Customization

`class` and `style` apply to the root.
Explicit root styles take precedence over metric props; unset metrics retain the original rendering.
Existing `spacing` retains its DU meaning and uses the CSS fallback when unset.
The component measures the loop gap so CSS overrides also affect the animation distance.
Private overlay forwarding copies public tooltip variables to the portalled Popover; the measured tooltip gap controls placement.
`text` and `tooltip_text` are caller content; use the application string lookup when translating them.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `tooltip_gap` | DU | `--op-marquee-text-tooltip-gap` | `1.5 DU` |
| `tooltip_max_width` | DU | `--op-marquee-text-tooltip-max-width` | `75 DU` |
| `tooltip_padding_x` | DU | `--op-marquee-text-tooltip-padding-x` | `gutter` |
| `tooltip_padding_y` | DU | `--op-marquee-text-tooltip-padding-y` | `pad_tight` |
| `tooltip_font_size` | TU | `--op-marquee-text-tooltip-font-size` | `type_caption_size` |
| `tooltip_line_height` | TU | `--op-marquee-text-tooltip-line-height` | `type_caption_line` |
| `font_size` | TU | `--op-marquee-text-font-size` | `inherit` |
| `line_height` | TU | `--op-marquee-text-line-height` | `inherit` |
| `spacing` | DU | `--op-marquee-text-spacing` | `6 DU` |
| `tooltip_shadow_y` | DU | `--op-marquee-text-tooltip-shadow-y` | `2 DU` |
| `tooltip_shadow_blur` | DU | `--op-marquee-text-tooltip-shadow-blur` | `6 DU` |

Border widths remain the fixed geometry specified above.

| Color variable | Default |
| :--- | :--- |
| `--op-marquee-text-text` | `inherit` |
| `--op-marquee-text-tooltip-background` | `surface_alt` |
| `--op-marquee-text-tooltip-border` | `accent` |
| `--op-marquee-text-tooltip-text` | `text_primary` |
| `--op-marquee-text-tooltip-shadow` | `rgb(0 0 0 / 35%)` |

| Slot | Place |
| :--- | :--- |
| `content` | Replaces both text copies; only one copy is exposed to assistive technology. |
| `tooltip_slot` | Replaces tooltip content; tooltip_text remains the default. |

### Strings

No built-in strings.
Visible text is supplied by the caller and can be resolved through the application string lookup.

### Icons

No built-in icons.
