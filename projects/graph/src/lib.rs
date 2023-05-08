#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use adjacency_matrix::{AdjacencyEdge, StaticDirected, StaticUndirected};
pub use graph_types::*;

pub mod graph_engines {
    pub use adjacency_list::{
        sparse_edges::{self as adjacency_edge_list, AdjacencyEdgeList},
        sparse_nodes::{self as adjacency_node_list, AdjacencyNodeList},
    };
}
