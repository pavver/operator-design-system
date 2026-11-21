# SidePanel

Titled panel at a screen edge that collapses to a narrow strip, such as a palette, an outline or a property panel next to a canvas.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | `75 DU` by default; collapsed `row_standard` (`10 DU`). |
| Header | `row_standard` tall: the title in `section_title` style, optional actions, and a `row_compact` [IconButton](icon-button.md) that collapses the panel; `0.25 DU` `divider` below. |
| Strip | Only the expand button, at the top. |
| Edge | `0.25 DU` `divider` on the edge facing the content. |
| Body | Fills the rest and scrolls. |

## Behavior

* The collapse button points to the panel's edge (`chevron_left` for a start panel); the expand button points away from it.
* Width changes animate with the `expand` motion.
* The collapsed state is a signal, so the application can remember it.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `title` | string signal | required | Header text and accessible name. |
| `collapsed` | bool signal | required | Collapsed state. |
| `side` | `PanelSide` | `Start` | `Start` or `End`. |
| `width` | DU | `75 DU` | Expanded width. |
| `collapsed_width` | DU | `row_standard` | Collapsed width. |
| `header_height` | DU | `row_standard` | Header height. |
| `header_padding_start` | DU | `pad_card` | Header start padding. |
| `header_padding_end` | DU | `gap_micro` | Header end padding. |
| `header_gap` | DU | `pad_tight` | Header items gap. |
| `strip_padding_top` | DU | `gap_micro` | Strip top padding. |
| `title_font_size` | TU | `type_section_title_size` | Title font size. |
| `header` | slot | none | Custom header replacing the default header. |
| `actions` | slot | none | Header actions before the collapse button. |
| `footer` | slot | none | Slot below the panel body. |
| `collapse_label` | string | `side-panel.collapse` | Accessible name of the collapse button. |
| `expand_label` | string | `side-panel.expand` | Accessible name of the expand button. |
| `collapse_icon` | icon | `side-panel.collapse` | Icon override for the collapse button. |
| `expand_icon` | icon | `side-panel.expand` | Icon override for the expand button. |
| `class` | string | none | Classes for the root element. |
| `style` | string | none | Inline styles for the root element. |
| `children` | view | required | Panel content. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-side-panel-width` | `75 DU` |
| `collapsed_width` | DU | `--op-side-panel-collapsed-width` | `row_standard` (`10 DU`) |
| `header_height` | DU | `--op-side-panel-header-height` | `row_standard` (`10 DU`) |
| `header_padding_start` | DU | `--op-side-panel-header-padding-start` | `pad_card` (`2 DU`) |
| `header_padding_end` | DU | `--op-side-panel-header-padding-end` | `gap_micro` (`0.5 DU`) |
| `header_gap` | DU | `--op-side-panel-header-gap` | `pad_tight` (`1 DU`) |
| `strip_padding_top` | DU | `--op-side-panel-strip-padding-top` | `gap_micro` (`0.5 DU`) |
| `title_font_size` | TU | `--op-side-panel-title-font-size` | `type_section_title_size` |

| Color variable | Default |
| :--- | :--- |
| `--op-side-panel-background` | `surface` |
| `--op-side-panel-divider` | `divider` |
| `--op-side-panel-title-color` | `text_primary` |

| Slot | Place |
| :--- | :--- |
| `header` | Replaces the default header bar. |
| `actions` | Inside the header bar, before the collapse button. |
| `footer` | Below the panel body. |

`class` and `style` apply to the root element.

### Strings

| Key | Default |
| :--- | :--- |
| `side-panel.collapse` | `Collapse` |
| `side-panel.expand` | `Expand` |

### Icons

| Key | Default |
| :--- | :--- |
| `side-panel.collapse` | `chevron_left` for `Start`, `chevron_right` for `End` |
| `side-panel.expand` | `chevron_right` for `Start`, `chevron_left` for `End` |
