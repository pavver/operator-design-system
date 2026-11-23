# Primitives

Small building blocks used by other components.

Status: **reviewed** on web.

## Icon

* Renders one icon from [tokens/icons](../../tokens/icons), see [iconography.md](../foundations/iconography.md).
* Size in DU, `icon_large` (`6 DU`) by default.
* Uses the current content color unless a color is given.
* Without a label it is hidden from assistive technology; with a label it is an image with that name.

## ProgressIndicator

* A circular arc covering 75% of the circle, rotating once per `spinner_lap` (`900 ms`), linear.
* Size and stroke in DU; `6 DU` with a `0.5 DU` stroke by default.
* Inside controls it uses `spinner_stroke` (`0.375 DU`) and the `pending` color.
* Round line caps.
* Async controls do not use it directly while `Pending`; they use `PendingIndicator`.

## PendingIndicator

* The single spinner shown inside async controls in the `Pending` state.
* Wraps `ProgressIndicator` with `spinner_stroke` (`0.375 DU`) and the `pending` color.
* Size in DU, `icon_small` (`4 DU`) by default.
* Fades in over `pending_indicator_fade` (`200 ms`).
* Gated by `use_pending_indicator`, which turns `true` once `Pending` has lasted `pending_indicator_delay` (`700 ms`).
* Replacing its visual changes the spinner in every control at once.
* Behavior is defined in [states.md](../foundations/states.md#delayed-spinner).

## Divider

* A horizontal line, full width, `0.25 DU` thick, in `divider`.

## Card

* A rectangular container with `pad_card` (`3 DU`) padding and a `0.25 DU` `border` outline.
* `surface` background by default; `surface_alt` for sunken displays.
* Corner radius `0 DU`, no shadow.
* Supports Quasar-style container slots: `header`, `actions`, `footer`, with default children as body.

## API

| Component | Properties |
| :--- | :--- |
| `Icon` | `icon`, `size` (`icon_large`, `6 DU`), `color`, `label`. |
| `ProgressIndicator` | `size` (`6 DU`), `stroke` (`0.5 DU`), `color`, `label` (`progress-indicator.loading`). |
| `PendingIndicator` | `size` (`4 DU`), `stroke` (`0.375 DU`). |
| `Divider` | `thickness` (`0.25 DU`). |
| `Card` | `alt` (`false`), `header`, `actions`, `footer`, `padding` (`3 DU`), `gap` (`2 DU`), `border_width` (`0.25 DU`), `children`. |

## Customization

| Metric prop | Unit | Variable | Default | Component |
| :--- | :--- | :--- | :--- | :--- |
| `size` | DU | `--op-icon-size` | `icon_large` (`6 DU`) | Icon |
| `size` | DU | `--op-progress-indicator-size` | `6 DU` | ProgressIndicator |
| `stroke` | DU | `--op-progress-indicator-stroke` | `0.5 DU` | ProgressIndicator |
| `size` | DU | `--op-pending-indicator-size` | `icon_small` (`4 DU`) | PendingIndicator |
| `stroke` | DU | `--op-pending-indicator-stroke` | `spinner_stroke` (`0.375 DU`) | PendingIndicator |
| `thickness` | DU | `--op-divider-thickness` | `border` (`0.25 DU`) | Divider |
| `padding` | DU | `--op-card-padding` | `pad_card` (`3 DU`) | Card |
| `gap` | DU | `--op-card-gap` | `gutter` (`2 DU`) | Card |
| `border_width` | DU | `--op-card-border-width` | `border` (`0.25 DU`) | Card |

| Color variable | Default | Component |
| :--- | :--- | :--- |
| `--op-progress-indicator-color` | `accent` | ProgressIndicator |
| `--op-pending-indicator-color` | `pending` | PendingIndicator |
| `--op-divider-color` | `divider` | Divider |
| `--op-card-background` | `surface` | Card |
| `--op-card-alt-background` | `surface_alt` | Card |
| `--op-card-border` | `border` | Card |

| Slot | Place | Component |
| :--- | :--- | :--- |
| `header` | Card header section above the body. | Card |
| `actions` | Card action section below the body. | Card |
| `footer` | Card footer section below the actions. | Card |

`class` and `style` apply to the root of each component.

### Strings

| Key | Default |
| :--- | :--- |
| `progress-indicator.loading` | `Loading` |
