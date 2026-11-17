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
| `placeholder`, `label` | string | none | Hint and accessible name. |
| `enabled` | bool | `true` | Enables input. |
| `min_lines` | integer | `2` | Height of the empty field. |

## Accessibility

* The textarea carries the label and `aria-invalid` while an error is shown.
* The completion list is a `listbox` of `option`s; focus stays in the textarea.
