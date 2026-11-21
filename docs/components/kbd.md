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
| `height` | DU | Key height; defaults to `6.5 DU`. |
| `wide_min_width` | DU | Minimum width of wide keys; defaults to `11 DU`. |
| `space_min_width` | DU | Minimum width of space bar; defaults to `26 DU`. |
| `padding_x` | DU | Horizontal key padding; defaults to `1 DU`. |
| `gap` | DU | Gap between keys; defaults to `0.5 DU`. |
| `font_size` | TU | Key font size; defaults to `caption` size. |
| `separator` | slot | Custom separator between keys. |
| `aria_label` | string | Accessible label; defaults to `keys`. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-kbd-height` | `6.5 DU` |
| `wide_min_width` | DU | `--op-kbd-wide-min-width` | `11 DU` |
| `space_min_width` | DU | `--op-kbd-space-min-width` | `26 DU` |
| `padding_x` | DU | `--op-kbd-padding-x` | `pad_tight` (`1 DU`) |
| `gap` | DU | `--op-kbd-gap` | `gap_micro` (`0.5 DU`) |
| `font_size` | TU | `--op-kbd-font-size` | `type_caption_size` |

| Color variable | Default |
| :--- | :--- |
| `--op-kbd-face` | `color-mix(in srgb, var(--op-text-primary) 10%, var(--op-surface))` |
| `--op-kbd-body` | `color-mix(in srgb, var(--op-background) 55%, var(--op-border-strong))` |
| `--op-kbd-text` | `text_primary` |
| `--op-kbd-separator` | `text_secondary` |

| Slot | Place |
| :--- | :--- |
| `separator` | Content between shortcut keys, replacing the default plus separator. |

`class` and `style` apply to the root.

### Strings

| Key | Default |
| :--- | :--- |
| `kbd.plus` | `+` |

## Accessibility

The group is named by the full shortcut string.
