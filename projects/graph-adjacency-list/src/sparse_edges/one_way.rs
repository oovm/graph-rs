use super::*;
use crate::DiGraphSEAL;
use graph_types::{errors::GraphError, EdgeID, IndeterminateEdge, NodeID, Query};

impl GraphEngine for DiGraphSEAL {
    type NodeIterator = PlaceholderNodeIterator;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeIterator = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn get_node_id<Q: Into<NodeQuery>>(&self, node: Q) -> Result<NodeID, GraphError> {
        match node.into() {
            NodeQuery::NodeID(v) => {
                if self.nodes.contains(&(v as u32)) {
                    Ok(v)
                }
                else {
                    Err(GraphError::not_found(Query::NodeID(v)))
                }
            }
        }
    }

    fn all_node_ids(&self) -> Self::NodeIterator {
        todo!()
    }

    fn get_edge_id<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<EdgeID, GraphError> {
        let query = edge.into();
        match query {
            EdgeQuery::EdgeID(v) => {
                if self.edges.contains_key(&(v as u32)) {
                    Ok(v)
                }
                else {
                    Err(GraphError::not_found(Query::EdgeID(v)))
                }
            }
            EdgeQuery::Dynamic(v) => self.find_edge_id(v.from as u32, v.goto as u32),
            EdgeQuery::Directed(v) => self.find_edge_id(v.from as u32, v.goto as u32),
            EdgeQuery::Undirected(v) => v.as_unsupported(),
        }
    }

    fn all_edge_ids(&self) -> Self::EdgeIterator {
        todo!()
    }

    fn get_bridges<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<IndeterminateEdge, GraphError> {
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
