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

* The card measures its socket centres in world units and reports them through `on_sockets` when card, header, row or socket geometry changes, including slot insertions and reactive metrics.
  Wires follow the measured DOM geometry without separate row-position constants in Rust.
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

## API

All graph components accept reactive `class` and `style` (`MaybeProp<String>`).
Metric props are reactive `MaybeProp<f32>` unless noted otherwise; their owners, units, variables and defaults are listed below.
`grid` keeps its DU meaning and accepts reactive overrides; `size` remains a pair of DU values.
Positions, `NodeCard.width`, `WorldRect` and wire endpoints retain world-coordinate units for compatibility with the camera and measured socket coordinates.
These application geometry inputs are separate from the DU component metrics.

| Component | Prop | Type | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| GraphCanvas | `camera` | `RwSignal<Camera>` | required | World camera. |
| GraphCanvas | `min_zoom`, `max_zoom` | `f64` | `0.25`, `2` | Zoom limits. |
| GraphCanvas | `pan_on_background` | bool | true | Primary-button panning. |
| GraphCanvas | `on_background_down` | pointer callback | none | Background presses. |
| GraphCanvas | `children`, `overlay` | children, optional children closure | required, none | World and screen content. |
| GraphCanvas | `is_empty`, `empty` | bool signal, Slot | false, empty | Show the empty-canvas slot when the caller has no nodes. |
| NodeCard | `x`, `y`, `width`, `title` | number signals, string signal | required | World rectangle and title. |
| NodeCard | `category`, `color` | optional string signals | none | Category and type tint; color writes `--op-graph-node-color`. |
| NodeCard | `selected`, `error` | bool signals | false | Visual states. |
| NodeCard | `help`, `help_label`, `help_icon` | view, string, icon | none, `graph.node-help`, `graph.node-help` | Help content and semantic overrides. |
| NodeCard | `actions`, `children` | optional children closures | none | Header actions and body rows. |
| NodeCard | `on_header_down`, `on_sockets` | callbacks | none | Dragging and measured `(id, side, y)` socket centres. |
| NodeCard | `header`, `body`, `footer` | Slot | empty | Named content overrides. |
| NodeRow | `label`, `end_label` | optional string signals | none | Caller-supplied row labels. |
| NodeRow | `input`, `output` | Slot | empty | Sockets; existing ViewFn callers remain supported. |
| NodeRow | `content`, `error`, `children` | bool, bool signal, optional children | false, false, none | Growing rows, state and controls. |
| Socket | `id`, `side` | string, SocketSide | required | Connection identity and card edge. |
| Socket | `kind`, `connected`, `highlighted`, `missing` | SocketKind, bool signals | Data, false | Shape and states. |
| Socket | `color`, `on_pointerdown` | optional string signal, callback | none | Type tint (`--op-graph-socket-color`) and interaction. |
| WireLayer | `children` | children | required | SVG wires. |
| Wire | `from`, `to`, `points` | point signals, points signal | required, required, empty | World route. |
| Wire | `color`, `state`, `exec` | optional string signal, WireState signal, bool | none, Normal, false | Type tint (`--op-graph-wire-color`) and appearance. |
| Wire | `label`, `on_pointerdown` | optional string signal, callback | none | Route label and interaction. |
| GraphFrame | `rect`, `title` | WorldRect signal, string signal | required | Region and fallback title. |
| GraphFrame | `color`, `selected` | optional string signal, bool signal | none, false | Tint (`--op-graph-frame-color`) and selection. |
| GraphFrame | `on_header_down`, `actions`, `title_slot` | callback, optional children closure, Slot | none, none, empty | Drag, actions and title content. |
| Minimap | `items`, `size`, `label` | items signal, `(f32, f32)`, string | required, `(50, 35)`, `graph.minimap` | Rectangles, DU dimensions and accessible name. |
| All | `class`, `style` | MaybeProp string | none | Root hooks; explicit style follows metrics. |

| GraphCanvas | `grid` | MaybeProp DU number | 5 | Geometry or typography override. |
| GraphCanvas | `grid_width` | MaybeProp DU number | border (`0.25 DU`) | Geometry or typography override. |
| WireLayer | `layer_size` | MaybeProp DU number | 0.25 | Geometry or typography override. |
| NodeCard | `header_height` | MaybeProp DU number | row_compact (`8 DU`) | Geometry or typography override. |
| NodeCard | `header_padding_start` | MaybeProp DU number | pad_card | Geometry or typography override. |
| NodeCard | `header_padding_end` | MaybeProp DU number | pad_tight | Geometry or typography override. |
| NodeCard | `header_gap` | MaybeProp DU number | pad_tight | Geometry or typography override. |
| NodeCard | `body_padding_y` | MaybeProp DU number | gap_micro | Geometry or typography override. |
| NodeCard | `label_width` | MaybeProp DU number | 40% of the card | Geometry or typography override. |
| NodeCard, NodeRow | `row_height` | MaybeProp DU number | row_standard (`10 DU`) | Geometry or typography override. |
| NodeCard, NodeRow | `row_padding_x` | MaybeProp DU number | pad_card | Geometry or typography override. |
| NodeCard, NodeRow | `row_padding_y` | MaybeProp DU number | gap_micro; content rows | Geometry or typography override. |
| NodeCard, NodeRow | `row_gap` | MaybeProp DU number | pad_tight | Geometry or typography override. |
| NodeCard, Socket | `socket_size` | MaybeProp DU number | 2.5 | Geometry or typography override. |
| Socket | `socket_border_width` | MaybeProp DU number | border_emphasis (`0.5 DU`) | Geometry or typography override. |
| Socket | `socket_outline_width` | MaybeProp DU number | border (`0.25 DU`) | Geometry or typography override. |
| Socket | `socket_outline_offset` | MaybeProp DU number | border (`0.25 DU`) | Geometry or typography override. |
| NodeCard, Wire, GraphFrame, Minimap | `border_width` | MaybeProp DU number | border (`0.25 DU`) | Geometry or typography override. |
| NodeCard, GraphFrame | `emphasis_width` | MaybeProp DU number | border_emphasis (`0.5 DU`) | Geometry or typography override. |
| NodeCard | `icon_size` | MaybeProp DU number | `icon_small` (`4 DU`) | Geometry or typography override. |
| NodeCard, NodeRow, GraphFrame | `font_size` | MaybeProp TU number | type_body_size | Geometry or typography override. |
| NodeCard, NodeRow, GraphFrame | `line_height` | MaybeProp TU number | type_body_line | Geometry or typography override. |
| NodeCard, Wire | `caption_font_size` | MaybeProp TU number | type_caption_size | Geometry or typography override. |
| NodeCard, Wire | `caption_line_height` | MaybeProp TU number | type_caption_line | Geometry or typography override. |
| NodeCard | `footer_padding_x` | MaybeProp DU number | pad_card | Geometry or typography override. |
| NodeCard | `footer_padding_y` | MaybeProp DU number | pad_tight | Geometry or typography override. |
| Wire | `wire_width` | MaybeProp DU number | border_emphasis (`0.5 DU`) | Geometry or typography override. |
| Wire | `wire_exec_width` | MaybeProp DU number | 0.75 | Geometry or typography override. |
| Wire | `wire_hit_width` | MaybeProp DU number | 3 | Geometry or typography override. |
| Wire | `wire_dash` | MaybeProp DU number | 2 | Geometry or typography override. |
| Wire | `wire_dash_gap` | MaybeProp DU number | 1 | Geometry or typography override. |
| Wire | `wire_detached_dash` | MaybeProp DU number | 1 | Geometry or typography override. |
| Wire | `wire_label_width` | MaybeProp DU number | 100 | Geometry or typography override. |
| Wire | `wire_label_height` | MaybeProp DU number | 6 | Geometry or typography override. |
| Wire | `wire_label_padding_x` | MaybeProp DU number | pad_tight | Geometry or typography override. |
| Wire | `wire_handle_min` | MaybeProp DU number | 10; Rust route calculations use the prop | Geometry or typography override. |
| GraphFrame | `frame_header_height` | MaybeProp DU number | row_compact (`8 DU`) | Geometry or typography override. |
| GraphFrame | `frame_padding_x` | MaybeProp DU number | pad_card | Geometry or typography override. |
| GraphFrame | `frame_gap` | MaybeProp DU number | pad_tight | Geometry or typography override. |
| Minimap | `minimap_width` | MaybeProp DU number | size.0 (`50 DU`) | Geometry or typography override. |
| Minimap | `minimap_height` | MaybeProp DU number | size.1 (`35 DU`) | Geometry or typography override. |
| Minimap | `minimap_offset_x` | MaybeProp DU number | margin_screen | Geometry or typography override. |
| Minimap | `minimap_offset_y` | MaybeProp DU number | margin_screen | Geometry or typography override. |
| Minimap | `minimap_item_min_size` | MaybeProp DU number | 0.25 | Geometry or typography override. |

## Customization

`class` and `style` apply to each component root, including the SVG root of WireLayer and the SVG group of Wire.
Each metric prop writes its corresponding `--op-graph-*` variable; descendants inherit shared metrics unless they set their own props.
Square geometry, state behavior and the existing default dimensions remain the defaults.
The `color` props and `MiniItem.color` remain available for application type colors and write family color variables.

| Metric prop | Component | Unit | Variable | Default |
| :--- | :--- | :--- | :--- | :--- |
| `grid` | GraphCanvas | DU | `--op-graph-grid` | 5 |
| `grid_width` | GraphCanvas | DU | `--op-graph-grid-width` | border (`0.25 DU`) |
| `layer_size` | WireLayer | DU | `--op-graph-layer-size` | 0.25 |
| `header_height` | NodeCard | DU | `--op-graph-header-height` | row_compact (`8 DU`) |
| `header_padding_start` | NodeCard | DU | `--op-graph-header-padding-start` | pad_card |
| `header_padding_end` | NodeCard | DU | `--op-graph-header-padding-end` | pad_tight |
| `header_gap` | NodeCard | DU | `--op-graph-header-gap` | pad_tight |
| `body_padding_y` | NodeCard | DU | `--op-graph-body-padding-y` | gap_micro |
| `label_width` | NodeCard | DU | `--op-graph-label-width` | 40% of the card |
| `row_height` | NodeCard, NodeRow | DU | `--op-graph-row-height` | row_standard (`10 DU`) |
| `row_padding_x` | NodeCard, NodeRow | DU | `--op-graph-row-padding-x` | pad_card |
| `row_padding_y` | NodeCard, NodeRow | DU | `--op-graph-row-padding-y` | gap_micro; content rows |
| `row_gap` | NodeCard, NodeRow | DU | `--op-graph-row-gap` | pad_tight |
| `socket_size` | NodeCard, Socket | DU | `--op-graph-socket-size` | 2.5 |
| `socket_border_width` | Socket | DU | `--op-graph-socket-border-width` | border_emphasis (`0.5 DU`) |
| `socket_outline_width` | Socket | DU | `--op-graph-socket-outline-width` | border (`0.25 DU`) |
| `socket_outline_offset` | Socket | DU | `--op-graph-socket-outline-offset` | border (`0.25 DU`) |
| `border_width` | NodeCard, Wire, GraphFrame, Minimap | DU | `--op-graph-border-width` | border (`0.25 DU`) |
| `emphasis_width` | NodeCard, GraphFrame | DU | `--op-graph-emphasis-width` | border_emphasis (`0.5 DU`) |
| `icon_size` | NodeCard | DU | `--op-graph-icon-size` | `icon_small` (`4 DU`) |
| `font_size` | NodeCard, NodeRow, GraphFrame | TU | `--op-graph-font-size` | type_body_size |
| `line_height` | NodeCard, NodeRow, GraphFrame | TU | `--op-graph-line-height` | type_body_line |
| `caption_font_size` | NodeCard, Wire | TU | `--op-graph-caption-font-size` | type_caption_size |
| `caption_line_height` | NodeCard, Wire | TU | `--op-graph-caption-line-height` | type_caption_line |
| `footer_padding_x` | NodeCard | DU | `--op-graph-footer-padding-x` | pad_card |
| `footer_padding_y` | NodeCard | DU | `--op-graph-footer-padding-y` | pad_tight |
| `wire_width` | Wire | DU | `--op-graph-wire-width` | border_emphasis (`0.5 DU`) |
| `wire_exec_width` | Wire | DU | `--op-graph-wire-exec-width` | 0.75 |
| `wire_hit_width` | Wire | DU | `--op-graph-wire-hit-width` | 3 |
| `wire_dash` | Wire | DU | `--op-graph-wire-dash` | 2 |
| `wire_dash_gap` | Wire | DU | `--op-graph-wire-dash-gap` | 1 |
| `wire_detached_dash` | Wire | DU | `--op-graph-wire-detached-dash` | 1 |
| `wire_label_width` | Wire | DU | `--op-graph-wire-label-width` | 100 |
| `wire_label_height` | Wire | DU | `--op-graph-wire-label-height` | 6 |
| `wire_label_padding_x` | Wire | DU | `--op-graph-wire-label-padding-x` | pad_tight |
| `wire_handle_min` | Wire | DU | `--op-graph-wire-handle-min` | 10; Rust route calculations use the prop |
| `frame_header_height` | GraphFrame | DU | `--op-graph-frame-header-height` | row_compact (`8 DU`) |
| `frame_padding_x` | GraphFrame | DU | `--op-graph-frame-padding-x` | pad_card |
| `frame_gap` | GraphFrame | DU | `--op-graph-frame-gap` | pad_tight |
| `minimap_width` | Minimap | DU | `--op-graph-minimap-width` | size.0 (`50 DU`) |
| `minimap_height` | Minimap | DU | `--op-graph-minimap-height` | size.1 (`35 DU`) |
| `minimap_offset_x` | Minimap | DU | `--op-graph-minimap-offset-x` | margin_screen |
| `minimap_offset_y` | Minimap | DU | `--op-graph-minimap-offset-y` | margin_screen |
| `minimap_item_min_size` | Minimap | DU | `--op-graph-minimap-item-min-size` | 0.25 |

Socket layout uses the same `row_height` variable for row size and socket placement.
NodeCard observes rendered card, header, body, rows and sockets and measures socket centres after slot or attribute changes; the result is independent of camera zoom.
Applications should use `on_sockets` for wire endpoints instead of reproducing header and row calculations.
Where layout geometry is also used in Rust, props feed both the variables and calculations.
Minimap additionally measures the rendered size and minimum-item probe, so inherited variables and explicit styles also keep its viewport and pointer mapping consistent.
Wire label dimensions apply to the SVG box and its centered CSS geometry.
`wire_handle_min` controls curve calculations through the prop; use the prop to change it, as its variable alone does not change the Rust route.
The body column limit defaults to proportional `fit-content(40%)`; `label_width` supplies a DU limit when set.
Full-size layers remain relative to their container.

| Color variable | Default |
| :--- | :--- |
| `--op-graph-background` | background |
| `--op-graph-divider` | divider |
| `--op-graph-surface` | surface |
| `--op-graph-header-background` | surface_alt |
| `--op-graph-text` | text_primary |
| `--op-graph-secondary-text` | text_secondary |
| `--op-graph-border` | border |
| `--op-graph-strong-border` | border_strong |
| `--op-graph-accent` | accent |
| `--op-graph-error` | error |
| `--op-graph-node-color` | transparent header accent; secondary_text category |
| `--op-graph-socket-color` | secondary_text |
| `--op-graph-wire-color` | secondary_text stroke; strong_border label |
| `--op-graph-frame-color` | accent; fill and header use 8% and 25% tints |
| `--op-graph-minimap-item` | text_disabled |

| Slot | Component | Place |
| :--- | :--- | :--- |
| `empty` | GraphCanvas | Screen-space content while `is_empty` is true. |
| `header` | NodeCard | Replaces title and category; help and actions remain available. |
| `body` | NodeCard | Replaces children with custom body rows in the shared grid. |
| `footer` | NodeCard | Below the body. |
| `input, output` | NodeRow | Socket content on each edge. |
| `title_slot` | GraphFrame | Replaces the frame title. |

### Strings

| Key | Prop | Default |
| :--- | :--- | :--- |
| `graph.node-help` | `help_label` | About this node |
| `graph.minimap` | `label` | Minimap |

### Icons

| Key | Prop | Default |
| :--- | :--- | :--- |
| `graph.node-help` | `help_icon` | `info` |
