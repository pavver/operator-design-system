# SwipeItem

Interactive list row that reveals contextual left and right actions on swipe.

Status: **draft** on web.

## Use

Use SwipeItem inside lists to provide rapid inline actions such as archiving, marking as read, or deleting.
It supports touch and pointer dragging with directional thresholds and automatic spring snap-back.

## Geometry

| Property | Default metric | Variable |
| :--- | :--- | :--- |
| Action slot width | `18 DU` | `--op-swipe-item-action-width` |
| Minimum row height | `12 DU` | `--op-swipe-item-height` |
| Trigger threshold | `15 DU` | `--op-swipe-item-threshold` |

## Behavior

* Swiping to the right reveals the left action slot behind the content.
* Swiping to the left reveals the right action slot.
* Dragging past the threshold and releasing triggers `on_left` or `on_right`.
* When released, the item smoothly slides back to its neutral resting position.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `left` | slot | none | Actions revealed when swiping to the right. |
| `right` | slot | none | Actions revealed when swiping to the left. |
| `on_left` | callback `()` | none | Triggered when swiped past threshold to the right. |
| `on_right` | callback `()` | none | Triggered when swiped past threshold to the left. |
| `threshold` | DU metric | `15 DU` | Horizontal distance required to trigger action. |
| `action_width` | DU metric | `18 DU` | Minimum width of revealed action slots. |
| `height` | DU metric | `12 DU` | Minimum height of the item. |
| `left_label` | string | `swipe-item.left` | Accessible label for left action. |
| `right_label` | string | `swipe-item.right` | Accessible label for right action. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `action_width` | DU | `--op-swipe-item-action-width` | `18 DU` |
| `height` | DU | `--op-swipe-item-height` | `12 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-swipe-item-background` | `surface` |
| `--op-swipe-item-left-bg` | `accent` |
| `--op-swipe-item-right-bg` | `error` |

## Strings

| Key | Default (en) | Description |
| :--- | :--- | :--- |
| `swipe-item.left` | `Left action` | Accessible name for the left action. |
| `swipe-item.right` | `Right action` | Accessible name for the right action. |

## Accessibility

* Standard touch-action `pan-y` ensures native vertical page scrolling remains responsive.
