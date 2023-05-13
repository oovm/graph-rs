use graph_derive::Graph;
use graph_types::{
    errors::GraphError,
    placeholder::{PlaceholderEdgeIterator, PlaceholderNodeIterator},
    Edge, EdgeID, EdgeQuery, GraphEngine, GraphKind, IndeterminateEdge, NodeID, NodeQuery, NodeRangeVisitor, NodesVisitor,
};
use std::mem::size_of;

/// https://reference.wolfram.com/language/ref/CycleGraph.html
#[repr(C)]
#[derive(Graph)]
pub struct CycleGraph {
    #[easy_graph(default = 5, wolfram = "CycleGraph")]
    mask: i32,
}

impl GraphEngine for CycleGraph {
    type NodeTraverser = PlaceholderNodeIterator;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeTraverser = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        match self.mask < 0 {
            true => GraphKind::Undirected,
            false => GraphKind::Directed,
        }
    }

    fn get_node(&self, node: NodeID) -> Result<NodeID, GraphError> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        self.rank()
    }

    fn all_nodes(&self) -> Self::NodeTraverser {
        todo!()
    }

    /// Numbered clockwise, if edge comes back, then edge id extra + 1.
    ///
    /// In a indirected graph, edges ids are counted from 0 to `self.count_edges() - 1`.
    ///
    /// And in a directed graph, edges ids are counted from 0 to `self.count_edges() - 1`.
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

    fn count_edges(&self) -> usize {
        match self.graph_kind() {
            GraphKind::Directed => self.rank(),
            GraphKind::Undirected => self.rank() * 2,
        }
    }

    /// Takes O(1) space, in fact it's always takes 32 bits.
    ///
    /// ```
    /// use graph_theory::{graph_engines::CycleGraph, GraphEngine};
    /// assert_eq!(CycleGraph::one_way(3).size_hint(), 4);
    /// assert_eq!(CycleGraph::one_way(4).size_hint(), 4);
    /// assert_eq!(CycleGraph::two_way(5).size_hint(), 4);
    /// ```
    fn size_hint(&self) -> usize {
        size_of::<CycleGraph>()
    }
}
