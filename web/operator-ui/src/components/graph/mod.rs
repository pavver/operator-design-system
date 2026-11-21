//! Node-graph editing parts: canvas, node cards with row stacks, sockets, wires, frames, minimap.
//!
//! Project-specific (`graph` feature): the components know nothing about an engine or its types;
//! the application positions nodes, keeps the graph and wires interactions.

mod camera;
mod canvas;
mod minimap;
mod node_card;
mod parts;
pub mod route;
mod sockets;

pub use camera::{Camera, WorldRect};
pub use canvas::{GraphCamera, GraphCanvas, wheel_factor};
pub use minimap::{MiniItem, Minimap, minimap_scale};
pub use node_card::{NodeCard, NodeRow, Socket, SocketKind, SocketPosition, SocketSide};
pub use parts::{GraphFrame, Wire, WireLayer, WireState};

pub(crate) const CSS: &str = include_str!("graph.css");

#[cfg(test)]
mod tests;
