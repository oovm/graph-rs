use crate::StaticDirected;
use graph_types::{Edge, EdgeInsertID, EdgeRemoveAction, NodesVisitor, GraphEngine, Node};
use std::{collections::BTreeMap};

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
    fn has_node(&self, node_id: usize) -> Option<usize> {
        todo!()
    }

    fn get_nodes(&self) -> NodesVisitor<Self> {
        todo!()
    }


    fn count_nodes(&self) -> usize {
        self.count_nodes
    }

    fn remove_node_with_edges(&mut self, _node_id: usize) {
        todo!()
    }

    fn insert_edge_with_nodes<E: Edge>(&mut self, _edge: E) -> EdgeInsertID {
        todo!()
    }


    fn remove_edge<E>(&mut self, _edge: E) where E: Into<EdgeRemoveAction> {
        todo!()
    }


    fn count_edges(&self) -> usize {
        self.count_edges
    }
}
