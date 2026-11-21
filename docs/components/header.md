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

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `title` | string signal | required | Title text. |
| `subtitle` | string signal | none | Subtitle text. |
| `on_back` | callback | none | Callback when the back button is pressed. |
| `avatar` | `HeaderAvatar` | none | Avatar shown before the title. |
| `on_title_click` | callback | none | Callback when the title area is clicked. |
| `actions` | slot | none | Actions slot at the trailing end. |
| `height` | DU | `13.5` | Header height. |
| `back_width` | DU | `row_control` | Width of the back button. |
| `gap_back` | DU | `pad_small` | Gap after the back button. |
| `padding_start` | DU | `margin_screen` | Start padding when back button is absent. |
| `gap_avatar` | DU | `pad_content` | Gap after the avatar. |
| `title_size` | TU | `type_app_title_size` | Title font size. |
| `title_line_height` | TU | `type_app_title_line` | Title line height. |
| `title_small_size` | TU | `type_row_title_size` | Title font size when subtitle is present. |
| `title_small_line_height` | TU | `type_row_title_line` | Title line height when subtitle is present. |
| `subtitle_size` | TU | `type_caption_size` | Subtitle font size. |
| `subtitle_line_height` | TU | `type_caption_line` | Subtitle line height. |
| `actions_gap` | DU | `pad_tight` | Gap between action elements. |
| `actions_padding_end` | DU | `gutter` | Right padding for actions. |
| `back_label` | string | `header.back` | Accessible name of the back button. |
| `back_icon` | icon | `header.back` | Back button icon override. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-header-height` | `13.5 DU` |
| `back_width` | DU | `--op-header-back-width` | `row_control` |
| `gap_back` | DU | `--op-header-gap-back` | `pad_small` |
| `padding_start` | DU | `--op-header-padding-start` | `margin_screen` |
| `gap_avatar` | DU | `--op-header-gap-avatar` | `pad_content` |
| `title_size` | TU | `--op-header-title-size` | `type_app_title_size` |
| `title_line_height` | TU | `--op-header-title-line-height` | `type_app_title_line` |
| `title_small_size` | TU | `--op-header-title-small-size` | `type_row_title_size` |
| `title_small_line_height` | TU | `--op-header-title-small-line-height` | `type_row_title_line` |
| `subtitle_size` | TU | `--op-header-subtitle-size` | `type_caption_size` |
| `subtitle_line_height` | TU | `--op-header-subtitle-line-height` | `type_caption_line` |
| `actions_gap` | DU | `--op-header-actions-gap` | `pad_tight` |
| `actions_padding_end` | DU | `--op-header-actions-padding-end` | `gutter` |

| Color variable | Default |
| :--- | :--- |
| `--op-header-background` | `surface` |
| `--op-header-border` | `border` |
| `--op-header-back-color` | `text_primary` |
| `--op-header-title-color` | `text_primary` |
| `--op-header-subtitle-color` | `text_secondary` |

| Slot | Place |
| :--- | :--- |
| `actions` | Right-aligned actions area. |

`class` and `style` apply to the root header element.

### Strings

| Key | Default |
| :--- | :--- |
| `header.back` | `Back` |

### Icons

| Key | Default |
| :--- | :--- |
| `header.back` | `arrow_back` |
