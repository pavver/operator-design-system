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

## Accessibility

* The tree has the `tree` role; rows are `treeitem`s with `aria-level`, `aria-expanded` and `aria-selected`.
* The scrolled area takes focus, and the arrow keys work while it has it.
