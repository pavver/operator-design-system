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
| Item | `row_two_line` (`13 DU`) tall: a `7 DU` icon tile with an `icon_medium` (`4.5 DU`) icon over a `caption` label, `gap_micro` apart. |
| Label | One line, ellipsis when longer. |
| Selected | `pressed` background, `text_primary` label and a `0.5 DU` bar on the start edge in the item colour or `accent`. |
| Edge | `0.25 DU` `divider` on the end edge. |
| Horizontal | Full width row of top tabs under the app bar for narrow layouts; items share the width (at least `16 DU`, the row scrolls when they do not fit), the selection bar and the divider move to the bottom edge. |

## Behavior

* A click selects the destination.
* Arrow Up and Down (Left and Right when horizontal) move the selection and wrap; only the selected item is in the tab order.
* An item can carry its own colour (a category colour); it tints the icon and the selection bar.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | `RailItem` list or signal | required | `RailItem::new(id, label, icon)`, optional `color`. |
| `selected` | string signal | required | Id of the selected destination. |
| `on_select` | callback `(id)` | none | Called on every selection. |
| `label` | string | `navigation-rail.label` | Accessible name. |
| `horizontal` | bool signal | `false` | Top tabs layout for narrow screens. |
| `width` | DU | `16 DU` | Rail width. |
| `item_height` | DU | `row_two_line` | Destination item height. |
| `item_padding_x` | DU | `gap_micro` | Destination item horizontal padding. |
| `gap` | DU | `gap_micro` | Gap between icon tile and label. |
| `icon_tile_size` | DU | `7 DU` | Size of the icon square tile. |
| `icon_size` | DU | `icon_medium` (`4.5 DU`) | Icon glyph size. |
| `indicator_thickness` | DU | `border_emphasis` | Selection indicator thickness. |
| `font_size` | TU | `type_caption_size` | Destination label font size. |
| `header` | slot | none | Slot above the destinations list. |
| `footer` | slot | none | Slot below the destinations list. |
| `option` | scoped slot `(RailItem)` | none | Custom rendering for each destination item. |
| `class` | string | none | Classes for the root element. |
| `style` | string | none | Inline styles for the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-navigation-rail-width` | `16 DU` |
| `item_height` | DU | `--op-navigation-rail-item-height` | `row_two_line` (`13 DU`) |
| `item_padding_x` | DU | `--op-navigation-rail-item-padding-x` | `gap_micro` (`0.5 DU`) |
| `gap` | DU | `--op-navigation-rail-gap` | `gap_micro` (`0.5 DU`) |
| `icon_tile_size` | DU | `--op-navigation-rail-icon-tile-size` | `7 DU` |
| `icon_size` | DU | `--op-navigation-rail-icon-size` | `icon_medium` (`4.5 DU`) |
| `indicator_thickness` | DU | `--op-navigation-rail-indicator-thickness` | `border_emphasis` (`0.5 DU`) |
| `font_size` | TU | `--op-navigation-rail-font-size` | `type_caption_size` |

| Color variable | Default |
| :--- | :--- |
| `--op-navigation-rail-background` | `surface` |
| `--op-navigation-rail-divider` | `divider` |
| `--op-navigation-rail-text` | `text_secondary` |
| `--op-navigation-rail-hover-background` | `pressed` |
| `--op-navigation-rail-hover-text` | `text_primary` |
| `--op-navigation-rail-selected-text` | `text_primary` |
| `--op-navigation-rail-selected-background` | `pressed` |
| `--op-navigation-rail-indicator-color` | `accent` |
| `--op-navigation-rail-selected-icon` | `accent` |

| Slot | Place |
| :--- | :--- |
| `header` | Above the destinations list. |
| `footer` | Below the destinations list. |
| `option` | Custom content inside the destination item button. |

`class` and `style` apply to the root element.

### Strings

| Key | Default |
| :--- | :--- |
| `navigation-rail.label` | `Navigation` |

### Icons

The component has no built-in icons; destinations pass their own icon data.

## Accessibility

* The rail is a `nav` with a vertical `tablist`; items are `tab`s with `aria-selected`.
