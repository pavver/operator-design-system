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

## Accessibility

Native table semantics; sortable headers carry `aria-sort`, selected rows carry `aria-selected`.
