# Menu

Floating list of commands.
It opens from an overflow button (⋮) or as a context menu at the pointer.

Status: **reviewed** on web.

## Use

* Secondary commands that do not deserve a visible button.
* Commands for one item in a list or table, through a right click or a long press.
* For choosing a value use [DropdownMenu](dropdown-menu.md); a menu runs commands.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | `45 DU` to `80 DU`, from the longest item. |
| Height | Content height, at most the viewport minus `8 DU`, then scrolls. |
| Padding | `1 DU` above and below the items. |
| Outline | `0.25 DU` `border_strong` plus a soft drop shadow. |
| Item | At least `8 DU` tall, `2 DU` start and `3 DU` end padding. |
| Icon slot | `4 DU`, kept empty for items without an icon so labels align. |
| Shortcut | `caption` style, `text_secondary`, at the end. |
| Separator | `0.25 DU` `divider` with `1 DU` above and below. |

## Items

* Default: `text_primary`, icon in `icon`.
* Danger: label and icon in `error`; place it last, after a separator.
* Disabled: `text_disabled`, cannot be chosen or focused.
* Hover and keyboard focus: `pressed` background; keyboard focus also shows a `0.5 DU` `accent` start bar.

## Placement

* From a button: under the button, aligned to its end edge; above it when there is no room below.
* Context menu: at the pointer, shifted to stay inside the viewport.

## Behavior

* Opening moves focus to the first enabled item.
* Arrow keys move between enabled items and wrap.
* Enter or a click runs the item and closes the menu.
* Escape, Tab, a press outside, or a window resize closes it.
* The overflow button shows the `Selected` border while its menu is open; pressing it again closes the menu.
* Commands run synchronously from the menu's point of view.
  An async command shows its result elsewhere, for example in a [Toast](toast.md) or on the affected row.

## API

`MenuItem` fields: `id`, `label`, `icon`, `shortcut`, `danger`, `enabled`, `separator`.

| Component | Properties |
| :--- | :--- |
| `Menu` | `open`, `anchor` (below an element or at a point), `items`, `on_select(id)`, `label`. |
| `MenuButton` | `items`, `on_select(id)`, `icon` (`more_vert`), `label` (`"More actions"`), `size` (`8 DU`). |
| `ContextMenu` | `items`, `on_select(id)`, `children`: the area that opens the menu. |

## Accessibility

`menu` with `menuitem` children; disabled items are marked `aria-disabled`.
The overflow button has `aria-haspopup` and `aria-expanded`.
