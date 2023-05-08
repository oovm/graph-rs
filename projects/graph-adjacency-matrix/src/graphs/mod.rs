use crate::StaticDirected;
use graph_types::{Edge, EdgeInsertID, EdgeQuery, GetEdgesVisitor, GraphEngine, Node, NodesVisitor};
use std::collections::BTreeMap;

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
    fn has_node(&self, node_id: usize) -> bool {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        self.count_nodes
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        todo!()
    }

    fn get_edges(&self) -> GetEdgesVisitor<Self> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        self.count_edges
    }
}
