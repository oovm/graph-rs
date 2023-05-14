use crate::utils::ShortEdge;
use std::collections::{btree_map, btree_set, btree_set::Iter, BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct AdjacencyEdgeAllNodes<'i> {
    nodes: btree_set::Iter<'i, u32>,
}
#[derive(Debug)]
pub struct AdjacencyEdgeAllEdges<'i> {
    edges: btree_map::Iter<'i, u32, ShortEdge>,
}

impl<'i> Iterator for AdjacencyEdgeAllNodes<'i> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.next().map(|x| *x as usize)
    }
}

impl<'i> DoubleEndedIterator for AdjacencyEdgeAllNodes<'i> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nodes.next_back().map(|x| *x as usize)
    }
}
impl<'i> AdjacencyEdgeAllNodes<'i> {
    pub fn new(edges: &'i BTreeSet<u32>) -> Self {
        Self { nodes: edges.iter() }
    }
}

impl<'i> Iterator for AdjacencyEdgeAllEdges<'i> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.next().map(|(x, _)| *x as usize)
    }
}

impl<'i> DoubleEndedIterator for AdjacencyEdgeAllEdges<'i> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.edges.next_back().map(|(x, _)| *x as usize)
    }
}
impl<'i> AdjacencyEdgeAllEdges<'i> {
    pub fn new(edges: &'i BTreeMap<u32, ShortEdge>) -> Self {
        Self { edges: edges.iter() }
    }
}
