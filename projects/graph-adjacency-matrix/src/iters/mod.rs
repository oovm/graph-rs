use crate::utils::AdjacencyEdge;
use graph_types::IndeterminateEdge;
use std::slice::Iter;

#[derive(Debug)]
pub struct AdjacencyMatrixAllBridges<'a> {
    edges: Iter<'a, AdjacencyEdge>,
}

impl Iterator for AdjacencyMatrixAllBridges<'_> {
    type Item = IndeterminateEdge;

    fn next(&mut self) -> Option<Self::Item> {
        let small = self.edges.next()?;
        Some(IndeterminateEdge { from: small.from as usize, goto: small.goto as usize })
    }
}

impl DoubleEndedIterator for AdjacencyMatrixAllBridges<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let small = self.edges.next_back()?;
        Some(IndeterminateEdge { from: small.from as usize, goto: small.goto as usize })
    }
}

impl<'a> AdjacencyMatrixAllBridges<'a> {
    pub fn new(edges: &'a [AdjacencyEdge]) -> Self {
        Self { edges: edges.iter() }
    }
}
