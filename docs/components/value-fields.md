# Value fields

Typed text fields: NumberField, DurationField, TimeField, DateTimeField and ColorField.
Each one looks like a [TextField](text-field.md) and edits one typed value through its text form.

Status: **draft** on web.

## Use

* NumberField: numbers with an optional unit (`21.5 °C`, `1200 kWh`), optionally empty.
* DurationField: signed durations such as delays and offsets (`1h 30m`, `-15m`).
* TimeField: a time of day (`07:30`).
* DateTimeField: a local date and time (`2026-11-17 07:30`) with a calendar.
* ColorField: a colour as `#rrggbb` with swatches and the system picker.

## Geometry

| Property | Value |
| :--- | :--- |
| Field | As [TextField](text-field.md): `row_standard` (`10 DU`), `pad_card` padding, tabular digits. |
| Unit | After the value, `text_secondary`, `pad_tight` from the text. |
| Picker button | `10 DU` wide, full height, `arrow_drop_down`; ColorField adds a `5 DU` colour preview before the arrow. |
| Calendar | Popup with a `row_compact` header (previous, month and year, next), a weekday row of `row_dense` and `row_compact` square day cells; weeks start on Monday. |
| Swatches | `row_compact` squares, eight per row, with a `0.5 DU` `accent` outline on the selected one; the system picker row follows. |

## Behavior

* Scrolling the page or another container outside the calendar or color popup closes it; scrolling inside it, or inside its own field, does not.
* Typing edits only the text; Enter or leaving the field parses it and commits the value.
* Valid text is re-formatted to the canonical form.
* Invalid text rolls back to the last value and the field pulses the error state, as an error rolls back a change everywhere.
* Escape restores the text of the current value without committing.
* Arrow Up and Down step the value: NumberField by `step` within `min` and `max`, DurationField by `step_ms`, TimeField by `step_s` around midnight.
* External changes of the value show up unless the field has focus.
* The calendar keeps the time of day; the selected day is filled with `accent`, today has an `accent` outline.
* Popups are [Popover](popover.md) layers and close on a press outside or Escape.

## Text forms

| Field | Accepted | Shown |
| :--- | :--- | :--- |
| NumberField | `1.5`, `1,5`, `12 000`, `-3e2` | Up to `decimals` fraction digits, no trailing zeros. |
| DurationField | `1h 30m`, `1.5h`, `90s`, `250ms`, `2d`, `1:30` (minutes and seconds), `1:02:03`, a bare number in `default_unit_ms`, a leading `-` | `-1d 2h 3m 4s 500ms`, zero as `0s`. |
| TimeField | `7:30`, `0730`, `7:30:15`, `9` | `07:30`, with seconds when they are not zero or `seconds` is set. |
| DateTimeField | `2026-11-17 07:30`, `2026-11-17T07:30:15`, a date alone at midnight | `2026-11-17 07:30`. |
| ColorField | `#abc`, `#aabbcc`, the same without `#` | `#aabbcc`. |

Parsing and formatting live in `value_field::parse` and need no date library.
Applications that need localized forms wrap [`ParsedField`](#parsedfield) with their own `parse` and `format`.

## API

| Component | Value | Properties |
| :--- | :--- | :--- |
| `NumberField` | `Option<f64>` | `min`, `max`, `step` (`1`), `decimals` (`3`), `unit`, `allow_empty`. |
| `DurationField` | `i64` milliseconds | `signed` (`true`), `default_unit_ms` (`1000`), `step_ms` (`1000`). |
| `TimeField` | `u32` seconds since midnight | `seconds`, `step_s` (`60`). |
| `DateTimeField` | `LocalDateTime` | `seconds`, `month_names`, `weekday_names`. |
| `ColorField` | `String` `#rrggbb` | `swatches`, `custom_label`. |
| `Calendar` | | `selected`, `on_pick`, `month_names`, `weekday_names`. |

Every field, including ColorField and ParsedField, takes `label`, `placeholder`, `enabled`, `state` and `on_commit`.

## ParsedField

The shared base: `value`, `format` (value to text), `parse` (text to value or `None`), optional `on_step`, `trailing` views and `input_mode`.

### Customization API

Every text-like input uses the public [Field](field.md) shell.
Labels are visible above the box; use `aria_label` for an accessible name alone.

| Property | Type | Default | Components | Description |
| :--- | :--- | :--- | :--- | :--- |
| `placeholder` | `string signal` | string key | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Translatable text; see Strings. |
| `label` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Visible label and accessible name. |
| `hint` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Helper text below the box. |
| `error` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Error text replaces hint and draws the error outline. |
| `variant` | `FieldVariant signal` | outlined | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Field appearance. |
| `aria_label` | `string signal` | string key | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Accessible name when label is unset. |
| `prepend` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Inside the box before the control. |
| `append` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Inside the box before built-in reveal, save or picker controls. |
| `before` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Outside the box before it. |
| `after` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Outside the box after it. |
| `label_slot` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Replaces the visible label. |
| `hint_slot` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Replaces the hint and error line. |
| `height` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Metric override; see below. |
| `padding_x` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Metric override; see below. |
| `slot_size` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Metric override; see below. |
| `gap` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | Metric override; see below. |
| `font_size` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField, Calendar | Metric override; see below. |
| `class` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField, Calendar | Additional root class. |
| `style` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField, Calendar | Additional root style. |
| `unit_padding_x` | `number` | unset | NumberField | Metric override; see below. |
| `unit_gap` | `number` | unset | NumberField | Metric override; see below. |
| `state` | `Signal<Option<ControlState>>` | none | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField | External control-state override. |
| `day_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `hour_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `minute_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `second_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `millisecond_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `picker_label` | `string signal` | string key | DateTimeField, ColorField | Translatable text; see Strings. |
| `popup_label` | `string signal` | string key | DateTimeField, ColorField | Translatable text; see Strings. |
| `icon` | `Option<IconData>` | icon key | DateTimeField, ColorField | Built-in icon override; see Icons. |
| `icon_size` | `number` | unset | DateTimeField, ColorField, Calendar | Metric override; see below. |
| `popup_shadow_y` | `number` | unset | DateTimeField, ColorField | Metric override; see below. |
| `popup_shadow_blur` | `number` | unset | DateTimeField, ColorField | Metric override; see below. |
| `calendar_padding` | `number` | unset | DateTimeField | Metric override; see below. |
| `calendar_cell_size` | `number` | unset | DateTimeField | Metric override; see below. |
| `calendar_header_height` | `number` | unset | DateTimeField | Metric override; see below. |
| `calendar_weekday_height` | `number` | unset | DateTimeField | Metric override; see below. |
| `preview_size` | `number` | unset | ColorField | Metric override; see below. |
| `picker_gap` | `number` | unset | ColorField | Metric override; see below. |
| `picker_padding_x` | `number` | unset | ColorField | Metric override; see below. |
| `popup_gap` | `number` | unset | ColorField | Metric override; see below. |
| `popup_padding` | `number` | unset | ColorField | Metric override; see below. |
| `swatch_size` | `number` | unset | ColorField | Metric override; see below. |
| `custom_height` | `number` | unset | ColorField | Metric override; see below. |
| `custom_padding_x` | `number` | unset | ColorField | Metric override; see below. |
| `custom_label` | `string signal` | string key | ColorField | Translatable text; see Strings. |
| `previous_label` | `string signal` | string key | Calendar | Translatable text; see Strings. |
| `next_label` | `string signal` | string key | Calendar | Translatable text; see Strings. |
| `previous_icon` | `Option<IconData>` | icon key | Calendar | Built-in icon override; see Icons. |
| `next_icon` | `Option<IconData>` | icon key | Calendar | Built-in icon override; see Icons. |
| `padding` | `number` | unset | Calendar | Metric override; see below. |
| `cell_size` | `number` | unset | Calendar | Metric override; see below. |
| `header_height` | `number` | unset | Calendar | Metric override; see below. |
| `weekday_height` | `number` | unset | Calendar | Metric override; see below. |
| `caption_font_size` | `number` | unset | Calendar | Metric override; see below. |
| `line_height` | `number` | unset | Calendar | Metric override; see below. |
| `caption_line_height` | `number` | unset | Calendar | Metric override; see below. |
| `header` | `slot` | none | Calendar | Before the popup options; Calendar: above the month navigation. |
| `footer` | `slot` | none | Calendar | After the popup options; Calendar: below the day grid. |

## Accessibility

* The input carries the label; `aria-invalid` is set while the error pulse shows.
* Picker buttons have labels and `aria-expanded`; the popups have the `dialog` role.

## Customization

`class` and `style` apply to the root; explicit `style` declarations follow metric props and win.
Unset metric props preserve token fallbacks.
All text-like inputs provide the same six field slots.

| Metric prop | Unit | Variable | Default | Components |
| :--- | :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard` | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField |
| `gap` | DU | `--op-field-gap` | `gap_micro` | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | NumberField, DurationField, TimeField, DateTimeField, ColorField, ParsedField, Calendar |
| `unit_padding_x` | DU | `--op-field-unit-padding-x` | `pad_card` | NumberField |
| `unit_gap` | DU | `--op-field-unit-gap` | `pad_tight` | NumberField |
| `icon_size` | DU | `--op-field-icon-size` | `icon_small` (`4 DU`); `icon_field` (`5 DU`) for picker arrows | DateTimeField, ColorField, Calendar |
| `popup_shadow_y` | DU | `--op-field-popup-shadow-y` | `2 DU` | DateTimeField, ColorField |
| `popup_shadow_blur` | DU | `--op-field-popup-shadow-blur` | `6 DU` | DateTimeField, ColorField |
| `calendar_padding` | DU | `--op-field-calendar-padding` | `pad_tight` | DateTimeField |
| `calendar_cell_size` | DU | `--op-field-calendar-cell-size` | `row_compact` | DateTimeField |
| `calendar_header_height` | DU | `--op-field-calendar-header-height` | `row_compact` | DateTimeField |
| `calendar_weekday_height` | DU | `--op-field-calendar-weekday-height` | `row_dense` | DateTimeField |
| `preview_size` | DU | `--op-field-preview-size` | `5 DU` | ColorField |
| `picker_gap` | DU | `--op-field-picker-gap` | `pad_tight` | ColorField |
| `picker_padding_x` | DU | `--op-field-picker-padding-x` | `pad_tight` | ColorField |
| `popup_gap` | DU | `--op-field-popup-gap` | `pad_tight` | ColorField |
| `popup_padding` | DU | `--op-field-popup-padding` | `pad_tight` | ColorField |
| `swatch_size` | DU | `--op-field-swatch-size` | `row_compact` | ColorField |
| `custom_height` | DU | `--op-field-custom-height` | `row_compact` | ColorField |
| `custom_padding_x` | DU | `--op-field-custom-padding-x` | `pad_tight` | ColorField |
| `padding` | DU | `--op-field-calendar-padding` | `pad_tight` | Calendar |
| `cell_size` | DU | `--op-field-calendar-cell-size` | `row_compact` | Calendar |
| `header_height` | DU | `--op-field-calendar-header-height` | `row_compact` | Calendar |
| `weekday_height` | DU | `--op-field-calendar-weekday-height` | `row_dense` | Calendar |
| `caption_font_size` | TU | `--op-field-caption-font-size` | `type_caption_size` | Calendar |
| `line_height` | TU | `--op-field-line-height` | `type_body_line` | Calendar |
| `caption_line_height` | TU | `--op-field-caption-line-height` | `type_caption_line` | Calendar |

| Color variable | Default |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |
| `--op-field-secondary-text` | `text_secondary` |
| `--op-field-popup-border` | `accent` for the popup frame, `border_strong` for the color preview |
| `--op-field-pressed` | `pressed` |
| `--op-field-accent` | `accent` |
| `--op-field-on-accent` | `on_accent` |
| `--op-field-popup-shadow` | `existing black at 35% opacity` |
| `--op-field-swatch-border` | `existing black at 25% opacity` |

| Slot | Place |
| :--- | :--- |
| `prepend` | Inside the box before the control. |
| `append` | Inside the box before built-in reveal, save or picker controls. |
| `before` | Outside the box before it. |
| `after` | Outside the box after it. |
| `label_slot` | Replaces the visible label. |
| `hint_slot` | Replaces the hint and error line. |
| `header` | Before the popup options; Calendar: above the month navigation. |
| `footer` | After the popup options; Calendar: below the day grid. |

ParsedField retains its existing `trailing: Option<ChildrenFn>`; `append` renders before those views.
Picker popups receive root metric overrides and inherited `--op-field-*` variables.
Calendar resolves month and weekday names independently, falling back for missing array entries.
DurationField accepts translated suffixes as well as canonical English forms.

### Strings

| Key | Prop | Default |
| :--- | :--- | :--- |
| `number-field.placeholder` | `placeholder` | `(empty)` |
| `number-field.label` | `aria_label` | `Number` |
| `duration-field.placeholder` | `placeholder` | `(empty)` |
| `duration-field.label` | `aria_label` | `Duration` |
| `duration-field.day-unit` | `day_unit` | `d` |
| `duration-field.hour-unit` | `hour_unit` | `h` |
| `duration-field.minute-unit` | `minute_unit` | `m` |
| `duration-field.second-unit` | `second_unit` | `s` |
| `duration-field.millisecond-unit` | `millisecond_unit` | `ms` |
| `time-field.placeholder` | `placeholder` | `(empty)` |
| `time-field.label` | `aria_label` | `Time` |
| `date-time-field.placeholder` | `placeholder` | `(empty)` |
| `date-time-field.label` | `aria_label` | `Date and time` |
| `date-time-field.pick` | `picker_label` | `Pick a date` |
| `date-time-field.popup` | `popup_label` | `Calendar` |
| `color-field.placeholder` | `placeholder` | `(empty)` |
| `color-field.label` | `aria_label` | `Colour` |
| `color-field.pick` | `picker_label` | `Pick a colour` |
| `color-field.popup` | `popup_label` | `Colours` |
| `color-field.custom` | `custom_label` | `Custom…` |
| `parsed-field.placeholder` | `placeholder` | `(empty)` |
| `parsed-field.label` | `aria_label` | `Value` |
| `calendar.previous` | `previous_label` | `Previous month` |
| `calendar.next` | `next_label` | `Next month` |
| `calendar.january` | `month_names` | `January` |
| `calendar.february` | `month_names` | `February` |
| `calendar.march` | `month_names` | `March` |
| `calendar.april` | `month_names` | `April` |
| `calendar.may` | `month_names` | `May` |
| `calendar.june` | `month_names` | `June` |
| `calendar.july` | `month_names` | `July` |
| `calendar.august` | `month_names` | `August` |
| `calendar.september` | `month_names` | `September` |
| `calendar.october` | `month_names` | `October` |
| `calendar.november` | `month_names` | `November` |
| `calendar.december` | `month_names` | `December` |
| `calendar.monday` | `weekday_names` | `Mo` |
| `calendar.tuesday` | `weekday_names` | `Tu` |
| `calendar.wednesday` | `weekday_names` | `We` |
| `calendar.thursday` | `weekday_names` | `Th` |
| `calendar.friday` | `weekday_names` | `Fr` |
| `calendar.saturday` | `weekday_names` | `Sa` |
| `calendar.sunday` | `weekday_names` | `Su` |

### Icons

| Key | Prop | Default |
| :--- | :--- | :--- |
| `date-time-field.expand` | `icon` | `arrow_drop_down` |
| `color-field.expand` | `icon` | `arrow_drop_down` |
| `calendar.previous` | `previous_icon` | `chevron_left` |
| `calendar.next` | `next_icon` | `chevron_right` |
