# Scrollbar

Rectangular overlay scrollbar that replaces the native one in every scrolling component.

Status: **reviewed** on web.

## Use

Every scroll container in the library uses this scrollbar, so scrolling looks the same in every browser and on every platform.
Use `ScrollView` for new scroll containers; use `Scrollbar` directly only when the scrolling element is fixed, such as a `textarea`.
The browser still owns the scroll position, so wheel, touch, keyboard, and assistive technology behave natively.

## Geometry

| Property | Value |
| :--- | :--- |
| Hit zone | `2 DU` (`gutter`) along the inner edge of the container, overlaying the content. |
| Idle thumb | `1 DU` (`pad_tight`) bar on the outer edge of the hit zone. |
| Hover and drag thumb | Fills the full `2 DU` hit zone. |
| Minimum thumb length | `6 DU`. |
| Corner | When both axes scroll, each bar stops `2 DU` short of the shared corner. |
| Shape | Corner radius `0 DU`, no shadow. |

The bar is drawn inside the container border and above its content.
It takes no layout space, so content never shifts when overflow appears.

## Colors and States

| State | Track | Thumb |
| :--- | :--- | :--- |
| Idle | Transparent | `border_strong` |
| Hover over the track (pointer devices) | `surface_alt` with a `0.25 DU` `divider` line on the inner edge | `border_strong`, full width |
| Hover over the thumb | Same as above | `text_secondary` |
| Dragging | Same as above | `accent` |
| No overflow | Hidden | Hidden |

Color and width changes take `state_color` (`150 ms`), linear.

## Interaction

1. Dragging the thumb scrolls proportionally; the pointer is captured until release.
2. Pressing the empty track pages toward the pointer by 90% of the visible length.
3. The bar follows scrolling, resizing of the container and its children, and content changes.
4. The bar is hidden from assistive technology; the scrolling element stays the accessible scroll region.
5. `ScrollView` chains scrolling like native scrolling: with nothing left to scroll, the wheel or swipe moves the page.
   Overlays (menus, dropdowns, Combobox, Dialog, Sheet) add `op-scroll-contain`, so the page under them stays still.

## Native Fallback

Scroll containers that are not built with `ScrollView` (for example application content inside `ThemeRoot`) get a thin native scrollbar.
It uses a `1 DU` `border_strong` thumb on a transparent track.
Horizontal strips that hide their scrollbar on purpose, such as [Tabs](tabs.md) and [Toolbar](toolbar.md), keep it hidden.

## Consumers

[DropdownMenu](dropdown-menu.md), [Combobox](combobox.md), [Menu](menu.md), [Dialog](dialog.md), [Sheet](sheet.md), [DataTable](data-table.md), [CodeBlock](code-block.md), [ConsoleTerminal](console-terminal.md), [TextArea](text-area.md), [SplitPane](split-pane.md), and [ScrollArea](scroll-area.md).

## API

### ScrollView

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `axis` | `ScrollAxis` | `Vertical` | `Vertical`, `Horizontal`, or `Both`. |
| `class` | string | none | Classes for the outer box (size, flex role). |
| `viewport_class` | string | none | Classes for the scrolled element (padding, gap, layout). |
| `height` | DU | none | Fixed height. |
| `max_height` | DU | none | Maximum height. |
| `track_size` | DU | `gutter` | Size of the track hit area. |
| `thumb_thickness` | DU | `pad_tight` | Idle thickness of the thumb. |
| `node_ref` | node ref | none | Reference to the scrolled element for programmatic scrolling. |
| `focusable` | bool | `false` | Makes the scrolled element focusable for keyboard scrolling. |
| `label` | string | `scroll-view.label` | Accessible name when focusable. |
| `header` | slot | none | Fixed content before the scrolled viewport. |
| `footer` | slot | none | Fixed content after the scrolled viewport. |
| `children` | view | required | Content. |

### Scrollbar

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `target` | node ref | required | The scrolling element; it must carry the `op-scroll-native-off` class. |
| `axis` | `ScrollAxis` | `Vertical` | Axis of this bar. |
| `corner` | bool | `false` | Leaves room for the crossing bar. |
| `track_size` | DU | `gutter` | Size of the track hit area. |
| `thumb_thickness` | DU | `pad_tight` | Idle thickness of the thumb. |

Place `Scrollbar` next to the target inside a positioned parent.

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-scrollbar-height` | none |
| `max_height` | DU | `--op-scrollbar-max-height` | `100%` |
| `track_size` | DU | `--op-scrollbar-track-size` | `gutter` |
| `thumb_thickness` | DU | `--op-scrollbar-thumb-thickness` | `pad_tight` |

| Color variable | Default |
| :--- | :--- |
| `--op-scrollbar-thumb-color` | `border_strong` |
| `--op-scrollbar-thumb-hover-color` | `text_secondary` |
| `--op-scrollbar-thumb-active-color` | `accent` |
| `--op-scrollbar-track-hover-background` | `surface_alt` |
| `--op-scrollbar-track-active-background` | `surface_alt` |
| `--op-scrollbar-divider-color` | `divider` |

| Slot | Place |
| :--- | :--- |
| `header` | Fixed content before the scrolled viewport in ScrollView. |
| `footer` | Fixed content after the scrolled viewport in ScrollView. |

`class` and `style` apply to the root element (`op-scrollbar` on Scrollbar, `op-scrollview` on ScrollView).

### Strings

| Key | Default |
| :--- | :--- |
| `scroll-view.label` | `""` |

### Icons

Scrollbar has no built-in semantic icons.
