# DropdownMenu

Selection field that opens a list of options directly below it.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Fills the parent; the open list has exactly the field width. |
| Field height | `row_standard` (`10 DU`), padding `pad_card` (`3 DU`). |
| Label | Optional, above the field, `caption` style, `pad_tight` gap. |
| List | Directly under the field, maximum height `50 DU`, then scrolls. |
| Option | Minimum height `row_control` (`12 DU`), padding `pad_card`. |
| Corner radius | `0 DU`. |

## Colors

| Part | Color |
| :--- | :--- |
| Field | `surface`, `text_primary`, state border. |
| Open field | `Selected` state: `accent` outline. |
| List | `surface` with an `accent` outline; the library [Scrollbar](scrollbar.md). |
| Selected option | `accent` at 8% background, `accent` bold text. |
| Pressed option | `pressed`. |

## Behavior

* Clicking the field opens or closes the list.
* Clicking outside the list closes it.
* Choosing an option updates the value, calls the callback, and closes the list.
* While `Pending`, the field ignores input and the arrow is replaced by the [delayed spinner](../foundations/states.md#delayed-spinner).
* `Disabled` uses the disabled colors and ignores input.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `options` | list of strings | required | The options. |
| `selected` | read-write string | required | The current option. |
| `state` | `ControlState?` | none | External state override. |
| `enabled` | bool | `true` | Interactivity. |
| `label` | string | none | Label above the field. |
| `on_select` | callback `(String)` | none | Called after a choice. |
