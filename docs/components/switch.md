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
| Thumb icon | `3 DU`; spinner `3.5 DU`. |
| Outline | State border with the spinner stroke (`0.375 DU`), following the capsule. |

## Colors

| Part | Off | On |
| :--- | :--- | :--- |
| Track | `switch_unchecked_track` | `switch_checked_track` |
| Thumb | `switch_unchecked_thumb` with a `close` icon | `switch_checked_thumb` with a `check` icon |
| Thumb outline | `border_strong` | `border_strong` |

The off state is deliberately red and the on state green, so the value can be read at a glance.
While pending, the track turns `surface_alt` and the thumb becomes a transparent `pending` spinner in the center.
Disabled uses `disabled_surface` for the track and `text_disabled` for the thumb.

## Behavior

1. A click requests the opposite value and enters `Pending`.
   The thumb moves to the center and shows a spinner; clicks are ignored.
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
| `label` | string | none | Accessible name. |

## Accessibility

The control has the `switch` role, reports its checked state, and reports busy while pending.
