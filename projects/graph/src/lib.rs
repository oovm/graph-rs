#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use graph_types::{
    DirectedEdge, DynamicEdge, Edge, EdgeQuery, Entry, EntryEngine, GraphEngine, GraphError, GraphErrorKind, GraphResult, Node,
    NodesVisitor, Query, UndirectedEdge, ValueProvider,
};

pub mod graph_engines {
    pub use adjacency_list::{AdjacencyEdgeList, AdjacencyNodeList};
    pub use adjacency_matrix::{AdjacencyEdge, StaticDirected, StaticUndirected};
    pub use graph_types::famous_graphs::CompleteGraph;
}

pub mod entry_engines {
    pub use graph_types::{DictStorage, EntryName, EntryWeight, ListStorage};
}
