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
* While a request is pending the whole group ignores new choices.
* Result feedback does not block new choices; a newer request supersedes the previous result timer.
* Choosing the selected option does nothing.
* The dot appears by scaling from 0 to full size over `150 ms`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | list of `(id, name)` | required | Options. |
| `selected` | string signal | required | Selected id. |
| `on_change` | async `(id)` | none | Commit request. |
| `enabled` | bool | `true` | Enables the group. |
| `label` | string | `radio-group.label` | Accessible name. |
| `horizontal` | bool | `false` | Lays the options out in a row. |
| `column_gap` | reactive DU | unset | Overrides `--op-radio-group-column-gap`; see Customization. |
| `gap` | reactive DU | unset | Overrides `--op-radio-group-gap`; see Customization. |
| `height` | reactive DU | unset | Overrides `--op-radio-group-height`; see Customization. |
| `padding_x` | reactive DU | unset | Overrides `--op-radio-group-padding-x`; see Customization. |
| `size` | reactive DU | unset | Overrides `--op-radio-group-size`; see Customization. |
| `dot_size` | reactive DU | unset | Overrides `--op-radio-group-dot-size`; see Customization. |
| `spinner_size` | reactive DU | unset | Overrides `--op-radio-group-spinner-size`; see Customization. |
| `font_size` | reactive TU | unset | Overrides `--op-radio-group-font-size`; see Customization. |
| `line_height` | reactive TU | unset | Overrides `--op-radio-group-line-height`; see Customization. |
| `spinner_stroke` | reactive DU | unset | Overrides `--op-radio-group-spinner-stroke`; see Customization. |
| `class` | reactive string | none | Applied to the root element. |
| `style` | reactive string | none | Applied to the root element. |
| `option` | scoped slot | none | Replaces an option label; receives the item. |
| `before_options` | slot | none | Before the options. |
| `after_options` | slot | none | After the options. |
| `loading_label` | reactive string | `radio-group.loading` | String override; see Strings. |

## Accessibility

* `radiogroup` with `radio` options and `aria-checked`.
* One tab stop: the selected option, or the first one when nothing is selected.
* Arrow keys move to the next or previous option and select it.

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `column_gap` | DU | `--op-radio-group-column-gap` | `var(--op-dim-margin-screen)` |
| `gap` | DU | `--op-radio-group-gap` | `var(--op-dim-pad-content)` |
| `height` | DU | `--op-radio-group-height` | `var(--op-dim-row-standard)` |
| `padding_x` | DU | `--op-radio-group-padding-x` | `var(--op-dim-pad-tight)` |
| `size` | DU | `--op-radio-group-size` | `calc(var(--op-du) * 5)` |
| `dot_size` | DU | `--op-radio-group-dot-size` | `var(--op-dim-pad-content)` |
| `spinner_size` | DU | `--op-radio-group-spinner-size` | `calc(var(--op-du) * 3)` |
| `font_size` | TU | `--op-radio-group-font-size` | `var(--op-type-body-size)` |
| `line_height` | TU | `--op-radio-group-line-height` | `var(--op-type-body-line)` |
| `spinner_stroke` | DU | `--op-radio-group-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| Color variable | Default |
| :--- | :--- |
| `--op-radio-group-accent` | `var(--op-accent)` |
| `--op-radio-group-border` | `var(--op-border)` |
| `--op-radio-group-emphasis-border` | `var(--op-border-strong)` |
| `--op-radio-group-disabled-border` | `var(--op-disabled-border)` |
| `--op-radio-group-disabled-background` | `var(--op-disabled-surface)` |
| `--op-radio-group-error` | `var(--op-error)` |
| `--op-radio-group-hover` | `var(--op-hover)` |
| `--op-radio-group-pending` | `var(--op-pending)` |
| `--op-radio-group-ripple` | `var(--op-ripple)` |
| `--op-radio-group-success` | `var(--op-success)` |
| `--op-radio-group-background` | `var(--op-surface)` |
| `--op-radio-group-disabled-text` | `var(--op-text-disabled)` |
| `--op-radio-group-text` | `var(--op-text-primary)` |
| Slot | Place |
| :--- | :--- |
| `option` | Replaces an option label; receives the item. |
| `before_options` | Before the options. |
| `after_options` | After the options. |

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `loading_label` | `radio-group.loading` | `Loading` |
| `label` | `radio-group.label` | `Options` |

### Icons

No built-in semantic icons.
