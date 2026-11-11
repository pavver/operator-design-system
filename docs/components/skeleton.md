# Skeleton

Placeholder shapes that hold the layout while content loads.

Status: **draft** on web.

## Use

* Lists, cards, and details whose shape is known before the data arrives.
* Use the shape of the real content so nothing jumps when it appears.
* For waits shorter than about `300 ms`, show nothing instead.

## Geometry

| Shape | Value |
| :--- | :--- |
| Text line | Height of the body font size, centered in one body line; full width. |
| Several lines | `1 DU` apart; the last line is 60% wide. |
| Block | Given width and height in DU. |
| Circle | Given size; only for avatar placeholders, the round exception of [Avatar](avatar.md). |

Corners are `0 DU` except for the circle.

## Motion

* A soft highlight from `pressed` sweeps over `surface_alt` once per `skeleton_lap` (`1500 ms`), linear.
* With reduced motion the shapes are static `surface_alt`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `width` | DU | full width | Width. |
| `height` | DU | one text line | Height. |
| `lines` | integer | `1` | Number of text lines. |
| `circle` | bool | `false` | Round shape. |

## Accessibility

The group is marked busy and named "Loading".
