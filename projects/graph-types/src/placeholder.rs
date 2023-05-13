#![allow(unused_variables)]

use crate::{
    edges::typed_edges::IndeterminateEdge, errors::GraphError, DirectedEdge, Edge, EdgeID, EdgeInsertID, EdgeQuery,
    GraphEngine, GraphKind, MutableGraph, NodeID, NodeQuery,
};

/// A placeholder graph engine.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlaceholderGraph;
/// A placeholder node iterator.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlaceholderNodeIterator;
/// A placeholder edge iterator.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlaceholderEdgeIterator;

impl GraphEngine for PlaceholderGraph {
    type NodeIterator = PlaceholderNodeIterator;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeIterator = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        unreachable!()
    }

    fn get_node_id<Q: Into<NodeQuery>>(&self, node: Q) -> Result<NodeID, GraphError> {
        unreachable!()
    }

    fn all_node_ids(&self) -> Self::NodeIterator {
        unreachable!()
    }

    fn get_edge_id<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<EdgeID, GraphError> {
        unreachable!()
    }

    fn all_edge_ids(&self) -> Self::EdgeIterator {
        unreachable!()
    }

    fn get_bridges<Q: Into<EdgeQuery>>(&self, edge: Q) -> Self::BridgeIterator {
        unreachable!()
    }

    fn all_bridges(&self) -> Self::BridgeIterator {
        unreachable!()
    }

    fn size_hint(&self) -> usize {
        0
    }
}

impl MutableGraph for PlaceholderGraph {
    fn insert_node(&mut self, node_id: usize) -> bool {
        unreachable!()
    }

    fn create_node(&mut self) -> usize {
        unreachable!()
    }

    fn remove_node_with_edges(&mut self, node_id: usize) {
        unreachable!()
    }

    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertID {
        unreachable!()
    }

    fn remove_edge<E>(&mut self, edge: E)
    where
        E: Into<EdgeQuery>,
    {
        unreachable!()
    }
}

impl Iterator for PlaceholderEdgeIterator {
    type Item = IndeterminateEdge;

    fn next(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}
impl DoubleEndedIterator for PlaceholderEdgeIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}

impl Iterator for PlaceholderNodeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}

impl DoubleEndedIterator for PlaceholderNodeIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}
