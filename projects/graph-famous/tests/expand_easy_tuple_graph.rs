#![allow(unused)]

use super::*;

#[derive(Graph)]
pub struct EasyTupleGraph(#[easy_graph] i32);

impl GraphEngine for EasyTupleGraph {
    fn graph_kind(&self) -> GraphKind {
        todo!()
    }

    fn has_node(&self, node_id: usize) -> Option<usize> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        todo!()
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        todo!()
    }

    fn has_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        todo!()
    }

    fn traverse_edges(&self) -> NodeRangeVisitor<Self> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        todo!()
    }
}
