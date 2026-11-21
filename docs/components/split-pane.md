# SplitPane

Two panes with a draggable divider, such as a list beside its details.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Divider line | `0.25 DU` `border`; `0.5 DU` `accent` on hover, focus, and drag. |
| Hit area | `2 DU` across the line. |
| Grip | `1 DU` × `6 DU` of `border_strong` stripes in the middle of the line. |
| Minimum pane | `min_size` (default `20 DU`) on each side. |
| Orientation | Side by side, or stacked with `vertical`. |

## Behavior

* Dragging the divider sets the share of the first pane; the panes never shrink below `min_size`.
* With the divider focused, the arrow keys move it by 2%, by 10% with Shift; Home and End move it to the limits.
* A double-click or Enter restores the initial share.
* Each pane scrolls on its own.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `start` | slot | required | First pane. |
| `end` | slot | required | Second pane. |
| `ratio` | float signal | `0.5` | Share of the first pane. |
| `vertical` | bool | `false` | Stacks the panes. |
| `min_size` | DU | `20` | Minimum pane size. |
| `handle_size` | DU | `gutter` | Size of the divider hit area. |
| `divider_width` | DU | `border` | Divider line width. |
| `divider_active_width` | DU | `border_thick` | Active/hovered divider line width. |
| `grip_width` | DU | `pad_tight` | Grip width. |
| `grip_length` | DU | `icon_large` | Grip length. |
| `grip` | slot | none | Custom grip slot. |
| `label` | string | `split-pane.handle` | Accessible label for the separator handle. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `min_size` | DU | `--op-split-pane-min-size` | `20 DU` |
| `handle_size` | DU | `--op-split-pane-handle-size` | `gutter` |
| `divider_width` | DU | `--op-split-pane-divider-width` | `border` |
| `divider_active_width` | DU | `--op-split-pane-divider-active-width` | `border_thick` |
| `grip_width` | DU | `--op-split-pane-grip-width` | `pad_tight` |
| `grip_length` | DU | `--op-split-pane-grip-length` | `icon_large` |

| Color variable | Default |
| :--- | :--- |
| `--op-split-pane-divider-color` | `border` |
| `--op-split-pane-divider-active-color` | `accent` |
| `--op-split-pane-grip-color` | `border_strong` |

| Slot | Place |
| :--- | :--- |
| `start` | First pane content. |
| `end` | Second pane content. |
| `grip` | Replaces the grip stripes on the handle. |

`class` and `style` apply to the root split-pane element.

### Strings

| Key | Default |
| :--- | :--- |
| `split-pane.handle` | `Splitter` |

### Icons

SplitPane has no built-in semantic icons.

## Accessibility

The divider is a focusable `separator` with `aria-valuenow` in percent.
