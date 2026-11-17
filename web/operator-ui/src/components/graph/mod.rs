//! Node-graph editing parts: canvas, node cards with row stacks, sockets, wires, frames, minimap.
//!
//! Project-specific (`graph` feature): the components know nothing about an engine or its types;
//! the application positions nodes, keeps the graph and wires interactions.

mod camera;
mod canvas;
mod node_card;
mod parts;
pub mod route;

pub use camera::{Camera, WorldRect};
pub use canvas::{GraphCamera, GraphCanvas, wheel_factor};
pub use node_card::{NodeCard, NodeRow, Socket, SocketKind, SocketPosition, SocketSide};
pub use parts::{GraphFrame, MiniItem, Minimap, Wire, WireLayer, WireState, minimap_scale};

pub(crate) const CSS: &str = include_str!("graph.css");

#[cfg(test)]
mod tests;
