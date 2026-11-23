# DataTable

Table of text cells with sortable columns, a sticky header, and row selection.

Status: **draft** on web.

## Use

* Lists of records compared by several attributes: cameras, events, users.
* For records with one or two attributes, [ListItem](list-item.md) reads better on narrow screens.

## Geometry

| Property | Value |
| :--- | :--- |
| Container | `0.25 DU` `border` outline; scrolls in both directions. |
| Header row | `8 DU`, `surface_alt`, `caption` style uppercase in `text_secondary`, `0.25 DU` `border` below; sticky. |
| Body row | `10 DU`, dense `8 DU`; `body_small` style; `0.25 DU` `divider` below. |
| Cell padding | `3 DU` horizontal. |
| Column width | Fixed in DU, or a share of the remaining width. |
| Numeric columns | End-aligned with tabular figures. |
| Loading bar | `0.5 DU` [LinearProgress](linear-progress.md) under the header. |

## Sorting

* A sortable header is a button; it shows a `3.5 DU` arrow at 50% on hover and in `accent` when the column is sorted.
* Each click cycles: ascending, descending, original order.
* Numeric columns compare numbers; cells without a number sort last.
* Text compares case-insensitively.

## Selection

* With selection or a row handler, rows are focusable and show the `pressed` background on hover.
* A focused row shows a `0.5 DU` `accent` ring inside it and, unless selected, the `hover` background, as in [States](../foundations/states.md).
* The selected row has the `selection` background and a `0.5 DU` `accent` bar on its first cell.
* Enter or Space on a focused row selects it.

## Empty and loading

* An empty table shows "No data", or the given content, typically a compact [EmptyState](empty-state.md).
* `loading` shows the indeterminate bar; the rows stay visible.

## API

`Column` fields: `title`, `width` (DU), `align`, `sortable`, `numeric`.
`TableRow` fields: `id`, `cells`.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `columns` | list of columns | required | Columns. |
| `rows` | list of rows | required | Data. |
| `selected` | optional id signal | none | Enables single selection. |
| `on_row_click` | callback `(id)` | none | Row activation. |
| `loading` | bool | `false` | Shows the bar. |
| `dense` | bool | `false` | `8 DU` rows. |
| `max_height` | DU | none | Scrolls under the sticky header. |
| `empty` | view | "No data" | Empty content. |
| `label` | string | none | Accessible name. |

### Customization API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `row_height` | optional number (DU) | `row_standard` | Overrides `--op-data-table-row-height`. |
| `dense_row_height` | optional number (DU) | `row_compact` | Overrides `--op-data-table-dense-row-height`. |
| `header_height` | optional number (DU) | `row_compact` | Overrides `--op-data-table-header-height`. |
| `padding_x` | optional number (DU) | `pad_card` | Overrides `--op-data-table-padding-x`. |
| `sort_gap` | optional number (DU) | `pad_tight` | Overrides `--op-data-table-sort-gap`. |
| `sort_padding` | optional number (DU) | `pad_tight` | Overrides `--op-data-table-sort-padding`. |
| `icon_size` | optional number (DU) | `icon_xsmall` (`3.5 DU`) | Overrides `--op-data-table-icon-size`. |
| `loading_thickness` | optional number (DU) | `0.5 DU` | Overrides `--op-data-table-loading-thickness`. |
| `empty_padding` | optional number (DU) | `section_gap` | Overrides `--op-data-table-empty-padding`. |
| `font_size` | optional number (TU) | `type_body_small_size` | Overrides `--op-data-table-font-size`. |
| `line_height` | optional number (TU) | `type_body_small_line` | Overrides `--op-data-table-line-height`. |
| `header_font_size` | optional number (TU) | `type_caption_size` | Overrides `--op-data-table-header-font-size`. |
| `header_letter_spacing` | optional number (TU) | `0.04 × type_caption_size` | Overrides `--op-data-table-header-letter-spacing`. |
| `max_height` | optional number (DU) | `none` | Overrides `--op-data-table-max-height`. |
| `row` | scoped slot | none | Scoped `TableRow`; replaces the cells inside the native body row; return `td` elements. |
| `cell` | scoped slot | none | Scoped `TableCell { row, column_index, column, value }`; replaces content inside a native body cell. |
| `header_cell` | scoped slot | none | Scoped `TableHeaderCell { column_index, column }`; replaces header content inside the existing sort button when sortable. |
| `empty` | slot | none | Empty table cell; defaults to the `data-table.empty` string. |
| `header` | slot | none | Before the scrolling table. |
| `footer` | slot | none | After the scrolling table. |
| `before_options` | slot | none | Full-width native table row before body rows. |
| `after_options` | slot | none | Full-width native table row after body rows. |
| `label` | optional string | `data-table.label` | Text override. |
| `empty_text` | optional string | `data-table.empty` | Text override. |
| `loading_label` | optional string | `data-table.loading` | Text override. |
| `sort_ascending_label` | optional string | `data-table.sort-ascending` | Text override. |
| `sort_descending_label` | optional string | `data-table.sort-descending` | Text override. |
| `sort_clear_label` | optional string | `data-table.sort-clear` | Text override. |
| `sort_ascending_icon` | optional icon | `data-table.sort-ascending` | Icon override. |
| `sort_descending_icon` | optional icon | `data-table.sort-descending` | Icon override. |
| `class` | optional string | none | Added to the root classes. |
| `style` | optional string | none | Appended after root metric declarations. |

## Accessibility

Native table semantics; sortable headers carry `aria-sort`, selected rows carry `aria-selected`.

## Customization

`class` and `style` apply to the root.
Explicit root styles take precedence over metric props; unset metrics retain the original rendering.
The row slot takes precedence over the cell slot.
Column titles and cell values remain caller content.
Sort button names combine the column title with the resolved action string; `aria-sort` retains the standard ARIA enum values.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `row_height` | DU | `--op-data-table-row-height` | `row_standard` |
| `dense_row_height` | DU | `--op-data-table-dense-row-height` | `row_compact` |
| `header_height` | DU | `--op-data-table-header-height` | `row_compact` |
| `padding_x` | DU | `--op-data-table-padding-x` | `pad_card` |
| `sort_gap` | DU | `--op-data-table-sort-gap` | `pad_tight` |
| `sort_padding` | DU | `--op-data-table-sort-padding` | `pad_tight` |
| `icon_size` | DU | `--op-data-table-icon-size` | `icon_xsmall` (`3.5 DU`) |
| `loading_thickness` | DU | `--op-data-table-loading-thickness` | `0.5 DU` |
| `empty_padding` | DU | `--op-data-table-empty-padding` | `section_gap` |
| `font_size` | TU | `--op-data-table-font-size` | `type_body_small_size` |
| `line_height` | TU | `--op-data-table-line-height` | `type_body_small_line` |
| `header_font_size` | TU | `--op-data-table-header-font-size` | `type_caption_size` |
| `header_letter_spacing` | TU | `--op-data-table-header-letter-spacing` | `0.04 × type_caption_size` |
| `max_height` | DU | `--op-data-table-max-height` | `none` |

Border widths remain the fixed geometry specified above.

| Color variable | Default |
| :--- | :--- |
| `--op-data-table-border` | `border` |
| `--op-data-table-header-background` | `surface_alt` |
| `--op-data-table-secondary-text` | `text_secondary` |
| `--op-data-table-text` | `text_primary` |
| `--op-data-table-accent` | `accent` |
| `--op-data-table-divider` | `divider` |
| `--op-data-table-hover` | `hover` |
| `--op-data-table-selected` | `selection` |
| `--op-data-table-loading` | `pending` |
| `--op-data-table-loading-track` | `color-mix(in srgb, pending 20%, surface_alt)` |

| Slot | Place |
| :--- | :--- |
| `row` | Scoped `TableRow`; replaces the cells inside the native body row; return `td` elements. |
| `cell` | Scoped `TableCell { row, column_index, column, value }`; replaces content inside a native body cell. |
| `header_cell` | Scoped `TableHeaderCell { column_index, column }`; replaces header content inside the existing sort button when sortable. |
| `empty` | Empty table cell; defaults to the `data-table.empty` string. |
| `header` | Before the scrolling table. |
| `footer` | After the scrolling table. |
| `before_options` | Full-width native table row before body rows. |
| `after_options` | Full-width native table row after body rows. |

### Strings

Resolution: prop, application lookup, English default.

| Key | Default |
| :--- | :--- |
| `data-table.label` | `empty; preserves the unnamed table` |
| `data-table.empty` | `No data` |
| `data-table.loading` | `Loading rows` |
| `data-table.sort-ascending` | `Sort ascending` |
| `data-table.sort-descending` | `Sort descending` |
| `data-table.sort-clear` | `Clear sorting` |

### Icons

| Key | Default |
| :--- | :--- |
| `data-table.sort-ascending` | `arrow_upward` |
| `data-table.sort-descending` | `arrow_downward` |
