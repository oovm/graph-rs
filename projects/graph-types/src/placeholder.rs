#![allow(unused_variables)]

use crate::{
    edges::typed_edges::IndeterminateEdge, DirectedEdge, Edge, EdgeID, EdgeInsertID, EdgeQuery, GraphEngine, GraphKind,
    MutableGraph, NodeQuery,
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlaceholderDirectionIterator;

impl GraphEngine for PlaceholderGraph {
    type NodeIterator = PlaceholderNodeIterator;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeIterator = PlaceholderEdgeIterator;

    type DirectionIterator = PlaceholderDirectionIterator;

    fn graph_kind(&self) -> GraphKind {
        unreachable!()
    }

    fn has_node<Q: Into<NodeQuery>>(&self, node: Q) -> Option<usize> {
        unreachable!()
    }

    fn traverse_nodes(&self) -> Self::NodeIterator {
        unreachable!()
    }

    fn has_edge<Q: Into<EdgeQuery>>(&self, edge: Q) -> Option<EdgeID> {
        unreachable!()
    }

    fn traverse_edges(&self) -> Self::EdgeIterator {
        unreachable!()
    }

    fn traverse_directions(&self) -> Self::DirectionIterator {
        unreachable!()
    }

    fn size_hint(&self) -> usize {
        0
    }
}

impl Iterator for PlaceholderDirectionIterator {
    type Item = IndeterminateEdge;

    fn next(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}

impl DoubleEndedIterator for PlaceholderDirectionIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}

impl MutableGraph for PlaceholderGraph {
    fn insert_node(&mut self, node_id: usize) -> usize {
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

impl Iterator for PlaceholderEdgeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}

impl DoubleEndedIterator for PlaceholderEdgeIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}
