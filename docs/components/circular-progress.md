# CircularProgress

Ring indicator displaying task completion percentage or indeterminate background activity.
It supports optional center numerical labels or arbitrary slots.

Status: **draft** on web.

## Use

Use `CircularProgress` when displaying completion in dashboards, stats widgets, or compact cards where linear bars would consume too much horizontal space.
Do not use `CircularProgress` for full-page loading bars; use [LoadingBar](loading-bar.md) or [LinearProgress](linear-progress.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Default size | `10 DU` (`--op-circular-progress-size`). |
| Thickness | `0.75 DU` (`--op-circular-progress-thickness`). |

## Behavior

* When `value` is provided, the ring renders deterministically from 0% to 100%.
* When `value` is omitted, the ring spins continuously in indeterminate mode.
* The ring color reflects the semantic `state` (`Pending`, `Success`, `Error`).
* Center numeric label scales proportionally with ring size or can be explicitly overridden via `font_size`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | option signal f32 | `None` | Progress value from `0.0` to `max`. |
| `max` | signal f32 | `100.0` | Maximum progress value. |
| `state` | signal `ControlState` | none | Tinting control state. |
| `show_value` | bool | `false` | Shows progress label in center. |
| `format` | CircularProgressFormat | `Percent` | Formats as `Percent` or `Count`. |
| `size` | DU | `10 DU` | Ring outer diameter. |
| `thickness` | DU | `0.75 DU` | Stroke thickness. |
| `font_size` | TU | `--op-circular-progress-font-size` | Center text font size. |
| `label` | string | `circular-progress.label` | Accessible description. |
| `center_slot` | Slot | none | Custom slot in the center of the ring. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-circular-progress-size` | `10 DU` |
| `thickness` | DU | `--op-circular-progress-thickness` | `0.75 DU` |
| `font_size` | TU | `--op-circular-progress-font-size` | `0.75 * size` |

| Color variable | Default |
| :--- | :--- |
| `--op-circular-progress-color` | `var(--op-accent)` |
| `--op-circular-progress-track-color` | `var(--op-surface-raised)` |
| `--op-circular-progress-text` | `var(--op-text-primary)` |

`class` and `style` apply to the root `div`.

### Strings

| Key | Default |
| :--- | :--- |
| `circular-progress.label` | `"Progress"` |

### Icons

`CircularProgress` renders no icons.

## Accessibility

`CircularProgress` implements `role="progressbar"` with `aria-valuenow`, `aria-valuemin`, and `aria-valuemax` attributes.
Screen readers announce the progress label and numeric state changes.
