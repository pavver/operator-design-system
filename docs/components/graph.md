# Graph

Node-graph editing parts: GraphCanvas, NodeCard with NodeRow and Socket, Wire, GraphFrame and Minimap.
They are project-specific: the `graph` feature is in the `extensions` group, not in `full`.
They know nothing about an engine or its types; the application keeps the graph, positions nodes and wires interactions.

Status: **draft** on web.

## GraphCanvas

* A world layer moved by a `Camera` (`screen = world × zoom + offset`) and a screen overlay for toolbars and the minimap.
* The wheel zooms around the pointer between `min_zoom` (`0.25`) and `max_zoom` (`2`); the middle button pans anywhere; a primary drag on the empty background pans when `pan_on_background` is set.
* `on_background_down` reports presses on the empty background, for selection boxes.
* The grid is drawn with `divider` lines every `grid` DU (`5`), following the camera.
* `Camera` has `to_world`, `to_screen`, `zoom_at`, `pan`, `visible`, `centered_on` and `fit` (zoom to content or selection).

## NodeCard

| Property | Value |
| :--- | :--- |
| Header | `row_compact` (`8 DU`), `surface_alt`, title in bold, category text and a `0.5 DU` top line in the category colour, an `info` icon with a [HoverCard](hover-card.md) for help, actions at the end. |
| Body | A two-column grid shared by all rows: labels take `fit-content(40%)`, controls the rest. |
| Row | `row_standard` (`10 DU`); a `content` row grows with its control (text, lists) from that minimum. |
| Socket | `2.5 DU` square with a `0.5 DU` outline in the type colour, filled when connected; exec sockets are triangles. Centred on the card edge at the middle of the row's first line. |
| Selected | `0.5 DU` `accent` outline. |
| Error | `0.5 DU` `error` line under the header; error rows have `error` labels. |

* The card measures its socket centres in world units and reports them through `on_sockets` whenever its size changes, so wires follow rows that grow with content and no pixel constants are shared between Rust and CSS.
* `on_header_down` reports presses on the header for dragging; the help icon and the actions do not start a drag.

## Wire

* A cubic curve from an output to an input, leaving and entering horizontally, through optional route points with smooth tangents.
* `0.5 DU` stroke in the type colour (`0.75 DU` for exec); `selected` uses `accent`, `preview` and `detached` are dashed.
* A wider invisible hit path makes thin wires easy to press.
* An optional label sits in a box at the middle of the route.

## GraphFrame

A titled region behind nodes with a tinted fill and a `row_compact` header in its colour; `on_header_down` drags it.

## Minimap

An overview of item rectangles and the visible area (`accent` outline), `50 × 35 DU` at the bottom-right of the overlay.
A press or drag centres the camera on that point.
