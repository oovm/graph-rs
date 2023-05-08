use graph_types::{DirectedEdge, Edge, EdgeInsertID, EdgeQuery, GetEdgesVisitor, GraphEngine, NodesVisitor};
use std::collections::BTreeMap;

type NodeID = u32;
type EdgeID = u32;

pub type UnGraph = AdjacencyEdgeList<true>;
pub type DiGraph = AdjacencyEdgeList<false>;

#[doc = include_str!("AdjacencyNodeList.html")]
#[derive(Debug)]
pub struct AdjacencyEdgeList {
    edges: BTreeMap<EdgeID, AdjacencyEdge>,
}

struct AdjacencyEdge {
    lhs: NodeID,
    rhs: NodeID,
}

impl<const TWO_WAY: bool> AdjacencyEdgeList<TWO_WAY> {
    pub(crate) fn has_node(&self, node: u32) -> bool {
        for edge in self.edges.values() {
            if edge.lhs == node || edge.rhs == node {
                return true;
            }
        }
        false
    }
}

impl GraphEngine for DiGraph {
    fn has_node(&self, node_id: usize) -> bool {
        self.has_node(node_id as u32)
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
