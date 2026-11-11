# Combobox

Text input with a filtered list of suggestions.
It is a [DropdownMenu](dropdown-menu.md) that can be searched by typing.

Status: **reviewed** on web.

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

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `options` | list of strings | required | Suggestions. |
| `selected` | string signal | required | Committed value. |
| `placeholder` | string | empty | Hint. |
| `label` | string | none | Label above the field and accessible name. |
| `allow_custom` | bool | `false` | Accepts values outside the list. |
| `on_select` | callback `(string)` | none | Called on every commit. |
| `enabled` | bool | `true` | Enables input. |

## Accessibility

The input has the `combobox` role with `aria-expanded`; the popup is a `listbox` of `option` items.
