# Editor

Browser rich text editing with an accessible toolbar and constrained HTML output.
Provides WYSIWYG editing on top of the Field shell with sanitized markup output.

Status: **draft** on web.

## Use

* Rich formatting for notes, comments, articles, or documentation.
* Producing sanitized HTML from interactive text formatting commands.

## Geometry

| Property | Value |
| :--- | :--- |
| Minimum height | `40 DU`. |
| Maximum height | `100 DU`. |
| Padding | `pad_small` (`1.5 DU`). |
| Gap | `gap_micro` (`0.5 DU`). |
| Corner radius | `0 DU`. |

## Behavior

* The editable area uses a contenteditable container with paste sanitization.
* Toolbar buttons execute formatting commands (bold, italic, lists, headings, links, undo/redo).
* Toolbar button active states track the current text selection state.
* Adding a link opens an inline link dialog that validates URLs against permitted protocols.
* Value changes sanitize output HTML and emit through `on_change`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | string signal | required | HTML content string signal. |
| `toolbar` | `Vec<EditorCommand>` | all | List of enabled toolbar commands. |
| `on_change` | callback `(String)` | none | Emits when sanitized HTML updates. |
| `enabled` | bool | `true` | Enables editing and toolbar buttons. |
| `readonly` | bool | `false` | Sets editor to view-only mode. |
| `state` | `ControlState` | none | Explicit control state override. |
| `label` | string | `editor.label` | Visible label and accessible name. |
| `placeholder` | string | `editor.placeholder` | Empty placeholder prompt. |
| `link_label` | string | `editor.url` | Accessible label for link input. |
| `apply_label` | string | `editor.apply` | Label for apply link button. |
| `cancel_label` | string | `editor.cancel` | Label for cancel link button. |
| `invalid_link_label` | string | `editor.invalid-link` | Error message for invalid URLs. |
| `header` | `Slot` | none | Slot above toolbar. |
| `actions` | `Slot` | none | Custom buttons inside toolbar. |
| `footer` | `Slot` | none | Slot below editable area. |
| `min_height` | DU metric | `40 DU` | Minimum height of the editor area. |
| `max_height` | DU metric | `100 DU` | Maximum height before scrolling. |
| `padding` | DU metric | `pad_small` | Internal editor padding. |
| `gap` | DU metric | `gap_micro` | Toolbar item gap. |
| `font_size` | TU metric | `type_body_size` | Body font size. |
| `line_height` | TU metric | `type_body_line` | Body line height. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `min_height` | DU | `--op-editor-min-height` | `40 DU` |
| `max_height` | DU | `--op-editor-max-height` | `100 DU` |
| `padding` | DU | `--op-editor-padding` | `pad_small` (`1.5 DU`) |
| `gap` | DU | `--op-editor-gap` | `gap_micro` (`0.5 DU`) |
| `font_size` | TU | `--op-editor-font-size` | `type_body_size` |
| `line_height` | TU | `--op-editor-line-height` | `type_body_line` |

| Color variable | Default |
| :--- | :--- |
| `--op-editor-background` | `surface` |
| `--op-editor-text` | `text_primary` |
| `--op-editor-border` | `border` |
| `--op-editor-accent` | `accent` |
| `--op-editor-selected-background` | `hover` |
| `--op-editor-disabled-text` | `text_disabled` |

| Slot | Place |
| :--- | :--- |
| `header` | Above the toolbar. |
| `actions` | Inside the toolbar row. |
| `footer` | Below the content area. |

`class` and `style` apply to the root Field element.

### Strings

| Key | Default |
| :--- | :--- |
| `editor.label` | `Rich text` |
| `editor.placeholder` | `Write something` |
| `editor.url` | `Link URL` |
| `editor.apply` | `Apply link` |
| `editor.cancel` | `Cancel` |
| `editor.invalid-link` | `Enter an HTTP, HTTPS, mail or relative URL` |
| `editor.bold` | `Bold` |
| `editor.italic` | `Italic` |
| `editor.underline` | `Underline` |
| `editor.strike` | `Strikethrough` |
| `editor.bullets` | `Bullet list` |
| `editor.numbers` | `Numbered list` |
| `editor.link` | `Link` |
| `editor.paragraph` | `Paragraph` |
| `editor.heading1` | `Heading 1` |
| `editor.heading2` | `Heading 2` |
| `editor.undo` | `Undo` |
| `editor.redo` | `Redo` |

### Icons

Editor does not define built-in icons.

## Accessibility

The editor content area uses `role="textbox"` with `aria-multiline="true"` and `aria-readonly`.
The toolbar has `role="toolbar"`, and command buttons use dense square icon buttons grouped by separators with tooltips.
Link errors report via `role="alert"`.
