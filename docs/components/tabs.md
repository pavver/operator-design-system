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
| Icon | `icon_small` (`4 DU`), `1 DU` before the label. |
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
| `label` | string | `tabs.label` | Accessible name of the tab list. |
| `height` | DU | `row_standard` | Tab height. |
| `padding_x` | DU | `pad_content` | Tab horizontal padding. |
| `gap` | DU | `pad_tight` | Gap between icon and label. |
| `font_size` | TU | `type_caption_size` | Label font size. |
| `count_min_width` | DU | `4 DU` | Minimum width of the counter badge. |
| `count_padding_x` | DU | `pad_tight` | Counter badge horizontal padding. |
| `indicator_thickness` | DU | `border_thick` | Indicator line thickness. |
| `option` | scoped slot | none | Custom rendering for each tab. |
| `before_options` | slot | none | Content before the tabs list. |
| `after_options` | slot | none | Content after the tabs list. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-tabs-height` | `row_standard` |
| `padding_x` | DU | `--op-tabs-padding-x` | `pad_content` |
| `gap` | DU | `--op-tabs-gap` | `pad_tight` |
| `font_size` | TU | `--op-tabs-font-size` | `type_caption_size` |
| `count_min_width` | DU | `--op-tabs-count-min-width` | `4 DU` |
| `count_padding_x` | DU | `--op-tabs-count-padding-x` | `pad_tight` |
| `indicator_thickness` | DU | `--op-tabs-indicator-thickness` | `border_thick` |

| Color variable | Default |
| :--- | :--- |
| `--op-tabs-text` | `text_secondary` |
| `--op-tabs-text-hover` | `text_primary` |
| `--op-tabs-text-active` | `accent` |
| `--op-tabs-count-background` | `surface_alt` |
| `--op-tabs-count-text` | `text_secondary` |
| `--op-tabs-count-active-background` | `accent` |
| `--op-tabs-count-active-text` | `on_accent` |
| `--op-tabs-divider` | `divider` |
| `--op-tabs-indicator-color` | `accent` |
| `--op-tabs-dock-background` | `surface_alt` |
| `--op-tabs-dock-border` | `border` |
| `--op-tabs-active-background` | `background` |

| Slot | Place |
| :--- | :--- |
| `option` | Scoped slot replacing default tab contents. |
| `before_options` | Before the tab list. |
| `after_options` | After the tab list. |

`class` and `style` apply to the root tabs element.

### Strings

| Key | Default |
| :--- | :--- |
| `tabs.label` | `Tabs` |

### Icons

Tabs has no built-in semantic icons.

## Accessibility

* `tablist` with `tab` items and `aria-selected`.
* One tab stop on the active tab; Left and Right arrows, Home, and End move and select.
