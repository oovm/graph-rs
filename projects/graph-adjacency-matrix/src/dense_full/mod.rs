use crate::{utils::AdjacencyCell, DiGraphAM};
use graph_types::{
    errors::GraphError, placeholder::PlaceholderNodeIterator, Edge, EdgeID, EdgeQuery, GraphEngine, GraphKind,
    IndeterminateEdge, MutableGraph, NodeID, Query,
};
use std::{
    fmt::{Debug, Display},
    mem::size_of,
    ops::Range,
};

mod display;
mod one_way;

#[derive(Clone, Debug)]
pub struct AdjacencyMatrix<const ONE_WAY: bool> {
    rank: usize,
    edges: Vec<IndeterminateEdge>,
    matrix: Vec<AdjacencyCell>,
}

impl<const ONE_WAY: bool> AdjacencyMatrix<ONE_WAY> {
    /// Returns the number of nodes in the graph.
    pub fn shrink_to_fit(&mut self) {
        self.edges.shrink_to_fit();
        self.matrix.shrink_to_fit();
    }
}

#[test]
fn fast_test() {
    let mut matrix = DiGraphAM::new(11, 13);
    println!("{:?}", matrix.size_hint());
    // matrix.mut_matrix().fill(1);
}
