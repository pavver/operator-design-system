# ScrollArea

Vertical scroll container with a thin scrollbar and stretch overscroll at the edges.

Status: **reviewed** on web.

## Use

Every vertically scrolling feed, list, and panel uses the same overscroll physics on all platforms.

## Geometry

| Property | Value |
| :--- | :--- |
| Scrollbar | The library [Scrollbar](scrollbar.md). |
| Height | Fills its parent, or a fixed or maximum height in DU. |
| Focus | `0.25 DU` `accent` outline inside the bounds. |

## Stretch overscroll

Pulling beyond an edge stretches the content instead of stopping hard or showing a glow.

1. At the top edge a downward pull, or at the bottom edge an upward pull, stretches the content along the scroll axis.
   The stretch is anchored at the edge being pulled.
2. The stretch grows with the pull distance and saturates: at 60% of the viewport height the content is 8% taller, which is the limit.
3. Releasing springs the content back over `overscroll_release` (`300 ms`) with the standard curve.
4. Pulling back toward the rest position reduces the stretch; past it, normal scrolling resumes.
5. The effect works even when the content is shorter than the viewport.

* Touch drives the stretch directly from the drag.
* The mouse wheel drives it from the wheel delta at an edge and springs back `120 ms` after the last wheel event.
* With reduced motion there is no stretch.
* The page behind never scrolls when the area reaches an edge.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `height` | DU | none | Fixed height. |
| `max_height` | DU | none | Maximum height. |
| `stretch` | bool | `true` | Enables stretch overscroll. |
| `label` | string | none | Makes the area a named region. |
| `children` | view | required | Content. |

## Accessibility

The area is focusable so keyboard users can scroll it; with a label it is a `region`.
