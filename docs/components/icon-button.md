# IconButton

Square button that shows only an icon and supports synchronous and async clicks.

Status: **reviewed** on web.

## Use

* Toolbar, header, and inline row actions.
* Compact actions where a label would not fit.

## Geometry

| Property | Value |
| :--- | :--- |
| Size | Square, `row_compact` (`8 DU`) by default. Common sizes are `7`, `8`, `9`, and `11 DU`. |
| Icon | `icon_small` (`4 DU`) by default, centered. |
| Corner radius | `0 DU`. |
| Outline | State border, `0.25 DU`. |

## States

| State | Background | Icon |
| :--- | :--- | :--- |
| `Idle` | `surface` | `icon` |
| `Pressed` | `pressed` | `icon` |
| `Selected` | `selection` | `icon` |
| `Pending` | `surface` | `pending` spinner at the icon size; running border |
| `Success` | `success` at 15% | `check` in `success` |
| `Error` | `error` at 15% | `close` in `error` |
| `Disabled` | `disabled_surface` | `icon_disabled` |

## Behavior

* A synchronous click runs its callback immediately and has no async states.
* An async click enters `Pending` and ignores further clicks until the result arrives.
* The result icon is shown for `500 ms`, then the original icon returns in the result color for `300 ms`.
* The result then fades out over `500 ms`.
* `loading = true` forces `Pending` from outside, for example while a parent loads data.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `icon` | icon | required | The icon. |
| `label` | string | none | Accessible name and tooltip. Required in practice. |
| `size` | DU | `8` | Button size. |
| `icon_size` | DU | `4` | Icon size. |
| `state` | `ControlState?` | none | External state override. |
| `enabled` | bool | `true` | Interactivity. |
| `loading` | bool | `false` | Forces `Pending`. |
| `on_click` | callback | none | Synchronous action. |
| `on_async_click` | async `() -> ActionResult` | none | Async action; takes precedence over `on_click`. |
