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
| `Above` | Centered above the anchor, `gap` DU away; flips below near the top edge. |
| `AtPoint` | At a pointer position, shifted to stay inside the viewport (context menus). |
| `SideStart` | Beside the anchor, top edges aligned, to the right or else to the left (submenus). |

The layer is always clamped to the viewport.
It is measured hidden in the first frame and shown placed in the next one, so it never flashes at a wrong position.

## Behavior

* Scrolling of any ancestor and resizing the window re-place an open layer; `dismiss_on_resize` closes it instead (menus).
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
| `gap` | DU | `0` | Distance to the anchor. |
| `dismiss_on_outside` | bool | `true` | Close on a press outside. |
| `dismiss_on_escape` | bool | `true` | Close on Escape. |
| `dismiss_on_resize` | bool | `false` | Close instead of following on resize. |
| `restore_focus` | bool | `true` | Return focus on close. |
| `passive` | bool | `false` | Pointer events pass through. |
| `class`, `role`, `label` | string | none | Layer element classes, ARIA role and label. |
| `node_ref` | element reference | none | The layer element. |
| `layer_id`, `parent_layer` | layer id | new id, none | Link a layer to the layer it opens from (`new_layer_id()`). |
| `on_placed` | callback | none | Runs after each placement, for example to focus the first item. |
| `children` | view | required | Layer content. |
