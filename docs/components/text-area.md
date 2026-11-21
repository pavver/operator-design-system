# TextArea

Multi-line text input that grows with its content and can be resized by the user.

Status: **reviewed** on web.

## Use

* Notes, messages, descriptions, and other free text longer than one line.
* For one line use [TextField](text-field.md).

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Full width of its container. |
| Padding | `2.5 DU` vertical, `3 DU` horizontal. |
| Height | Grows from `min_rows` to `max_rows` body lines plus padding, then scrolls. |
| Background | `surface`; `disabled_surface` when disabled. |
| Outline | State border, see [states.md](../foundations/states.md). |
| Counter | `caption` style under the end edge, `1 DU` below the field. |
| Resize grip | `2 DU` triangle in the bottom end corner, diagonal hatch lines in `border_strong`; `text_secondary` on hover, `accent` while dragging. The scrollbar stops above it. |

## Behavior

* Focus shows the `Focused` border.
* The height follows the content on every change, including changes made by the caller.
* External `state` shows Pending, Success, or Error on the border, as on [TextField](text-field.md).
* With `max_length` the input stops at the limit and the counter shows `count / max`.
* The counter turns `warning` at 90% of the limit and `error` at the limit.
* By default the height is content-driven.
* Dragging the grip sets the height by hand, from `min_rows` up to `resize_max_rows` lines; the text scrolls when it is longer.
* A hand-set height stays until the user double-clicks the grip, which returns to content-driven height.
* The grip is hidden when the field is disabled.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | string signal | required | Text. |
| `placeholder` | string | empty | Hint shown while empty. |
| `label` | string | none | Visible label above the box and accessible name. |
| `min_rows` | integer | `3` | Initial height in lines. |
| `max_rows` | integer | `8` | Height limit before scrolling. |
| `resizable` | bool | `true` | Shows the resize grip. |
| `resize_max_rows` | integer | `20` | Largest height, in lines, reachable with the grip. |
| `max_length` | integer | none | Character limit with counter. |
| `state` | control state | none | External state. |
| `enabled` | bool | `true` | Enables input. |

### Customization API

Every text-like input uses the public [Field](field.md) shell.
Labels are visible above the box; use `aria_label` for an accessible name alone.

| Property | Type | Default | Components | Description |
| :--- | :--- | :--- | :--- | :--- |
| `placeholder` | `string signal` | string key | TextArea | Translatable text; see Strings. |
| `label` | `string signal` | none | TextArea | Visible label and accessible name. |
| `hint` | `string signal` | none | TextArea | Helper text below the box. |
| `error` | `string signal` | none | TextArea | Error text replaces hint and draws the error outline. |
| `variant` | `FieldVariant signal` | outlined | TextArea | Field appearance. |
| `aria_label` | `string signal` | string key | TextArea | Accessible name when label is unset. |
| `prepend` | `slot` | none | TextArea | Inside the box before the control. |
| `append` | `slot` | none | TextArea | Inside the box before built-in reveal, save or picker controls. |
| `before` | `slot` | none | TextArea | Outside the box before it. |
| `after` | `slot` | none | TextArea | Outside the box after it. |
| `label_slot` | `slot` | none | TextArea | Replaces the visible label. |
| `hint_slot` | `slot` | none | TextArea | Replaces the hint and error line. |
| `height` | `number` | unset | TextArea | Metric override; see below. |
| `padding_x` | `number` | unset | TextArea | Metric override; see below. |
| `slot_size` | `number` | unset | TextArea | Metric override; see below. |
| `gap` | `number` | unset | TextArea | Metric override; see below. |
| `font_size` | `number` | unset | TextArea | Metric override; see below. |
| `class` | `string signal` | none | TextArea | Additional root class. |
| `style` | `string signal` | none | TextArea | Additional root style. |
| `padding_y` | `number` | unset | TextArea | Metric override; see below. |
| `line_height` | `number` | unset | TextArea | Metric override; see below. |
| `grip_size` | `number` | unset | TextArea | Metric override; see below. |
| `caption_font_size` | `number` | unset | TextArea | Metric override; see below. |
| `caption_line_height` | `number` | unset | TextArea | Metric override; see below. |
| `counter_gap` | `number` | unset | TextArea | Metric override; see below. |
| `resize_label` | `string signal` | string key | TextArea | Translatable text; see Strings. |
| `counter_text` | `string signal` | string key | TextArea | Translatable text; see Strings. |
| `state` | `Signal<Option<ControlState>>` | none | TextArea | External control-state override. |

## Customization

`class` and `style` apply to the root; explicit `style` declarations follow metric props and win.
Unset metric props preserve token fallbacks.
All text-like inputs provide the same six field slots.

| Metric prop | Unit | Variable | Default | Components |
| :--- | :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `content height; optional minimum box height` | TextArea |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` | TextArea |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` | TextArea |
| `gap` | DU | `--op-field-gap` | `gap_micro` | TextArea |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | TextArea |
| `padding_y` | DU | `--op-field-padding-y` | `pad_content` | TextArea |
| `line_height` | TU | `--op-field-line-height` | `type_body_line` | TextArea |
| `grip_size` | DU | `--op-field-grip-size` | `gutter` | TextArea |
| `counter_gap` | DU | `--op-field-counter-gap` | `pad_tight` | TextArea |
| `caption_font_size` | TU | `--op-field-caption-font-size` | `type_caption_size` | TextArea |
| `caption_line_height` | TU | `--op-field-caption-line-height` | `type_caption_line` | TextArea |

| Color variable | Default |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |
| `--op-field-disabled-background` | `disabled_surface` |
| `--op-field-accent` | `accent` |
| `--op-field-disabled-text` | `text_disabled` |
| `--op-field-grip` | `border_strong` |
| `--op-field-secondary-text` | `text_secondary` |
| `--op-field-warning` | `warning` |
| `--op-field-error` | `error` |

| Slot | Place |
| :--- | :--- |
| `prepend` | Inside the box before the control. |
| `append` | Inside the box before built-in reveal, save or picker controls. |
| `before` | Outside the box before it. |
| `after` | Outside the box after it. |
| `label_slot` | Replaces the visible label. |
| `hint_slot` | Replaces the hint and error line. |

### Strings

| Key | Prop | Default |
| :--- | :--- | :--- |
| `text-area.placeholder` | `placeholder` | `(empty)` |
| `text-area.label` | `aria_label` | `Text` |
| `text-area.resize` | `resize_label` | `Drag to resize, double-click to fit the text` |
| `text-area.counter` | `counter_text` | `{count} / {max}` |

### Icons

This component has no built-in icons.
