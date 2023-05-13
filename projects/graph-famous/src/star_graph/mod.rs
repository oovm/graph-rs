use graph_derive::Graph;
use graph_types::{
    errors::GraphError,
    placeholder::{PlaceholderEdgeIterator, PlaceholderNodeIterator},
    EdgeID, EdgeQuery, GraphEngine, GraphKind, IndeterminateEdge, NodeID, NodeQuery, NodeRangeVisitor, NodesVisitor,
};
use std::mem::size_of;

// https://reference.wolfram.com/language/ref/StarGraph.html
#[repr(C)]
#[derive(Graph)]
pub struct StarGraph {
    #[easy_graph(default = 5, wolfram = "StarGraph")]
    mask: i32,
}

impl GraphEngine for StarGraph {
    type NodeIterator = PlaceholderNodeIterator;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeIterator = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        match self.mask < 0 {
            true => GraphKind::Undirected,
            false => GraphKind::Directed,
        }
    }

    fn get_node_id<Q: Into<NodeQuery>>(&self, node: Q) -> Result<NodeID, GraphError> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        self.rank()
    }

    fn traverse_nodes(&self) -> Self::NodeIterator {
        todo!()
    }

    fn get_edge_id<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<EdgeID, GraphError> {
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

    fn count_edges(&self) -> usize {
        match self.graph_kind() {
            GraphKind::Directed => self.rank(),
            GraphKind::Undirected => self.rank() * 2,
        }
    }

    /// Takes O(1) space, in fact it's always takes 32 bits.
    ///
    /// ```
    /// use graph_theory::{graph_engines::StarGraph, GraphEngine};
    /// assert_eq!(StarGraph::one_way(3).size_hint() * 8, 32);
    /// assert_eq!(StarGraph::one_way(4).size_hint() * 8, 32);
    /// assert_eq!(StarGraph::two_way(5).size_hint() * 8, 32);
    /// ```
    fn size_hint(&self) -> usize {
        size_of::<StarGraph>()
    }
}
