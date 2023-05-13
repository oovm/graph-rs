#![allow(unused_variables)]

use crate::{
    edges::typed_edges::IndeterminateEdge, errors::GraphError, DirectedEdge, Edge, EdgeID, EdgeInsertID, EdgeQuery,
    GraphEngine, GraphKind, MutableGraph, NodeID,
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

impl<'a> GraphEngine<'a> for PlaceholderGraph {
    type NeighborIterator = PlaceholderNodeIterator;

    type BridgeIterator = PlaceholderEdgeIterator;

    /// A placeholder node iterator.
    type NodeTraverser = PlaceholderNodeIterator;
    type EdgeTraverser = PlaceholderNodeIterator;
    type BridgeTraverser = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        unreachable!()
    }

    fn get_node(&self, node: NodeID) -> Result<NodeID, GraphError> {
        unreachable!()
    }

    fn all_nodes(&self) -> Self::NodeTraverser {
        unreachable!()
    }

    fn all_neighbors(&'a self, node: usize) -> Self::NeighborIterator {
        unreachable!()
    }

    fn get_edge<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<EdgeID, GraphError> {
        unreachable!()
    }

    fn all_edges(&self) -> Self::EdgeTraverser {
        unreachable!()
    }

    fn get_bridges(&'a self, from: NodeID, goto: NodeID) -> Self::BridgeIterator {
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
