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
| `start` | view | required | First pane. |
| `end` | view | required | Second pane. |
| `ratio` | float signal | `0.5` | Share of the first pane. |
| `vertical` | bool | `false` | Stacks the panes. |
| `min_size` | DU | `20` | Minimum pane size. |

## Accessibility

The divider is a focusable `separator` with `aria-valuenow` in percent.
