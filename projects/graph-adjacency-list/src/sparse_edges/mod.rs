use crate::utils::ShortEdge;
use graph_types::{
    placeholder::{PlaceholderDirectionIterator, PlaceholderEdgeIterator, PlaceholderNodeIterator},
    Edge, EdgeDirection, EdgeInsertID, EdgeQuery, GraphEngine, GraphKind, MutableGraph, NodeQuery, NodeRangeVisitor,
    NodesVisitor,
};
use std::collections::{BTreeMap, BTreeSet};

mod one_way;
mod two_way;

#[doc = include_str!("AdjacencyEdgeList.html")]
#[derive(Debug)]
pub struct AdjacencyEdgeList<const ONE_WAY: bool> {
    nodes: BTreeSet<u32>,
    edges: BTreeMap<u32, ShortEdge>,
}
