# Typography

## Font

Cascadia Code is the only typeface on every platform.
Fallbacks are `Cascadia Mono`, then the platform monospace font.
Cascadia Code is licensed under the SIL Open Font License 1.1.
Ship the unmodified font file.
A subset counts as a modified version under the license and must not use the reserved font name.

A monospace font keeps columns aligned and prevents layout shifts when numbers change.
It also gives the interface its technical character.

## Weights

* **Regular (400):** descriptions, hints, secondary read-outs.
* **Medium (500):** buttons, field text, row titles, unselected segments.
* **Bold (700):** screen titles, selected segments, tooltips, critical values.

Hierarchy comes from weight and color rather than from large size differences.

## Type Scale

All sizes are in Type Units, see [units.md](units.md).

| Style | Size | Line height | Weight | Usage |
| :--- | :--- | :--- | :--- | :--- |
| `app_title` | `20 TU` | `26 TU` | 700 | Application and top bar titles. |
| `screen_title` | `18 TU` | `24 TU` | 500 | Screen and section headers. |
| `section_title` | `14 TU` | `18 TU` | 500 | Group headers inside a screen. |
| `row_title` | `15 TU` | `20 TU` | 500 | Primary text in list rows. |
| `body` | `14 TU` | `18 TU` | 400 | Base text, field values, button labels. |
| `body_small` | `13 TU` | `16 TU` | 400 | Segment labels, secondary text. |
| `caption` | `12 TU` | `15 TU` | 400 | Labels, metadata, tooltips. |
| `console` | `12 TU` | `16 TU` | 400 | Logs and terminals. |

## Rules

* Text containers never have a fixed width when the text can change or be translated.
* Single-line text truncates with an ellipsis.
* Multi-line text lets its container grow vertically.
* Numbers that update live use tabular figures.
