use crate::StaticDirected;
use graph_types::{Edge, EdgeInsertID, EdgeRemoveAction, GraphEngine, Node};
use std::{borrow::Cow, collections::BTreeMap};

#[derive(Clone, Debug)]
pub struct AdjacencyMatrix<N> {
    count_nodes: usize,
    count_edges: usize,
    adjacency: AdjacencyStorage,
    vertexes: BTreeMap<usize, N>,
}

/// Inner storage layout of the adjacency matrix
#[derive(Clone, Debug)]
enum AdjacencyStorage {
    LowerTriangularFixed(StaticDirected),
    SquareFixed(StaticDirected),
}

impl<N> GraphEngine for AdjacencyMatrix<N>
where
    N: Node + Clone,
{


    fn count_nodes(&self) -> usize {
        self.count_nodes
    }

    fn remove_node_with_edges(&mut self, node_id: usize) {
        todo!()
    }

    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertID {
        todo!()
    }


    fn remove_edge<E>(&mut self, edge: E) where E: Into<EdgeRemoveAction> {
        todo!()
    }


    fn count_edges(&self) -> usize {
        self.count_edges
    }
}
