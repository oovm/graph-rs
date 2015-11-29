use crate::StaticDirected;
use graph_types::{Edge, Graph, Node};
use std::{borrow::Cow, collections::BTreeMap};

#[derive(Clone, Debug)]
pub struct AdjacencyMatrix<N, E> {
    count_nodes: usize,
    count_edges: usize,
    adjacency: AdjacencyStorage,
    vertexes: BTreeMap<usize, N>,
    edges: BTreeMap<usize, E>,
}

/// Inner storage layout of the adjacency matrix
#[derive(Clone, Debug)]
enum AdjacencyStorage {
    LowerTriangularFixed(StaticDirected),
    SquareFixed(StaticDirected),
}

impl<N, E> Graph for AdjacencyMatrix<N, E>
where
    N: Node + Clone,
    E: Edge + Clone,
{
    type NodeIndex = N;

    fn count_nodes(&self) -> usize {
        self.count_nodes
    }

    fn count_edges(&self) -> usize {
        self.count_edges
    }
}

impl<N, E> AdjacencyMatrix<N, E> {}

impl<N, E> AdjacencyMatrix<N, E> {
    pub fn cast_dynamic(&self) -> AdjacencyMatrix<N, E> {
        todo!()
    }
    pub fn cast_fixed(&self) -> AdjacencyMatrix<N, E> {
        todo!()
    }
}
