use crate::utils::ShortEdge;
use graph_types::{
    placeholder::{PlaceholderDirectionIterator, PlaceholderEdgeIterator, PlaceholderNodeIterator},
    Edge, EdgeDirection, EdgeInsertID, EdgeQuery, GraphEngine, GraphKind, MutableGraph, NodeQuery, NodeRangeVisitor,
    NodesVisitor,
};
use std::collections::{BTreeMap, BTreeSet};

mod one_way;
mod two_way;

type NodeID = u32;
type EdgeID = u32;

#[doc = include_str!("AdjacencyEdgeList.html")]
#[derive(Debug)]
pub struct AdjacencyEdgeList<const ONE_WAY: bool> {
    nodes: BTreeSet<NodeID>,
    edges: BTreeMap<EdgeID, ShortEdge>,
}
