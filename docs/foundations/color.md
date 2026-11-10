# Color

## Concept

The neutrals follow the gray workbench of modern code editors: flat, low-contrast surfaces that stay out of the way.
The signal colors are warm graphite tones: amber for focus and selection, teal for work in progress, green and red for results.
Blue is deliberately absent from the accent set.
Color always means something; there are no decorative colors.

The exact values live in [tokens.toml](../../tokens/tokens.toml) under `[themes.operator.dark]` and `[themes.operator.light]`.
That file is the single source of truth; this document explains the roles.

## Roles

### Surfaces

| Token | Dark | Light | Role |
| :--- | :--- | :--- | :--- |
| `background` | `#181818` | `#F8F8F8` | Application background. |
| `surface` | `#1F1F1F` | `#FFFFFF` | Controls, fields, cards. |
| `surface_alt` | `#141414` | `#F0F0F0` | Sunken areas: segment tracks, terminals. |
| `pressed` | `#2A2A2A` | `#EBEBEB` | Pressed background. |
| `selection` | `#3A2F1F` | `#F5E6D0` | Selected background. |
| `disabled_surface` | `#1B1B1B` | `#F3F3F3` | Disabled background. |

### Lines

| Token | Dark | Light | Role |
| :--- | :--- | :--- | :--- |
| `border` | `#3A3A3A` | `#D0D0D0` | Default outline. |
| `border_strong` | `#6E6E6E` | `#767676` | Pressed outline. |
| `divider` | `#2B2B2B` | `#E5E5E5` | Separators between rows. |
| `disabled_border` | `#2A2A2A` | `#E2E2E2` | Disabled outline. |

### Content

| Token | Dark | Light | Role |
| :--- | :--- | :--- | :--- |
| `text_primary` | `#CCCCCC` | `#3B3B3B` | Main text. |
| `text_secondary` | `#9D9D9D` | `#616161` | Secondary text, unselected options. |
| `text_disabled` | `#5A5A5A` | `#A0A0A0` | Disabled text, placeholders. |
| `icon` | `#C5C5C5` | `#424242` | Default icons. |

### Signals

| Token | Dark | Light | Meaning |
| :--- | :--- | :--- | :--- |
| `accent` | `#F0A53A` | `#A35C00` | Focus, selection, the current value. |
| `pending` | `#6FB3C8` | `#2C7489` | Work in progress. |
| `success` | `#8CBF6E` | `#477A2E` | Confirmed result. |
| `error` | `#E8664F` | `#B3361F` | Failed result, destructive action. |
| `warning` | `#DCC36A` | `#8A6D00` | Attention without failure. |
| `info` | `#6FB3C8` | `#2C7489` | Neutral information. |

## Rules

* A signal color appears only when its meaning applies.
  Amber never decorates; teal appears only while something is pending.
* Tinted backgrounds are the signal color at `15%` opacity composited over the surface.
  Pressed destructive buttons use `30%`.
  The slider fill uses `accent` at `35%` so the pending border stays visible over it.
* Status layers fade by opacity only.
  Their RGB channels never interpolate toward the base color, which prevents gray intermediate frames.
* Dark and light palettes have an identical key set.
  The token generator rejects a palette with missing or extra keys.
* Component-specific groups: `status_*` for [StatusBadge](../components/status-badge.md), `switch_*` for [AsyncSwitch](../components/switch.md), and `term_*` for [ConsoleTerminal](../components/console-terminal.md).
