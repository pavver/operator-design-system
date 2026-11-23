# Sheet

Panel that slides in from a screen edge over a scrim.
On the left it is the navigation drawer; on the right a details panel; at the bottom a sheet of actions.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Side sheet width | `size` (default `75 DU`), at most 85% of the viewport. |
| Bottom sheet height | Content height, at most `size` and 85% of the viewport. |
| Background | `surface`; a `0.25 DU` `border` on the edge facing the content. |
| Header | Optional, at least `12 DU`: title in `row_title` style, close button, `divider` below. |
| Handle | Bottom sheet only: `10 DU` × `1 DU` `border_strong` bar, `2 DU` from the top. |
| Scrim | Black at 55%. |
| Corners | `0 DU`. |

## Behavior

* The sheet slides in from its edge over `sheet_slide` (`200 ms`) with the standard curve while the scrim fades in.
* Opening moves focus into the sheet.
* Escape, a scrim click, or the close button closes it.
* The body scrolls on its own.
* While the sheet is open, the page behind it does not scroll, also under the scrim.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `open` | bool signal | required | Visibility. |
| `side` | Left, Right, or Bottom | Left | Edge. |
| `title` | string | none | Header and accessible name. |
| `size` | reactive DU metric | `75` | Width or maximum height. |
| `children` | view | required | Content. |

The customization properties are optional and accept reactive values where indicated.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | none | Root hooks. |
| `handle_width` | reactive DU metric | design-system value | Sets `--op-sheet-handle-width`. |
| `handle_height` | reactive DU metric | design-system value | Sets `--op-sheet-handle-height`. |
| `gap` | reactive DU metric | design-system value | Sets `--op-sheet-gap`. |
| `header_height` | reactive DU metric | design-system value | Sets `--op-sheet-header-height`. |
| `header_padding_start` | reactive DU metric | design-system value | Sets `--op-sheet-header-padding-start`. |
| `title_font_size` | reactive TU metric | design-system value | Sets `--op-sheet-title-font-size`. |
| `title_line_height` | reactive TU metric | design-system value | Sets `--op-sheet-title-line-height`. |
| `font_size` | reactive TU metric | design-system value | Sets `--op-sheet-font-size`. |
| `line_height` | reactive TU metric | design-system value | Sets `--op-sheet-line-height`. |
| `padding` | reactive DU metric | design-system value | Sets `--op-sheet-padding`. |
| `footer_padding` | reactive DU metric | design-system value | Sets `--op-sheet-footer-padding`. |
| `close_size` | reactive DU metric | design-system value | Sets `--op-sheet-close-size`. |
| `close_icon_size` | reactive DU metric | design-system value | Sets `--op-sheet-close-icon-size`. |
| `header` | `Slot` | none | Replaces the title content. |
| `actions` | `Slot` | none | Actions after the body. |
| `footer` | `Slot` | none | Content after the body and actions. |
| `aria_label` | reactive string | `sheet.label` | Accessible name; takes precedence over `title`. |
| `close_label` | reactive string | `sheet.close` | Close button accessible name. |
| `close_icon` | icon | `sheet.close` | Close button icon. |

## Customization

`class` and `style` apply to the scrim root, with variables inherited by the sheet panel.
Metric props write the corresponding variables; an explicit `style` declaration wins.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-sheet-size` | `75 DU` |
| `handle_width` | DU | `--op-sheet-handle-width` | `10 DU` |
| `handle_height` | DU | `--op-sheet-handle-height` | `pad_tight` |
| `gap` | DU | `--op-sheet-gap` | `gutter` |
| `header_height` | DU | `--op-sheet-header-height` | `row_control` |
| `header_padding_start` | DU | `--op-sheet-header-padding-start` | `margin_screen` |
| `title_font_size` | TU | `--op-sheet-title-font-size` | `row_title_size` |
| `title_line_height` | TU | `--op-sheet-title-line-height` | `row_title_line` |
| `font_size` | TU | `--op-sheet-font-size` | `body_size` |
| `line_height` | TU | `--op-sheet-line-height` | `body_line` |
| `padding` | DU | `--op-sheet-padding` | `0 DU` |
| `footer_padding` | DU | `--op-sheet-footer-padding` | `0 DU` |
| `close_size` | DU | `--op-sheet-close-size` | `row_compact` |
| `close_icon_size` | DU | `--op-sheet-close-icon-size` | `icon_small` (`4 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-sheet-scrim` | `rgb(0 0 0 / 55%)` |
| `--op-sheet-background` | `surface` |
| `--op-sheet-border` | `border` |
| `--op-sheet-handle` | `border_strong` |
| `--op-sheet-divider` | `divider` |
| `--op-sheet-text` | `text_primary` |

| Slot | Place |
| :--- | :--- |
| `header` | Replaces the title content. |
| `actions` | Actions after the body. |
| `footer` | Content after the body and actions. |

### Strings

| Key | Default | Prop |
| :--- | :--- | :--- |
| `sheet.close` | `Close` | `close_label` |
| `sheet.label` | `empty; falls back to title` | `aria_label` |

### Icons

| Key | Default | Prop |
| :--- | :--- | :--- |
| `sheet.close` | `close` | `close_icon` |

## Accessibility

`dialog` role with `aria-modal`.
