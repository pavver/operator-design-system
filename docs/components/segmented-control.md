# SegmentedControl

A row of mutually exclusive options with a sliding selection indicator.
The async variant confirms each selection with a backend and rolls back on error.

Status: **reviewed** on web.

## Use

* Choosing one of two to five short options: a mode, a quality level, a filter.
* Compact toolbars with icon-only or icon-and-text segments.

## Anatomy

```
┌───────────────────────────────────────────┐
│┌─────────────┐                            │
││    Auto     │     Day          Night     │
│└─────────────┘                            │
└───────────────────────────────────────────┘
  indicator      items
```

* **Container:** a sunken track with an outline.
* **Indicator:** one segment wide; slides under the items and owns a state border.
* **Items:** text, an icon, or an icon followed by text.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Fills the parent; items share it equally. |
| Height | `9 DU` by default. |
| Inner padding | `gap_micro` (`0.5 DU`). |
| Corner radius | `0 DU`. |
| Item icon | `icon_small` (`4 DU`). |
| Icon-to-text gap | `pad_small` (`1.5 DU`). |
| Text | `body_small` style; weight 500, selected 700. |

## Colors

| Part | Color |
| :--- | :--- |
| Container | `surface_alt` with a `border` outline. |
| Indicator | `accent` at `15%`, `accent` outline. |
| Indicator (async) | `accent` at `15%`, `border` outline, so the state border stays readable. |
| Unselected item | `text_secondary`. |
| Selected item | `accent`. |
| Disabled | `disabled_surface`, `disabled_border`, `text_disabled`; no indicator. |

## Behavior

### Synchronous

* Clicking an item selects it immediately.
* The indicator slides to the new item over `segment_slide` (`150 ms`), linear.
* Item text colors change over the same duration and curve, so the accent travels with the indicator.

### Async

1. Clicking an item that is not selected moves the indicator to it.
   The indicator enters `Pending` and the target item shows a `pending` spinner instead of its content.
2. While pending, all items ignore input.
3. On **success** the selection is committed.
   The indicator and the selected item flash `Success` for `500 ms`, then fade back to the selected look.
4. On **error** the indicator slides back to the previous item with the same `150 ms` linear movement.
   The indicator and the previous item flash `Error` for `600 ms`, then fade back to the selected look.

The slide animation is independent of the state border.
Result states change colors instantly, but they never cancel or shorten the slide.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | list of `SegmentItem` | required | The options. |
| `selected` | read-write index | required | The committed selection. |
| `enabled` | bool | `true` | Interactivity. |
| `on_select` | async `(usize) -> ActionResult` | none | Enables the async variant. |
| `height` | DU | `9` | Control height. |

`SegmentItem` holds an optional text and an optional icon.
At least one must be present.

## Accessibility

* The container has the `tablist` role.
* Each item has the `tab` role and reports whether it is selected.
