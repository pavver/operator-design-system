# PtzPad

Directional camera pan/tilt/zoom controller with interactive joystick drag, arrow navigation, and Operator state feedback.

Status: **draft** on web.

## Use

Use `PtzPad` to control motorized or digital pan, tilt, and zoom cameras, robotics, or directional viewports.
Do not use `PtzPad` for simple linear stepped adjustments; use [Stepper](stepper.md) or [Slider](slider.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | Circular (`50%` radius exception documented for directional joystick). |
| Default pad diameter | `40 DU`. |
| Thumb diameter | `15 DU`. |
| Border | Standard `0.25 DU` inside the bounds. |

## Behavior

* Dragging the central thumb or clicking on the pad activates directional panning and tilting.
* Released pointers snap back smoothly to the center and emit a stop command.
* Eight directional arrow indicators highlight actively to reflect current orientation.
* Supports keyboard navigation using the arrow keys (`ArrowUp`, `ArrowDown`, `ArrowLeft`, `ArrowRight`).
* Keys `+` and `-` trigger zoom in and zoom out actions.
* Async actions reflect `Pending`, `Success`, and `Error` states directly on the pad and center thumb dot.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_move` | callback `(PtzDirection, f32)` | none | Emits active direction and normalized speed `[0.0, 1.0]`. |
| `on_zoom` | callback `PtzZoomDirection` | none | Emits zoom in or zoom out trigger. |
| `on_async_command` | AsyncAction | none | Asynchronous command handler. |
| `state` | ControlState | none | Explicit control state override. |
| `enabled` | bool | `true` | Enabled flag; blocks interaction when false. |
| `show_diagonals` | bool | `true` | Renders diagonal direction arrows. |
| `show_zoom` | bool | `true` | Renders integrated zoom in and zoom out buttons. |
| `size` | DU | `40.0` | Pad outer diameter in DU (`--op-ptz-pad-size`). |
| `thumb_size` | DU | `15.0` | Thumb diameter in DU (`--op-ptz-pad-thumb-size`). |
| `border_width` | DU | `0.25` | Border stroke width in DU (`--op-ptz-pad-border-width`). |
| `label` | string | `"PTZ Camera Control"` | Accessible label. |
| `zoom_in_label` | string | `"Zoom In"` | Accessible label for zoom in. |
| `zoom_out_label` | string | `"Zoom Out"` | Accessible label for zoom out. |
| `center_slot` | Slot | none | Slot replacing the center thumb indicator dot. |
| `zoom_in_slot` | Slot | none | Slot replacing the default zoom in button. |
| `zoom_out_slot` | Slot | none | Slot replacing the default zoom out button. |
| `extra_controls` | Slot | none | Custom controls rendered beneath the pad. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-ptz-pad-size` | `40 DU` |
| `thumb_size` | DU | `--op-ptz-pad-thumb-size` | `15 DU` |
| `border_width` | DU | `--op-ptz-pad-border-width` | `0.25 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-ptz-pad-bg` | `var(--op-surface-alt)` |
| `--op-ptz-pad-border` | `var(--op-border)` |
| `--op-ptz-pad-active-color` | `var(--op-accent)` |
| `--op-ptz-pad-inactive-color` | `rgba(255, 255, 255, 0.4)` |
| `--op-ptz-thumb-bg` | `var(--op-surface)` |
| `--op-ptz-thumb-border` | `var(--op-border-strong)` |
| `--op-ptz-pad-disabled-bg` | `var(--op-disabled-surface)` |
| `--op-ptz-pad-disabled-border` | `var(--op-disabled-border)` |

`class` and `style` apply to the root element.

### Strings

| Key | Default |
| :--- | :--- |
| `ptz-pad.label` | `"PTZ Camera Control"` |
| `ptz-pad.zoom-in` | `"Zoom In"` |
| `ptz-pad.zoom-out` | `"Zoom Out"` |

### Icons

| Key | Default |
| :--- | :--- |
| Up arrow | `icons::ARROW_UPWARD` |
| Down arrow | `icons::ARROW_DOWNWARD` |
| Left arrow | `icons::ARROW_BACK` |
| Right arrow | `icons::CHEVRON_RIGHT` |
| Zoom in | `icons::ADD` |
| Zoom out | `icons::REMOVE` |

## Accessibility

The pad is focusable and receives keyboard arrow inputs.
Screen readers receive the descriptive label provided by `label`.
Zoom buttons are accessible icon buttons with explicit labels.
