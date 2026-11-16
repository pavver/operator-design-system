# Combobox

Text input with a filtered list of suggestions.
It is a [DropdownMenu](dropdown-menu.md) that can be searched by typing.

Status: **draft** on web.

## Use

* Choosing one value from a long list: cameras, zones, users.
* With `allow_custom`, entering a new value or picking a known one, for example tags.

## Geometry

* The field is a [TextField](text-field.md) with a trailing `arrow_drop_down` icon.
* The list is the DropdownMenu popup: under the field, full width, at most `50 DU` tall, `accent` outline.
* The matched part of each option is shown in `accent` bold.
* The active option, moved by the keyboard or the pointer, has the `pressed` background.
* The selected option keeps the DropdownMenu selected style.

## Behavior

* Focus, a click on the field, or Arrow Down opens the list with all options, whatever text is in the field.
* On opening, the selected option is active and scrolled to the middle of the list.
  With nothing selected, the first option that contains the field text is active instead, or the first option.
* Any change of the text switches the list to filtering by a case-insensitive substring and makes the first match active.
* Moving the active option with the keyboard scrolls it into view.
* An option is chosen with a click, or with Enter on the active option.
* Arrow keys move the active option and wrap around.
* Escape closes the list and restores the committed value.
* Leaving the field commits typed text that exactly matches an option.
  Other text is committed only with `allow_custom`; otherwise the field reverts to the committed value.
* With no matches the list shows "No matches", or "Press Enter to use this value" with `allow_custom`.
* The suggestion list is a [VirtualList](virtual-list.md): only the visible options are rendered, so lists with thousands of options stay fast.
* The list is a [Popover](popover.md) layer as wide as the field, so it is not clipped by scrolling containers.

## Options

An option is a plain string or a `ComboOption`:

| Field | Description |
| :--- | :--- |
| `value` | Committed value. |
| `label` | Text in the list and in the field; the value when empty. |
| `detail` | Secondary text at the end of the row in `caption` style, for example a type or a unit. |
| `icon` | `4 DU` icon before the label in `text_secondary`. |
| `section` | Header above a group; consecutive options with the same section share one header. |

* Section headers are list rows of the same height in `caption` style, uppercase `text_secondary`, with a `divider` line above all but the first.
* Typing matches the label or the value; sections without matches disappear.
* The options can be a signal, so a list loaded on demand fills in while the list is open.

## Typed values

`parse_custom` turns typed text into a value or rejects it with `None` (a number in range, a duration, an entity id).
A rejected text reverts to the committed value, like unknown text without `allow_custom`.

## ComboboxPopup

The same list with a search field in a floating layer opened at a point, without a field on the page: quick add on a canvas, a wire menu, a command palette.

* It opens at the anchor (by default the pointer position) with an empty search that has focus.
* Arrows move the active option, Enter or a click picks it and closes the layer.
* Enter with no match passes the typed text to `on_custom` when it is set.
* Escape and a press outside close it.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `open` | bool signal | required | Shows the layer. |
| `anchor` | `PopoverAnchor` signal | required | Where it opens. |
| `options` | options | required | See *Options*. |
| `on_select` | callback `(ComboOption)` | required | Called with the picked option. |
| `on_custom` | callback `(string)` | none | Enter with text and no match. |
| `placement` | `Placement` | `AtPoint` | See [Popover](popover.md). |
| `width` | DU | `80` | Layer width. |
| `placeholder`, `label`, `empty_text`, `custom_hint` | string | | Texts. |

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `options` | strings, `ComboOption`s or a signal of them | required | See *Options*. |
| `selected` | string signal | required | Committed value. |
| `placeholder` | string | empty | Hint. |
| `label` | string | none | Label above the field and accessible name. |
| `allow_custom` | bool | `false` | Accepts values outside the list. |
| `parse_custom` | callback `(string) -> option<string>` | none | Validates and normalizes typed values; implies `allow_custom`. |
| `on_select` | callback `(string)` | none | Called on every commit. |
| `enabled` | bool | `true` | Enables input. |
| `empty_text` | string | `No matches` | Row shown when nothing matches. |
| `custom_hint` | string | `Press Enter to use this value` | Row shown when nothing matches but typed text is accepted. |

## Accessibility

The input has the `combobox` role with `aria-expanded`; the popup is a `listbox` of `option` items.
