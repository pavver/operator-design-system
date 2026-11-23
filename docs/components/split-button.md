# SplitButton

Primary action button with an attached toggle that opens a stack of secondary actions.
Clicking the main button runs the default action, while the toggle opens an [ActionStack](action-stack.md) under the whole control.

Status: **reviewed** on web.

## Use

* Grouping related actions under a single visual control when space is constrained.
* Providing a quick primary shortcut while keeping alternate commands accessible.

## Geometry

| Property | Value |
| :--- | :--- |
| Height | `row_standard` (`10 DU`). |
| Toggle width | `row_standard` (`10 DU`). |
| Icon size | `icon_small` (`4 DU`). |
| Corner radius | `0 DU`. |

## Behavior

* The main button runs the primary action with the [ActionButton](action-button.md) states.
* The toggle is the icon-only trigger of an [ActionStack](action-stack.md) anchored to the whole control.
* The stack opens below by default, aligned to the start edge of the control, and follows the [ActionStack](action-stack.md) placement, scroll and dismissal rules.
* Each secondary action runs with its own pending, success and error feedback, and the stack closes after a success.
* While the primary action runs, the toggle is disabled; while a secondary action runs, the main button is disabled.
* When the control is disabled or `state` is pending or disabled, the toggle is inert.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `actions` | `Vec<ActionStackItem>` | required | Secondary actions in the stack. |
| `on_click` | `AsyncAction` | none | Primary action handler. |
| `text` | string | `split-button.text` | Label of the main button. |
| `enabled` | bool | `true` | Enables interaction. |
| `state` | `ControlState` | none | Explicit state of the main button. |
| `toggle_label` | string | `split-button.toggle` | Accessible name and tooltip of the toggle. |
| `toggle_icon` | icon | `split-button.toggle` | Icon of the toggle. |
| `side` | `LayerSide` | `Below` | Preferred side of the stack. |
| `align` | `LayerAlign` | `Start` | Preferred alignment of the stack across the control. |
| `height` | DU metric | `row_standard` | Height of the entire control. |
| `toggle_size` | DU metric | `row_standard` | Width of the toggle button. |
| `icon_size` | DU metric | `icon_small` (`4 DU`) | Size of the toggle icon. |
| `leading` | `Slot` | none | Leading slot in the main button. |
| `trailing` | `Slot` | none | Trailing slot in the main button. |
| `content` | `Slot` | none | Replaces main button text. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-split-button-height` | `row_standard` (`10 DU`) |
| `toggle_size` | DU | `--op-split-button-toggle-size` | `row_standard` (`10 DU`) |
| `icon_size` | DU | `--op-split-button-icon-size` | `icon_small` (`4 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-split-button-text` | `inherit` |
| `--op-split-button-background` | `surface` |

The toggle takes the hover, focus and disabled colors of [IconButton](icon-button.md), and the stack takes the [ActionStack](action-stack.md) variables.

| Slot | Place |
| :--- | :--- |
| `leading` | Leading area of the main button. |
| `trailing` | Trailing area of the main button. |
| `content` | Custom content replacing the main text. |

`class` and `style` apply to the root element.

### Strings

| Key | Default |
| :--- | :--- |
| `split-button.text` | `Action` |
| `split-button.toggle` | `More actions` |

### Icons

| Key | Default |
| :--- | :--- |
| `split-button.toggle` | `arrow_drop_down` |

## Accessibility

The toggle has `aria-haspopup="menu"`, reports `aria-expanded`, and uses `toggle_label` as its accessible name and tooltip.
The stack is a `menu` of `menuitem` actions with the [ActionStack](action-stack.md) keyboard support: Arrow Down and Arrow Up on the toggle open it, the arrows, Home and End move between actions, and Escape or Tab close it.
