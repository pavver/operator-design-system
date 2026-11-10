# ListItem

The basic row of every list: title, optional subtitle, and leading and trailing slots.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Fills the parent. |
| Padding | `pad_card` (`3 DU`) horizontal, `gutter` (`2 DU`) vertical. |
| Minimum height | `row_avatar` (`14 DU`) with leading content, `row_two_line` (`13 DU`) with a subtitle, else `row_standard` (`10 DU`). |
| Leading gap | `pad_card` between the leading slot and the text. |
| Title | `row_title` style, weight 500, 1 line by default. |
| Subtitle | `body_small` style, `text_secondary`, 2 lines by default. |
| Divider | `0.25 DU` line in `divider` under the row. |
| Corner radius | `0 DU`. |

Rows grow with their content; truncated text ends with an ellipsis.
A single-line row centers its content vertically; a multi-line row aligns it to the top.

## States

| State | Background | Text |
| :--- | :--- | :--- |
| Idle | `background` | `text_primary` / `text_secondary` |
| Pressed | `pressed` with ripple | — |
| Selected | `selection` | — |
| Disabled | `disabled_surface` | `text_disabled` |

## Behavior

* The row is clickable only when it has a click handler and is enabled.
* Trailing controls such as switches handle their own input independently of the row.

## API

| Property | Type | Default |
| :--- | :--- | :--- |
| `title` | string | required |
| `subtitle` | string | none |
| `leading` | view | none |
| `trailing` | view | none |
| `selected` | bool | `false` |
| `enabled` | bool | `true` |
| `title_max_lines` | integer | `1` |
| `subtitle_max_lines` | integer | `2` |
| `on_click` | callback | none |
