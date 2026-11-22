# PttButton

Tactical push-to-talk button with press-to-talk transmission, upward slide hands-free lock, and Operator states.
Provides explicit visual feedback for transmitting, locked, error, and disabled conditions.

It ships in the media extension: the `ptt-button` feature is in the `media` and `extensions` groups, not in `full`.

Status: **draft** on web.

## Use

Use `PttButton` in dispatch interfaces, tactical audio channels, and hands-free voice communicators.
Holding the button starts audio transmission; sliding up locks transmission open for hands-free speaking.
A quick tap while locked releases the hands-free transmission back to idle.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` standard (supports optional circular exception). |
| Standard border | `0.5 DU` emphasis border (`--op-ptt-button-border-width`). |
| Default size | `22.5 DU` (`--op-ptt-button-size`). |

## Behavior

* Pointer down immediately marks the button as pressed and transmitting (`is_talking = true`).
* Sliding upward past the lock threshold latches transmission hands-free (`is_locked = true`).
* Releasing pointer while unlocked returns the button to idle (`is_talking = false`).
* Clicking while locked unlatches transmission back to idle.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `talking` | `bool` | `false` | Whether transmission is active. |
| `locked` | `bool` | `false` | Whether transmission is locked hands-free. |
| `state` | `ControlState` | `Idle` | Operator control state override. |
| `disabled` | `bool` | `false` | Disables interaction. |
| `circular` | `bool` | `false` | Optional round shape exception. |
| `size` | `f32` | `22.5` | Button size in Design Units. |
| `border_width` | `f32` | `0.5` | Border stroke width in Design Units. |
| `label` | `string` | none | Custom label overriding state default. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |
| `on_state_change` | `Option<Callback<(bool, bool)>>` | none | State change event `(is_talking, is_locked)`. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `size` | `--op-ptt-button-size` | `22.5` | DU |
| `border_width` | `--op-ptt-button-border-width` | `0.5` | DU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-ptt-button-bg` | `var(--op-surface)` | Rest background color. |
| `--op-ptt-button-border` | `var(--op-border)` | Rest border color. |
| `--op-ptt-button-color` | `var(--op-text-primary)` | Rest icon and label color. |
| `--op-ptt-button-active-border` | `var(--op-accent)` | Active transmitting border. |
| `--op-ptt-button-locked-border` | `var(--op-success)` | Locked hands-free border. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `ptt-button.idle` | `"PUSH TO TALK"` | Resting label. |
| `ptt-button.talking` | `"TALKING"` | Active transmitting label. |
| `ptt-button.locked` | `"HANDS-FREE"` | Locked transmission label. |
| `ptt-button.hint` | `"Slide up to lock"` | Hint displayed while holding. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `ptt-button.mic` | `icons::ARROW_UPWARD` | Transmit direction icon. |
| `ptt-button.lock` | `icons::LOCK` | Hands-free lock icon. |

## Accessibility

* Renders as an HTML `<button>` element with `aria-pressed` mirroring the transmitting state.
* Accessible status announcements reflect idle, active, and locked states via `aria-label`.
* Keyboard focus is visibly marked with high-contrast outline.
