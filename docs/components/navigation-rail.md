# NavigationRail

Vertical bar of top-level destinations at the start edge: an icon over a short label.

Status: **draft** on web.

## Use

* Switching between a few (three to eight) sections of a screen, such as node categories or app areas.
* For more destinations or longer names use a list or [Tabs](tabs.md).

## Geometry

| Property | Value |
| :--- | :--- |
| Width | `16 DU`; the rail scrolls vertically without a visible scrollbar when the items do not fit. |
| Item | `row_two_line` (`13 DU`) tall: a `7 DU` icon tile with a `4.5 DU` icon over a `caption` label, `gap_micro` apart. |
| Label | One line, ellipsis when longer. |
| Selected | `pressed` background, `text_primary` label and a `0.5 DU` bar on the start edge in the item colour or `accent`. |
| Edge | `0.25 DU` `divider` on the end edge. |

## Behavior

* A click selects the destination.
* Arrow Up and Down move the selection and wrap; only the selected item is in the tab order.
* An item can carry its own colour (a category colour); it tints the icon and the selection bar.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | `RailItem` list or signal | required | `RailItem::new(id, label, icon)`, optional `color`. |
| `selected` | string signal | required | Id of the selected destination. |
| `on_select` | callback `(id)` | none | Called on every selection. |
| `label` | string | `Navigation` | Accessible name. |

## Accessibility

* The rail is a `nav` with a vertical `tablist`; items are `tab`s with `aria-selected`.
