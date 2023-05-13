use graph_derive::Graph;
use graph_types::{
    errors::GraphError,
    placeholder::{PlaceholderEdgeIterator, PlaceholderNodeIterator},
    EdgeID, EdgeQuery, GraphEngine, GraphKind, IndeterminateEdge, NodeID, NodeQuery, NodeRangeVisitor, NodesVisitor,
};
use std::mem::size_of;

/// https://reference.wolfram.com/language/ref/WheelGraph.html
#[repr(C)]
#[derive(Graph)]
pub struct WheelGraph {
    #[easy_graph(default = 5, wolfram = "WheelGraph")]
    mask: i32,
}

impl GraphEngine for WheelGraph {
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
    /// use graph_theory::{graph_engines::WheelGraph, GraphEngine};
    /// assert_eq!(WheelGraph::one_way(3).size_hint(), 4);
    /// assert_eq!(WheelGraph::one_way(4).size_hint(), 4);
    /// assert_eq!(WheelGraph::two_way(5).size_hint(), 4);
    /// ```
    fn size_hint(&self) -> usize {
        size_of::<WheelGraph>()
    }
}
