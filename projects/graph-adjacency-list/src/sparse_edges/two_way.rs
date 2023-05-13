use super::*;
use graph_types::{errors::GraphError, EdgeID, IndeterminateEdge, NodeID};

impl GraphEngine for AdjacencyEdgeList<{ GraphKind::Undirected.is_one_way() }> {
    type NodeTraverser = PlaceholderNodeIterator;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeTraverser = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn get_node(&self, node: NodeID) -> Result<NodeID, GraphError> {
        todo!()
    }

    fn all_nodes(&self) -> Self::NodeTraverser {
        todo!()
    }

    fn get_edge<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<EdgeID, GraphError> {
        todo!()
    }

    fn all_edges(&self) -> Self::EdgeTraverser {
        todo!()
    }

    fn get_bridges<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<IndeterminateEdge, GraphError> {
        todo!()
    }

    fn all_bridges(&self) -> Self::BridgeIterator {
        todo!()
    }
}

impl MutableGraph for AdjacencyEdgeList<{ GraphKind::Undirected.is_one_way() }> {
    fn insert_node(&mut self, node_id: usize) -> bool {
        todo!()
    }

    fn create_node(&mut self) -> usize {
        todo!()
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

impl AdjacencyEdgeList<{ GraphKind::Undirected.is_one_way() }> {
    pub(crate) fn insert_one_way_edge(&mut self, start: usize, end: usize) -> usize {
        let id = self.edges.len() as u32 + 1;
        self.edges.insert(id, ShortEdge::new(start, end));
        id as usize
    }
}
