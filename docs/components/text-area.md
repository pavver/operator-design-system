# TextArea

Multi-line text input that grows with its content and can be resized by the user.

Status: **reviewed** on web.

## Use

* Notes, messages, descriptions, and other free text longer than one line.
* For one line use [TextField](text-field.md).

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Full width of its container. |
| Padding | `2.5 DU` vertical, `3 DU` horizontal. |
| Height | Grows from `min_rows` to `max_rows` body lines plus padding, then scrolls. |
| Background | `surface`; `disabled_surface` when disabled. |
| Outline | State border, see [states.md](../foundations/states.md). |
| Counter | `caption` style under the end edge, `1 DU` below the field. |
| Resize grip | `2 DU` triangle in the bottom end corner, diagonal hatch lines in `border_strong`; `text_secondary` on hover, `accent` while dragging. The scrollbar stops above it. |

## Behavior

* Focus shows the `Focused` border.
* The height follows the content on every change, including changes made by the caller.
* External `state` shows Pending, Success, or Error on the border, as on [TextField](text-field.md).
* With `max_length` the input stops at the limit and the counter shows `count / max`.
* The counter turns `warning` at 90% of the limit and `error` at the limit.
* By default the height is content-driven.
* Dragging the grip sets the height by hand, from `min_rows` up to `resize_max_rows` lines; the text scrolls when it is longer.
* A hand-set height stays until the user double-clicks the grip, which returns to content-driven height.
* The grip is hidden when the field is disabled.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | string signal | required | Text. |
| `placeholder` | string | empty | Hint shown while empty. |
| `label` | string | none | Accessible name. |
| `min_rows` | integer | `3` | Initial height in lines. |
| `max_rows` | integer | `8` | Height limit before scrolling. |
| `resizable` | bool | `true` | Shows the resize grip. |
| `resize_max_rows` | integer | `20` | Largest height, in lines, reachable with the grip. |
| `max_length` | integer | none | Character limit with counter. |
| `state` | control state | none | External state. |
| `enabled` | bool | `true` | Enables input. |
