#![allow(unused)]

use super::*;

#[derive(Graph)]
pub struct EasyTupleGraph(#[easy_graph] i32);

impl GraphEngine for EasyTupleGraph {
    fn graph_kind(&self) -> GraphKind {
        todo!()
    }

    fn get_node_id(&self, node_id: usize) -> Option<usize> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        todo!()
    }

    fn all_node_ids(&self) -> NodesVisitor<Self> {
        todo!()
    }

    fn get_edge_id<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        todo!()
    }

    fn all_edge_ids(&self) -> NodeRangeVisitor<Self> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        todo!()
    }
}
