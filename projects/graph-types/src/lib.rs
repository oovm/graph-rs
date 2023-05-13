#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate core;

mod edges;
mod entries;
pub mod errors;
mod graphs;
pub mod placeholder;
pub mod storages;
mod vertexes;

pub use crate::{
    edges::{
        actions::EdgeInsertID,
        typed_edges::{DirectedEdge, DynamicEdge, UndirectedEdge},
        Edge, EdgeDirection, EdgeID,
    },
    entries::{
        query::{query_dynamic::Query, query_edge::EdgeQuery, query_node::NodeQuery},
        EntryEngine, GraphEntry, GraphKind,
    },
    graphs::{named::NamedGraph, weighted::WeightedGraph, GraphEngine, MutableGraph},
    vertexes::{
        get_iter::NodesVisitor, node_range_visitor::NodeRangeVisitor, node_slice_visitor::NodeSliceVisitor, Node, NodeID,
    },
};

#[cfg(feature = "wolfram_wxf")]
pub use wolfram_wxf::{ToWolfram, WolframValue};
