#![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod dense_edges;
mod dense_nodes;
mod sparse_edges;
mod sparse_nodes;
pub(crate) mod utils;

pub use crate::{
    dense_edges::AdjacencyEdgeList,
    dense_nodes::AdjacencyNodeList,
    sparse_edges::{one_way_iter::*, AdjacencyEdgeDict},
    sparse_nodes::AdjacencyNodeDict,
};

/// Sparse adjacency list, edge-first directed graph
pub type DiGraphAED = AdjacencyEdgeDict<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Sparse adjacency list, edge-first undirected graph
pub type UnGraphAED = AdjacencyEdgeDict<{ graph_types::GraphKind::Directed.is_two_way() }>;
/// Sparse adjacency list, node-first directed graph
pub type DiGraphAND = AdjacencyNodeDict<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Sparse adjacency list, node-first undirected graph
pub type UnGraphAND = AdjacencyNodeDict<{ graph_types::GraphKind::Directed.is_two_way() }>;
/// Dense adjacency list, edge-first directed graph
pub type DiGraphAEL = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Dense adjacency list, edge-first undirected graph
pub type UnGraphAEL = AdjacencyEdgeList<{ graph_types::GraphKind::Directed.is_two_way() }>;
/// Dense adjacency list, node-first directed graph
pub type DiGraphANL = AdjacencyNodeList<{ graph_types::GraphKind::Directed.is_one_way() }>;
/// Dense adjacency list, node-first undirected graph
pub type UnGraphANL = AdjacencyNodeList<{ graph_types::GraphKind::Directed.is_two_way() }>;
