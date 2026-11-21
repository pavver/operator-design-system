# AsyncCheckbox

Square checkbox with an optional label; the value changes only after the backend confirms it.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Box | `5 DU` square, corner radius `0 DU`. |
| Check icon | `3.5 DU`; spinner `3 DU`. |
| Box-to-label gap | `gutter` (`2 DU`). |
| Hit area padding | `pad_small` vertical, `pad_tight` horizontal. |
| Label | `body` size, weight 500, `text_primary`. |

## States

| State | Box |
| :--- | :--- |
| Unchecked | `surface`, `border` outline. |
| Checked | `accent` at 15%, `accent` outline at 1.5x stroke, `accent` check. |
| `Pending` | Running `pending` segment, check stays, then the [delayed spinner](../foundations/states.md#delayed-spinner) replaces it. |
| `Success` | Pulsing `success` outline; the check, if shown, is `success`. |
| `Error` | Pulsing `error` outline; the restored check, if shown, is `error`. |
| `Disabled` | `disabled_surface`, `disabled_border`, `icon_disabled` check, `text_disabled` label. |

## Behavior

1. A click requests the opposite value and enters `Pending`; clicks are ignored while pending.
2. On **success** the new value is shown.
3. On **error** the previous value stays.
4. After `800 ms` the result fades out over `500 ms`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `checked` | bool | required | The confirmed value. |
| `text` | string | none | Label next to the box; the whole row is clickable. |
| `state` | `ControlState?` | none | External state override. |
| `enabled` | bool | `true` | Interactivity. |
| `on_change` | async `(bool) -> ActionResult` | none | Called with the requested value. |
| `size` | reactive DU | unset | Overrides `--op-checkbox-size`; see Customization. |
| `gap` | reactive DU | unset | Overrides `--op-checkbox-gap`; see Customization. |
| `padding_x` | reactive DU | unset | Overrides `--op-checkbox-padding-x`; see Customization. |
| `padding_y` | reactive DU | unset | Overrides `--op-checkbox-padding-y`; see Customization. |
| `icon_size` | reactive DU | unset | Overrides `--op-checkbox-icon-size`; see Customization. |
| `spinner_size` | reactive DU | unset | Overrides `--op-checkbox-spinner-size`; see Customization. |
| `font_size` | reactive TU | unset | Overrides `--op-checkbox-font-size`; see Customization. |
| `line_height` | reactive TU | unset | Overrides `--op-checkbox-line-height`; see Customization. |
| `spinner_stroke` | reactive DU | unset | Overrides `--op-checkbox-spinner-stroke`; see Customization. |
| `class` | reactive string | none | Applied to the root element. |
| `style` | reactive string | none | Applied to the root element. |
| `label_slot` | slot | none | Replaces the optional text label. |
| `loading_label` | reactive string | `checkbox.loading` | String override; see Strings. |
| `label` | reactive string | `checkbox.label` | String override; see Strings. |
| `checked_icon` | icon | `checkbox.checked` | Semantic icon override; see Icons. |

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-checkbox-size` | `calc(var(--op-du) * 5)` |
| `gap` | DU | `--op-checkbox-gap` | `var(--op-dim-gutter)` |
| `padding_x` | DU | `--op-checkbox-padding-x` | `var(--op-dim-pad-tight)` |
| `padding_y` | DU | `--op-checkbox-padding-y` | `var(--op-dim-pad-small)` |
| `icon_size` | DU | `--op-checkbox-icon-size` | `calc(var(--op-du) * 3.5)` |
| `spinner_size` | DU | `--op-checkbox-spinner-size` | `calc(var(--op-du) * 3)` |
| `font_size` | TU | `--op-checkbox-font-size` | `var(--op-type-body-size)` |
| `line_height` | TU | `--op-checkbox-line-height` | `var(--op-type-body-line)` |
| `spinner_stroke` | DU | `--op-checkbox-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| Color variable | Default |
| :--- | :--- |
| `--op-checkbox-accent` | `var(--op-accent)` |
| `--op-checkbox-border` | `var(--op-border)` |
| `--op-checkbox-emphasis-border` | `var(--op-border-strong)` |
| `--op-checkbox-disabled-border` | `var(--op-disabled-border)` |
| `--op-checkbox-disabled-background` | `var(--op-disabled-surface)` |
| `--op-checkbox-error` | `var(--op-error)` |
| `--op-checkbox-hover` | `var(--op-hover)` |
| `--op-checkbox-disabled-icon` | `var(--op-icon-disabled)` |
| `--op-checkbox-pending` | `var(--op-pending)` |
| `--op-checkbox-ripple` | `var(--op-ripple)` |
| `--op-checkbox-success` | `var(--op-success)` |
| `--op-checkbox-background` | `var(--op-surface)` |
| `--op-checkbox-disabled-text` | `var(--op-text-disabled)` |
| `--op-checkbox-text` | `var(--op-text-primary)` |
| Slot | Place |
| :--- | :--- |
| `label_slot` | Replaces the optional text label. |

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `loading_label` | `checkbox.loading` | `Loading` |
| `label` | `checkbox.label` | `Checkbox` |

### Icons

| Prop | Key | Default |
| :--- | :--- | :--- |
| `checked_icon` | `checkbox.checked` | `check` |
