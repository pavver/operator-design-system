# Kbd

Keyboard key or shortcut, shown inline with text or in menus.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Key | `6.5 DU` tall, at least `6.5 DU` wide; corners `0 DU`. |
| Wide key | At least `11 DU` for Shift, Enter, Tab, Backspace, Caps Lock, Ctrl, Alt, Cmd, and Win; the label sits at the start. |
| Space bar | At least `26 DU`. |
| Body | A darker key body shows as `0.25 DU` at the top, `0.5 DU` on the sides, and a `1 DU` front edge at the bottom; a soft drop shadow sits under it. |
| Body color | `background` mixed with `border_strong`, lighter toward the top. |
| Top face | `text_primary` at about 10% over `surface`, with a slight vertical gradient and a `0.25 DU` highlight along its top edge. |
| Type | `caption` size, semibold, centered on the top face. |
| Separator | `+` in `text_secondary`, `0.5 DU` from the keys. |

## Behavior

* A shortcut string is split at `+`: `Ctrl+Shift+K` shows three keys.
* `Ctrl++` shows the plus key itself.
* `Up`, `Down`, `Left`, and `Right` (or `ArrowUp` and so on) print as arrows.

## API

| Property | Type | Description |
| :--- | :--- | :--- |
| `keys` | string | Key or shortcut, for example `Esc` or `Ctrl+K`. |

## Accessibility

The group is named by the full shortcut string.
