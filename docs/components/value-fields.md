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

Every field also takes `label`, `placeholder`, `enabled`, `state` and `on_commit`.

## ParsedField

The shared base: `value`, `format` (value to text), `parse` (text to value or `None`), optional `on_step`, `trailing` views and `input_mode`.

## Accessibility

* The input carries the label; `aria-invalid` is set while the error pulse shows.
* Picker buttons have labels and `aria-expanded`; the popups have the `dialog` role.
