# Knob

Circular value control with pointer capture and keyboard stepping.
Allows radial tuning of continuous or stepped numeric values.

Status: **draft** on web.

## Use

* Audio dials, gauge controls, and compact parameter tuning.
* Rotary adjustments on touch and pointer screens.

## Geometry

| Property | Value |
| :--- | :--- |
| Size | `24 DU`. |
| Stroke thickness | `pad_small` (`1.5 DU`). |
| Shape | Circular boundary. Round exception allowed. |

## Behavior

* Dragging around the center adjusts the value radially.
* Pointer capture ensures continuous dragging outside the control bounds.
* Arrow keys step the value by `step`.
* Page Up and Page Down jump by 10 steps.
* Home and End snap to minimum and maximum.
* An async commit action runs on pointer release or keyboard adjustment.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | f32 signal | required | Current numeric value. |
| `min` | f32 | `0.0` | Minimum allowed value. |
| `max` | f32 | `100.0` | Maximum allowed value. |
| `step` | f32 | `1.0` | Stepping increment. |
| `readonly` | bool | `false` | Disallows changing value. |
| `enabled` | bool | `true` | Enables interaction. |
| `state` | `ControlState` | none | Explicit control state override. |
| `on_change` | callback `(f32)` | none | Emits on value update. |
| `on_commit` | `AsyncAction<f32>` | none | Async commit on release with error rollback. |
| `label` | string | `knob.label` | Accessible slider label. |
| `size` | DU metric | `24 DU` | Diameter of the circular control. |
| `thickness` | DU metric | `pad_small` | Stroke width of track and arc. |
| `font_size` | TU metric | `type_body_size` | Font size of centered text. |
| `content` | `Slot` | none | Custom center content replacing default readout. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-knob-size` | `24 DU` |
| `thickness` | DU | `--op-knob-thickness` | `pad_small` (`1.5 DU`) |
| `font_size` | TU | `--op-knob-font-size` | `type_body_size` |

| Color variable | Default |
| :--- | :--- |
| `--op-knob-text` | `text_primary` |
| `--op-knob-background` | `surface` |
| `--op-knob-border` | `border` |
| `--op-knob-accent` | `accent` |
| `--op-knob-disabled-text` | `text_disabled` |

| Slot | Place |
| :--- | :--- |
| `content` | Replaces the centered text display. |

`class` and `style` apply to the root element.

### Strings

| Key | Default |
| :--- | :--- |
| `knob.label` | `Value` |

### Icons

Knob does not define built-in icons.

## Accessibility

Knob carries the `slider` role with `aria-valuemin`, `aria-valuemax`, and `aria-valuenow`.
Keyboard arrows and paging keys provide standard accessible value manipulation.
