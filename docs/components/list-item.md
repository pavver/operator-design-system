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
* A focused row shows a `0.5 DU` `accent` ring inside it and the `hover` background while idle, as in [States](../foundations/states.md).
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

### Customization API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `min_height` | optional number (DU) | `row_avatar` with leading content, `row_two_line` with subtitle, otherwise `row_standard` | Overrides `--op-list-item-min-height`. |
| `padding_x` | optional number (DU) | `pad_card` | Overrides `--op-list-item-padding-x`. |
| `padding_y` | optional number (DU) | `gutter` | Overrides `--op-list-item-padding-y`. |
| `leading_padding` | optional number (DU) | `gap_micro` | Overrides `--op-list-item-leading-padding`. |
| `gap` | optional number (DU) | `pad_card` | Overrides `--op-list-item-gap`. |
| `text_padding` | optional number (DU) | `border` | Overrides `--op-list-item-text-padding`. |
| `subtitle_gap` | optional number (DU) | `0.75 DU` | Overrides `--op-list-item-subtitle-gap`. |
| `title_font_size` | optional number (TU) | `type_row_title_size` | Overrides `--op-list-item-title-font-size`. |
| `title_line_height` | optional number (TU) | `type_row_title_line` | Overrides `--op-list-item-title-line-height`. |
| `subtitle_font_size` | optional number (TU) | `type_body_small_size` | Overrides `--op-list-item-subtitle-font-size`. |
| `subtitle_line_height` | optional number (TU) | `type_body_small_line` | Overrides `--op-list-item-subtitle-line-height`. |
| `leading` | slot | none | Before the text; retains the leading slot wrapper. |
| `trailing` | slot | none | After the text; retains alignment for rows with a subtitle. |
| `content` | slot | none | Replaces the title and subtitle inside the text wrapper. |
| `class` | optional string | none | Added to the root classes. |
| `style` | optional string | none | Appended after root metric declarations. |

## Customization

`class` and `style` apply to the root.
Explicit root styles take precedence over metric props; unset metrics retain the original rendering.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `min_height` | DU | `--op-list-item-min-height` | `row_avatar` with leading content, `row_two_line` with subtitle, otherwise `row_standard` |
| `padding_x` | DU | `--op-list-item-padding-x` | `pad_card` |
| `padding_y` | DU | `--op-list-item-padding-y` | `gutter` |
| `leading_padding` | DU | `--op-list-item-leading-padding` | `gap_micro` |
| `gap` | DU | `--op-list-item-gap` | `pad_card` |
| `text_padding` | DU | `--op-list-item-text-padding` | `border` |
| `subtitle_gap` | DU | `--op-list-item-subtitle-gap` | `0.75 DU` |
| `title_font_size` | TU | `--op-list-item-title-font-size` | `type_row_title_size` |
| `title_line_height` | TU | `--op-list-item-title-line-height` | `type_row_title_line` |
| `subtitle_font_size` | TU | `--op-list-item-subtitle-font-size` | `type_body_small_size` |
| `subtitle_line_height` | TU | `--op-list-item-subtitle-line-height` | `type_body_small_line` |

Border widths remain the fixed geometry specified above.

| Color variable | Default |
| :--- | :--- |
| `--op-list-item-background` | `background` |
| `--op-list-item-pressed` | `pressed` |
| `--op-list-item-selected` | `selection` |
| `--op-list-item-disabled-background` | `disabled_surface` |
| `--op-list-item-text` | `text_primary` |
| `--op-list-item-disabled-text` | `text_disabled` |
| `--op-list-item-secondary-text` | `text_secondary` |
| `--op-list-item-divider` | `divider` |
| `--op-list-item-hover` | `hover` |
| `--op-list-item-focus` | `accent` |

| Slot | Place |
| :--- | :--- |
| `leading` | Before the text; retains the leading slot wrapper. |
| `trailing` | After the text; retains alignment for rows with a subtitle. |
| `content` | Replaces the title and subtitle inside the text wrapper. |

### Strings

No built-in strings.
Visible text is supplied by the caller and can be resolved through the application string lookup.

### Icons

No built-in icons.
