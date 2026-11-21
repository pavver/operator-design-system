# LoadingOverlay

Semi-transparent backdrop with a spinner and status message covering its parent container.
It completely blocks user interaction with the covered area while asynchronous operations are running.

Status: **draft** on web.

## Use

Use `LoadingOverlay` over cards, tables, forms, or modal views when a background network operation is modifying their contents.
Do not use `LoadingOverlay` for brief micro-interactions inside buttons; use [ActionButton](action-button.md) pending state instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Bounds | Stretches across the parent container (`inset: 0`). |
| Spinner size | `8 DU` (`--op-loading-overlay-spinner-size`). |
| Spinner stroke | `0.5 DU` (`--op-loading-overlay-spinner-stroke`). |

## Behavior

* When `showing` is `true`, the overlay appears immediately and captures all pointer events.
* It renders a centered [ProgressIndicator](primitives.md) and an optional descriptive label.
* If used with children, it wraps the children in a relatively positioned container automatically.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `showing` | signal bool | required | Activates the overlay and blocks input. |
| `label` | string | `loading-overlay.loading` | Text displayed below the spinner. |
| `spinner_size` | DU | `8 DU` | Spinner diameter. |
| `spinner_stroke` | DU | `0.5 DU` | Spinner stroke thickness. |
| `gap` | DU | `2 DU` | Spacing between spinner and text. |
| `font_size` | TU | `--op-loading-overlay-font-size` | Label text size. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `spinner_size` | DU | `--op-loading-overlay-spinner-size` | `8 DU` |
| `spinner_stroke` | DU | `--op-loading-overlay-spinner-stroke` | `0.5 DU` |
| `gap` | DU | `--op-loading-overlay-gap` | `2 DU` |
| `font_size` | TU | `--op-loading-overlay-font-size` | `inherit` |

| Color variable | Default |
| :--- | :--- |
| `--op-loading-overlay-background` | `var(--op-overlay-scrim, rgba(0, 0, 0, 0.6))` |
| `--op-loading-overlay-spinner` | `var(--op-accent)` |
| `--op-loading-overlay-text` | `var(--op-text-primary)` |

`class` and `style` apply to the overlay `div`.

### Strings

| Key | Default |
| :--- | :--- |
| `loading-overlay.loading` | `"Loading..."` |

### Icons

`LoadingOverlay` uses the vector progress ring from `ProgressIndicator`.

## Accessibility

`LoadingOverlay` carries `role="status"` with `aria-live="polite"` and sets `aria-busy="true"` on the parent area while active.
User input within the covered zone is completely suspended.
