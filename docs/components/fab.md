# Fab

Floating action button that expands into a stack of actions.
The button stays anchored on screen or within a container, revealing contextual actions upon activation.

Status: **draft** on web.

## Use

* Highlighting a key contextual workflow in a view.
* Expanding into secondary speed-dial operations on demand.

## Geometry

| Property | Value |
| :--- | :--- |
| Size | `row_standard` (`10 DU`). |
| Action gap | `gap_small` (`2 DU`). |
| Corner radius | `0 DU`. |
| Action stack | As wide as its widest action, never narrower than the trigger; the shared [Popover](popover.md) `accent` frame. |

## Behavior

* Clicking the main FAB toggles the anchored overlay.
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
| `actions` | `Vec<FabAction>` | required | Actions shown in the speed dial list. |
| `open` | bool signal | internal | Expansion state signal. |
| `enabled` | bool | `true` | Enables interaction. |
| `side` | `LayerSide` signal | `Below` | Preferred side of the trigger: `Above`, `Below`, `Left` or `Right`. |
| `align` | `LayerAlign` signal | `Center` | Preferred alignment across that side: `Start`, `Center` or `End`. |
| `label` | string | `fab.label` | Accessible label for the main trigger. |
| `icon` | icon | `fab.open` | Icon shown in the main trigger. |
| `size` | DU metric | `row_standard` | Diameter or size of the trigger. |
| `gap` | DU metric | `gap_small` | Gap between stacked actions. |
| `content` | `Slot` | none | Custom content replacing trigger text. |
| `leading` | `Slot` | none | Leading content inside trigger. |
| `trailing` | `Slot` | none | Trailing content inside trigger. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-fab-size` | `row_standard` (`10 DU`) |
| `gap` | DU | `--op-fab-gap` | `gap_small` (`2 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-fab-background` | `surface` |
| `--op-fab-text` | `text_primary` |
| `--op-fab-border` | `accent` |

| Slot | Place |
| :--- | :--- |
| `leading` | Leading area inside the trigger button. |
| `trailing` | Trailing area inside the trigger button. |
| `content` | Replaces the trigger button text. |

`class` and `style` apply to the root wrapper.

### Strings

| Key | Default |
| :--- | :--- |
| `fab.label` | `Actions` |

### Icons

| Key | Default |
| :--- | :--- |
| `fab.open` | `add` |

## Accessibility

The trigger button sets `aria-haspopup="dialog"` and `aria-expanded`.
Opening the speed dial automatically focuses the first action button.
