use super::*;
use crate::AdjacencyMatrixAllBridges;
use graph_types::iterators::BridgeRange;
use std::slice::Iter;

impl<'a> GraphEngine<'a> for DiGraphAM {
    type NeighborIterator = PlaceholderNodeIterator;
    type BridgeIterator = BridgeRange<'a, DiGraphAM>;
    type NodeTraverser = Range<usize>;
    type EdgeTraverser = Range<usize>;
    type BridgeTraverser = AdjacencyMatrixAllBridges<'a>;

    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn get_node(&self, node: NodeID) -> Result<NodeID, GraphError> {
        Query::check_node_range(node, self.count_nodes())
    }

    fn all_nodes(&self) -> Self::NodeTraverser {
        0..self.count_nodes()
    }

    fn count_nodes(&self) -> usize {
        self.rank
    }

    fn all_neighbors(&'a self, node: usize) -> Self::NeighborIterator {
        todo!()
    }
    fn get_outgoing(&'a self, node: NodeID) -> Self::NeighborIterator {
        todo!()
    }
    fn get_incoming(&'a self, node: NodeID) -> Self::NeighborIterator {
        todo!()
    }

    fn get_edge(&self, edge: EdgeID) -> Result<EdgeID, GraphError> {
        Query::check_node_range(edge, self.count_edges())
    }

    fn all_edges(&self) -> Self::EdgeTraverser {
        0..self.count_edges()
    }

    fn count_edges(&self) -> usize {
        self.edges.len()
    }

    fn get_bridge(&self, edge: NodeID) -> Result<IndeterminateEdge, GraphError> {
        todo!()
    }

    fn get_bridges(&'a self, from: NodeID, goto: NodeID) -> Self::BridgeIterator {
        todo!()
    }

    fn all_bridges(&'a self) -> Self::BridgeTraverser {
        AdjacencyMatrixAllBridges::new(&self.edges)
    }

    fn size_hint(&self) -> usize {
        size_of::<Self>()
            + self.edges.capacity() * size_of::<IndeterminateEdge>()
            + self.matrix.capacity() * size_of::<AdjacencyCell>()
    }
}

impl DiGraphAM {
    /// Returns the number of nodes in the graph.
    pub fn new(nodes: usize, edges: usize) -> Self {
        Self {
            rank: nodes,
            edges: vec![IndeterminateEdge { from: 0, goto: 0 }; edges],
            matrix: vec![AdjacencyCell::default(); nodes * nodes],
        }
    }
    fn find_first_edge(&self, start: u32, end: u32) -> Result<EdgeID, GraphError> {
        todo!()
    }
}
