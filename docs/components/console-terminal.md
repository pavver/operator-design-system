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

| Line contains | Color |
| :--- | :--- |
| `[ERROR]`, `[CRITICAL]` | `term_bright_red` |
| `[WARNING]` | `term_bright_yellow` |
| `[DEBUG]` | `term_bright_blue` |
| `[SUCCESS]` | `term_bright_green` |
| `[API]`, `[UI]`, `[TX]`, `[RX]` | `term_bright_cyan` |
| anything else | `term_foreground` |

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
