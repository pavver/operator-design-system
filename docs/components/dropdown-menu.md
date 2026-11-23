# DropdownMenu

Selection field that opens a list of options directly below it.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Fills the parent; the open list has exactly the field width. |
| Field height | `row_standard` (`10 DU`), padding `pad_card` (`3 DU`). |
| Label | Optional, above the field, `caption` style, `pad_tight` gap. |
| List | Directly under the field, maximum height `50 DU`, then scrolls. |
| Option | Minimum height `row_control` (`12 DU`), padding `pad_card`. |
| Corner radius | `0 DU`. |

## Colors

| Part | Color |
| :--- | :--- |
| Field | `surface`, `text_primary`, state border. |
| Open field | `Selected` state: `accent` outline. |
| List | `surface` with an `accent` outline; the library [Scrollbar](scrollbar.md). |
| Selected option | `accent` at 8% background, `accent` bold text. |
| Pressed option | `pressed`. |
| Focused option | `0.5 DU` `accent` ring inside, `hover` background unless selected. |

## Behavior

* Scrolling the page or another container outside the list closes it; scrolling inside it, or inside its own field, does not.
* Clicking the field opens or closes the list.
* Clicking outside the list closes it.
* Choosing an option updates the value, calls the callback, and closes the list.
* While `Pending`, the field ignores input and the arrow is replaced by the [delayed spinner](../foundations/states.md#delayed-spinner).
* `Disabled` uses the disabled colors and ignores input.
* The option list is a [VirtualList](virtual-list.md): only the visible options are rendered, so long lists stay fast; opening scrolls the selected option into the middle.
* The list is a [Popover](popover.md) layer as wide as the field, so it is not clipped by scrolling containers.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `options` | list of strings | required | The options. |
| `selected` | read-write string | required | The current option. |
| `state` | `ControlState?` | none | External state override. |
| `enabled` | bool | `true` | Interactivity. |
| `label` | string | none | Label above the field. |
| `on_select` | callback `(String)` | none | Called after a choice. |

### Customization API

Every text-like input uses the public [Field](field.md) shell.
Labels are visible above the box; use `aria_label` for an accessible name alone.

| Property | Type | Default | Components | Description |
| :--- | :--- | :--- | :--- | :--- |
| `state` | `Signal<Option<ControlState>>` | none | DropdownMenu | External control-state override. |
| `label` | `string signal` | none | DropdownMenu | Visible label and accessible name. |
| `hint` | `string signal` | none | DropdownMenu | Helper text below the box. |
| `error` | `string signal` | none | DropdownMenu | Error text replaces hint and draws the error outline. |
| `variant` | `FieldVariant signal` | outlined | DropdownMenu | Field appearance. |
| `aria_label` | `string signal` | string key | DropdownMenu | Accessible name when label is unset. |
| `prepend` | `slot` | none | DropdownMenu | Inside the box before the control. |
| `append` | `slot` | none | DropdownMenu | Inside the box before built-in reveal, save or picker controls. |
| `before` | `slot` | none | DropdownMenu | Outside the box before it. |
| `after` | `slot` | none | DropdownMenu | Outside the box after it. |
| `label_slot` | `slot` | none | DropdownMenu | Replaces the visible label. |
| `hint_slot` | `slot` | none | DropdownMenu | Replaces the hint and error line. |
| `height` | `number` | unset | DropdownMenu | Metric override; see below. |
| `padding_x` | `number` | unset | DropdownMenu | Metric override; see below. |
| `slot_size` | `number` | unset | DropdownMenu | Metric override; see below. |
| `gap` | `number` | unset | DropdownMenu | Metric override; see below. |
| `font_size` | `number` | unset | DropdownMenu | Metric override; see below. |
| `class` | `string signal` | none | DropdownMenu | Additional root class. |
| `style` | `string signal` | none | DropdownMenu | Additional root style. |
| `icon` | `Option<IconData>` | icon key | DropdownMenu | Built-in icon override; see Icons. |
| `icon_size` | `number` | unset | DropdownMenu | Metric override; see below. |
| `row_height` | `number` | unset | DropdownMenu | Metric override; see below. |
| `max_height` | `number` | unset | DropdownMenu | Metric override; see below. |
| `option_padding_x` | `number` | unset | DropdownMenu | Metric override; see below. |
| `trigger_gap` | `number` | unset | DropdownMenu | Metric override; see below. |
| `option` | `Option<ScopedSlot<String>>` | none | DropdownMenu | Scoped option content: ComboOption for Combobox, String for DropdownMenu. |
| `empty` | `slot` | none | DropdownMenu | Replaces the empty list message. |
| `before_options` | `slot` | none | DropdownMenu | Before the option rows. |
| `after_options` | `slot` | none | DropdownMenu | After the option rows. |
| `header` | `slot` | none | DropdownMenu | Before the popup options; Calendar: above the month navigation. |
| `footer` | `slot` | none | DropdownMenu | After the popup options; Calendar: below the day grid. |
| `loading_label` | string signal | `dropdown-menu.loading` | DropdownMenu | Accessible loading label. |
| `spinner_size`, `spinner_stroke` | number | unset | DropdownMenu | Spinner size and stroke in DU. |

## Customization

`class` and `style` apply to the root; explicit `style` declarations follow metric props and win.
Unset metric props preserve token fallbacks.
All text-like inputs provide the same six field slots.

| Metric prop | Unit | Variable | Default | Components |
| :--- | :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard` | DropdownMenu |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` | DropdownMenu |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` | DropdownMenu |
| `gap` | DU | `--op-field-gap` | `gap_micro` | DropdownMenu |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | DropdownMenu |
| `icon_size` | DU | `--op-field-icon-size` | `icon_small` (`4 DU`); `icon_field` (`5 DU`) for picker arrows | DropdownMenu |
| `row_height` | DU | `--op-field-row-height` | `row_control` | DropdownMenu |
| `max_height` | DU | `--op-field-max-height` | `50 DU` | DropdownMenu |
| `option_padding_x` | DU | `--op-field-option-padding-x` | `pad_card` | DropdownMenu |
| `trigger_gap` | DU | `--op-field-trigger-gap` | `gutter` | DropdownMenu |
| `spinner_size` | DU | `--op-field-spinner-size` | `icon_small` (`4 DU`) | DropdownMenu |
| `spinner_stroke` | DU | `--op-field-spinner-stroke` | `spinner_stroke` | DropdownMenu |

| Color variable | Default |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |
| `--op-field-pending` | `pending` |
| `--op-field-pressed` | `pressed` |
| `--op-field-disabled-background` | `disabled_surface` |
| `--op-field-disabled-text` | `text_disabled` |
| `--op-field-secondary-text` | `text_secondary` |
| `--op-field-accent` | `accent` |

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
| `dropdown-menu.loading` | `loading_label` | `Loading` |
| `dropdown-menu.label` | `aria_label` | `Choose a value` |

### Icons

| Key | Prop | Default |
| :--- | :--- | :--- |
| `dropdown-menu.expand` | `icon` | `arrow_drop_down` |
