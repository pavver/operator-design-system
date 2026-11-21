# Rating

Discrete rating with pointer preview and roving keyboard focus.
Enables quick visual selection of a discrete numeric score.
Sitting directly on the page surface as an inline control without its own background or frame.

Status: **draft** on web.

## Use

* Gathering user feedback, reviews, or priority scores.
* Displaying qualitative ratings on items.

## Geometry

| Property | Value |
| :--- | :--- |
| Item size | `row_compact` (`8 DU`). |
| Item gap | `gap_micro` (`0.5 DU`). |
| Corner radius | `0 DU`. |

## Behavior

* Sits directly on the page surface without an enclosing frame or background.
* Hovering over items shows a real-time rating preview.
* Clicking an item selects the score and triggers the commit handler.
* Clicking the currently selected score resets the value to zero unless `no_reset` is active.
* Keyboard focus produces a focus ring around the focused star only.
* Arrow keys increment or decrement the rating with roving focus.
* When pending during an async save, a small spinner runs inside the chosen star.
* Success and failure feedback plays the Operator result flash on the chosen star.
* A failed async save rolls back to the previous value with the same animation as the forward change.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | u32 signal | required | Current rating score. |
| `max` | u32 | `5` | Maximum number of items. |
| `readonly` | bool | `false` | Prevents value modification. |
| `enabled` | bool | `true` | Enables input. |
| `state` | `ControlState` | none | Explicit control state override. |
| `on_change` | callback `(u32)` | none | Emits on value change. |
| `on_commit` | `AsyncAction<u32>` | none | Async commit handler with rollback on error. |
| `no_reset` | bool | `false` | Disallows resetting to zero when clicking current value. |
| `label` | string | `rating.label` | Accessible group label. |
| `item_label` | string | `rating.item` | Template string for each item label. |
| `icon` | icon | `rating.icon` | Icon for unselected items. |
| `icon_selected` | icon | `rating.icon-selected` | Icon for selected items. |
| `size` | DU metric | `row_compact` | Item button width and height. |
| `gap` | DU metric | `gap_micro` | Spacing between rating items. |
| `leading` | `Slot` | none | Slot before the rating items. |
| `trailing` | `Slot` | none | Slot after the rating items. |
| `content` | `Slot` | none | Custom content inside rating group. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-rating-size` | `row_compact` (`8 DU`) |
| `gap` | DU | `--op-rating-gap` | `gap_micro` (`0.5 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-rating-background` | `transparent` |
| `--op-rating-text` | `text_secondary` |
| `--op-rating-accent` | `accent` |
| `--op-rating-hover` | `hover` |
| `--op-rating-focus` | `accent` |
| `--op-rating-pending` | `pending` |
| `--op-rating-success` | `success` |
| `--op-rating-error` | `error` |
| `--op-rating-disabled-text` | `text_disabled` |

| Slot | Place |
| :--- | :--- |
| `leading` | Before the rating items. |
| `trailing` | After the rating items. |
| `content` | Preceding the items within the group. |

`class` and `style` apply to the root radiogroup element.

### Strings

| Key | Default |
| :--- | :--- |
| `rating.label` | `Rating` |
| `rating.item` | `Rate {value} of {max}` |

### Icons

| Key | Default |
| :--- | :--- |
| `rating.icon` | `check_circle` |
| `rating.icon-selected` | `check_circle` |

## Accessibility

Rating acts as an accessible `radiogroup` with radio items.
Arrow keys cycle through items and roving `tabindex` ensures predictable keyboard navigation.
Keyboard focus indicator is displayed around the focused star only.
