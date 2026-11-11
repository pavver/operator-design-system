# Slider

Track-and-thumb slider with optional discrete steps, a value tooltip, and an async commit after release.

Status: **reviewed** on web.

## Use

* Adjusting a numeric parameter such as volume, brightness, zoom, or a threshold.
* Saving the value to a backend once the user lets go.

## Anatomy

```
              ┌───┐
              │ 7 │                 tooltip
              └───┘
▓▓▓▓▓▓▓▓▓▓▓▓▓▓( • )·  ·  ·  ·  ·    track, fill, thumb, ticks
```

* **Track:** a rectangle that owns a state border.
* **Fill:** covers the track from the minimum to the current value.
* **Ticks:** dots at each step position.
* **Thumb:** a circle that carries the status content.
* **Tooltip:** the current value above the thumb.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Fills the parent. |
| Height | `row_compact` (`8 DU`). |
| Rail inset | `3 DU` on each side, so the thumb never leaves the bounds. |
| Track height | `2 DU`, corner radius `0 DU`. |
| Thumb | Circle, `icon_large` (`6 DU`) diameter. Round shape is an allowed exception. |
| Tick | Circle, `pad_tight` (`1 DU`) diameter. |
| Idle dot in thumb | Circle, `pad_small` (`1.5 DU`) diameter. |
| Thumb status icon | `3.5 DU`; spinner `3 DU` with `spinner_stroke`. |
| Tooltip | `10 DU` above the thumb center, `caption` style, bold, `0.25 DU` outline. |

## Colors

| Part | Color |
| :--- | :--- |
| Track | `surface` with a state border. |
| Fill | `accent` at `35%`. |
| Tick, unfilled | `border` at `70%`. |
| Tick, filled | `accent`. |
| Thumb | `surface` with a circular state border. |
| Thumb content | `accent` dot; result colors follow the state. |
| Disabled | Fill and content `text_disabled`; ticks `text_disabled` at `40%` and `60%`. |

## States

| State | Track | Thumb |
| :--- | :--- | :--- |
| `Idle` | `border` outline. | `border` outline, `accent` dot. |
| `Pending` | Running `pending` segment. | Static outline; dot, then the [delayed spinner](../foundations/states.md#delayed-spinner). |
| `Success` | Pulsing `success` outline. | Pulsing outline, `check` icon. |
| `Error` | Pulsing `error` outline. | Pulsing outline, `close` icon. |
| `Disabled` | `disabled_border`. | `disabled_surface`, no interaction. |

The thumb does not draw its own pending segment; the track shows it instead.
Track, thumb, and thumb content fade out of a result state together.

## Behavior

### Input

* Pressing anywhere on the control moves the thumb to the pointer and starts a drag.
* The pointer is captured, so the drag continues outside the control.
* During a drag the thumb follows the pointer exactly, without animation.
* Jumps that are not drags animate over `segment_slide` (`150 ms`) with the standard curve: track clicks, keyboard steps, and rollbacks.
* With a step size the value snaps to `min + n * step`.
* Arrow keys change the value by one step, or by 1% of the range for a continuous slider.
* Pressing the control must not select text or start a native drag.
  A cancelled pointer gesture restores the last committed value.

### Tooltip

* The tooltip appears when a drag starts.
* It stays while the drag continues and hides `800 ms` after release.
* Ranges wider than one unit show integers or one decimal; ranges up to one unit show two decimals.

### Async commit

1. Release, or a keyboard step, commits the new value when it differs from the last committed value.
2. The slider enters `Pending` and ignores input.
3. On **success** the value becomes the new committed value and the slider shows `Success`.
4. On **error** the value animates back to the last committed value and the slider shows `Error`.
5. After `800 ms` the slider returns to `Idle` and the result fades out over `500 ms`.

The slider stays interactive during `Success` and `Error`.
A new drag during a result flash starts a new cycle.

An external value change while idle becomes the new committed value.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | read-write float | required | The current value. |
| `min` | float | `0` | Lower bound. |
| `max` | float | `1` | Upper bound. |
| `step` | float | `0` | Step size; `0` means continuous. |
| `state` | `ControlState?` | none | External state override. |
| `enabled` | bool | `true` | Interactivity. |
| `on_change_finished` | `(f32)` callback | none | Called on every commit attempt. |
| `on_commit` | async `(f32) -> ActionResult` | none | Async commit with rollback on error. |
| `label` | string | none | Accessible name. |

## Accessibility

* The control has the `slider` role with minimum, maximum, and current value.
* It is focusable while interactive.
* The focus ring is drawn around the thumb.
