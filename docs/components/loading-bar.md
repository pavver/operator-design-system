# LoadingBar

Thin animated progress bar pinned to the edge of a container or page viewport.
Driven by a signal to show asynchronous page-level background activity.

Status: **draft** on web.

## Use

Use `LoadingBar` at the top of an application layout, navigation shell, or card container to signal that background data fetching is occurring without interrupting user actions.
Do not use `LoadingBar` when user interaction must be blocked; use [LoadingOverlay](loading-overlay.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Thickness | `0.5 DU` (`--op-loading-bar-thickness`). |
| Position | Attached to the `top` or `bottom` edge of the nearest positioned ancestor. |

## Behavior

* Fades into view when `pending` becomes `true`.
* Displays a continuous linear sweep gradient across the bar.
* Fades out smoothly when `pending` becomes `false`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `pending` | signal bool | required | Controls whether the bar is active and visible. |
| `position` | LoadingBarPosition | `Top` | Placement edge (`Top` or `Bottom`). |
| `thickness` | DU | `0.5 DU` | Bar thickness in DU. |
| `label` | string | `loading-bar.loading` | Accessible description. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `thickness` | DU | `--op-loading-bar-thickness` | `0.5 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-loading-bar-color` | `var(--op-accent)` |

`class` and `style` apply to the root `div`.

### Strings

| Key | Default |
| :--- | :--- |
| `loading-bar.loading` | `"Loading page"` |

### Icons

`LoadingBar` uses no icons.

## Accessibility

`LoadingBar` carries `role="progressbar"` with `aria-busy="true"` and an accessible label.
It does not intercept pointer clicks (`pointer-events: none`).
