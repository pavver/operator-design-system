# Tabs

Switches between views of the same level, such as Chats and Functions on the main screen.

Status: **reviewed** on web.

## Variants

### Folder

The variant of the Android main screen.
Inactive tabs sit in a `surface_alt` dock with a `0.25 DU` `border`.
The active tab is cut out of the dock: it has the `background` color of the content below, a `0.5 DU` `accent` outline on its top and sides, and no bottom edge.
A `0.5 DU` `accent` baseline runs under the whole dock, so the active tab opens into the content like a folder.

### Line

A `0.25 DU` `divider` under the tabs and a `0.5 DU` `accent` underline under the active tab.

## Geometry

| Property | Value |
| :--- | :--- |
| Tab height | `10 DU`. |
| Tab padding | `2.5 DU` horizontal; the width follows the label. |
| Label | `caption` size, uppercase, medium weight; bold `accent` when active, `text_secondary` otherwise. |
| Icon | `4 DU`, `1 DU` before the label. |
| Counter | After the label, `4 DU` minimum width; `accent` background on the active tab. |
| Fill | Optional: tabs share the full width equally. |

## Behavior

* The active surface (folder) or underline (line) slides to the new tab over `segment_slide` (`150 ms`), linear like every sliding indicator, and resizes to its width.
* Label color changes over the same time.
* Tabs that do not fit scroll horizontally without a visible scrollbar.
* The component switches the selection only; the caller renders the panel.
  With a pager, the caller may drive the indicator from the scroll offset, as the Android app does.

## API

`TabItem` fields: `label`, `icon`, `count` (hidden at zero).

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | list of tab items | required | Tabs. |
| `selected` | index signal | required | Active tab. |
| `variant` | Folder or Line | Folder | Style. |
| `fill` | bool | `false` | Equal widths over the full row. |
| `label` | string | none | Accessible name of the tab list. |

## Accessibility

* `tablist` with `tab` items and `aria-selected`.
* One tab stop on the active tab; Left and Right arrows, Home, and End move and select.
