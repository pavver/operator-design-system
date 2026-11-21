# Image

Enhanced image container supporting aspect ratio reservations, object fit modes, skeleton loading placeholders, error fallbacks, and caption overlays.

Status: **draft** on web.

## Use

Use `Image` for displaying photographs, previews, illustrations, and media assets where layout stability during image downloads is critical.
Do not use `Image` for small inline iconography; use [Icon](primitives.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Dimensions | Configurable with `width`, `height`, and proportional `ratio`. |

## Behavior

* While the remote source image is downloading, a [Skeleton](skeleton.md) placeholder is rendered.
* When the image loads successfully, it fades into view.
* If downloading fails, the error fallback displays an error icon and message.
* Changing `src` resets the loading and error states automatically.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `src` | signal string | required | Image URL. |
| `alt` | string | `""` | Accessible alternative text. |
| `ratio` | f32 | none | Aspect ratio (width / height). |
| `fit` | ImageFit | `Cover` | Fitting mode (`Cover`, `Contain`, `Fill`, `None`, `ScaleDown`). |
| `position` | string | `"center"` | CSS object-position value. |
| `loading` | string | `"lazy"` | Native browser loading attribute. |
| `caption` | string | none | Overlay caption text. |
| `caption_slot` | Slot | none | Custom caption slot. |
| `loading_slot` | Slot | none | Custom loading slot replacing the default skeleton. |
| `error_slot` | Slot | none | Custom error slot replacing the default error state. |
| `error_label` | string | `image.error` | Error message string. |
| `width` | DU | none | Width in DU. |
| `height` | DU | none | Height in DU. |
| `on_load` | callback `()` | none | Triggered on successful image load. |
| `on_error` | callback `()` | none | Triggered on load error. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-image-width` | `100%` |
| `height` | DU | `--op-image-height` | `auto` |

| Color variable | Default |
| :--- | :--- |
| `--op-image-background` | `var(--op-surface-subtle)` |
| `--op-image-placeholder-bg` | `var(--op-surface-subtle)` |
| `--op-image-caption-bg` | `rgba(0, 0, 0, 0.7)` |
| `--op-image-caption-text` | `#ffffff` |

`class` and `style` apply to the root `div`.

### Strings

| Key | Default |
| :--- | :--- |
| `image.error` | `"Failed to load image"` |

### Icons

| Key | Default |
| :--- | :--- |
| Error fallback icon | `icons::ERROR` |

## Accessibility

The inner `<img>` receives the provided `alt` description.
Decorative placeholders and error fallbacks do not interfere with standard screen reader descriptions.
