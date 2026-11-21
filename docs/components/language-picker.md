# LanguagePicker

Combobox variant for choosing an interface language.

Status: **draft** on web.

## Use

* Selecting the active interface language for the application.
* Filtering through supported languages by native name, English name, or standard BCP 47 code.
* Presenting languages with recognizable country flag graphics and standardized dual labels.

## Geometry

* The input shell is a [Field](field.md) with an `arrow_drop_down` trailing icon.
* The selected language's flag is displayed at the start of the field in the leading prepend slot when `show_flags` is true.
* Each option in the list displays a flag icon followed by the language label.
* The flag is rendered in a square container of `6 DU` width and `4 DU` height by default.
* The flag container has square corners with `0 DU` corner radius.
* A `0.25 DU` inner border using the existing `--op-border` token is drawn inside the flag container so white stripes remain visible on light surfaces.
* Flag colors are the real flag colors (they are content, not UI tokens): this is an intentional content exception to design token rules.

## Behavior

* Typing in the input field filters the list by case-insensitive substring match against the native name, English name, or BCP 47 code.
* Options are formatted as 'Native name (English name)' by default, for example 'Українська (Ukrainian)' or 'Deutsch (German)'.
* When native and English names match (such as 'English'), only the single name is shown without redundant parentheses.
* Navigating options with the arrow keys wraps around and scrolls the active item into view.
* Pressing Enter or clicking commits the selected language's BCP 47 code.
* Blurring the field commits exact matches or reverts uncommitted text to the previously selected language.

## Options

A language option is represented by `LanguageOption`:

| Field | Type | Description |
| :--- | :--- | :--- |
| `code` | `String` | BCP 47 language code (e.g. `uk`, `en`, `de`). |
| `native` | `String` | Native name of the language. |
| `english` | `String` | English name of the language. |
| `flag` | `Option<Flag>` | Associated flag representation. |

Built-in languages are available through the `Language` enum (`Language::En`, `Language::Uk`, etc.) and convert directly via `LanguageOption::from(Language)`.
Callers can also build fully custom language options using `LanguageOption::new(code, native, english)`.

## Flags

Flags can be built-in, custom SVG markup, or omitted:

| Variant | Description |
| :--- | :--- |
| `Flag::Builtin(&'static str)` | Built-in country code: `UA`, `GB`, `US`, `DE`, `FR`, `ES`, `IT`, `PL`, `PT`, `NL`, `CZ`, `SE`, `JP`, `CN`, `KR`, `TR`. |
| `Flag::Svg(String)` | Inline SVG markup provided by the application. |
| `Flag::None` | Omits the flag icon for the option. |

Flag colors use the authentic colors of each national flag as content graphics rather than theme color tokens.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | `RwSignal<String>` | required | Committed BCP 47 language code. |
| `languages` | `Signal<Vec<LanguageOption>>` | all built-in languages | Available languages. |
| `show_flags` | `Signal<bool>` | `true` | Shows flag graphics next to options. |
| `format` | `Option<Callback<LanguageOption, String>>` | `None` | Custom label formatter. |
| `label` | `MaybeProp<String>` | `language-picker.label` | Visible label above the field. |
| `aria_label` | `MaybeProp<String>` | `language-picker.label` | Accessible name for screen readers. |
| `width` | optional number (DU) | `100%` | Root width override. |
| `placeholder` | `MaybeProp<String>` | empty | Placeholder text inside the input. |
| `hint` | `MaybeProp<String>` | `None` | Help text below the field. |
| `error` | `MaybeProp<String>` | `None` | Error message below the field. |
| `variant` | `Signal<FieldVariant>` | `Outlined` | Visual variant of the field shell. |
| `enabled` | `Signal<bool>` | `true` | Enables user input. |
| `empty_text` | `MaybeProp<String>` | `language-picker.empty` | Message displayed when no language matches. |
| `on_select` | `Option<Callback<String>>` | `None` | Callback invoked with the committed language code. |

### Customization API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `flag_width` | optional number (DU) | `6 DU` | Overrides `--op-language-picker-flag-width`. |
| `flag_height` | optional number (DU) | `4 DU` | Overrides `--op-language-picker-flag-height`. |
| `height` | optional number (DU) | `row_standard` | Field height override. |
| `padding_x` | optional number (DU) | `pad_control` | Field horizontal padding override. |
| `slot_size` | optional number (DU) | `row_dense` | Leading and trailing slot size override. |
| `gap` | optional number (DU) | `gutter` | Spacing between field elements override. |
| `font_size` | optional number (TU) | `inherit` | Field text font size override. |
| `option` | scoped slot | `None` | Scoped `LanguageOption`; customizes option rendering. |
| `empty` | slot | `None` | Custom view when no languages match. |
| `before_options` | slot | `None` | Slot rendered before the option list. |
| `after_options` | slot | `None` | Slot rendered after the option list. |
| `header` | slot | `None` | Header content inside the dropdown popover. |
| `footer` | slot | `None` | Footer content inside the dropdown popover. |
| `prepend` | slot | `None` | Content inside the box before the input. |
| `append` | slot | `None` | Content inside the box after the input. |
| `before` | slot | `None` | Content outside the box before it. |
| `after` | slot | `None` | Content outside the box after it. |
| `label_slot` | slot | `None` | Replaces the label content above the box. |
| `hint_slot` | slot | `None` | Replaces the hint content below the box. |
| `class` | optional string | `None` | Classes added to the root element. |
| `style` | optional string | `None` | Inline styles appended to the root element. |

## Accessibility

* The input element carries `role="combobox"` with `aria-autocomplete="list"`.
* The popup layer carries `role="listbox"`, and each item has `role="option"`.
* The active option is tracked via selection and focus states.
* Flag images are marked with `aria-hidden="true"` so screen readers speak only the descriptive language names.
* Translatable accessible labels are provided via `language-picker.label`.

## Customization

`class` and `style` apply to the component root.
Metric props declare CSS custom properties with token fallbacks.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-language-picker-width` | `100%` |
| `flag_width` | DU | `--op-language-picker-flag-width` | `6 DU` |
| `flag_height` | DU | `--op-language-picker-flag-height` | `4 DU` |

The list opens in the overlay host, outside the component root, so its minimum width is a theme-level variable: `--op-language-picker-list-min-width` (default `64 DU`).
The list is at least that wide even when the field is compact, so language names are not cut.

| Color variable | Default |
| :--- | :--- |
| `--op-language-picker-border` | `--op-border` |

| Slot | Place |
| :--- | :--- |
| `option` | Scoped `LanguageOption`; replaces default row rendering. |
| `empty` | Replaces the empty message when no language matches. |
| `before_options` | Inside the dropdown list before the items. |
| `after_options` | Inside the dropdown list after the items. |
| `header` | Above the dropdown list. |
| `footer` | Below the dropdown list. |
| `prepend` | Inside the field box before the input. |
| `append` | Inside the field box before the dropdown arrow. |
| `before` | Outside the box on the leading side. |
| `after` | Outside the box on the trailing side. |

## Strings

| Key | Default | Description |
| :--- | :--- | :--- |
| `language-picker.label` | `Language` | Accessible name and default visible label. |
| `language-picker.empty` | `No matching language` | Text displayed when filtering finds no matches. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `combobox.expand` | `arrow_drop_down` | Dropdown indicator icon. |
