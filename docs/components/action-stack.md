# ActionStack

A button that expands into a stack of asynchronous actions.
It sits in the layout flow like any button; each action in the stack runs on its own and shows its own pending state and result.

Unlike [Menu](menu.md), whose items are commands that close the menu on selection, every item here is an [ActionButton](action-button.md) with its own async feedback.

Status: **reviewed** on web.

## Use

* A few related asynchronous operations behind one button, such as the actions of a camera or a stream.
* Secondary operations that should not take room in the layout until needed.

## Geometry

| Property | Value |
| :--- | :--- |
| Size | `row_standard` (`10 DU`). |
| Action gap | `gap_small` (`2 DU`). |
| Corner radius | `0 DU`. |
| Action stack | As wide as its widest action, never narrower than the trigger; the shared [Popover](popover.md) `accent` frame. |

## Behavior

* Clicking the trigger toggles the anchored stack.
* With `icon_only`, the trigger is a square [IconButton](icon-button.md); [SplitButton](split-button.md) uses this mode for its toggle.
* The stack opens on the `side` of the trigger with the `align` alignment: below and centered by default.
* `align` across a side above or below: `Center`, `Start` (left edges aligned, it sticks out to the right) or `End` (right edges aligned, it sticks out to the left).
* `align` across a side to the left or right: `Center`, `Start` (top edges aligned, it sticks out downwards) or `End` (bottom edges aligned, it sticks out upwards).
* Both are preferences that bend to keep the stack on screen:
  * it opens on the opposite side when it fits only there;
  * a stack to the left or right falls back below, then above, when it fits on neither of those sides;
  * it shifts along the trigger near a screen edge;
  * when it fits nowhere, it takes the roomier of above and below and scrolls with the library [Scrollbar](scrollbar.md).
* While open, clicking an action runs it: the action and the trigger show [Pending](../foundations/states.md) (running segment, then the delayed spinner) and the other actions are blocked.
* While the action runs, the stack stays open: presses outside, Escape and scrolling do not close it, and it follows the trigger.
* Success flashes on the action and then closes the layer; an error flashes and keeps it open.
* Actions close when disabled, when clicking outside, or when the page or another container scrolls; scrolling the stack itself keeps it open.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `actions` | `Vec<ActionStackItem>` | required | Actions shown in the stack. |
| `open` | bool signal | internal | Expansion state signal. |
| `enabled` | bool | `true` | Enables interaction. |
| `side` | `LayerSide` signal | `Below` | Preferred side of the trigger: `Above`, `Below`, `Left` or `Right`. |
| `align` | `LayerAlign` signal | `Center` | Preferred alignment across that side: `Start`, `Center` or `End`. |
| `label` | string | `action-stack.label` | Accessible label for the main trigger. |
| `icon` | icon | `action-stack.open` | Icon shown in the main trigger. |
| `size` | DU metric | `row_standard` | Diameter or size of the trigger. |
| `icon_size` | DU metric | `icon_small` (`4 DU`) | Size of the trigger icon. |
| `gap` | DU metric | `gap_small` | Gap between stacked actions. |
| `icon_only` | bool | `false` | Square trigger with only the icon; `label` becomes its accessible name and tooltip. |
| `anchor_to` | `NodeRef<Div>` | the trigger | Element the stack opens from, such as a whole composite control. |
| `busy` | bool signal | none | Set to true while an action runs, so a composite control can block its other parts. |
| `content` | `Slot` | none | Custom content replacing trigger text. |
| `leading` | `Slot` | none | Leading content inside trigger. |
| `trailing` | `Slot` | none | Trailing content inside trigger. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-action-stack-size` | `row_standard` (`10 DU`) |
| `icon_size` | DU | `--op-action-stack-icon-size` | `icon_small` (`4 DU`) |
| `gap` | DU | `--op-action-stack-gap` | `gap_small` (`2 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-action-stack-background` | `surface` |
| `--op-action-stack-text` | `text_primary` |
| `--op-action-stack-border` | `accent` |
| `--op-action-stack-focus` | `accent` |
| `--op-action-stack-focus-background` | `hover` |

| Slot | Place |
| :--- | :--- |
| `leading` | Leading area inside the trigger button. |
| `trailing` | Trailing area inside the trigger button. |
| `content` | Replaces the trigger button text. |

`class` and `style` apply to the root wrapper.

### Strings

| Key | Default |
| :--- | :--- |
| `action-stack.label` | `Actions` |

### Icons

| Key | Default |
| :--- | :--- |
| `action-stack.open` | `add` |

## Accessibility

The trigger button sets `aria-haspopup="menu"` and `aria-expanded`.
The stack has `role="menu"` with the trigger label as its name, and each action has `role="menuitem"`.
Opening the stack focuses the first action, or the last one when it was opened with Arrow Up.
The focused action shows a `0.5 DU` `accent` ring inside its bounds, so the stack's scroll view never clips it, and a `hover` fill while it is at rest.

| Key | Where | Action |
| :--- | :--- | :--- |
| Enter, Space | Trigger | Opens or closes the stack. |
| Arrow Down | Closed trigger | Opens the stack and focuses the first action. |
| Arrow Up | Closed trigger | Opens the stack and focuses the last action. |
| Arrow Down, Arrow Up | Stack | Moves to the next or previous enabled action and wraps. |
| Home, End | Stack | Moves to the first or last enabled action. |
| Enter, Space | Action | Runs the action. |
| Escape | Stack | Closes the stack and returns focus to the trigger. |
| Tab | Stack | Closes the stack and returns focus to the trigger. |

While an action runs, Escape and Tab do not close the stack.
