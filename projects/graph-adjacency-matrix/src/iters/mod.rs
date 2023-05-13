use graph_types::IndeterminateEdge;
use std::slice::Iter;

#[derive(Debug)]
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

impl<'a> AdjacencyMatrixAllBridges<'a> {
    pub fn new(edges: &'a [IndeterminateEdge]) -> Self {
        Self { edges: edges.iter() }
    }
}
