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
