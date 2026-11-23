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
* The active option, moved by the keyboard or the pointer, has a `0.5 DU` `accent` ring inside it and, unless selected, the `hover` background, as in [States](../foundations/states.md).
* The selected option keeps the DropdownMenu selected style.

## Behavior

* Scrolling the page or another container outside the list closes it; scrolling inside it, or inside its own field, does not.
* Focus, a click on the field, or Arrow Down opens the list with all options, whatever text is in the field.
* A press on the `prepend` content (such as a flag) or on the arrow focuses the text and opens the list, or closes it when it is open; buttons and links the caller places there keep their own presses.
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
| `icon` | `icon_small` (`4 DU`) icon before the label in `text_secondary`. |
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

### Customization API

Every text-like input uses the public [Field](field.md) shell.
Labels are visible above the box; use `aria_label` for an accessible name alone.

| Property | Type | Default | Components | Description |
| :--- | :--- | :--- | :--- | :--- |
| `placeholder` | `string signal` | string key | Combobox, ComboboxPopup | Translatable text; see Strings. |
| `label` | `string signal` | none | Combobox, ComboboxPopup | Visible label and accessible name. |
| `hint` | `string signal` | none | Combobox, ComboboxPopup | Helper text below the box. |
| `error` | `string signal` | none | Combobox, ComboboxPopup | Error text replaces hint and draws the error outline. |
| `variant` | `FieldVariant signal` | outlined | Combobox, ComboboxPopup | Field appearance. |
| `aria_label` | `string signal` | string key | Combobox, ComboboxPopup | Accessible name when label is unset. |
| `prepend` | `slot` | none | Combobox, ComboboxPopup | Inside the box before the control. |
| `append` | `slot` | none | Combobox, ComboboxPopup | Inside the box before built-in reveal, save or picker controls. |
| `before` | `slot` | none | Combobox, ComboboxPopup | Outside the box before it. |
| `after` | `slot` | none | Combobox, ComboboxPopup | Outside the box after it. |
| `label_slot` | `slot` | none | Combobox, ComboboxPopup | Replaces the visible label. |
| `hint_slot` | `slot` | none | Combobox, ComboboxPopup | Replaces the hint and error line. |
| `height` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `padding_x` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `slot_size` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `gap` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `font_size` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `class` | `string signal` | none | Combobox, ComboboxPopup | Additional root class. |
| `style` | `string signal` | none | Combobox, ComboboxPopup | Additional root style. |
| `state` | `Signal<Option<ControlState>>` | none | Combobox | External control-state override. |
| `icon` | `Option<IconData>` | icon key | Combobox | Built-in icon override; see Icons. |
| `icon_size` | `number` | unset | Combobox | Metric override; see below. |
| `row_height` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `max_height` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `option_gap` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `option_padding_x` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `option_icon_size` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `detail_line_height` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `letter_spacing` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `detail_font_size` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `section_padding_y` | `number` | unset | Combobox, ComboboxPopup | Metric override; see below. |
| `option` | `Option<ScopedSlot<ComboOption>>` | none | Combobox, ComboboxPopup | Scoped option content: ComboOption for Combobox, String for DropdownMenu. |
| `empty` | `slot` | none | Combobox, ComboboxPopup | Replaces the empty list message. |
| `before_options` | `slot` | none | Combobox, ComboboxPopup | Before the option rows. |
| `after_options` | `slot` | none | Combobox, ComboboxPopup | After the option rows. |
| `header` | `slot` | none | Combobox, ComboboxPopup | Before the popup options; Calendar: above the month navigation. |
| `footer` | `slot` | none | Combobox, ComboboxPopup | After the popup options; Calendar: below the day grid. |
| `width` | `f32` | `80 DU` | ComboboxPopup | Metric override; see below. |
| `screen_margin` | `number` | unset | ComboboxPopup | Metric override; see below. |

## Accessibility

The input has the `combobox` role with `aria-expanded`; the popup is a `listbox` of `option` items.

## Customization

`class` and `style` apply to the root; explicit `style` declarations follow metric props and win.
Unset metric props preserve token fallbacks.
All text-like inputs provide the same six field slots.

| Metric prop | Unit | Variable | Default | Components |
| :--- | :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard; ComboboxPopup uses row_control` | Combobox, ComboboxPopup |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` | Combobox, ComboboxPopup |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` | Combobox, ComboboxPopup |
| `gap` | DU | `--op-field-gap` | `gap_micro` | Combobox, ComboboxPopup |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | Combobox, ComboboxPopup |
| `icon_size` | DU | `--op-field-icon-size` | `icon_small` (`4 DU`); `icon_field` (`5 DU`) for picker arrows | Combobox |
| `row_height` | DU | `--op-field-row-height` | `row_control` | Combobox, ComboboxPopup |
| `max_height` | DU | `--op-field-max-height` | `50 DU` | Combobox, ComboboxPopup |
| `option_gap` | DU | `--op-field-option-gap` | `pad_tight` | Combobox, ComboboxPopup |
| `option_padding_x` | DU | `--op-field-option-padding-x` | `pad_card` | Combobox, ComboboxPopup |
| `detail_font_size` | TU | `--op-field-detail-font-size` | `type_caption_size` | Combobox, ComboboxPopup |
| `detail_line_height` | TU | `--op-field-detail-line-height` | `type_caption_line` | Combobox, ComboboxPopup |
| `letter_spacing` | TU | `--op-field-letter-spacing` | `0.06em` | Combobox, ComboboxPopup |
| `option_icon_size` | DU | `--op-field-option-icon-size` | `icon_small` (`4 DU`) | Combobox, ComboboxPopup |
| `section_padding_y` | DU | `--op-field-section-padding-y` | `pad_tight` | Combobox, ComboboxPopup |
| `width` | DU | `--op-field-popup-width` | `80 DU` | ComboboxPopup |
| `screen_margin` | DU | `--op-field-screen-margin` | `margin_screen` | ComboboxPopup |

| Color variable | Default |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |
| `--op-field-pressed` | `pressed` |
| `--op-field-secondary-text` | `text_secondary` |
| `--op-field-accent` | `accent` |
| `--op-field-divider` | `divider` |
| `--op-field-input-background` | `surface_alt` |
| `--op-field-popup-border` | `accent` |
| `--op-field-disabled-background` | `disabled_surface` |
| `--op-field-disabled-text` | `text_disabled` |

| Slot | Place |
| :--- | :--- |
| `prepend` | Inside the box before the control. |
| `append` | Inside the box before built-in reveal, save or picker controls. |
| `before` | Outside the box before it. |
| `after` | Outside the box after it. |
| `label_slot` | Replaces the visible label. |
| `hint_slot` | Replaces the hint and error line. |
| `option` | Scoped option content: ComboOption for Combobox, String for DropdownMenu. |
| `empty` | Replaces the empty list message. |
| `before_options` | Before the option rows. |
| `after_options` | After the option rows. |
| `header` | Before the popup options; Calendar: above the month navigation. |
| `footer` | After the popup options; Calendar: below the day grid. |

Resolved row metrics also update virtual scrolling and keyboard reveal geometry.
Popup content inherits the resolved `--op-field-*` variables from the field root.

### Strings

| Key | Prop | Default |
| :--- | :--- | :--- |
| `combobox.placeholder` | `placeholder` | `(empty)` |
| `combobox.label` | `aria_label` | `Choose a value` |
| `combobox.empty` | `empty_text` | `No matches` |
| `combobox.custom-hint` | `custom_hint` | `Press Enter to use this value` |
| `combobox-popup.placeholder` | `placeholder` | `(empty)` |
| `combobox-popup.label` | `aria_label` | `Search` |

### Icons

| Key | Prop | Default |
| :--- | :--- | :--- |
| `combobox.expand` | `icon` | `arrow_drop_down` |
