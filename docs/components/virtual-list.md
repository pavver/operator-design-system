# VirtualList

Vertical list of fixed-height rows that renders only the rows inside the viewport.
It keeps long lists (thousands of options, entities or records) as fast as short ones.

Status: **draft** on web.

## Use

* Lists that can grow beyond a few hundred rows: picker options, entity catalogs, search results.
* The list scrolls inside a [ScrollView](scrollbar.md), so it uses the Operator scrollbar.
* Rows must have one fixed height; content that wraps or grows does not fit a virtual list.
  Variable-height content such as [ConsoleTerminal](console-terminal.md) lines is not virtualized.

## Geometry

| Property | Value |
| :--- | :--- |
| Row height | Fixed, in DU, given by the caller (for example `row_control` for menu options, `row_compact` for dense lists). |
| Height | Fixed `height` or `max_height` in DU; the list is as tall as its rows up to the limit. |
| Rendered rows | The visible rows plus `6` above and below (`overscan`). |

## Behavior

* Scrolling and resizing the viewport update the rendered window in the same frame.
* The DOM holds at most the visible rows plus the overscan, whatever the item count.
* Rows keep their identity by a caller-given key, so focus and state survive scrolling within the window.
* `VirtualListHandle::reveal(index, center)` scrolls a row into view, optionally into the middle; components use it for keyboard navigation.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | list signal | required | All items. |
| `handle` | `VirtualListHandle` | required | Row height in DU; scrolls the list from the outside. |
| `height` | DU | none | Fixed height. |
| `max_height` | DU | none | Height limit. |
| `overscan` | integer | `6` | Extra rows above and below the window. |
| `focusable` | bool | `false` | Makes the scrolled element focusable for keyboard scrolling. |
| `key` | `item -> key` | required | Stable identity of an item. |
| `children` | `(index, item) -> view` | required | Renders one row. |

## Accessibility

* The component does not add a role; the caller sets it (for example `listbox` with `option` rows).
* Rows outside the window are not in the accessibility tree, so set `aria-setsize` and `aria-posinset` on rows when the full count matters.
