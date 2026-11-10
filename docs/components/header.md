# Header

Top bar of a screen with back navigation, an optional avatar, title, subtitle, and actions.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Fills the screen. |
| Height | `13.5 DU` by default. |
| Bottom line | `0.25 DU` in `border`. |
| Back button | `row_control` (`12 DU`) wide, full height, `arrow_back` icon `icon_medium` (`4.5 DU`). |
| Start padding | `margin_screen` (`4 DU`) when there is no back button; `pad_small` after it. |
| Avatar | `9 DU`, followed by a `pad_content` (`2.5 DU`) gap. |
| Title | `app_title` style, bold; `row_title` size when a subtitle is present. |
| Subtitle | `caption` style, `text_secondary`, one line with ellipsis. |
| Actions | Right-aligned slot. |
| Background | `surface`. |

## Behavior

* The back button calls its callback and shows a ripple.
* When a title callback exists, the avatar and title area is clickable, for example to open a profile.
* Title and subtitle truncate instead of pushing the actions out.

## API

| Property | Type | Default |
| :--- | :--- | :--- |
| `title` | string | required |
| `subtitle` | string | none |
| `on_back` | callback | none |
| `avatar` | `HeaderAvatar { id, name, group, status }` | none |
| `on_title_click` | callback | none |
| `actions` | view | none |
| `height` | DU | `13.5` |
