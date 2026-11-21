# Stepper

`− value +` control for integers that stays interactive while pending, so the user can step quickly.

Status: **reviewed** on web.

## Use

* Small integer settings: a preset number, a count, a level.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | `35 DU` by default; the label next to it takes the remaining space. |
| Height | `row_compact` (`8 DU`). |
| Buttons | `8 DU` squares at both ends, icons `icon_small` (`4 DU`). |
| Value | Fills the middle; `surface_alt` background, bold tabular figures. |
| Corner radius | `0 DU`. |
| Outline | State border around the whole control; each part has its own `border` outline. |

## States

| Part | Look |
| :--- | :--- |
| Button | `surface`, `icon`; pressed `pressed` with `border_strong`. |
| Button at a limit | `disabled_surface`, `icon_disabled`, ignores clicks. |
| Clicked button while pending | Sign, then the [delayed spinner](../foundations/states.md#delayed-spinner); consecutive clicks do not restart the delay. |
| Clicked button on result | `check` in `success` or `close` in `error`. |
| Opposite button while pending | Muted like a disabled button, but still clickable. |
| Whole control | Running segment while pending, pulsing result outline after. |

## Behavior

This component is the documented exception to the rule that Pending blocks input.

* Each click computes the target from the latest requested value, not from the last confirmed value.
* Every click starts a new request and supersedes the previous one.
* On **success** the displayed value becomes the target.
* On **error** the requested value resets to the last confirmed value.
* A result is held for `800 ms` and then fades out over `500 ms`.
* Without an async handler the value changes immediately.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | read-write integer | required | The confirmed value. |
| `min` | integer | none | Lower limit. |
| `max` | integer | none | Upper limit. |
| `step` | integer | `1` | Step size. |
| `state` | `ControlState?` | none | External state override. |
| `enabled` | bool | `true` | Interactivity. |
| `on_async_change` | async `(i32) -> ActionResult` | none | Commits the requested value. |
| `width` | DU | `35` | Control width. |
| `height` | reactive DU | unset | Overrides `--op-stepper-height`; see Customization. |
| `button_size` | reactive DU | unset | Overrides `--op-stepper-button-size`; see Customization. |
| `icon_size` | reactive DU | unset | Overrides `--op-stepper-icon-size`; see Customization. |
| `spinner_size` | reactive DU | unset | Overrides `--op-stepper-spinner-size`; see Customization. |
| `font_size` | reactive TU | unset | Overrides `--op-stepper-font-size`; see Customization. |
| `line_height` | reactive TU | unset | Overrides `--op-stepper-line-height`; see Customization. |
| `spinner_stroke` | reactive DU | unset | Overrides `--op-stepper-spinner-stroke`; see Customization. |
| `class` | reactive string | none | Applied to the root element. |
| `style` | reactive string | none | Applied to the root element. |
| `content` | slot | none | Replaces the displayed value. |
| `loading_label` | reactive string | `stepper.loading` | String override; see Strings. |
| `success_label` | reactive string | `stepper.success` | String override; see Strings. |
| `error_label` | reactive string | `stepper.error` | String override; see Strings. |
| `increase_label` | reactive string | `stepper.increase` | String override; see Strings. |
| `decrease_label` | reactive string | `stepper.decrease` | String override; see Strings. |
| `success_icon` | icon | `stepper.success` | Semantic icon override; see Icons. |
| `error_icon` | icon | `stepper.error` | Semantic icon override; see Icons. |
| `increase_icon` | icon | `stepper.increase` | Semantic icon override; see Icons. |
| `decrease_icon` | icon | `stepper.decrease` | Semantic icon override; see Icons. |

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-stepper-width` | `calc(var(--op-du) * 35)` |
| `height` | DU | `--op-stepper-height` | `var(--op-dim-row-compact)` |
| `button_size` | DU | `--op-stepper-button-size` | `var(--op-dim-row-compact)` |
| `icon_size` | DU | `--op-stepper-icon-size` | `var(--op-dim-icon-small)` |
| `spinner_size` | DU | `--op-stepper-spinner-size` | `calc(var(--op-du) * 3)` |
| `font_size` | TU | `--op-stepper-font-size` | `var(--op-type-body-size)` |
| `line_height` | TU | `--op-stepper-line-height` | `var(--op-type-body-line)` |
| `spinner_stroke` | DU | `--op-stepper-spinner-stroke` | `var(--op-dim-spinner-stroke)` |

The existing numeric size props keep their names and DU meaning and now also accept reactive values.

| Color variable | Default |
| :--- | :--- |
| `--op-stepper-accent` | `var(--op-accent)` |
| `--op-stepper-border` | `var(--op-border)` |
| `--op-stepper-emphasis-border` | `var(--op-border-strong)` |
| `--op-stepper-disabled-border` | `var(--op-disabled-border)` |
| `--op-stepper-disabled-background` | `var(--op-disabled-surface)` |
| `--op-stepper-error` | `var(--op-error)` |
| `--op-stepper-hover` | `var(--op-hover)` |
| `--op-stepper-icon` | `var(--op-icon)` |
| `--op-stepper-disabled-icon` | `var(--op-icon-disabled)` |
| `--op-stepper-pending` | `var(--op-pending)` |
| `--op-stepper-pressed` | `var(--op-pressed)` |
| `--op-stepper-ripple` | `var(--op-ripple)` |
| `--op-stepper-success` | `var(--op-success)` |
| `--op-stepper-background` | `var(--op-surface)` |
| `--op-stepper-background-alt` | `var(--op-surface-alt)` |
| `--op-stepper-disabled-text` | `var(--op-text-disabled)` |
| `--op-stepper-text` | `var(--op-text-primary)` |
| Slot | Place |
| :--- | :--- |
| `content` | Replaces the displayed value. |

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `loading_label` | `stepper.loading` | `Loading` |
| `success_label` | `stepper.success` | `Success` |
| `error_label` | `stepper.error` | `Error` |
| `increase_label` | `stepper.increase` | `Increase` |
| `decrease_label` | `stepper.decrease` | `Decrease` |

### Icons

| Prop | Key | Default |
| :--- | :--- | :--- |
| `success_icon` | `stepper.success` | `check` |
| `error_icon` | `stepper.error` | `close` |
| `increase_icon` | `stepper.increase` | `add` |
| `decrease_icon` | `stepper.decrease` | `remove` |
