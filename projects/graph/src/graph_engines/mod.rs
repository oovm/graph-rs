#![doc = include_str!("readme.md")]

pub use adjacency_list::{AdjacencyEdgeList, AdjacencyNodeList};
pub use adjacency_matrix::{AdjacencyEdge, StaticDirected, StaticUndirected};
pub use graph_families::{CompleteGraph, CycleGraph, StarGraph};
