# Parallax

Visual banner whose background image translates at a fraction of scroll velocity.

Status: **draft** on web.

## Use

Use Parallax to add depth to landing headers, hero banners, or promotional sections.
It automatically tracks window scrolling and disables translation when the user prefers reduced motion.

## Geometry

| Property | Default metric | Variable |
| :--- | :--- | :--- |
| Height | `75 DU` | `--op-parallax-height` |

## Behavior

* The background image translates vertically slower than the scroll rate based on the configured speed factor.
* When `prefers-reduced-motion` is detected in media queries, parallax translation is disabled and the image remains centered.
* Children content is overlaid on top of the image and scrolls naturally.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `src` | string signal | required | Source URL or data URI of the background image. |
| `alt` | string | none | Alternative text for the image. |
| `height` | DU metric | `75 DU` | Height of the parallax container. |
| `speed` | f32 signal | `0.5` | Fractional speed factor (0.0 to 1.0). |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-parallax-height` | `75 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-parallax-background` | `surface_alt` |

## Accessibility

* Parallax translation automatically deactivates when `(prefers-reduced-motion: reduce)` matches.
* Background images provide optional `alt` descriptions.
