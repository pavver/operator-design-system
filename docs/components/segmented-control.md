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
   The indicator enters `Pending` and the target item keeps its content until the [delayed spinner](../foundations/states.md#delayed-spinner) replaces it.
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
| `padding` | reactive DU | unset | Overrides `--op-segmented-control-padding`; see Customization. |
| `gap` | reactive DU | unset | Overrides `--op-segmented-control-gap`; see Customization. |
| `icon_size` | reactive DU | unset | Overrides `--op-segmented-control-icon-size`; see Customization. |
| `font_size` | reactive TU | unset | Overrides `--op-segmented-control-font-size`; see Customization. |
| `line_height` | reactive TU | unset | Overrides `--op-segmented-control-line-height`; see Customization. |
| `spinner_stroke` | reactive DU | unset | Overrides `--op-segmented-control-spinner-stroke`; see Customization. |
| `width` | reactive DU | unset | Overrides `--op-segmented-control-width`; see Customization. |
| `class` | reactive string | none | Applied to the root element. |
| `style` | reactive string | none | Applied to the root element. |
| `option` | scoped slot | none | Replaces an option label; receives the item. |
| `before_options` | slot | none | Before the options. |
| `after_options` | slot | none | After the options. |
| `loading_label` | reactive string | `segmented-control.loading` | String override; see Strings. |
| `label` | reactive string | `segmented-control.label` | String override; see Strings. |

`SegmentItem` holds an optional text and an optional icon.
At least one must be present.

## Accessibility

* The container has the `tablist` role.
* Each item has the `tab` role and reports whether it is selected.

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-segmented-control-height` | `calc(var(--op-du) * 9)` |
| `padding` | DU | `--op-segmented-control-padding` | `var(--op-dim-gap-micro)` |
| `gap` | DU | `--op-segmented-control-gap` | `var(--op-dim-pad-small)` |
| `icon_size` | DU | `--op-segmented-control-icon-size` | `var(--op-dim-icon-small)` |
| `font_size` | TU | `--op-segmented-control-font-size` | `var(--op-type-body-small-size)` |
| `line_height` | TU | `--op-segmented-control-line-height` | `var(--op-type-body-small-line)` |
| `spinner_stroke` | DU | `--op-segmented-control-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| `width` | DU | `--op-segmented-control-width` | `100%` |

The existing numeric size props keep their names and DU meaning and now also accept reactive values.

| Color variable | Default |
| :--- | :--- |
| `--op-segmented-control-accent` | `var(--op-accent)` |
| `--op-segmented-control-border` | `var(--op-border)` |
| `--op-segmented-control-emphasis-border` | `var(--op-border-strong)` |
| `--op-segmented-control-disabled-border` | `var(--op-disabled-border)` |
| `--op-segmented-control-disabled-background` | `var(--op-disabled-surface)` |
| `--op-segmented-control-error` | `var(--op-error)` |
| `--op-segmented-control-hover` | `var(--op-hover)` |
| `--op-segmented-control-pending` | `var(--op-pending)` |
| `--op-segmented-control-pressed` | `var(--op-pressed)` |
| `--op-segmented-control-ripple` | `var(--op-ripple)` |
| `--op-segmented-control-success` | `var(--op-success)` |
| `--op-segmented-control-background` | `var(--op-surface)` |
| `--op-segmented-control-background-alt` | `var(--op-surface-alt)` |
| `--op-segmented-control-disabled-text` | `var(--op-text-disabled)` |
| `--op-segmented-control-secondary-text` | `var(--op-text-secondary)` |
| Slot | Place |
| :--- | :--- |
| `option` | Replaces an option label; receives the item. |
| `before_options` | Before the options. |
| `after_options` | After the options. |

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `loading_label` | `segmented-control.loading` | `Loading` |
| `label` | `segmented-control.label` | `Options` |

### Icons

No built-in semantic icons.

Icons supplied as item data or through the existing `icon` prop remain caller-owned.

A new request supersedes an older result timer, so that timer cannot clear the newer feedback.
