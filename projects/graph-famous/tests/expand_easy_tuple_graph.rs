#![allow(unused)]

use super::*;

#[derive(Graph)]
pub struct EasyTupleGraph(#[easy_graph] i32);

impl GraphEngine for EasyTupleGraph {
    fn graph_kind(&self) -> GraphKind {
        todo!()
    }

    fn get_node(&self, node_id: usize) -> Option<usize> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        todo!()
    }

    fn all_nodes(&self) -> NodesVisitor<Self> {
        todo!()
    }

    fn get_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        todo!()
    }

    fn all_edges(&self) -> NodeRangeVisitor<Self> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        todo!()
    }
}
