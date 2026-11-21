# SplitButton

Primary action button with an attached dropdown menu toggle.
Clicking the main button executes the default action, while clicking the attached toggle button opens an anchored menu of secondary actions.

Status: **draft** on web.

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

* The main button triggers the primary action directly.
* The attached toggle opens or closes the anchored menu.
* Pressing Arrow Down on the toggle opens the menu.
* When the control is disabled or pending, both the main action and toggle are inert.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | `Vec<MenuItem>` | required | Actions rendered in the dropdown menu. |
| `on_select` | callback `(String)` | required | Emits selected menu item id. |
| `on_click` | `AsyncAction` | none | Primary action handler. |
| `text` | string | `split-button.text` | Label of the main button. |
| `enabled` | bool | `true` | Enables interaction. |
| `state` | `ControlState` | none | Explicit control state override. |
| `toggle_label` | string | `split-button.toggle` | Accessible name for the dropdown toggle. |
| `toggle_icon` | icon | `split-button.toggle` | Semantic icon for dropdown toggle. |
| `height` | DU metric | `row_standard` | Height of the entire control. |
| `toggle_size` | DU metric | `row_standard` | Width of the toggle button. |
| `icon_size` | DU metric | `icon_small` | Size of the toggle icon. |
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
| `--op-split-button-border` | `border` |
| `--op-split-button-hover` | `hover` |
| `--op-split-button-accent` | `accent` |
| `--op-split-button-disabled-text` | `text_disabled` |

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

The toggle button has `aria-haspopup="menu"` and reports `aria-expanded`.
Keyboard arrow navigation on the toggle triggers menu display.
