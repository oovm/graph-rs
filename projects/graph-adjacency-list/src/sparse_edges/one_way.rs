use super::*;
use crate::{
    iters::{AdjacencyEdgeAllEdges, AdjacencyEdgeAllNodes},
    DiGraphSEAL,
};
use graph_types::{errors::GraphError, EdgeID, IndeterminateEdge, NodeID, Query};

impl<'a> GraphEngine<'a> for DiGraphSEAL {
    type NeighborIterator = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;
    type NodeTraverser = AdjacencyEdgeAllNodes<'a>;
    type EdgeTraverser = AdjacencyEdgeAllEdges<'a>;
    type BridgeTraverser = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn get_node(&self, node: NodeID) -> Result<NodeID, GraphError> {
        if self.nodes.contains(&(node as u32)) { Ok(node) } else { Err(GraphError::not_found(Query::NodeID(node))) }
    }

    fn all_nodes(&'a self) -> Self::NodeTraverser {
        AdjacencyEdgeAllNodes::new(&self.nodes)
    }

    fn all_neighbors(&'a self, node: NodeID) -> Self::NeighborIterator {
        todo!()
    }

    fn get_edge(&self, edge: EdgeID) -> Result<EdgeID, GraphError> {
        if self.edges.contains_key(&(edge as u32)) { Ok(edge) } else { Err(GraphError::not_found(Query::EdgeID(edge))) }
    }

    fn all_edges(&'a self) -> Self::EdgeTraverser {
        AdjacencyEdgeAllEdges::new(&self.edges)
    }

    fn get_bridge(&self, edge: EdgeID) -> Result<IndeterminateEdge, GraphError> {
        todo!()
    }

    fn get_bridges(&'a self, from: NodeID, goto: NodeID) -> Self::BridgeIterator {
        todo!()
    }

    fn all_bridges(&self) -> Self::BridgeIterator {
        todo!()
    }
}

impl MutableGraph for DiGraphSEAL {
    fn insert_node(&mut self, node_id: usize) -> bool {
        self.nodes.insert(node_id as u32)
    }

    fn create_node(&mut self) -> usize {
        let id = self.nodes.iter().last().map(|x| x + 1).unwrap_or(0);
        self.nodes.insert(id);
        id as usize
    }

    fn remove_node_with_edges(&mut self, node_id: usize) {
        let _id = node_id as u32;
        todo!()
    }
    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertID {
        let lhs = edge.lhs();
        let rhs = edge.rhs();
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
            EdgeDirection::Indeterminate => {
                todo!()
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
            EdgeQuery::Directed(_di) => {
                todo!()
            }
            EdgeQuery::Undirected(_) => {
                todo!()
            }
            EdgeQuery::Dynamic(_) => {
                todo!()
            }
        }
    }
}

impl DiGraphSEAL {
    pub(crate) fn find_edge_id(&self, from: u32, goto: u32) -> Result<EdgeID, GraphError> {
        todo!()
    }

    pub(crate) fn insert_one_way_edge(&mut self, start: usize, end: usize) -> usize {
        let id = self.edges.len() as u32 + 1;
        self.edges.insert(id, ShortEdge::new(start, end));
        id as usize
    }
}
