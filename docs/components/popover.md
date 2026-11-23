# Popover

Floating layer attached to an anchor: the shared base of [Menu](menu.md), [DropdownMenu](dropdown-menu.md), [Combobox](combobox.md), [Tooltip](tooltip.md) and [MarqueeText](marquee-text.md) tooltips.
It has no look of its own; the component that uses it styles the layer.

Status: **draft** on web.

## Layer

* Every `ThemeRoot` holds one overlay host after its content.
  Popovers mount there, so ancestors with transforms, clipping, `overflow` or their own stacking context never move or cut them, and the theme still applies.
* Outside a theme root the layer mounts in the document body.
* Layers stack in the order they open; a later layer covers an earlier one.

## Placement

| Placement | Position |
| :--- | :--- |
| `BelowStart` | Under the anchor, start edges aligned; flips above when there is no room below. |
| `BelowEnd` | Under the anchor, end edges aligned; flips above when there is no room below. |
| `BelowStretch` | Under the anchor with the anchor's width, for lists of a field. |
| `Above` | Centered above the anchor, `gap` DU away; flips below when it fits only there. |
| `Below` | Centered below the anchor, `gap` DU away; flips above when it fits only there. |
| `Side(side, align)` | On a preferred `LayerSide` with a preferred `LayerAlign`; flips to the opposite side, then (from the left or right) below and above, and shifts along the anchor to stay on screen. |
| `AtPoint` | At a pointer position, shifted to stay inside the viewport (context menus). |
| `SideStart` | Beside the anchor, top edges aligned, to the right or else to the left (submenus). |

The layer is always clamped to the viewport.
A centered or side layer (`Above`, `Below`, `Side`) near a screen edge shifts along the anchor to stay on screen, and is never wider than the viewport.
When it fits neither above nor below, it takes the side with more room, gets that height as its limit and clips.
Content that can grow puts its rows in a [ScrollView](scrollbar.md), which fills the limited layer and scrolls with the library scrollbar instead of the native one.
It is measured hidden in the first frame and shown placed in the next one, so it never flashes at a wrong position.

## Appearance

| Property | Value |
| :--- | :--- |
| Surface | `surface`. |
| Frame | `0.25 DU` `accent` border inside the bounds, the same for every floating layer. |

Components built on Popover keep this frame and only map its color to their own variable, for example `--op-menu-border`.

## Behavior

* Scrolling of any ancestor and resizing the window re-place an open layer; `dismiss_on_resize` closes it instead (menus).
* `dismiss_on_scroll` closes the layer when the page or any container outside it scrolls, instead of following the anchor.
* Scrolling inside the layer neither re-places nor closes it, so its own scroll view keeps its offset and smooth wheel and touch scrolling.
* A press outside the layer and the anchor closes it (`dismiss_on_outside`, default on).
  A layer opened from another one (`parent_layer`) counts as inside its parent, so a press in a submenu keeps the menu open.
* Escape closes it (`dismiss_on_escape`, default on).
* When the layer closes while focus is inside it, focus returns to the element that had it before opening (`restore_focus`, default on).
* A `passive` layer lets pointer events pass through (tooltips).

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `open` | bool signal | required | Shows the layer. |
| `anchor` | `PopoverAnchor` signal | required | Element box (`PopoverAnchor::of(el)`) or pointer point. |
| `placement` | `Placement` signal | `BelowStart` | See *Placement*. |
| `gap` | reactive DU metric | `0` | Distance to the anchor. |
| `dismiss_on_outside` | bool | `true` | Close on a press outside. |
| `dismiss_on_escape` | bool | `true` | Close on Escape. |
| `dismiss_on_resize` | bool | `false` | Close instead of following on resize. |
| `dismiss_on_scroll` | bool | `false` | Close instead of following when anything outside the layer scrolls. |
| `restore_focus` | bool | `true` | Return focus on close. |
| `passive` | bool | `false` | Pointer events pass through. |
| `class`, `role`, `label` | string | none | Layer element classes, ARIA role and label. |
| `node_ref` | element reference | none | The layer element. |
| `layer_id`, `parent_layer` | layer id | new id, none | Link a layer to the layer it opens from (`new_layer_id()`). |
| `on_placed` | callback | none | Runs after each placement, for example to focus the first item. |
| `children` | view | required | Layer content. |

The customization properties are optional and accept reactive values where indicated.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | none | Root hooks. |
| `offset_x` | reactive DU metric | design-system value | Sets `--op-popover-offset-x`. |
| `offset_y` | reactive DU metric | design-system value | Sets `--op-popover-offset-y`. |
| `header` | `Slot` | none | Content before the default children. |
| `actions` | `Slot` | none | Content after the default children, before the footer. |
| `footer` | `Slot` | none | Content after the body and actions. |
| `aria_label` | reactive string | `popover.label` | Accessible name; takes precedence over the compatible `label` prop. |

## Customization

`class` and `style` apply to the floating layer.
Metric props write the corresponding variables; an explicit `style` declaration wins.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `offset_x` | DU | `--op-popover-offset-x` | `0 DU` |
| `offset_y` | DU | `--op-popover-offset-y` | `0 DU` |
| `gap` | DU | `--op-popover-gap` | `0 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-popover-background` | `surface` |
| `--op-popover-border` | `accent` |
| `--op-popover-text` | `inherit` |

| Slot | Place |
| :--- | :--- |
| `header` | Content before the default children. |
| `actions` | Content after the default children, before the footer. |
| `footer` | Content after the body and actions. |

`placement` keeps its existing enum and signal API.
`gap` controls separation from the anchor in DU for below, above, and side placements.
`offset_x` and `offset_y` translate the placed layer in viewport axes after clamping.
The optional `label` prop remains compatible; `aria_label` adds a reactive override.
A private size observer re-places the layer when metrics, slots, or translated content resize it.

### Strings

| Key | Default | Prop |
| :--- | :--- | :--- |
| `popover.label` | `empty` | `aria_label / label` |

### Icons

This component has no built-in icons.
