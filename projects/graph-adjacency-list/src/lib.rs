#![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod sparse_edges;
mod sparse_nodes;
pub(crate) mod utils;

pub use sparse_edges::AdjacencyEdgeList;
pub use sparse_nodes::AdjacencyNodeList;

/// A directed graph using an adjacency list as its underlying storage.
pub type DiGraphAEL = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// An undirected graph using an adjacency list as its underlying storage.
pub type UnGraphAEL = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_two_way() }>;
