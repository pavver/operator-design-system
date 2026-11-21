# ConsoleTerminal

Scrollable log view with tag-based line coloring.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Fills the parent. |
| Height | `45 DU` by default. |
| Padding | `gutter` (`2 DU`). |
| Text | `console` style. |
| Outline | `0.25 DU` in `border`. |
| Corner radius | `0 DU`. |

## Colors

The terminal uses only the `term_*` palette, never the interface surfaces or signal colors.

Colors and text attributes come from xterm SGR escape sequences (`ESC [ … m`) in the log text, as in a real terminal.
The terminal never colors a line by its content.

| Sequence | Effect |
| :--- | :--- |
| `30`–`37`, `40`–`47` | Foreground and background from `term_black` … `term_white`. |
| `90`–`97`, `100`–`107` | Foreground and background from `term_bright_black` … `term_bright_white`. |
| `38;5;n`, `48;5;n` | xterm 256-color palette; `0`–`15` map to the `term_*` tokens. |
| `38;2;r;g;b`, `48;2;r;g;b` | True color; the colon forms `38:2::r:g:b` and `38:5:n` work too. |
| `39`, `49` | Default foreground `term_foreground` and default background. |
| `1`, `2`, `3`, `4`, `9` | Bold, dim (60% opacity), italic, underline, strikethrough. |
| `7` | Inverse: swaps foreground and background, using the terminal colors for defaults. |
| `22`, `23`, `24`, `27`, `29` | Turn the matching attribute off. |
| `0` or empty | Reset. |

* The style carries over from one entry to the next, until a reset.
* Other escape sequences (cursor movement, erase, OSC titles and links, charset selection) are removed.
* A newline inside an entry starts a new line.
* A lone carriage return rewrites the current line, so progress output shows only its last state.

The background is `term_background`.

## Behavior

* When a line is added, the view scrolls to the newest line.
* The user can scroll back through the history.
* Long lines wrap.

## API

| Property | Type | Default |
| :--- | :--- | :--- |
| `logs` | list of strings | required |
| `height` | DU | `45` |

### Customization API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `gap` | optional number (DU) | `gap_micro` | Overrides `--op-console-terminal-gap`. |
| `padding` | optional number (DU) | `gutter` | Overrides `--op-console-terminal-padding`. |
| `font_size` | optional number (TU) | `type_console_size` | Overrides `--op-console-terminal-font-size`. |
| `line_height` | optional number (TU) | `type_console_line` | Overrides `--op-console-terminal-line-height`. |
| `height` | optional number (DU) | `45 DU` | Overrides `--op-console-terminal-height`. |
| `header` | slot | none | Before the scrolling log. |
| `footer` | slot | none | After the scrolling log. |
| `empty` | slot | none | When the input log is empty; the default remains blank. |
| `label` | optional string | `console-terminal.label` | Text override. |
| `class` | optional string | none | Added to the root classes. |
| `style` | optional string | none | Appended after root metric declarations. |

## Customization

`class` and `style` apply to the root.
Explicit root styles take precedence over metric props; unset metrics retain the original rendering.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-console-terminal-gap` | `gap_micro` |
| `padding` | DU | `--op-console-terminal-padding` | `gutter` |
| `font_size` | TU | `--op-console-terminal-font-size` | `type_console_size` |
| `line_height` | TU | `--op-console-terminal-line-height` | `type_console_line` |
| `height` | DU | `--op-console-terminal-height` | `45 DU` |

Border widths remain the fixed geometry specified above.

| Color variable | Default |
| :--- | :--- |
| `--op-console-terminal-background` | `term_background` |
| `--op-console-terminal-text` | `term_foreground` |
| `--op-console-terminal-border` | `border` |
| `--op-console-terminal-hover-border` | `border_strong` |
| `--op-console-terminal-black` | `term_black` |
| `--op-console-terminal-bright-black` | `term_bright_black` |
| `--op-console-terminal-red` | `term_red` |
| `--op-console-terminal-bright-red` | `term_bright_red` |
| `--op-console-terminal-green` | `term_green` |
| `--op-console-terminal-bright-green` | `term_bright_green` |
| `--op-console-terminal-yellow` | `term_yellow` |
| `--op-console-terminal-bright-yellow` | `term_bright_yellow` |
| `--op-console-terminal-blue` | `term_blue` |
| `--op-console-terminal-bright-blue` | `term_bright_blue` |
| `--op-console-terminal-magenta` | `term_magenta` |
| `--op-console-terminal-bright-magenta` | `term_bright_magenta` |
| `--op-console-terminal-cyan` | `term_cyan` |
| `--op-console-terminal-bright-cyan` | `term_bright_cyan` |
| `--op-console-terminal-white` | `term_white` |
| `--op-console-terminal-bright-white` | `term_bright_white` |

| Slot | Place |
| :--- | :--- |
| `header` | Before the scrolling log. |
| `footer` | After the scrolling log. |
| `empty` | When the input log is empty; the default remains blank. |

### Strings

Resolution: prop, application lookup, English default.

| Key | Default |
| :--- | :--- |
| `console-terminal.label` | `empty; preserves the unnamed log` |

### Icons

No built-in icons.
