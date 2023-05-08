use graph_types::{DirectedEdge, DynamicEdge, Edge, EdgeInsertID, EdgeQuery, GetEdgesVisitor, GraphEngine, NodesVisitor};
use std::collections::BTreeMap;

type NodeID = u32;
type EdgeID = u32;

#[doc = include_str!("AdjacencyNodeList.html")]
#[derive(Debug)]
pub struct AdjacencyEdgeList {
    edges: BTreeMap<EdgeID, DynamicEdge>,
}

impl GraphEngine for AdjacencyEdgeList {
    fn has_node(&self, node_id: usize) -> bool {
        for edge in self.edges.values() {
            if edge.from == node_id || edge.goto == node_id {
                return true;
            }
        }
        false
    }

    fn remove_node_with_edges(&mut self, node_id: usize) {
        todo!()
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        todo!()
    }

    fn get_edges(&self) -> GetEdgesVisitor<Self> {
        GetEdgesVisitor::new(self)
    }

    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertID {
        match edge.into() {
            EdgeQuery::EdgeID(i) => {
                self.edges.insert(i as u32, AdjacencyEdge { lhs: 0, rhs: 0 });
            }
            EdgeQuery::Directed(_) => {}
            EdgeQuery::Undirected(_) => {}
        }
        todo!()
    }

    fn remove_edge<E>(&mut self, edge: E)
    where
        E: Into<EdgeQuery>,
    {
        match edge.into() {
            EdgeQuery::EdgeID(i) => {
                self.edges.remove(&(i as u32));
            }
            EdgeQuery::Directed(_) => {}
            EdgeQuery::Undirected(_) => {}
        }
    }

    fn count_edges(&self) -> usize {
        self.edges.len()
    }
}
