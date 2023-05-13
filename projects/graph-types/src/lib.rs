#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod edges;
mod entries;
mod errors;
mod graphs;
mod vertexes;

pub use crate::{
    edges::{
        actions::EdgeInsertID,
        get_iter::EdgesVisitor,
        typed_edges::{DirectedEdge, DynamicEdge, UndirectedEdge},
        Edge, EdgeDirection, EdgeID,
    },
    entries::{
        query::{query_dynamic::Query, query_edge::EdgeQuery, query_node::NodeQuery},
        storages::{btree::DictStorage, vector::ListStorage},
        GraphEntry, GraphKind,
    },
    errors::{
        placeholder::{PlaceholderEdgeIterator, PlaceholderGraph, PlaceholderNodeIterator},
        GraphError, GraphErrorKind, GraphResult,
    },
    graphs::{
        named::NamedGraph,
        valued::{EntryEngine, ValueProvider},
        weighted::WeightedGraph,
        GraphEngine, MutableGraph,
    },
    vertexes::{get_iter::NodesVisitor, Node, NodeID},
};

#[cfg(feature = "wolfram_wxf")]
pub use wolfram_wxf::{ToWolfram, WolframValue};
