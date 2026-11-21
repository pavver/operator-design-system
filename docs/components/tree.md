# Tree

Expandable hierarchy of rows: areas and their devices, entity domains and entities, folders.
Only the visible rows are rendered, so trees with thousands of nodes stay fast.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Row | `row_compact` (`8 DU`), one line. |
| Indent | `4 DU` per level plus `1 DU`. |
| Arrow | `chevron_right` in a `4 DU` slot, turned `90°` when expanded; leaves keep the empty slot so labels align. |
| Icon | Optional, `4 DU`, `text_secondary`. |
| Detail | Optional `caption` text at the end, `text_secondary`. |
| Selected | `pressed` background and a `0.5 DU` `accent` bar on the start edge. |

## Behavior

* A click selects a row; a click on the arrow or a double click expands or collapses it.
* Arrow Up and Down move the selection; Right expands a node or enters its first child; Left collapses a node or goes to its parent; Home and End jump to the first and last row.
* The selection scrolls into view.
* Rows are a [VirtualList](virtual-list.md).

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `nodes` | `TreeNode` list or signal | required | `TreeNode::new(id, label)` with `icon`, `detail`, `children`; ids are unique in the tree. |
| `expanded` | set of ids signal | required | Expanded nodes. |
| `selected` | optional id signal | required | Selected node. |
| `on_select` | callback `(id)` | none | Called on every selection. |
| `height`, `max_height` | DU | none | Size of the scrolled area. |
| `label` | string | `Tree` | Accessible name. |

### Customization API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `row_height` | optional number (DU) | `row_compact` | Overrides `--op-tree-row-height`. |
| `indent` | optional number (DU) | `4 DU` | Overrides `--op-tree-indent`. |
| `padding_start` | optional number (DU) | `1 DU` | Overrides `--op-tree-padding-start`. |
| `padding_end` | optional number (DU) | `pad_tight` | Overrides `--op-tree-padding-end`. |
| `gap` | optional number (DU) | `gap_micro` | Overrides `--op-tree-gap`. |
| `icon_size` | optional number (DU) | `icon_small` | Overrides `--op-tree-icon-size`. |
| `font_size` | optional number (TU) | `inherit` | Overrides `--op-tree-font-size`. |
| `line_height` | optional number (TU) | `inherit` | Overrides `--op-tree-line-height`. |
| `detail_font_size` | optional number (TU) | `type_caption_size` | Overrides `--op-tree-detail-font-size`. |
| `detail_line_height` | optional number (TU) | `type_caption_line` | Overrides `--op-tree-detail-line-height`. |
| `height` | optional number (DU) | `none` | Overrides `--op-tree-height`. |
| `max_height` | optional number (DU) | `100%` when unset | Overrides `--op-tree-max-height`. |
| `option` | scoped slot | none | Scoped `TreeRow`; replaces only the node label, retaining selection, expansion, indentation, icons and detail. |
| `empty` | slot | none | When there are no visible nodes; the default remains blank. |
| `header` | slot | none | Above the tree rows. |
| `footer` | slot | none | Below the tree rows. |
| `before_options` | slot | none | Before the visible rows. |
| `after_options` | slot | none | After the visible rows. |
| `label` | optional string | `tree.label` | Text override. |
| `chevron_icon` | optional icon | `tree.chevron` | Icon override. |
| `class` | optional string | none | Added to the root classes. |
| `style` | optional string | none | Appended after root metric declarations. |

## Accessibility

* The tree has the `tree` role; rows are `treeitem`s with `aria-level`, `aria-expanded` and `aria-selected`.
* The scrolled area takes focus, and the arrow keys work while it has it.

## Customization

`class` and `style` apply to the root.
Explicit root styles take precedence over metric props; unset metrics retain the original rendering.
A private measurement marker and ResizeObserver keep VirtualList row heights and keyboard reveal offsets synchronized with the CSS variable.
Existing `height` and `max_height` props retain their DU meaning for the scrolled area.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `row_height` | DU | `--op-tree-row-height` | `row_compact` |
| `indent` | DU | `--op-tree-indent` | `4 DU` |
| `padding_start` | DU | `--op-tree-padding-start` | `1 DU` |
| `padding_end` | DU | `--op-tree-padding-end` | `pad_tight` |
| `gap` | DU | `--op-tree-gap` | `gap_micro` |
| `icon_size` | DU | `--op-tree-icon-size` | `icon_small` |
| `font_size` | TU | `--op-tree-font-size` | `inherit` |
| `line_height` | TU | `--op-tree-line-height` | `inherit` |
| `detail_font_size` | TU | `--op-tree-detail-font-size` | `type_caption_size` |
| `detail_line_height` | TU | `--op-tree-detail-line-height` | `type_caption_line` |
| `height` | DU | `--op-tree-height` | `none` |
| `max_height` | DU | `--op-tree-max-height` | `100%` when unset |

Border widths remain the fixed geometry specified above.

| Color variable | Default |
| :--- | :--- |
| `--op-tree-text` | `text_primary` |
| `--op-tree-hover` | `pressed` |
| `--op-tree-selected` | `pressed` |
| `--op-tree-accent` | `accent` |
| `--op-tree-secondary-text` | `text_secondary` |

| Slot | Place |
| :--- | :--- |
| `option` | Scoped `TreeRow`; replaces only the node label, retaining selection, expansion, indentation, icons and detail. |
| `empty` | When there are no visible nodes; the default remains blank. |
| `header` | Above the tree rows. |
| `footer` | Below the tree rows. |
| `before_options` | Before the visible rows. |
| `after_options` | After the visible rows. |

### Strings

Resolution: prop, application lookup, English default.

| Key | Default |
| :--- | :--- |
| `tree.label` | `Tree` |

### Icons

| Key | Default |
| :--- | :--- |
| `tree.chevron` | `chevron_right` |

Node icons are caller data and are not replaced by the chevron lookup.
