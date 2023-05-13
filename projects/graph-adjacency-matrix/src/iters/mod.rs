use graph_types::IndeterminateEdge;
use std::slice::Iter;

pub struct AdjacencyMatrixAllBridges<'a> {
    edges: Iter<'a, IndeterminateEdge>,
}

impl Iterator for AdjacencyMatrixAllBridges<'_> {
    type Item = IndeterminateEdge;

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.next().copied()
    }
}

impl DoubleEndedIterator for AdjacencyMatrixAllBridges<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.edges.next_back().copied()
    }
}
