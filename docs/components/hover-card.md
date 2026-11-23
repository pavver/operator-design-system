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
| Surface | `surface_alt`, the shared [Popover](popover.md) `0.25 DU` `accent` frame inside the bounds and the raised-layer shadow, like [Menu](menu.md). |

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
| `width` | reactive DU metric | `80` | Card width. |
| `placement` | `Placement` | `BelowStart` | Position next to the anchor. |
| `label` | string | none | Accessible name of the card. |
| `open_on_focus` | bool | `true` | Open on keyboard focus. |

The customization properties are optional and accept reactive values where indicated.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | none | Root hooks. |
| `shadow_y` | reactive DU metric | design-system value | Sets `--op-hover-card-shadow-y`. |
| `shadow_blur` | reactive DU metric | design-system value | Sets `--op-hover-card-shadow-blur`. |
| `viewport_margin` | reactive DU metric | design-system value | Sets `--op-hover-card-viewport-margin`. |
| `padding` | reactive DU metric | design-system value | Sets `--op-hover-card-padding`. |
| `gap` | reactive DU metric | design-system value | Sets `--op-hover-card-gap`. |
| `max_height` | reactive DU metric | design-system value | Sets `--op-hover-card-max-height`. |
| `offset` | reactive DU metric | design-system value | Sets `--op-hover-card-offset`. |
| `font_size` | reactive TU metric | design-system value | Sets `--op-hover-card-font-size`. |
| `line_height` | reactive TU metric | design-system value | Sets `--op-hover-card-line-height`. |
| `max_width` | reactive DU metric | design-system value | Sets `--op-hover-card-max-width`. |
| `header` | `Slot` | none | Content before the card content. |
| `actions` | `Slot` | none | Actions after the card content. |
| `footer` | `Slot` | none | Content after the body and actions. |
| `aria_label` | reactive string | `hover-card.label` | Accessible name; takes precedence over `label`. |

## Customization

`class` and `style` apply to the floating card layer.
Metric props write the corresponding variables; an explicit `style` declaration wins.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `shadow_y` | DU | `--op-hover-card-shadow-y` | `2 DU` |
| `shadow_blur` | DU | `--op-hover-card-shadow-blur` | `6 DU` |
| `viewport_margin` | DU | `--op-hover-card-viewport-margin` | `margin_screen` |
| `padding` | DU | `--op-hover-card-padding` | `pad_card` |
| `gap` | DU | `--op-hover-card-gap` | `pad_tight` |
| `width` | DU | `--op-hover-card-width` | `80 DU` |
| `max_height` | DU | `--op-hover-card-max-height` | `100 DU` |
| `offset` | DU | `--op-hover-card-offset` | `1.5 DU` |
| `font_size` | TU | `--op-hover-card-font-size` | `body_size` |
| `line_height` | TU | `--op-hover-card-line-height` | `body_line` |
| `max_width` | DU | `--op-hover-card-max-width` | `calc(100vw - var(--op-hover-card-viewport-margin, margin_screen) * 2)` |

| Color variable | Default |
| :--- | :--- |
| `--op-hover-card-background` | `surface_alt` |
| `--op-hover-card-border` | `accent` |
| `--op-hover-card-shadow` | `rgb(0 0 0 / 35%)` |
| `--op-hover-card-text` | `text_primary` |

| Slot | Place |
| :--- | :--- |
| `header` | Content before the card content. |
| `actions` | Actions after the card content. |
| `footer` | Content after the body and actions. |

`header` precedes `card`; `actions` and `footer` follow it inside the same padded body.

### Strings

| Key | Default | Prop |
| :--- | :--- | :--- |
| `hover-card.label` | `empty` | `aria_label / label` |

### Icons

This component has no built-in icons.

## Accessibility

* The card has the `dialog` role and the given label; the anchor reports `aria-expanded`.
