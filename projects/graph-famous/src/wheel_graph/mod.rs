use graph_derive::Graph;
use graph_types::{
    errors::GraphError,
    placeholder::{PlaceholderEdgeIterator, PlaceholderNodeIterator},
    EdgeID, EdgeQuery, GraphEngine, GraphKind, IndeterminateEdge, NodeID,
};
use std::mem::size_of;

/// https://reference.wolfram.com/language/ref/WheelGraph.html
#[repr(C)]
#[derive(Graph)]
pub struct WheelGraph {
    #[easy_graph(default = 5, wolfram = "WheelGraph")]
    mask: i32,
}

impl<'a> GraphEngine<'a> for WheelGraph {
    type NeighborIterator = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;
    type NodeTraverser = PlaceholderNodeIterator;
    type EdgeTraverser = PlaceholderNodeIterator;
    type BridgeTraverser = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        match self.mask < 0 {
            true => GraphKind::Undirected,
            false => GraphKind::Directed,
        }
    }

    fn get_node(&self, node: NodeID) -> Result<NodeID, GraphError> {
        todo!()
    }

    fn all_nodes(&self) -> Self::NodeTraverser {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        self.rank()
    }

    fn all_neighbors(&'a self, node: NodeID) -> Self::NeighborIterator {
        todo!()
    }

    fn get_edge(&self, edge: EdgeID) -> Result<EdgeID, GraphError> {
        todo!()
    }

    fn all_edges(&'a self) -> Self::EdgeTraverser {
        todo!()
    }

    fn count_edges(&self) -> usize {
        match self.graph_kind() {
            GraphKind::Directed => self.rank(),
            GraphKind::Undirected => self.rank() * 2,
        }
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
