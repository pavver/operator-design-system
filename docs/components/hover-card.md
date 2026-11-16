# HoverCard

Floating card with details of an element: what a node does and how to use it, a preview, a short help with links.
Unlike a [Tooltip](tooltip.md), it holds any content, including links and buttons, and stays open while the pointer is over it.

Status: **draft** on web.

## Use

* Help behind an `info` icon, for example the description of a node type.
* Details that do not fit next to an element and are not worth a dialog.
* Plain one-line hints stay in [Tooltip](tooltip.md).

## Geometry

| Property | Value |
| :--- | :--- |
| Width | `80 DU` by default, never wider than the viewport minus `margin_screen` on both sides. |
| Height | Up to `100 DU`; longer content scrolls inside the card. |
| Padding | `pad_card`; content blocks `pad_tight` apart. |
| Distance | `1.5 DU` from the anchor. |
| Surface | `surface_alt`, `0.25 DU` `border_strong` inside the bounds and the raised-layer shadow, like [Menu](menu.md). |

## Behavior

* A mouse opens the card after hovering the anchor for the tooltip delay.
* The card stays open while the pointer is over the anchor or the card; it closes `200 ms` after the pointer leaves both.
* A press on the anchor (touch, pen or a click) toggles the card, so it works without hover.
* Keyboard focus on the anchor opens it (`open_on_focus`).
* Escape and a press outside the card and the anchor close it.
* The card is a [Popover](popover.md) layer below the anchor (flipped above without room), so clipping ancestors never cut it.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `card` | view | required | Card content. |
| `children` | view | required | The anchor, usually an [IconButton](icon-button.md) with `info`. |
| `delay_ms` | integer | tooltip delay | Hover time before opening. |
| `width` | DU | `80` | Card width. |
| `placement` | `Placement` | `BelowStart` | Position next to the anchor. |
| `label` | string | none | Accessible name of the card. |
| `open_on_focus` | bool | `true` | Open on keyboard focus. |

## Accessibility

* The card has the `dialog` role and the given label; the anchor reports `aria-expanded`.
