# Menu

Floating list of commands.
It opens from an overflow button (⋮) or as a context menu at the pointer.

Status: **draft** on web.

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
| Submenu arrow | `chevron_right`, `4 DU`, `text_secondary`, at the end of an item with a submenu. |

## Items

* Default: `text_primary`, icon in `icon`.
* Danger: label and icon in `error`; place it last, after a separator.
* Disabled: `text_disabled`, cannot be chosen or focused.
* Hover and keyboard focus: `pressed` background; keyboard focus also shows a `0.5 DU` `accent` start bar.

## Submenus

* An item with children (`MenuItem::submenu`) opens a submenu beside itself instead of running a command.
* The submenu opens on hover, a click or Arrow Right, top-aligned to the item, to the right or, without room, to the left.
* The item stays highlighted while its submenu is open; hovering another item closes it.
* Arrow Left closes the submenu and returns focus to its item.
* Picking a command at any level closes the whole menu chain; a press inside any submenu does not close its parents.
* Submenus nest; keep them at most two levels deep.

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
* The menu is a [Popover](popover.md) layer: it mounts in the overlay host, so clipping or transformed ancestors never cut it.

## API

`MenuItem` fields: `id`, `label`, `icon`, `shortcut`, `danger`, `enabled`, `separator`.

| Component | Properties |
| :--- | :--- |
| `Menu` | `open`, `anchor` (below an element, at a point or beside an element), `items`, `on_select(id)`, `label`, `parent_layer` (set for submenus). |
| `MenuItem` | `new(id, label)`, `icon`, `shortcut`, `danger`, `disabled`, `separated`, `submenu(items)`. |
| `MenuButton` | `items`, `on_select(id)`, `icon` (`more_vert`), `label` (`"More actions"`), `size` (`8 DU`). |
| `ContextMenu` | `items`, `on_select(id)`, `children`: the area that opens the menu. |

The customization properties are optional and accept reactive values where indicated.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | none | Root hooks. |
| `min_width` | reactive DU metric | design-system value | Sets `--op-menu-min-width`. |
| `max_width` | reactive DU metric | design-system value | Sets `--op-menu-max-width`. |
| `viewport_margin` | reactive DU metric | design-system value | Sets `--op-menu-viewport-margin`. |
| `shadow_y` | reactive DU metric | design-system value | Sets `--op-menu-shadow-y`. |
| `shadow_blur` | reactive DU metric | design-system value | Sets `--op-menu-shadow-blur`. |
| `gap` | reactive DU metric | design-system value | Sets `--op-menu-gap`. |
| `item_height` | reactive DU metric | design-system value | Sets `--op-menu-item-height`. |
| `padding_end` | reactive DU metric | design-system value | Sets `--op-menu-padding-end`. |
| `padding_start` | reactive DU metric | design-system value | Sets `--op-menu-padding-start`. |
| `icon_size` | reactive DU metric | design-system value | Sets `--op-menu-icon-size`. |
| `shortcut_font_size` | reactive TU metric | design-system value | Sets `--op-menu-shortcut-font-size`. |
| `padding_y` | reactive DU metric | design-system value | Sets `--op-menu-padding-y`. |
| `font_size` | reactive TU metric | design-system value | Sets `--op-menu-font-size`. |
| `line_height` | reactive TU metric | design-system value | Sets `--op-menu-line-height`. |
| `max_height` | reactive DU metric | design-system value | Sets `--op-menu-max-height`. |
| `option` | `ScopedSlot<MenuItem>` | none | Scoped `MenuItem` content inside the existing menuitem button; preserves keyboard, disabled, danger, selection, and submenu behavior. |
| `empty` | `Slot` | none | Content when there are no items. |
| `before_options` | `Slot` | none | Content before the scrollable items. |
| `after_options` | `Slot` | none | Content after the scrollable items. |
| `header` | `Slot` | none | Replaces the title content. |
| `footer` | `Slot` | none | Content after the body and actions. |
| `label` | reactive string | `menu.label` | Accessible name of the menu. |
| `submenu_icon` | icon | `menu.submenu` | Submenu arrow override. |
| `MenuButton.icon` | icon | `menu.more` | Overflow icon override. |
| `MenuButton.label` | reactive string | `menu.more-actions` | Overflow button name. |
| `MenuButton.size` | DU | `row_compact` | Button size; writes `--op-menu-button-size`. |
| `ContextMenu.label` | reactive string | `menu.context` | Context menu name. |

## Customization

`class` and `style` apply to the floating menu layer; MenuButton and ContextMenu apply them to their trigger roots.
Metric props write the corresponding variables; an explicit `style` declaration wins.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `min_width` | DU | `--op-menu-min-width` | `45 DU` |
| `max_width` | DU | `--op-menu-max-width` | `80 DU` |
| `viewport_margin` | DU | `--op-menu-viewport-margin` | `margin_screen` |
| `shadow_y` | DU | `--op-menu-shadow-y` | `2 DU` |
| `shadow_blur` | DU | `--op-menu-shadow-blur` | `6 DU` |
| `gap` | DU | `--op-menu-gap` | `gutter` |
| `item_height` | DU | `--op-menu-item-height` | `row_compact` |
| `padding_end` | DU | `--op-menu-padding-end` | `pad_card` |
| `padding_start` | DU | `--op-menu-padding-start` | `gutter` |
| `icon_size` | DU | `--op-menu-icon-size` | `icon_small` |
| `shortcut_font_size` | TU | `--op-menu-shortcut-font-size` | `caption_size` |
| `padding_y` | DU | `--op-menu-padding-y` | `pad_tight` |
| `font_size` | TU | `--op-menu-font-size` | `body_size` |
| `line_height` | TU | `--op-menu-line-height` | `body_line` |
| `max_height` | DU | `--op-menu-max-height` | `calc(100vh - var(--op-menu-viewport-margin, margin_screen) * 2)` |
| `MenuButton.size` | DU | `--op-menu-button-size` | `row_compact` |

| Color variable | Default |
| :--- | :--- |
| `--op-menu-background` | `surface` |
| `--op-menu-border` | `border_strong` |
| `--op-menu-shadow` | `rgb(0 0 0 / 35%)` |
| `--op-menu-text` | `text_primary` |
| `--op-menu-hover` | `hover` |
| `--op-menu-focus` | `accent` |
| `--op-menu-danger` | `error` |
| `--op-menu-disabled` | `text_disabled` |
| `--op-menu-icon` | `icon` |
| `--op-menu-secondary` | `text_secondary` |
| `--op-menu-divider` | `divider` |
| `--op-menu-pressed` | `pressed` |
| `--op-menu-ripple` | `ripple` |

| Slot | Place |
| :--- | :--- |
| `option` | Scoped `MenuItem` content inside the existing menuitem button; preserves keyboard, disabled, danger, selection, and submenu behavior. |
| `empty` | Content when there are no items. |
| `before_options` | Content before the scrollable items. |
| `after_options` | Content after the scrollable items. |
| `header` | Replaces the title content. |
| `footer` | Content after the body and actions. |

The metric variables, `class`, `style`, all menu slots, and the submenu icon override propagate to every submenu.
The `option` slot replaces row content only; keep it free of nested interactive controls.

### Strings

| Key | Default | Prop |
| :--- | :--- | :--- |
| `menu.label` | `Menu` | `label` |
| `menu.more-actions` | `More actions` | `MenuButton.label` |
| `menu.context` | `Context menu` | `ContextMenu.label` |

### Icons

| Key | Default | Prop |
| :--- | :--- | :--- |
| `menu.submenu` | `chevron_right` | `submenu_icon` |
| `menu.more` | `more_vert` | `MenuButton.icon` |

## Accessibility

`menu` with `menuitem` children; disabled items are marked `aria-disabled`.
The overflow button has `aria-haspopup` and `aria-expanded`.
