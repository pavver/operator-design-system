# CodeBlock

Monospaced block for code, configuration, and log output, with a copy button.

Status: **reviewed** on web.

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

## Arborium

The optional `arborium` Cargo feature enables Tree-sitter-based syntax highlighting through the arborium crate.
The operator-ui crate enables no language grammars by default.
An application enables `arborium` together with the specific `arborium/lang-*` features it requires in its own `Cargo.toml`.
When arborium does not support the requested language or the grammar is not compiled in, highlighting falls back to the built-in scanner.
Compiling arborium for WebAssembly requires an LLVM and clang toolchain that supports the `wasm32-unknown-unknown` target.
Enabling arborium increases the compiled release WebAssembly binary size.
In a minimal application rendering a highlighted code block with `arborium/lang-rust`, the raw WebAssembly size increased by approximately 3.50 MB (from 2.28 MB to 5.78 MB), and the gzip-compressed size increased by approximately 634 KB (from 490 KB to 1.12 MB).

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

### Customization API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `header_height` | optional number (DU) | `row_compact` | Overrides `--op-code-block-header-height`. |
| `header_gap` | optional number (DU) | `gutter` | Overrides `--op-code-block-header-gap`. |
| `header_padding_start` | optional number (DU) | `pad_card` | Overrides `--op-code-block-header-padding-start`. |
| `header_padding_end` | optional number (DU) | `pad_tight` | Overrides `--op-code-block-header-padding-end`. |
| `padding` | optional number (DU) | `pad_card` | Overrides `--op-code-block-padding`. |
| `number_width` | optional number (DU) | `8 DU` | Overrides `--op-code-block-number-width`. |
| `number_padding` | optional number (DU) | `pad_card` | Overrides `--op-code-block-number-padding`. |
| `copy_size` | optional number (DU) | `row_dense` | Overrides `--op-code-block-copy-size`. |
| `icon_size` | optional number (DU) | `3.5 DU` | Overrides `--op-code-block-icon-size`. |
| `font_size` | optional number (TU) | `type_console_size` | Overrides `--op-code-block-font-size`. |
| `line_height` | optional number (TU) | `type_console_line` | Overrides `--op-code-block-line-height`. |
| `header_font_size` | optional number (TU) | `type_caption_size` | Overrides `--op-code-block-header-font-size`. |
| `header_letter_spacing` | optional number (TU) | `0.04 × type_caption_size` | Overrides `--op-code-block-header-letter-spacing`. |
| `max_height` | optional number (DU) | `100%` | Overrides `--op-code-block-max-height`. |
| `header` | slot | none | Replaces the language label; keeps the header and copy button. |
| `actions` | slot | none | Additional header content before the copy button. |
| `footer` | slot | none | Below the scrolling code. |
| `content` | slot | none | Replaces the scrolling code body. |
| `copy_label` | optional string | `code-block.copy` | Text override. |
| `copy_icon` | optional icon | `code-block.copy` | Icon override. |
| `class` | optional string | none | Added to the root classes. |
| `style` | optional string | none | Appended after root metric declarations. |

## Customization

`class` and `style` apply to the root.
Explicit root styles take precedence over metric props; unset metrics retain the original rendering.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `header_height` | DU | `--op-code-block-header-height` | `row_compact` |
| `header_gap` | DU | `--op-code-block-header-gap` | `gutter` |
| `header_padding_start` | DU | `--op-code-block-header-padding-start` | `pad_card` |
| `header_padding_end` | DU | `--op-code-block-header-padding-end` | `pad_tight` |
| `padding` | DU | `--op-code-block-padding` | `pad_card` |
| `number_width` | DU | `--op-code-block-number-width` | `8 DU` |
| `number_padding` | DU | `--op-code-block-number-padding` | `pad_card` |
| `copy_size` | DU | `--op-code-block-copy-size` | `row_dense` |
| `icon_size` | DU | `--op-code-block-icon-size` | `3.5 DU` |
| `font_size` | TU | `--op-code-block-font-size` | `type_console_size` |
| `line_height` | TU | `--op-code-block-line-height` | `type_console_line` |
| `header_font_size` | TU | `--op-code-block-header-font-size` | `type_caption_size` |
| `header_letter_spacing` | TU | `--op-code-block-header-letter-spacing` | `0.04 × type_caption_size` |
| `max_height` | DU | `--op-code-block-max-height` | `100%` |

Border widths remain the fixed geometry specified above.

| Color variable | Default |
| :--- | :--- |
| `--op-code-block-background` | `surface_alt` |
| `--op-code-block-border` | `border` |
| `--op-code-block-divider` | `divider` |
| `--op-code-block-secondary-text` | `text_secondary` |
| `--op-code-block-text` | `text_primary` |
| `--op-code-block-number` | `text_disabled` |
| `--op-code-block-syntax-keyword` | `syntax_keyword` |
| `--op-code-block-syntax-string` | `syntax_string` |
| `--op-code-block-syntax-number` | `syntax_number` |
| `--op-code-block-syntax-comment` | `syntax_comment` |
| `--op-code-block-syntax-type` | `syntax_type` |
| `--op-code-block-syntax-function` | `syntax_function` |
| `--op-code-block-syntax-constant` | `syntax_constant` |
| `--op-code-block-syntax-property` | `syntax_property` |
| `--op-code-block-syntax-meta` | `syntax_meta` |
| `--op-code-block-syntax-punctuation` | `syntax_punctuation` |

| Slot | Place |
| :--- | :--- |
| `header` | Replaces the language label; keeps the header and copy button. |
| `actions` | Additional header content before the copy button. |
| `footer` | Below the scrolling code. |
| `content` | Replaces the scrolling code body. |

### Strings

Resolution: prop, application lookup, English default.

| Key | Default |
| :--- | :--- |
| `code-block.copy` | `Copy code` |

The copy button retains IconButton's `icon-button.loading`, `icon-button.success` and `icon-button.error` feedback keys.

### Icons

| Key | Default |
| :--- | :--- |
| `code-block.copy` | `content_copy` |

The copy button retains IconButton's `icon-button.success` and `icon-button.error` icon keys.
