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
* The stack opens centered above the trigger, or below it with `below`.
* It flips to the other side when it fits only there, and shifts sideways near a screen edge.
* When it fits on neither side, it takes the side with more room and scrolls with the library [Scrollbar](scrollbar.md).
* While open, clicking any action executes it and closes the layer.
* Actions close when disabled, when clicking outside, or when the page or another container scrolls; scrolling the stack itself keeps it open.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `actions` | `Vec<FabAction>` | required | Actions shown in the speed dial list. |
| `open` | bool signal | internal | Expansion state signal. |
| `enabled` | bool | `true` | Enables interaction. |
| `below` | bool | `false` | Prefers the side below the FAB instead of above. |
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
