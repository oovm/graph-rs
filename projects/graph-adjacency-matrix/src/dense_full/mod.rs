use crate::{
    utils::{AdjacencyCell, AdjacencyEdge},
    AdjacencyMatrixAllBridges, DiGraphAM,
};
use graph_types::{
    errors::GraphError, iterators::BridgeRange, placeholder::PlaceholderNodeIterator, Edge, EdgeID, GraphEngine, GraphKind,
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
    edges: Vec<AdjacencyEdge>,
    matrix: Vec<AdjacencyCell>,
    max_degree: usize,
}

impl<const ONE_WAY: bool> AdjacencyMatrix<ONE_WAY> {
    /// Shrinks the capacity of the matrix as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.edges.shrink_to_fit();
        self.matrix.shrink_to_fit();
    }
    /// Returns the number of edges in the graph.
    pub fn max_degree(&self) -> usize {
        self.max_degree
    }

    fn get_size(&self) -> usize {
        size_of::<Self>()
            + self.edges.capacity() * size_of::<AdjacencyEdge>()
            + self.matrix.capacity() * size_of::<AdjacencyCell>()
    }
}

#[test]
fn fast_test() {
    let mut matrix = DiGraphAM::new(100, 3000);
    println!("{:?}", matrix.size_hint());
    // matrix.mut_matrix().fill(1);
}
