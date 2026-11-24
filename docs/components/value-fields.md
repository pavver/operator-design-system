# Value fields

Typed text fields: NumberField, DurationField, TimeField, DateTimeField, DateRangeField and ColorField, with the ColorPicker, ClockPicker and DurationPicker editors.
Each one looks like a [TextField](text-field.md) and edits one typed value through its text form.

Status: **draft** on web.

## Use

* NumberField: numbers with an optional unit (`21.5 °C`, `1200 kWh`), optionally empty.
* DurationField: signed durations such as delays and offsets (`1h 30m`, `-15m`) with a stepper popup.
* DurationPicker: one column per unit with increase and decrease buttons, for panels and dialogs.
* TimeField: a time of day (`07:30`) with a clock dial.
* ClockPicker: the clock dial alone: drag the hand to the hour, then to the minute.
* DateTimeField: a local date and time (`2026-11-17 07:30`) with a calendar and a clock dial.
* DateRangeField: a range of days such as `12 Jul – 5 Nov 2026` with quick ranges and two linked months.
* ColorField: a colour as `#rrggbb` (or `#rrggbbaa` with opacity) with a ColorPicker popup.
* ColorPicker: the colour editor alone, for panels and dialogs: spectrum, RGB, HSL and colour temperature modes, opacity, a colour code and swatches.

## Geometry

| Property | Value |
| :--- | :--- |
| Field | As [TextField](text-field.md): `row_standard` (`10 DU`), `pad_card` padding, tabular digits. |
| Steppers | NumberField `steppers` adds − and + buttons `slot_size` wide after the unit, with `icon_field` icons; a button at its bound turns off. |
| Unit | After the value, `text_secondary`, `pad_tight` from the text. |
| Picker button | `10 DU` wide, full height, `arrow_drop_down`; ColorField adds an `icon_field` (`5 DU`) colour preview before the arrow. |
| Calendar | Popup with a `row_compact` header (previous, month and year, next), a weekday row of `row_dense` and `row_compact` square day cells; weeks start on Monday. |
| ColorPicker | `80 DU` wide, `pad_content` padding, `gutter` between rows; a row of mode tabs `row_compact` high with a `0.5 DU` `accent` underline on the selected one. |
| Date range popup | A `36 DU` column of `row_compact` quick ranges with a `0.5 DU` `accent` start stripe on the matching one and a divider, then two calendars side by side and a caption hint; the calendars wrap below each other when the screen is narrow. |
| DurationPicker | `pad_content` padding; `16 DU` columns `gutter` apart, each a `row_compact` increase button, a `row_control` number in `type_section_title_size`, a caption and a `row_compact` decrease button; signed pickers start with a `row_standard` sign switch. |
| Date and time popup | The calendar, then a `48 DU` ClockPicker after a divider; they wrap below each other when the screen is narrow. |
| Month and year grids | Three columns and four rows filling the day grid, with the same `accent` fill for the selected month or year and outline for the current one. |
| Range days | The ends are filled with `accent`; the days between are tinted with `accent` at 22%. |
| ClockPicker | `pad_content` padding; a readout of two `row_avatar` buttons in `type_screen_title_size` with a `0.5 DU` `accent` underline on the active hand, an optional AM/PM pair; a `64 DU` round dial in `surface_alt` with `row_compact` numbers, a `0.5 DU` `accent` hand and a round `row_compact` tip. |
| Spectrum plane | `40 DU` high, saturation across and brightness up; an `icon_small` (`4 DU`) square marker with a `0.5 DU` light ring and a `0.25 DU` dark outline. |
| Channel row | A caption, a `row_dense` bar with a `pad_small` marker that overhangs it by `gap_micro`, and a `14 DU` number input `row_compact` high; opacity bars show a checkerboard. |
| Colour code | A `row_compact` input with an `icon_field` (`5 DU`) sample, the code, and the CSS form of the current mode. |
| Swatches | `row_compact` squares filling the width, `gap_micro` apart, with a `0.5 DU` `accent` outline on the selected one. |

## Geometry exceptions

The ClockPicker dial, its centre, the hand tip and the selected number are round; everything else keeps `0 DU` corners.

## Behavior

* Scrolling the page or another container outside the calendar or colour popup closes it; scrolling inside it, or inside its own field, does not.
* Typing edits only the text; Enter or leaving the field parses it and commits the value.
* Valid text is re-formatted to the canonical form.
* Invalid text rolls back to the last value and the field pulses the error state, as an error rolls back a change everywhere.
* Escape restores the text of the current value without committing.
* NumberField − and + buttons step like Arrow Down and Up and commit at once.
* Arrow Up and Down step the value: NumberField by `step` within `min` and `max`, DurationField by `step_ms`, TimeField by `step_s` around midnight.
* External changes of the value show up unless the field has focus.
* The calendar title opens a grid of months, and again a grid of twelve years; picking a year returns to its months and picking a month returns to its days; the arrows move by a month, a year or twelve years.
* In DateTimeField a picked day keeps the popup open for the dial; the dial changes the time live, and releasing the minute hand closes the popup; with `no_clock` a picked day closes it.
* The calendar keeps the time of day; the selected day is filled with `accent`, today has an `accent` outline.
* Popups are [Popover](popover.md) layers and close on a press outside or Escape.
* DurationPicker buttons and Arrow Up and Down in a column move the whole duration by one unit, so minutes carry into hours; they change the magnitude and keep the sign, which the sign switch flips; unsigned pickers stop at zero.
* Typing a number into a column replaces that unit; the largest shown unit holds everything above it and units below the smallest shown one are dropped.
* DateRangeField: the first press on a day starts the range, the pointer or focus previews it, and the second press ends it, commits and closes the popup; the two days may come in any order.
* A quick range commits and closes at once; the one equal to the value is marked.
* Opening the popup shows the month of the range start and the next one; their arrows move both together.
* ClockPicker sets the hour first: dragging moves the hand and the value live, releasing commits it and switches to minutes; releasing the minute hand commits and runs `on_done`, which closes the TimeField popup.
* In 24-hour mode the outer ring holds `12` and `1`–`11` and the inner ring `00` and `13`–`23`; with `hour12` the dial shows `12` and `1`–`11` and AM/PM switches the half of the day.
* Minutes snap to `minute_step`; seconds of the value are kept.
* On the dial, arrows step the active hand by one hour or one `minute_step`, Page keys by five, and Enter moves from hours to minutes or finishes.
* ColorPicker updates the value live while the plane or a bar is dragged and runs `on_commit` when the edit ends: on release, Enter in a number or the code, a swatch, or a key on a bar.
* The plane moves saturation with Arrow Left and Right and brightness with Arrow Up and Down; bars step with arrows, by ten steps with Shift or Page keys, and jump with Home and End.
* Greys and black keep the hue and saturation they had, so dragging through them does not reset the other bars.
* The temperature bar runs from `min_kelvin` to `max_kelvin` along the black-body curve; while the colour comes from it, `temperature` holds the kelvin value, and any other edit sets it to `None`.
* Without `alpha` the value is always opaque; with it, an opaque colour stays `#rrggbb` and a translucent one becomes `#rrggbbaa`.
* Invalid text in a number or the code input restores the shown value.

## Text forms

| Field | Accepted | Shown |
| :--- | :--- | :--- |
| NumberField | `1.5`, `1,5`, `12 000`, `-3e2` | Up to `decimals` fraction digits, no trailing zeros. |
| DurationField | `1h 30m`, `1.5h`, `90s`, `250ms`, `2d`, `1:30` (minutes and seconds), `1:02:03`, a bare number in `default_unit_ms`, a leading `-` | `-1d 2h 3m 4s 500ms`, zero as `0s`. |
| TimeField | `7:30`, `0730`, `7:30:15`, `9` | `07:30`, with seconds when they are not zero or `seconds` is set. |
| DateTimeField | `2026-11-17 07:30`, `2026-11-17T07:30:15`, a date alone at midnight | `2026-11-17 07:30`. |
| DateRangeField | `12 Jul – 5 Nov 2026`, `3 – 10 Aug 2026`, `12 липня – 5 листопада`, `2026-07-12..2026-11-05`, `12.07 - 5.11.2026`, one day; joined by `–`, `—`, ` - `, `..` or ` to `; month names match by their first three letters; a missing end year is this year and a missing start year is the end's (a year earlier if the start would follow the end) | `12 Jul – 5 Nov 2026`, `3 – 10 Aug 2026`, `20 Dec 2026 – 3 Jan 2027`, `5 Nov 2026`. |
| ColorField, ColorPicker code | `#abc`, `#abcd`, `#aabbcc`, `#aabbccdd`, the same without `#`, `rgb()`, `rgba()`, `hsl()`, `hsla()`, `hsv()`, a temperature such as `2700K` | `#aabbcc`, or `#aabbccdd` with `alpha` when translucent. |

Parsing and formatting live in `value_field::parse` and `value_field::color_math` and need no date or colour library.
Applications that need localized forms wrap [`ParsedField`](#parsedfield) with their own `parse` and `format`.

## API

| Component | Value | Properties |
| :--- | :--- | :--- |
| `NumberField` | `Option<f64>` | `min`, `max`, `step` (`1`), `decimals` (`3`), `unit`, `allow_empty`, `steppers`. |
| `DurationField` | `i64` milliseconds | `signed` (`true`), `default_unit_ms` (`1000`), `step_ms` (`1000`), `no_picker`, `picker_days`, `picker_seconds` (`true`), `picker_milliseconds`. |
| `DurationPicker` | `i64` milliseconds | `signed`, `days`, `seconds` (`true`), `milliseconds`, `enabled`, `on_commit`. |
| `TimeField` | `u32` seconds since midnight | `seconds`, `step_s` (`60`), `hour12`, `minute_step` (`1`), `no_picker`. |
| `ClockPicker` | `u32` seconds since midnight | `hour12`, `minute_step` (`1`), `enabled`, `on_commit`, `on_done`. |
| `DateTimeField` | `LocalDateTime` | `seconds`, `no_clock`, `hour12`, `minute_step` (`1`), `month_names`, `weekday_names`. |
| `ColorField` | `String` `#rrggbb` or `#rrggbbaa` | `swatches`, `alpha`, `modes`, `min_kelvin` (`2000`), `max_kelvin` (`6500`), `temperature`. |
| `ColorPicker` | `String` `#rrggbb` or `#rrggbbaa` | `swatches`, `alpha`, `modes` (`ColorMode::Spectrum`, `Rgb`, `Hsl`, `Temperature`; empty means all), `min_kelvin`, `max_kelvin`, `temperature: Option<RwSignal<Option<u32>>>`, `enabled`, `on_commit`. |
| `DateRangeField` | `Option<DateRange>` | `presets` (`DateRangePreset::defaults()`), `allow_empty`, `short_month_names`, `month_names`, `weekday_names`. |
| `Calendar` | | `selected`, `on_pick`, `range`, `on_hover`, `month`, `hide_previous`, `hide_next`, `month_names`, `weekday_names`. |

`DateRange` holds inclusive `start` and `end` days as `LocalDate` `(year, month, day)`; `DateRangePreset` covers today, yesterday, this and last week, month and year, the last `n` days and custom ranges.

Every field, including ColorField and ParsedField, takes `label`, `placeholder`, `enabled`, `state` and `on_commit`.

## ParsedField

The shared base: `value`, `format` (value to text), `parse` (text to value or `None`), optional `on_step`, `trailing` views and `input_mode`.

### Customization API

Every text-like input uses the public [Field](field.md) shell.
Labels are visible above the box; use `aria_label` for an accessible name alone.

| Property | Type | Default | Components | Description |
| :--- | :--- | :--- | :--- | :--- |
| `placeholder` | `string signal` | string key | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Translatable text; see Strings. |
| `label` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Visible label and accessible name. |
| `hint` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Helper text below the box. |
| `error` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Error text replaces hint and draws the error outline. |
| `variant` | `FieldVariant signal` | outlined | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Field appearance. |
| `aria_label` | `string signal` | string key | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Accessible name when label is unset. |
| `prepend` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Inside the box before the control. |
| `append` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Inside the box before built-in reveal, save or picker controls. |
| `before` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Outside the box before it. |
| `after` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Outside the box after it. |
| `label_slot` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Replaces the visible label. |
| `hint_slot` | `slot` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Replaces the hint and error line. |
| `height` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Metric override; see below. |
| `padding_x` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Metric override; see below. |
| `slot_size` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Metric override; see below. |
| `gap` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Metric override; see below. |
| `font_size` | `number` | unset | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField, Calendar | Metric override; see below. |
| `class` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField, Calendar | Additional root class. |
| `style` | `string signal` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField, Calendar | Additional root style. |
| `increase_label` | `string signal` | string key | NumberField | Translatable text; see Strings. |
| `decrease_label` | `string signal` | string key | NumberField | Translatable text; see Strings. |
| `increase_icon` | `Option<IconData>` | icon key | NumberField | Built-in icon override; see Icons. |
| `decrease_icon` | `Option<IconData>` | icon key | NumberField | Built-in icon override; see Icons. |
| `unit_padding_x` | `number` | unset | NumberField | Metric override; see below. |
| `unit_gap` | `number` | unset | NumberField | Metric override; see below. |
| `state` | `Signal<Option<ControlState>>` | none | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | External control-state override. |
| `day_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `hour_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `minute_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `second_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `millisecond_unit` | `string signal` | string key | DurationField | Translatable text; see Strings. |
| `picker_label` | `string signal` | string key | DateTimeField, DateRangeField, ColorField, TimeField, DurationField | Translatable text; see Strings. |
| `popup_label` | `string signal` | string key | DateTimeField, DateRangeField, ColorField, TimeField, DurationField | Translatable text; see Strings. |
| `icon` | `Option<IconData>` | icon key | DateTimeField, DateRangeField, ColorField, TimeField, DurationField | Built-in icon override; see Icons. |
| `icon_size` | `number` | unset | DateTimeField, DateRangeField, ColorField, TimeField, DurationField, Calendar | Metric override; see below. |
| `popup_shadow_y` | `number` | unset | DateTimeField, DateRangeField, ColorField, TimeField, DurationField | Metric override; see below. |
| `popup_shadow_blur` | `number` | unset | DateTimeField, DateRangeField, ColorField, TimeField, DurationField | Metric override; see below. |
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
| `picker_width` | `number` | unset | ColorField | Metric override; see below. |
| `plane_height` | `number` | unset | ColorField | Metric override; see below. |
| `bar_height` | `number` | unset | ColorField | Metric override; see below. |
| `spectrum_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `rgb_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `hsl_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `temperature_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `plane_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `hue_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `saturation_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `brightness_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `lightness_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `red_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `green_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `blue_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `alpha_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `hex_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `swatches_label` | `string signal` | string key | ColorPicker | Translatable text; see Strings. |
| `width` | `number` | unset | ColorPicker | Metric override; see below. |
| `padding` | `number` | unset | ColorPicker | Metric override; see below. |
| `gap` | `number` | unset | ColorPicker | Metric override; see below. |
| `plane_height` | `number` | unset | ColorPicker | Metric override; see below. |
| `bar_height` | `number` | unset | ColorPicker | Metric override; see below. |
| `thumb_size` | `number` | unset | ColorPicker | Metric override; see below. |
| `tab_height` | `number` | unset | ColorPicker | Metric override; see below. |
| `number_width` | `number` | unset | ColorPicker | Metric override; see below. |
| `swatch_size` | `number` | unset | ColorPicker | Metric override; see below. |
| `font_size` | `number` | unset | ColorPicker | Metric override; see below. |
| `header` | `slot` | none | ColorPicker | Above the mode tabs. |
| `footer` | `slot` | none | ColorPicker | Below the swatches. |
| `presets_label` | `string signal` | string key | DateRangeField | Translatable text; see Strings. |
| `start_hint` | `string signal` | string key | DateRangeField | Translatable text; see Strings. |
| `end_hint` | `string signal` | string key | DateRangeField | Translatable text; see Strings. |
| `calendar_cell_size` | `number` | unset | DateRangeField | Metric override; see below. |
| `presets_width` | `number` | unset | DateRangeField | Metric override; see below. |
| `preset_height` | `number` | unset | DateRangeField | Metric override; see below. |
| `days_label` | `string signal` | string key | DurationPicker | Translatable text; see Strings. |
| `hours_label` | `string signal` | string key | DurationPicker | Translatable text; see Strings. |
| `minutes_label` | `string signal` | string key | DurationPicker | Translatable text; see Strings. |
| `seconds_label` | `string signal` | string key | DurationPicker | Translatable text; see Strings. |
| `milliseconds_label` | `string signal` | string key | DurationPicker | Translatable text; see Strings. |
| `increase_label` | `string signal` | string key | DurationPicker | Translatable text; see Strings. |
| `decrease_label` | `string signal` | string key | DurationPicker | Translatable text; see Strings. |
| `negative_label` | `string signal` | string key | DurationPicker | Translatable text; see Strings. |
| `increase_icon` | `Option<IconData>` | icon key | DurationPicker | Built-in icon override; see Icons. |
| `decrease_icon` | `Option<IconData>` | icon key | DurationPicker | Built-in icon override; see Icons. |
| `padding` | `number` | unset | DurationPicker | Metric override; see below. |
| `gap` | `number` | unset | DurationPicker | Metric override; see below. |
| `column_width` | `number` | unset | DurationPicker | Metric override; see below. |
| `button_height` | `number` | unset | DurationPicker | Metric override; see below. |
| `number_height` | `number` | unset | DurationPicker | Metric override; see below. |
| `icon_size` | `number` | unset | DurationPicker | Metric override; see below. |
| `font_size` | `number` | unset | DurationPicker | Metric override; see below. |
| `header` | `slot` | none | DurationPicker | Above the columns. |
| `footer` | `slot` | none | DurationPicker | Below the columns. |
| `clock_size` | `number` | unset | TimeField | Metric override; see below. |
| `hours_label` | `string signal` | string key | ClockPicker | Translatable text; see Strings. |
| `minutes_label` | `string signal` | string key | ClockPicker | Translatable text; see Strings. |
| `am_label` | `string signal` | string key | ClockPicker | Translatable text; see Strings. |
| `pm_label` | `string signal` | string key | ClockPicker | Translatable text; see Strings. |
| `size` | `number` | unset | ClockPicker | Metric override; see below. |
| `padding` | `number` | unset | ClockPicker | Metric override; see below. |
| `gap` | `number` | unset | ClockPicker | Metric override; see below. |
| `number_size` | `number` | unset | ClockPicker | Metric override; see below. |
| `hand_width` | `number` | unset | ClockPicker | Metric override; see below. |
| `readout_font_size` | `number` | unset | ClockPicker | Metric override; see below. |
| `font_size` | `number` | unset | ClockPicker | Metric override; see below. |
| `header` | `slot` | none | ClockPicker | Above the readout. |
| `footer` | `slot` | none | ClockPicker | Below the dial. |
| `previous_label` | `string signal` | string key | Calendar | Translatable text; see Strings. |
| `next_label` | `string signal` | string key | Calendar | Translatable text; see Strings. |
| `jump_label` | `string signal` | string key | Calendar | Translatable text; see Strings. |
| `previous_year_label` | `string signal` | string key | Calendar | Translatable text; see Strings. |
| `next_year_label` | `string signal` | string key | Calendar | Translatable text; see Strings. |
| `previous_years_label` | `string signal` | string key | Calendar | Translatable text; see Strings. |
| `next_years_label` | `string signal` | string key | Calendar | Translatable text; see Strings. |
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
* DateRangeField quick ranges are a labelled `listbox`; the hint under the calendars is a polite live region.
* ColorPicker modes are a `tablist`; the plane and the bars are `slider`s with `aria-valuetext`; number and code inputs carry the channel names; swatches are a labelled `listbox`.

## Customization

`class` and `style` apply to the root; explicit `style` declarations follow metric props and win.
Unset metric props preserve token fallbacks.
All text-like inputs provide the same six field slots.

| Metric prop | Unit | Variable | Default | Components |
| :--- | :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField |
| `gap` | DU | `--op-field-gap` | `gap_micro` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField, Calendar |
| `unit_padding_x` | DU | `--op-field-unit-padding-x` | `pad_card` | NumberField |
| `unit_gap` | DU | `--op-field-unit-gap` | `pad_tight` | NumberField |
| `icon_size` | DU | `--op-field-icon-size` | `icon_small` (`4 DU`); `icon_field` (`5 DU`) for picker arrows | DateTimeField, DateRangeField, ColorField, TimeField, DurationField, Calendar |
| `popup_shadow_y` | DU | `--op-field-popup-shadow-y` | `2 DU` | DateTimeField, DateRangeField, ColorField, TimeField, DurationField |
| `popup_shadow_blur` | DU | `--op-field-popup-shadow-blur` | `6 DU` | DateTimeField, DateRangeField, ColorField, TimeField, DurationField |
| `calendar_padding` | DU | `--op-field-calendar-padding` | `pad_tight` | DateTimeField |
| `calendar_cell_size` | DU | `--op-field-calendar-cell-size` | `row_compact` | DateTimeField, DateRangeField |
| `calendar_header_height` | DU | `--op-field-calendar-header-height` | `row_compact` | DateTimeField |
| `calendar_weekday_height` | DU | `--op-field-calendar-weekday-height` | `row_dense` | DateTimeField |
| `preview_size` | DU | `--op-field-preview-size` | `icon_field` (`5 DU`) | ColorField |
| `picker_gap` | DU | `--op-field-picker-gap` | `pad_tight` | ColorField |
| `picker_padding_x` | DU | `--op-field-picker-padding-x` | `pad_tight` | ColorField |
| `popup_gap`, `gap` | DU | `--op-field-popup-gap` | `gutter` | ColorField, ColorPicker |
| `popup_padding`, `padding` | DU | `--op-field-popup-padding` | `pad_content` | ColorField, ColorPicker |
| `swatch_size` | DU | `--op-field-swatch-size` | `row_compact` | ColorField, ColorPicker |
| `picker_width`, `width` | DU | `--op-field-color-width` | `80 DU` | ColorField, ColorPicker |
| `plane_height` | DU | `--op-field-color-plane-height` | `40 DU` | ColorField, ColorPicker |
| `bar_height` | DU | `--op-field-color-bar-height` | `row_dense` | ColorField, ColorPicker |
| `thumb_size` | DU | `--op-field-color-thumb-size` | `icon_small` (`4 DU`) | ColorPicker |
| `tab_height` | DU | `--op-field-color-tab-height` | `row_compact` | ColorPicker |
| `number_width` | DU | `--op-field-color-number-width` | `14 DU` | ColorPicker |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | ColorPicker |
| `presets_width` | DU | `--op-field-range-presets-width` | `36 DU` | DateRangeField |
| `preset_height` | DU | `--op-field-range-preset-height` | `row_compact` | DateRangeField |
| `column_width` | DU | `--op-field-duration-column-width` | `16 DU` | DurationPicker |
| `button_height` | DU | `--op-field-duration-button-height` | `row_compact` | DurationPicker |
| `number_height` | DU | `--op-field-duration-number-height` | `row_control` | DurationPicker |
| `clock_size`, `size` | DU | `--op-field-clock-size` | `64 DU` | TimeField, ClockPicker |
| `number_size` | DU | `--op-field-clock-number-size` | `row_compact` | ClockPicker |
| `hand_width` | DU | `--op-field-clock-hand-width` | `border_thick` | ClockPicker |
| `readout_font_size` | TU | `--op-field-clock-readout-font-size` | `type_screen_title_size` | ClockPicker |
| `padding`, `gap`, `font_size` | DU, DU, TU | `--op-field-popup-padding`, `--op-field-popup-gap`, `--op-field-font-size` | `pad_content`, `gutter`, `type_body_size` | ClockPicker |
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
| `--op-field-surface-alt` | `surface_alt` for the ClockPicker dial and readout |
| `--op-field-range` | `accent` at 22% for the days inside a range |
| `--op-field-border` | `border` for ColorPicker number and code inputs |
| `--op-field-hover` | `hover` for ColorPicker tabs |
| `--op-field-divider` | `divider` under ColorPicker tabs |
| `--op-field-checker` | `border` for the checkerboard under translucent colours |
| `--op-field-color-thumb` | `on_media` for the plane and bar markers |
| `--op-field-color-thumb-outline` | `shadow` around the plane and bar markers |

| Slot | Place |
| :--- | :--- |
| `prepend` | Inside the box before the control. |
| `append` | Inside the box before built-in reveal, save or picker controls. |
| `before` | Outside the box before it. |
| `after` | Outside the box after it. |
| `label_slot` | Replaces the visible label. |
| `hint_slot` | Replaces the hint and error line. |
| `header` | Before the popup options; Calendar: above the month navigation. |
| `footer` | After the popup options; Calendar: below the day grid; ColorPicker: below the swatches. |

ParsedField retains its existing `trailing: Option<ChildrenFn>`; `append` renders before those views.
Picker popups receive root metric overrides and inherited `--op-field-*` variables.
Calendar resolves month and weekday names independently, falling back for missing array entries.
DurationField accepts translated suffixes as well as canonical English forms.

### Strings

| Key | Prop | Default |
| :--- | :--- | :--- |
| `number-field.increase` | `increase_label` | `Increase` |
| `number-field.decrease` | `decrease_label` | `Decrease` |
| `number-field.placeholder` | `placeholder` | `(empty)` |
| `number-field.label` | `aria_label` | `Number` |
| `duration-field.placeholder` | `placeholder` | `(empty)` |
| `duration-field.label` | `aria_label` | `Duration` |
| `duration-field.day-unit` | `day_unit` | `d` |
| `duration-field.hour-unit` | `hour_unit` | `h` |
| `duration-field.minute-unit` | `minute_unit` | `m` |
| `duration-field.second-unit` | `second_unit` | `s` |
| `duration-field.millisecond-unit` | `millisecond_unit` | `ms` |
| `duration-field.pick` | `picker_label` | `Pick a duration` |
| `duration-field.popup` | `popup_label` | `Duration` |
| `duration-picker.days` | `days_label` | `Days` |
| `duration-picker.hours` | `hours_label` | `Hours` |
| `duration-picker.minutes` | `minutes_label` | `Minutes` |
| `duration-picker.seconds` | `seconds_label` | `Seconds` |
| `duration-picker.milliseconds` | `milliseconds_label` | `Milliseconds` |
| `duration-picker.increase` | `increase_label` | `Increase {unit}` |
| `duration-picker.decrease` | `decrease_label` | `Decrease {unit}` |
| `duration-picker.negative` | `negative_label` | `Negative` |
| `time-field.placeholder` | `placeholder` | `(empty)` |
| `time-field.label` | `aria_label` | `Time` |
| `time-field.pick` | `picker_label` | `Pick a time` |
| `time-field.popup` | `popup_label` | `Clock` |
| `clock-picker.hours` | `hours_label` | `Hours` |
| `clock-picker.minutes` | `minutes_label` | `Minutes` |
| `clock-picker.am` | `am_label` | `AM` |
| `clock-picker.pm` | `pm_label` | `PM` |
| `date-range-field.placeholder` | `placeholder` | `(empty)` |
| `date-range-field.label` | `aria_label` | `Date range` |
| `date-range-field.pick` | `picker_label` | `Pick dates` |
| `date-range-field.popup` | `popup_label` | `Date range` |
| `date-range-field.presets` | `presets_label` | `Quick ranges` |
| `date-range-field.start-hint` | `start_hint` | `Pick the first day` |
| `date-range-field.end-hint` | `end_hint` | `Pick the last day` |
| `date-range-field.months` | `short_month_names` | `Jan,Feb,Mar,Apr,May,Jun,Jul,Aug,Sep,Oct,Nov,Dec` |
| `date-range-field.today` | `presets` | `Today` |
| `date-range-field.yesterday` | `presets` | `Yesterday` |
| `date-range-field.this-week` | `presets` | `This week` |
| `date-range-field.last-week` | `presets` | `Last week` |
| `date-range-field.this-month` | `presets` | `This month` |
| `date-range-field.last-month` | `presets` | `Last month` |
| `date-range-field.last-days` | `presets` | `Last {n} days` |
| `date-range-field.this-year` | `presets` | `This year` |
| `date-range-field.last-year` | `presets` | `Last year` |
| `date-time-field.placeholder` | `placeholder` | `(empty)` |
| `date-time-field.label` | `aria_label` | `Date and time` |
| `date-time-field.pick` | `picker_label` | `Pick a date` |
| `date-time-field.popup` | `popup_label` | `Calendar` |
| `color-field.placeholder` | `placeholder` | `(empty)` |
| `color-field.label` | `aria_label` | `Colour` |
| `color-field.pick` | `picker_label` | `Pick a colour` |
| `color-field.popup` | `popup_label` | `Colours` |
| `color-picker.spectrum` | `spectrum_label` | `Spectrum` |
| `color-picker.rgb` | `rgb_label` | `RGB` |
| `color-picker.hsl` | `hsl_label` | `HSL` |
| `color-picker.temperature` | `temperature_label` | `Temperature` |
| `color-picker.plane` | `plane_label` | `Saturation and brightness` |
| `color-picker.hue` | `hue_label` | `Hue` |
| `color-picker.saturation` | `saturation_label` | `Saturation` |
| `color-picker.brightness` | `brightness_label` | `Brightness` |
| `color-picker.lightness` | `lightness_label` | `Lightness` |
| `color-picker.red` | `red_label` | `Red` |
| `color-picker.green` | `green_label` | `Green` |
| `color-picker.blue` | `blue_label` | `Blue` |
| `color-picker.alpha` | `alpha_label` | `Opacity` |
| `color-picker.hex` | `hex_label` | `Colour code` |
| `color-picker.swatches` | `swatches_label` | `Presets` |
| `parsed-field.placeholder` | `placeholder` | `(empty)` |
| `parsed-field.label` | `aria_label` | `Value` |
| `calendar.previous` | `previous_label` | `Previous month` |
| `calendar.next` | `next_label` | `Next month` |
| `calendar.jump` | `jump_label` | `Choose month and year` |
| `calendar.previous-year` | `previous_year_label` | `Previous year` |
| `calendar.next-year` | `next_year_label` | `Next year` |
| `calendar.previous-years` | `previous_years_label` | `Previous years` |
| `calendar.next-years` | `next_years_label` | `Next years` |
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
| `number-field.increase` | `increase_icon` | `add` |
| `number-field.decrease` | `decrease_icon` | `remove` |
| `duration-field.expand` | `icon` | `arrow_drop_down` |
| `duration-picker.increase` | `increase_icon` | `add` |
| `duration-picker.decrease` | `decrease_icon` | `remove` |
| `time-field.expand` | `icon` | `arrow_drop_down` |
| `date-range-field.expand` | `icon` | `arrow_drop_down` |
| `date-time-field.expand` | `icon` | `arrow_drop_down` |
| `color-field.expand` | `icon` | `arrow_drop_down` |
| `calendar.previous` | `previous_icon` | `chevron_left` |
| `calendar.next` | `next_icon` | `chevron_right` |
