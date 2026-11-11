# RadioGroup

Single choice from a short list of options that are all visible at once.

Status: **reviewed** on web.

## Use

* Two to six mutually exclusive options that need a label each.
* For two or three short options in a row, [SegmentedControl](segmented-control.md) is more compact.
* For long lists use [DropdownMenu](dropdown-menu.md) or [Combobox](combobox.md).

## Geometry

The round mark and dot are an allowed exception to the rectangular geometry, like the [Switch](switch.md) capsule.
The circle is the universal sign of a single choice and separates the radio from the square [Checkbox](checkbox.md) at a glance.

| Property | Value |
| :--- | :--- |
| Row height | At least `10 DU`. |
| Mark | `5 DU` circle with the state border, which follows the circle. |
| Dot | `2.5 DU` `accent` circle in the center when selected. |
| Gap | `2.5 DU` between the mark and the label. |
| Horizontal layout | Rows wrap with `4 DU` between options. |

## States

| State | Look |
| :--- | :--- |
| Unselected | `border` outline, no dot. |
| Selected | `accent` at 15%, `accent` outline at 1.5x stroke, `accent` dot; the label keeps its color, as in [Checkbox](checkbox.md). |
| Pressed | `border_strong` outline. |
| Pending | Pending segment on the target border; the [delayed spinner](../foundations/states.md#delayed-spinner) in the target mark. |
| Success / Error | Result pulse on the target mark for `result_hold` (`800 ms`). |
| Disabled | Disabled colors, no interaction. |

## Behavior

* Without an async handler the selection changes at once.
* With an async handler the value commits only after success, like [AsyncSwitch](switch.md):
  1. The target mark shows the pending segment and later the [delayed spinner](../foundations/states.md#delayed-spinner); the old option stays selected.
  2. On success the dot moves to the target, which pulses green.
  3. On error the dot stays, the target pulses red.
* While a request runs the whole group ignores new choices.
* Choosing the selected option does nothing.
* The dot appears by scaling from 0 to full size over `150 ms`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | list of `(id, name)` | required | Options. |
| `selected` | string signal | required | Selected id. |
| `on_change` | async `(id)` | none | Commit request. |
| `enabled` | bool | `true` | Enables the group. |
| `label` | string | none | Accessible name. |
| `horizontal` | bool | `false` | Lays the options out in a row. |

## Accessibility

* `radiogroup` with `radio` options and `aria-checked`.
* One tab stop: the selected option, or the first one when nothing is selected.
* Arrow keys move to the next or previous option and select it.
