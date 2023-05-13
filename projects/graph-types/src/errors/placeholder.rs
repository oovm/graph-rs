#![allow(unused_variables)]

use crate::{EdgeID, EdgeQuery, GraphEngine, GraphKind, NodeQuery};

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
    type EdgeIterator = PlaceholderEdgeIterator;
    type NeighborIterator = PlaceholderNodeIterator;

    fn graph_kind(&self) -> GraphKind {
        unreachable!()
    }

    fn has_node(&self, query: NodeQuery) -> Option<usize> {
        unreachable!()
    }

    fn traverse_nodes(&self) -> Self::NodeIterator {
        unreachable!()
    }

    fn has_edge(&self, query: EdgeQuery) -> Option<EdgeID> {
        unreachable!()
    }

    fn traverse_edges(&self) -> Self::EdgeIterator {
        unreachable!()
    }
    fn size_hint(&self) -> usize {
        0
    }
}

impl Iterator for PlaceholderNodeIterator {
    type Item = NodeQuery;

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
    type Item = EdgeQuery;

    fn next(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}

impl DoubleEndedIterator for PlaceholderEdgeIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}
