# EmptyState

Message in place of content that does not exist yet or was filtered out.
It explains why the area is empty and what to do next.

Status: **reviewed** on web.

## Use

* Empty lists, tables, search results, and panels.
* Say what would normally be here and how to get it; avoid jokes and illustrations.

## Geometry

| Property | Value |
| :--- | :--- |
| Layout | Centered column. |
| Padding | `10 DU` vertical and `4 DU` horizontal; compact: `6 DU` × `3 DU`. |
| Icon | `icon_display` (`12 DU`), compact `icon_xlarge` (`8 DU`), `text_disabled`, `2 DU` above the title. |
| Title | `row_title` style, `text_primary`. |
| Description | Body style, `text_secondary`, at most `90 DU` wide. |
| Actions | `3 DU` below the text, centered, `2 DU` apart. |

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `title` | string | `empty-state.title` | What is missing. |
| `description` | string | none | Why, and what to do. |
| `icon` | icon | `empty-state.icon` | Symbol of the content. |
| `actions` | slot | none | One or two buttons. |
| `icon_slot` | slot | none | Custom icon content. |
| `title_slot` | slot | none | Custom title content. |
| `compact` | bool | `false` | Smaller variant for panels and table bodies. |
| `icon_size` | DU | `12` (compact `8`) | Icon size. |
| `padding_x` | DU | `4` (compact `3`) | Horizontal padding. |
| `padding_y` | DU | `10` (compact `6`) | Vertical padding. |
| `gap` | DU | `1` | Gap between elements. |
| `icon_gap` | DU | `2` | Gap below the icon. |
| `title_size` | TU | `type_row_title_size` | Title font size. |
| `max_width` | DU | `90` | Description maximum width. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `icon_size` | DU | `--op-empty-state-icon-size` | `icon_display` (`12 DU`); compact `icon_xlarge` (`8 DU`) |
| `padding_x` | DU | `--op-empty-state-padding-x` | `margin_screen` (`4 DU`; compact `pad_card`, `3 DU`) |
| `padding_y` | DU | `--op-empty-state-padding-y` | `10 DU` (compact `section_gap`, `6 DU`) |
| `gap` | DU | `--op-empty-state-gap` | `pad_tight` (`1 DU`) |
| `icon_gap` | DU | `--op-empty-state-icon-gap` | `gutter` (`2 DU`) |
| `title_size` | TU | `--op-empty-state-title-size` | `type_row_title_size` |
| `max_width` | DU | `--op-empty-state-max-width` | `90 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-empty-state-title` | `text_primary` |
| `--op-empty-state-text` | `text_secondary` |
| `--op-empty-state-icon` | `text_disabled` |

| Slot | Place |
| :--- | :--- |
| `actions` | Buttons under the text. |
| `icon_slot` | Custom icon slot replacing the built-in icon. |
| `title_slot` | Custom title slot replacing the title text. |

`class` and `style` apply to the root.

### Strings

| Key | Default |
| :--- | :--- |
| `empty-state.title` | `No items` |

### Icons

| Key | Default |
| :--- | :--- |
| `empty-state.icon` | `inbox` |
