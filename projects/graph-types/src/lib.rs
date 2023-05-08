#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod edges;
mod entries;
mod errors;
mod famous_graphs;
mod graphs;
mod storage;
mod vertexes;

pub use crate::{
    edges::{
        typed_edges::{DirectedEdge, DynamicEdge, UndirectedEdge}, get_iter::GetEdgesVisitor, actions::{EdgeRemoveAction, EdgeInsertID}, mut_iter::MutEdgesVisitor,
        Edge, EdgeDirection,
    },
    entries::{Entry, EntryName, EntryWeight, Query},
    errors::{GraphError, GraphErrorKind, GraphResult},
    famous_graphs::complete_graph::CompleteGraph,
    graphs::{
        weighted::{GraphData, ValueProvider},
        GraphEngine,
    },
    storage::{btree::DictStorage, vector::ListStorage},
    vertexes::{get_iter::GetNodesVisitor, Node},
};

pub use num_traits::Zero;
#[cfg(feature = "wolfram_wxf")]
pub use wolfram_wxf::{ToWolfram, WolframValue};

#[cfg(feature = "dashmap")]
pub use crate::storage::shared::SharedStorage;
