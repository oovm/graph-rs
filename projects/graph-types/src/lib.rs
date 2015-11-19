#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod edges;
mod errors;
mod graphs;
mod query;
mod storage;
mod vertexes;

pub use crate::{
    edges::{
        directed::DirectedEdge, get_iter::GetEdgesVisitor, mut_iter::MutEdgesVisitor, undirected::UndirectedEdge, Edge,
        EdgeDirection,
    },
    errors::{GraphError, GraphErrorKind, GraphResult},
    graphs::{
        weighted::{WeightedGraph, WeightsProvider},
        Graph,
    },
    query::{Entry, Query},
    vertexes::{get_iter::GetNodesVisitor, mut_iter::MutNodesVisitor, simple::PureNode, IntoNode, Node},
};
pub use num_traits::Zero;
#[cfg(feature = "wolfram_wxf")]
pub use wolfram_wxf::{ToWolfram, WolframValue};

#[cfg(feature = "dashmap")]
pub use crate::storage::shared::SharedStorage;
