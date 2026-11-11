# CodeBlock

Monospaced block for code, configuration, and log output, with a copy button.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Background | `surface_alt` with a `0.25 DU` `border` outline. |
| Header | Optional, at least `8 DU`: language label in uppercase `caption`, copy button at the end, `divider` below. |
| Body | `console` style, `3 DU` padding. |
| Line numbers | `8 DU` column in `text_disabled`, end-aligned, not selectable. |
| Height | Content height, or `max_height` and then scrolling. |

## Syntax Highlighting

The `language` property also selects the highlighter.
Built-in languages: Rust (`rust`, `rs`), Kotlin (`kotlin`, `kt`, `kts`), TOML, JSON (`json`, `jsonc`), YAML (`yaml`, `yml`), and shell (`bash`, `sh`, `shell`, `zsh`, `console`).
Any other language is shown without highlighting.

| Token | Color |
| :--- | :--- |
| Keywords | `syntax_keyword` |
| Strings and characters | `syntax_string` |
| Numbers | `syntax_number` |
| Comments | `syntax_comment`, italic |
| Types and capitalized names | `syntax_type` |
| Functions, macros, shell built-ins | `syntax_function` |
| `true`, `false`, `null`, `ALL_CAPS` names | `syntax_constant` |
| Keys of TOML, YAML, and JSON; shell variables | `syntax_property` |
| Attributes, annotations, lifetimes, TOML tables, YAML anchors | `syntax_meta` |
| Operators and brackets | `syntax_punctuation` |

The built-in highlighter is a small line scanner, so it does not understand string templates, heredocs, or YAML block scalars.

## Behavior

* Long lines scroll horizontally; with `wrap` they wrap and break anywhere.
* The copy button copies the whole text as an async action with its success or error state.
* The header is hidden when there is no language and copying is off.
* Tabs are four columns wide.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `code` | string | required | Content. |
| `language` | string | none | Header label and highlighter. |
| `line_numbers` | bool | `false` | Numbers each line. |
| `wrap` | bool | `false` | Wraps long lines. |
| `copy` | bool | `true` | Shows the copy button. |
| `max_height` | DU | none | Height limit. |
