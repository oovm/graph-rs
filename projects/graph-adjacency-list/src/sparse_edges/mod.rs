use graph_types::{
    DirectedEdge, DynamicEdge, Edge, EdgeDirection, EdgeInsertID, EdgeQuery, GetEdgesVisitor, GraphEngine, NodesVisitor,
};
use std::collections::BTreeMap;

type NodeID = u32;
type EdgeID = u32;

#[doc = include_str!("AdjacencyEdgeList.html")]
#[derive(Debug)]
pub struct AdjacencyEdgeList {
    edges: BTreeMap<EdgeID, ShortEdge>,
}

pub struct ShortEdge {
    from: NodeID,
    goto: NodeID,
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
        match edge.direction() {
            EdgeDirection::Disconnect => EdgeInsertID::Nothing,
            EdgeDirection::Dynamic => {}
            EdgeDirection::TwoWay => {}
            EdgeDirection::Forward => {}
            EdgeDirection::Reverse => {}
        }

        match edge.into() {
            EdgeQuery::EdgeID(_) => {
                panic!("Cannot insert edge id to AdjacencyEdgeList")
            }
            EdgeQuery::Directed(v) => {
                let e1 = self.insert_one_way_edge(v.from as u32, v.goto as u32);
                EdgeInsertID::OneEdge(e1)
            }
            EdgeQuery::Undirected(v) => {
                let e1 = self.insert_one_way_edge(v.from as u32, v.goto as u32);
                let e2 = self.insert_one_way_edge(v.goto as u32, v.from as u32);
                EdgeInsertID::TwoEdges(e1, e2)
            }
        }
    }

    fn remove_edge<E>(&mut self, edge: E)
    where
        E: Into<EdgeQuery>,
    {
        match edge.into() {
            EdgeQuery::EdgeID(i) => {
                self.edges.remove(&(i as u32));
            }
            EdgeQuery::Directed(di) => {
                todo!()
            }
            EdgeQuery::Undirected(_) => {
                todo!()
            }
        }
    }

    fn count_edges(&self) -> usize {
        self.edges.len()
    }
}

impl AdjacencyEdgeList {
    pub(crate) fn insert_one_way_edge(&mut self, start: u32, end: u32) -> usize {
        let id = self.edges.len() as u32 + 1;
        self.edges.insert(id, ShortEdge { from: start, goto: end });
        id as usize
    }
}
