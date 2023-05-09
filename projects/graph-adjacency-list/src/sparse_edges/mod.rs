use crate::utils::ShortEdge;
use graph_types::{
    DirectedEdge, Edge, EdgeDirection, EdgeInsertID, EdgeQuery, EdgesVisitor, GraphEngine, GraphKind, MutableGraph,
    NodesVisitor,
};
use std::collections::{BTreeMap, BTreeSet};

type NodeID = u32;
type EdgeID = u32;

#[doc = include_str!("AdjacencyEdgeList.html")]
#[derive(Debug)]
pub struct AdjacencyEdgeList {
    nodes: BTreeSet<NodeID>,
    edges: BTreeMap<EdgeID, ShortEdge>,
}

impl GraphEngine for AdjacencyEdgeList {
    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn has_node(&self, node_id: usize) -> Option<usize> {
        self.nodes.contains(&(node_id as u32)).then(|| node_id)
    }

    fn count_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        let max_id = self.nodes.last().map(|s| *s as usize).unwrap_or(0);
        NodesVisitor::range(self, 0..=max_id)
    }

    fn has_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        match edge.into() {
            EdgeQuery::EdgeID(i) => self.edges.contains_key(&(i as u32)).then_some(i),
            EdgeQuery::Directed(_) => {
                todo!()
            }
            EdgeQuery::Undirected(_) => {
                todo!()
            }
        }
    }

    fn traverse_edges(&self) -> EdgesVisitor<Self> {
        EdgesVisitor::range(self, 0..=self.count_edges())
    }

    fn count_edges(&self) -> usize {
        self.edges.len()
    }

    fn size_hint(&self) -> usize {
        todo!()
    }
}

impl MutableGraph for AdjacencyEdgeList {
    fn insert_node(&mut self, node_id: usize) -> usize {
        todo!()
    }

    fn remove_node_with_edges(&mut self, node_id: usize) {
        let id = node_id as u32;
        todo!()
    }
    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertID {
        let lhs = edge.lhs() as u32;
        let rhs = edge.rhs() as u32;
        match edge.direction() {
            EdgeDirection::Disconnect => EdgeInsertID::Nothing,
            EdgeDirection::TwoWay => {
                let e1 = self.insert_one_way_edge(lhs, rhs);
                let e2 = self.insert_one_way_edge(rhs, lhs);
                EdgeInsertID::TwoEdges(e1, e2)
            }
            EdgeDirection::Forward => {
                let e1 = self.insert_one_way_edge(lhs, rhs);
                EdgeInsertID::OneEdge(e1)
            }
            EdgeDirection::Reverse => {
                let e1 = self.insert_one_way_edge(rhs, lhs);
                EdgeInsertID::OneEdge(e1)
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
}

impl AdjacencyEdgeList {
    pub(crate) fn insert_one_way_edge(&mut self, start: u32, end: u32) -> usize {
        let id = self.edges.len() as u32 + 1;
        self.edges.insert(id, ShortEdge { from: start, goto: end });
        id as usize
    }
}
