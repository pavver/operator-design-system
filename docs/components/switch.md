# AsyncSwitch

Capsule on/off switch whose value changes only after the backend confirms it.

Status: **reviewed** on web.

## Use

* Enabling or disabling a feature on a device or service.
* Settings rows, usually as the trailing element of a [ListItem](list-item.md).

## Geometry

The capsule track and the round thumb are an allowed exception to the rectangular geometry.

| Property | Value |
| :--- | :--- |
| Track | `13 DU` × `row_compact` (`8 DU`), fully rounded. |
| Thumb | `row_dense` (`6 DU`) circle, `pad_tight` (`1 DU`) from the track edge. |
| Thumb travel | `5 DU` between off and on; centered (`2.5 DU`) while pending. |
| Thumb icon | `icon_xsmall` (`3.5 DU`); spinner `3.5 DU`. |
| Outline | State border with the spinner stroke (`0.375 DU`), following the capsule. |

## Colors

| Part | Off | On |
| :--- | :--- | :--- |
| Track | `switch_unchecked_track` | `switch_checked_track` |
| Thumb | `switch_unchecked_thumb` with a `close` icon | `switch_checked_thumb` with a `check` icon |
| Thumb outline | `border_strong` | `border_strong` |

The off state is deliberately red and the on state green, so the value can be read at a glance.
While pending, the track turns `surface_alt` and the thumb moves to the center.
It keeps its fill and icon until the [delayed spinner](../foundations/states.md#delayed-spinner) appears; then it turns transparent and shows the spinner.
Disabled uses `disabled_surface` for the track and `text_disabled` for the thumb.

## Behavior

1. A click requests the opposite value and enters `Pending`.
   The thumb moves to the center and shows the spinner after the delay; clicks are ignored.
2. On **success** the thumb moves to the new side and the track pulses `success`.
3. On **error** the thumb returns to the original side and the track pulses `error`.
4. After `800 ms` the switch returns to `Idle` and the result fades out over `500 ms`.

A change of the external value resets the displayed value.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `checked` | bool | required | The confirmed value. |
| `state` | `ControlState?` | none | External state override. |
| `enabled` | bool | `true` | Interactivity. |
| `on_toggle` | async `(bool) -> ActionResult` | none | Called with the requested value. |
| `label` | string | `switch.label` | Accessible name. |
| `width` | reactive DU | unset | Overrides `--op-switch-width`; see Customization. |
| `height` | reactive DU | unset | Overrides `--op-switch-height`; see Customization. |
| `thumb_size` | reactive DU | unset | Overrides `--op-switch-thumb-size`; see Customization. |
| `thumb_inset` | reactive DU | unset | Overrides `--op-switch-thumb-inset`; see Customization. |
| `icon_size` | reactive DU | unset | Overrides `--op-switch-icon-size`; see Customization. |
| `spinner_size` | reactive DU | unset | Overrides `--op-switch-spinner-size`; see Customization. |
| `spinner_stroke` | reactive DU | unset | Overrides `--op-switch-spinner-stroke`; see Customization. |
| `class` | reactive string | none | Applied to the root element. |
| `style` | reactive string | none | Applied to the root element. |
| `content` | slot | none | Replaces the thumb content. |
| `loading_label` | reactive string | `switch.loading` | String override; see Strings. |
| `checked_icon` | icon | `switch.checked` | Semantic icon override; see Icons. |
| `unchecked_icon` | icon | `switch.unchecked` | Semantic icon override; see Icons. |

## Accessibility

The control has the `switch` role, reports its checked state, and reports busy while pending.

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-switch-width` | `calc(var(--op-du) * 13)` |
| `height` | DU | `--op-switch-height` | `var(--op-dim-row-compact)` |
| `thumb_size` | DU | `--op-switch-thumb-size` | `var(--op-dim-row-dense)` |
| `thumb_inset` | DU | `--op-switch-thumb-inset` | `var(--op-dim-pad-tight)` |
| `icon_size` | DU | `--op-switch-icon-size` | `calc(var(--op-du) * 3)` |
| `spinner_size` | DU | `--op-switch-spinner-size` | `calc(var(--op-du) * 3.5)` |
| `spinner_stroke` | DU | `--op-switch-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| Color variable | Default |
| :--- | :--- |
| `--op-switch-accent` | `var(--op-accent)` |
| `--op-switch-border` | `var(--op-border)` |
| `--op-switch-emphasis-border` | `var(--op-border-strong)` |
| `--op-switch-checked-thumb` | palette token `switch_checked_thumb` |
| `--op-switch-checked-track` | palette token `switch_checked_track` |
| `--op-switch-disabled-border` | `var(--op-disabled-border)` |
| `--op-switch-disabled-background` | `var(--op-disabled-surface)` |
| `--op-switch-error` | `var(--op-error)` |
| `--op-switch-pending` | `var(--op-pending)` |
| `--op-switch-ripple` | `var(--op-ripple)` |
| `--op-switch-success` | `var(--op-success)` |
| `--op-switch-background` | `var(--op-surface)` |
| `--op-switch-background-alt` | `var(--op-surface-alt)` |
| `--op-switch-disabled-text` | `var(--op-text-disabled)` |
| `--op-switch-icon` | `var(--op-toggle-cloud)` |
| `--op-switch-unchecked-thumb` | palette token `switch_unchecked_thumb` |
| `--op-switch-unchecked-track` | palette token `switch_unchecked_track` |
| Slot | Place |
| :--- | :--- |
| `content` | Replaces the thumb content. |

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `loading_label` | `switch.loading` | `Loading` |
| `label` | `switch.label` | `Switch` |

### Icons

| Prop | Key | Default |
| :--- | :--- | :--- |
| `checked_icon` | `switch.checked` | `check` |
| `unchecked_icon` | `switch.unchecked` | `close` |
