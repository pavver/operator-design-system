# AspectRatio

Container that preserves a specific aspect ratio regardless of container resizing.
It fits video embeds, map previews, and photographic banners.

Status: **draft** on web.

## Use

Use `AspectRatio` whenever content must maintain proportional width-to-height dimensions (such as 16:9, 4:3, or 1:1) within flexible grid or flex containers.
Do not use `AspectRatio` when content should expand naturally with its text volume.

## Geometry

| Property | Value |
| :--- | :--- |
| Default ratio | `16 / 9` (`1.7778`). |
| Content bounds | The child container fills `100%` width and height of the calculated ratio box. |

## Behavior

* The component uses modern CSS `aspect-ratio` to reserve space before child assets finish downloading, preventing Cumulative Layout Shift (CLS).
* Child elements are pinned to the inner box (`inset: 0`).

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `ratio` | f32 | `16.0 / 9.0` | Proportional ratio of width divided by height. |
| `max_width` | DU | none | Maximum container width constraint. |
| `max_height` | DU | none | Maximum container height constraint. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `max_width` | DU | `--op-aspect-ratio-max-width` | `none` |
| `max_height` | DU | `--op-aspect-ratio-max-height` | `none` |

`class` and `style` apply to the root `div`.

### Strings

`AspectRatio` renders no text strings.

### Icons

`AspectRatio` uses no icons.

## Accessibility

`AspectRatio` provides structural layout and does not alter the accessibility tree of its child components.
