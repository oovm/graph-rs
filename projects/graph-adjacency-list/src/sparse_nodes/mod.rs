use graph_types::{
    errors::GraphError,
    placeholder::{PlaceholderEdgeIterator, PlaceholderNodeIterator},
    Edge, EdgeDirection, EdgeInsertID, EdgeQuery, GraphEngine, GraphKind, IndeterminateEdge, MutableGraph, NodeID, NodeQuery,
    NodeRangeVisitor, NodesVisitor,
};
use std::collections::BTreeMap;

type EdgeID = u32;
type StartNodeID = u32;
type EndNodeID = u32;

#[doc = include_str!("AdjacencyNodeList.html")]
#[derive(Debug)]
pub struct AdjacencyNodeList {
    head_nodes: BTreeMap<StartNodeID, NodeNeighbors>,
    last_edge: EdgeID,
}

#[derive(Debug)]
struct NodeNeighbors {
    end_nodes: BTreeMap<EdgeID, EndNodeID>,
}

impl Default for AdjacencyNodeList {
    fn default() -> Self {
        Self { head_nodes: BTreeMap::new(), last_edge: 0 }
    }
}

impl GraphEngine for AdjacencyNodeList {
    type NodeIterator = PlaceholderNodeIterator;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeIterator = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        todo!()
    }

    fn get_node_id<Q: Into<NodeQuery>>(&self, node: Q) -> Result<NodeID, GraphError> {
        todo!()
    }

    fn traverse_nodes(&self) -> Self::NodeIterator {
        todo!()
    }

    fn get_edge_id<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<graph_types::EdgeID, GraphError> {
        todo!()
    }

    fn traverse_edges(&self) -> Self::EdgeIterator {
        todo!()
    }

    fn get_bridge<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<IndeterminateEdge, GraphError> {
        todo!()
    }

    fn traverse_bridges(&self) -> Self::BridgeIterator {
        todo!()
    }
}

impl MutableGraph for AdjacencyNodeList {
    fn insert_node(&mut self, node_id: usize) -> bool {
        let id = node_id as u32;
        self.head_nodes.entry(id).or_insert_with(|| NodeNeighbors { end_nodes: BTreeMap::new() });
        node_id < (u32::MAX as usize)
    }

    fn create_node(&mut self) -> usize {
        todo!()
    }

    fn remove_node_with_edges(&mut self, node_id: usize) {
        self.head_nodes.remove(&(node_id as u32));
    }

    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertID {
        let lhs = edge.lhs() as u32;
        let rhs = edge.rhs() as u32;
        match edge.direction() {
            EdgeDirection::Disconnect => EdgeInsertID::Nothing,
            EdgeDirection::Forward => {
                let e1 = self.insert_directed_edge(lhs, rhs);
                EdgeInsertID::OneEdge(e1)
            }
            EdgeDirection::Reverse => {
                let e1 = self.insert_directed_edge(rhs, lhs);
                EdgeInsertID::OneEdge(e1)
            }
            EdgeDirection::TwoWay => {
                let e1 = self.insert_directed_edge(lhs, rhs);
                let e2 = self.insert_directed_edge(rhs, lhs);
                EdgeInsertID::TwoEdges(e1, e2)
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
            EdgeQuery::EdgeID(v) => {
                let edge_id = v as u32;
                for (_, node) in self.head_nodes.iter_mut() {
                    node.end_nodes.remove(&edge_id);
                    // edge id is unique in the graph
                    break;
                }
            }
            EdgeQuery::Directed(v) => {
                let start_node_id = v.lhs() as u32;
                let end_node_id = v.rhs() as u32;
                if let Some(node) = self.head_nodes.get_mut(&start_node_id) {
                    // notice that there are multiple edges between two nodes
                    node.end_nodes.retain(|_, &mut v| v != end_node_id);
                }
            }
            EdgeQuery::Undirected(v) => {
                panic!("remove undirected edge {v} is not supported in directed graph");
            }
            EdgeQuery::Dynamic(_) => {
                todo!()
            }
        }
    }
}

impl AdjacencyNodeList {
    /// The low level interface for inserting a directed edge
    pub(crate) fn insert_directed_edge(&mut self, from: u32, goto: u32) -> usize {
        self.last_edge += 1;
        let new_edge_id = self.last_edge;
        let from_node = self.head_nodes.entry(from).or_insert_with(|| NodeNeighbors { end_nodes: BTreeMap::new() });
        from_node.end_nodes.insert(new_edge_id, goto);
        new_edge_id as usize
    }
}
