# Video

Responsive frame for embedding external video streams and players.
It preserves aspect ratio across varying screen viewports.

Status: **draft** on web.

## Use

Use `Video` for embedding video feeds, training material, tutorials, or surveillance camera monitors.
Do not use `Video` for local static images; use [Image](image.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Default ratio | `16 / 9` (`1.7778`). |
| Frame bounds | Fills `100%` width and height of the calculated ratio box. |

## Behavior

* The video container uses CSS `aspect-ratio` to reserve space and prevent layout shifts while the iframe initializes.
* The inner `iframe` stretches seamlessly to fill the parent bounds.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `src` | signal string | required | Video embed URL. |
| `title` | string | `video.title` | Accessible title for the iframe. |
| `ratio` | f32 | `16.0 / 9.0` | Proportional aspect ratio. |
| `max_width` | DU | none | Maximum container width. |
| `max_height` | DU | none | Maximum container height. |
| `allow` | string | common video permissions | Permissions policy for media features. |
| `allow_fullscreen` | bool | `true` | Allows the player to enter fullscreen mode. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `max_width` | DU | `--op-video-max-width` | `none` |
| `max_height` | DU | `--op-video-max-height` | `none` |

| Color variable | Default |
| :--- | :--- |
| `--op-video-background` | `var(--op-surface-subtle)` |

`class` and `style` apply to the root `div`.

### Strings

| Key | Default |
| :--- | :--- |
| `video.title` | `"Embedded video"` |

### Icons

`Video` does not use icons.

## Accessibility

The embedded `<iframe>` receives an accessible `title` via the `title` prop or `video.title` translation.
