# ActionButton

Rectangular button that runs an async action and shows its outcome.

Status: **reviewed** on web.

## Anatomy

* **Container:** owns the state border and the result background.
* **Label:** optional icon followed by text.
* **Status content:** spinner or result icon, drawn in place of the label.
* **Ghost label:** an invisible copy of the label that keeps the width stable while status content is shown.

## Geometry

| Property | Value |
| :--- | :--- |
| Height | `row_standard` (`10 DU`). |
| Width | Content plus `margin_screen` (`4 DU`) on each side. |
| Corner radius | `0 DU`. |
| Icon | `icon_small` (`4 DU`), `gutter` (`2 DU`) gap to the text. |
| Text | `body` size, weight 500, centered on every line. |

## States

| State | Background | Content | Outline |
| :--- | :--- | :--- | :--- |
| `Idle` | `surface` | `text_primary` | `border` |
| `Pressed` | `pressed` | `text_primary` | `border_strong` |
| `Pending` | `surface` | Label, then the [delayed spinner](../foundations/states.md#delayed-spinner) | Running segment, unless disabled by `show_pending_border` |
| `Success` | `success` at 15% | `success` | Pulsing `success` |
| `Error` | `error` at 15% | `error` | Pulsing `error` |
| `Disabled` | `disabled_surface` | `text_disabled` | `disabled_border` |

A **dangerous** button uses `error` content on `error` at 15% when idle, and `error` at 30% when pressed.

## Behavior

1. A click starts the action and enters `Pending`; clicks are ignored while pending.
2. On completion the button enters the result state for `800 ms`.
   For the first `500 ms` only the result icon is shown.
   For the remaining `300 ms` the label returns in the result color.
3. The result then fades out over `500 ms`.

With `keep_flash_icon_on_success`, a success keeps the check icon for the whole `800 ms`.
A dangerous button ignores clicks during its result phase.

## API

| Property | Type | Default |
| :--- | :--- | :--- |
| `text` | string | required |
| `icon` | icon | none |
| `state` | `ControlState?` | none |
| `enabled` | bool | `true` |
| `show_pending_border` | bool | `true` |
| `keep_flash_icon_on_success` | bool | `false` |
| `dangerous` | bool | `false` |
| `on_click` | async `() -> ActionResult` | none |
