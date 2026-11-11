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
