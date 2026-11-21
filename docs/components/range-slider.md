# RangeSlider

Two ordered thumbs on the shared Slider track and thumb visuals.
Allows choosing an interval bounded by a lower and an upper value.

Status: **draft** on web.

## Use

* Setting price, time, or frequency ranges.
* Defining numeric filtering brackets.

## Geometry

| Property | Value |
| :--- | :--- |
| Height | `row_compact` (`8 DU`). |
| Rail inset | `3 DU`. |
| Track height | `2 DU`. |
| Thumb | Circle, `icon_large` (`6 DU`) diameter. Round exception allowed. |
| Idle dot in thumb | Circle, `pad_small` (`1.5 DU`) diameter. |

## Behavior

* Clicking or dragging selects and moves the nearest thumb.
* Thumbs maintain ordering: the lower thumb cannot exceed the upper thumb.
* Dragging captures pointer events for smooth dragging outside element bounds.
* Releasing or stepping with the keyboard invokes the async commit handler.
* On error, values roll back to the previously committed values.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | `(f32, f32)` signal | required | Lower and upper values tuple. |
| `min` | f32 | `0.0` | Minimum range bound. |
| `max` | f32 | `100.0` | Maximum range bound. |
| `step` | f32 | `1.0` | Stepping increment. |
| `readonly` | bool | `false` | Prevents adjustments. |
| `enabled` | bool | `true` | Enables input. |
| `state` | `ControlState` | none | Explicit control state override. |
| `on_change` | callback `((f32, f32))` | none | Real-time interval updates. |
| `on_commit` | `AsyncAction<(f32, f32)>` | none | Async commit handler with rollback on failure. |
| `label` | string | `range-slider.label` | Accessible group label. |
| `lower_label` | string | `range-slider.lower` | Accessible label for lower thumb. |
| `upper_label` | string | `range-slider.upper` | Accessible label for upper thumb. |
| `height` | DU metric | `row_compact` | Overall height of the slider. |
| `rail_inset` | DU metric | `3 DU` | Inset for thumb boundaries. |
| `track_height` | DU metric | `2 DU` | Thickness of the track rail. |
| `thumb_size` | DU metric | `icon_large` | Diameter of the thumbs. |
| `dot_size` | DU metric | `pad_small` | Diameter of the thumb center dot. |
| `width` | DU metric | none | Explicit width override. |
| `thumb` | `Slot` | none | Replaces standard thumb visual. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-slider-height` | `row_compact` (`8 DU`) |
| `rail_inset` | DU | `--op-slider-rail-inset` | `3 DU` |
| `track_height` | DU | `--op-slider-track-height` | `2 DU` |
| `thumb_size` | DU | `--op-slider-thumb-size` | `icon_large` (`6 DU`) |
| `dot_size` | DU | `--op-slider-dot-size` | `pad_small` (`1.5 DU`) |
| `width` | DU | `--op-slider-width` | fills parent |

| Color variable | Default |
| :--- | :--- |
| `--op-slider-accent` | `accent` |
| `--op-slider-border` | `border` |

| Slot | Place |
| :--- | :--- |
| `thumb` | Custom visual inside the thumb slot. |

`class` and `style` apply to the root group element.

### Strings

| Key | Default |
| :--- | :--- |
| `range-slider.label` | `Range` |
| `range-slider.lower` | `Lower value` |
| `range-slider.upper` | `Upper value` |

### Icons

RangeSlider does not define built-in icons.

## Accessibility

The root represents a `group` containing two separate `slider` controls.
Each thumb provides `aria-valuemin`, `aria-valuemax`, and `aria-valuenow`, with independent arrow key focus.
