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

/// Sparse, edge-first, adjacency list-based directed graph.
pub type DiGraphSEAL = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Sparse, node-first, adjacency list-based undirected graph.
pub type UnGraphSEAL = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_two_way() }>;
/// Sparse, node-first, adjacency list-based directed graph.
pub type DiGraphSNAL = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Sparse, node-first, adjacency list-based undirected graph.
pub type UnGraphSNAL = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_two_way() }>;

/// Dense, node-first, incidence matrix-based directed graph.
pub type DiGraphDNIM = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Dense, node-first, incidence matrix-based undirected graph.
pub type UnGraphDNIM = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_two_way() }>;
/// Compressed sparse row based directed graph.
pub type DiGraphCSR = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Compressed sparse row based undirected graph.
pub type UnGraphCSR = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_two_way() }>;
