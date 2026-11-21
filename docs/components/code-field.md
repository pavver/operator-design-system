# CodeField

Multi-line code input with syntax colours, diagnostics and completion: templates, formulas, expressions.
It is project-specific: the `code-field` feature is in the `extensions` group, not in `full`.
The language lives in the application, which passes a highlighter and a completion source.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Field | Full width, `surface` with the field outline states; `pad_tight` vertical and `pad_card` horizontal padding. |
| Text | `console` type style in the Operator font, wrapping anywhere; the field grows with the text from `min_lines` (`2`). |
| Colours | `syntax_*` tokens per `CodeToken`. |
| Diagnostics | Wavy `error` or `warning` underline; the first message shows under the text in `caption` style. |
| Completion | A [DropdownMenu](dropdown-menu.md)-style list under the caret, `50–90 DU` wide, options with details. |

## Behavior

* A transparent textarea lies over the highlighted text, so editing, selection and the browser's undo work as usual.
* `highlight(text)` returns `CodeSpan`s (byte range, token, optional message); syntax spans colour the text, error and warning spans underline it.
* `complete((text, caret))` runs on every edit and on Ctrl+Space; a result (`CodeCompletion`: the replaced byte range and `ComboOption`s) opens the list at the caret.
* While the list is open, Arrow Up and Down move, Enter or Tab inserts, Escape closes; a click inserts too.
* An insert goes through `insertText`, so Ctrl+Z undoes it.
* Leaving the field or Ctrl+Enter commits through `on_commit` when the text changed.
* The caret position is measured in an unscaled mirror, so completion works inside a zoomed canvas.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | string signal | required | The text. |
| `highlight` | callback `(text) -> Vec<CodeSpan>` | none | Colours and diagnostics. |
| `complete` | callback `((text, caret byte)) -> Option<CodeCompletion>` | none | Completion source. |
| `on_commit` | callback `(text)` | none | Blur or Ctrl+Enter with a changed text. |
| `placeholder`, `label` | optional string | `code-field.placeholder`, none | Placeholder and visible label; label also names the textarea. |
| `enabled` | bool | `true` | Enables input. |
| `min_lines` | integer | `2` | Height of the empty field. |

CodeField uses the public [Field](field.md) shell for label, hint, error, slots, variant and pending/disabled behavior.
The diagnostic message remains below the field; when `diagnostics_hover` is provided it becomes a focusable HoverCard trigger.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `aria_label` | optional string | `code-field.label` | Accessible name when no label is set. |
| `completion_label` | optional string | `code-field.completions` | Accessible completion list name. |
| `diagnostics_label` | optional string | `code-field.diagnostics` | Accessible diagnostics hover name. |
| `hint`, `error` | optional string | none | Field helper text and error; error replaces hint. |
| `variant` | FieldVariant signal | Outlined | Supports borderless embedded editing. |
| `state` | optional ControlState signal | none | External pending and result state; pending blocks editing and closes completion. |
| `tab_size` | integer | 2 | Tab width in spaces; `--op-field-tab-size`. |
| `class`, `style` | MaybeProp string | none | Outer root hooks. |
| `prepend`, `append`, `before`, `after`, `label_slot`, `hint_slot`, `gutter`, `diagnostics_hover` | Slot | empty | Named content, detailed below. |
| `completion_item` | optional ScopedSlot<ComboOption> | none | Custom completion item content; picking and keyboard handling stay built in. |

All metric props below are optional reactive `MaybeProp<f32>` values.

| Metric prop | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `height` | DU number | 0; content determines height | Writes the variable in the Customization table. |
| `padding_x` | DU number | pad_card | Writes the variable in the Customization table. |
| `slot_size` | DU number | row_standard | Writes the variable in the Customization table. |
| `gap` | DU number | gap_micro | Writes the variable in the Customization table. |
| `font_size` | TU number | type_console_size for editor; type_body_size for completion | Writes the variable in the Customization table. |
| `padding_y` | DU number | pad_tight | Writes the variable in the Customization table. |
| `line_height` | TU number | type_console_line | Writes the variable in the Customization table. |
| `caption_font_size` | TU number | type_caption_size | Writes the variable in the Customization table. |
| `caption_line_height` | TU number | type_caption_line | Writes the variable in the Customization table. |
| `completion_min_width` | DU number | 50 | Writes the variable in the Customization table. |
| `completion_max_width` | DU number | 90 | Writes the variable in the Customization table. |
| `row_height` | DU number | row_control | Writes the variable in the Customization table. |
| `max_height` | DU number | 50 | Writes the variable in the Customization table. |
| `option_padding_x` | DU number | pad_card | Writes the variable in the Customization table. |
| `option_gap` | DU number | pad_tight | Writes the variable in the Customization table. |
| `option_icon_size` | DU number | icon_small | Writes the variable in the Customization table. |
| `detail_font_size` | TU number | type_caption_size | Writes the variable in the Customization table. |
| `detail_line_height` | TU number | type_caption_line | Writes the variable in the Customization table. |
| `section_padding_y` | DU number | pad_tight | Writes the variable in the Customization table. |
| `letter_spacing` | TU number | 0.06em for completion section titles | Writes the variable in the Customization table. |
| `gutter_width` | DU number | auto; content width | Writes the variable in the Customization table. |
| `gutter_gap` | DU number | 0 | Writes the variable in the Customization table. |

## Customization

`class` and `style` apply to the outer root surrounding the Field shell, diagnostics and completion.
CodeField uses `--op-field-*` variables shared with other fields.
The textarea and highlighting layer share padding, font size, line height and tab size, keeping text and the measured caret aligned.
The caret mirror copies their resolved CSS geometry, including overrides inside a zoomed canvas.
Completion copies field variables from the root into the portal, and its virtual list measures row height and maximum height from the same variables used for rendering.
`min_lines` writes the dimensionless `--op-field-min-lines` variable and keeps its existing integer API.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | 0; content determines height |
| `padding_x` | DU | `--op-field-padding-x` | pad_card |
| `slot_size` | DU | `--op-field-slot-size` | row_standard |
| `gap` | DU | `--op-field-gap` | gap_micro |
| `font_size` | TU | `--op-field-font-size` | type_console_size for editor; type_body_size for completion |
| `padding_y` | DU | `--op-field-padding-y` | pad_tight |
| `line_height` | TU | `--op-field-line-height` | type_console_line |
| `caption_font_size` | TU | `--op-field-caption-font-size` | type_caption_size |
| `caption_line_height` | TU | `--op-field-caption-line-height` | type_caption_line |
| `completion_min_width` | DU | `--op-field-completion-min-width` | 50 |
| `completion_max_width` | DU | `--op-field-completion-max-width` | 90 |
| `row_height` | DU | `--op-field-row-height` | row_control |
| `max_height` | DU | `--op-field-max-height` | 50 |
| `option_padding_x` | DU | `--op-field-option-padding-x` | pad_card |
| `option_gap` | DU | `--op-field-option-gap` | pad_tight |
| `option_icon_size` | DU | `--op-field-option-icon-size` | icon_small |
| `detail_font_size` | TU | `--op-field-detail-font-size` | type_caption_size |
| `detail_line_height` | TU | `--op-field-detail-line-height` | type_caption_line |
| `section_padding_y` | DU | `--op-field-section-padding-y` | pad_tight |
| `letter_spacing` | TU | `--op-field-letter-spacing` | 0.06em for completion section titles |
| `gutter_width` | DU | `--op-field-gutter-width` | auto; content width |
| `gutter_gap` | DU | `--op-field-gutter-gap` | 0 |

| Color variable | Default |
| :--- | :--- |
| `--op-field-background` | surface |
| `--op-field-text` | text_primary |
| `--op-field-secondary-text` | text_secondary |
| `--op-field-accent` | accent |
| `--op-field-disabled-background` | disabled_surface |
| `--op-field-disabled-text` | text_disabled |
| `--op-field-error` | error |
| `--op-field-warning` | warning |
| `--op-field-pressed` | pressed |
| `--op-field-divider` | divider |
| `--op-field-syntax-keyword` | syntax_keyword |
| `--op-field-syntax-string` | syntax_string |
| `--op-field-syntax-number` | syntax_number |
| `--op-field-syntax-comment` | syntax_comment |
| `--op-field-syntax-type` | syntax_type |
| `--op-field-syntax-function` | syntax_function |
| `--op-field-syntax-constant` | syntax_constant |
| `--op-field-syntax-property` | syntax_property |
| `--op-field-syntax-meta` | syntax_meta |
| `--op-field-syntax-punctuation` | syntax_punctuation |

The shared Field shell also supplies its standard outline and result states.

| Slot | Place |
| :--- | :--- |
| `prepend, append` | Inside the Field box, before and after the editor. |
| `before, after` | Outside the box. |
| `label_slot` | Replaces the visible label. |
| `hint_slot` | Replaces Field hint and error content. |
| `gutter` | Optional leading column alongside the text, aligned to the editor line height and vertical padding. |
| `completion_item` | Scoped item content receiving a `ComboOption`, including value, label, detail and icon. |
| `diagnostics_hover` | HoverCard content attached to the first diagnostic message; available by hover and keyboard focus. |

### Strings

| Key | Prop | Default |
| :--- | :--- | :--- |
| `code-field.placeholder` | `placeholder` | Empty string |
| `code-field.label` | `aria_label` | Code |
| `code-field.completions` | `completion_label` | Completions |
| `code-field.diagnostics` | `diagnostics_label` | Diagnostics |

### Icons

CodeField has no built-in icons or semantic icon keys.
Completion icons are caller-supplied `ComboOption.icon` data, and `completion_item` can replace their presentation.

## Accessibility

* The textarea carries the resolved label and `aria-invalid` while an error diagnostic or explicit error is shown.
* The completion list is a named `listbox` of `option`s; focus stays in the textarea.
* The diagnostic hover trigger is keyboard-focusable when the slot is supplied.
