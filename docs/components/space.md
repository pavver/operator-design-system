# Space

Flexible spacer that expands to fill available space in a flex layout.
It pushes neighbouring elements apart along the main axis.

Status: **draft** on web.

## Use

Use `Space` between controls in a toolbar, header, or card actions row to separate leading and trailing items.
Do not use `Space` when equal spacing is better handled by `gap` on the parent container.

## Geometry

| Property | Value |
| :--- | :--- |
| Default size | `0 DU` minimum size with `flex-grow: 1`. |
| Bounds | Stretches to take up remaining space along the flex direction. |

## Behavior

* `Space` has no content and is hidden from assistive technologies (`aria-hidden="true"`).
* It adapts to both flex rows and flex columns.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `min_width` | f32 (DU) | none | Minimum width reservation in DU. |
| `min_height` | f32 (DU) | none | Minimum height reservation in DU. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `min_width` | DU | `--op-space-min-width` | `0` |
| `min_height` | DU | `--op-space-min-height` | `0` |

`class` and `style` apply to the root `div`.

### Strings

`Space` renders no user-facing or accessible text strings.

### Icons

`Space` uses no icons.

## Accessibility

`Space` is purely visual and is marked with `aria-hidden="true"`.
