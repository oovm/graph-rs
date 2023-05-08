use graph_types::{Edge, EdgeInsertID, EdgeQuery, GetEdgesVisitor, GraphEngine, NodesVisitor};
use std::fmt::Debug;

/// https://reference.wolfram.com/language/ref/CycleGraph.html
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CycleGraph {
    rank: usize,
}

impl GraphEngine for CycleGraph {
    fn has_node(&self, node_id: usize) -> bool {
        node_id < self.rank
    }

    fn remove_node_with_edges(&mut self, _: usize) {
        self.exception("remove node")
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        NodesVisitor::range(self, 0..self.count_nodes())
    }

    fn get_edges(&self) -> GetEdgesVisitor<Self> {
        todo!()
    }

    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertID {
        todo!()
    }

    fn remove_edge<E>(&mut self, edge: E)
    where
        E: Into<EdgeQuery>,
    {
        todo!()
    }

    fn count_edges(&self) -> usize {
        todo!()
    }
}
