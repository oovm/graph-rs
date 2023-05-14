#![doc = include_str!("readme.md")]

pub use adjacency_list::*;
pub use adjacency_matrix::{AdjacencyMatrix, TriangularAdjacencyMatrix};
pub use graph_families::{CompleteGraph, CycleGraph, StarGraph, WheelGraph};
pub use graph_types::placeholder::*;

/// Dense, node-first, incidence matrix-based directed graph.
pub type DiGraphDNIM = AdjacencyEdgeDict<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Dense, node-first, incidence matrix-based undirected graph.
pub type UnGraphDNIM = AdjacencyEdgeDict<{ graph_types::GraphKind::Directed.is_two_way() }>;
/// Compressed sparse row based directed graph.
pub type DiGraphCSR = AdjacencyEdgeDict<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Compressed sparse row based undirected graph.
pub type UnGraphCSR = AdjacencyEdgeDict<{ graph_types::GraphKind::Directed.is_two_way() }>;
