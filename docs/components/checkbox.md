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
| `Pending` | Running `pending` segment, `pending` spinner instead of the check. |
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
