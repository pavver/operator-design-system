# Tooltip

Short hint attached to a control or a piece of text.
It names an icon-only button or explains an abbreviation; it never holds interactive content.

Status: **reviewed** on web.

## Use

* Icon buttons without a visible label.
* Abbreviations, truncated values, and units that need a word of explanation.
* Do not put actions, links, or essential information in a tooltip; touch users may never see it.

## Geometry

| Property | Value |
| :--- | :--- |
| Type | `caption` style. |
| Background | `surface_alt` with a `0.25 DU` `border_strong` outline and the same drop shadow as [Menu](menu.md), so it stands out from the page. |
| Padding | `1 DU` × `2 DU`. |
| Width | Content width, up to `75 DU`; at most 6 lines, then an ellipsis. |
| Offset | `1.5 DU` from the anchor. |
| Corners | `0 DU`. |

## Behavior

### Opening

* Mouse: opens after `tooltip_show_delay` (`500 ms`) of hover; leaving the anchor closes it at once.
* Keyboard: opens when the anchor receives focus and closes on blur or Escape.
* Touch: a `long_press` (`500 ms`) opens it; it closes `3500 ms` after the finger lifts.
* A long press that opened the tooltip does not also activate the anchor.
* A press anywhere outside the anchor closes it.
* An empty text or a disabled tooltip never opens.
* The native browser tooltip (`title` attribute) is never used, so only one tooltip can appear.
* [IconButton](icon-button.md) shows its `label` in a tooltip on its own; wrapping it in a `Tooltip` replaces that text instead of adding a second tooltip.

### Placement

1. Centered above the anchor.
2. Flipped below when there is no room above.
3. Clamped horizontally to the viewport.

The same placement and look are used by the tooltip of [MarqueeText](marquee-text.md).

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `text` | string | required | Hint text. |
| `delay_ms` | integer | `500` | Hover delay before opening. |
| `enabled` | bool | `true` | Turns the tooltip off without removing it. |
| `children` | view | required | The anchor. |

## Accessibility

* The bubble has the `tooltip` role.
* The anchor keeps its own accessible name; an icon button still needs its `label`.
