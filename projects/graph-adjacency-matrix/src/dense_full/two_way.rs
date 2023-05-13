use super::*;
use crate::AdjacencyMatrixAllBridges;
use std::slice::Iter;

impl<'a> GraphEngine<'a> for DiGraphAM {
    type NeighborIterator = PlaceholderNodeIterator;
    type BridgeIterator = DiGraphBridges<'a>;
    type NodeTraverser = Range<usize>;
    type EdgeTraverser = Range<usize>;
    type BridgeTraverser = AdjacencyMatrixAllBridges<'a>;

    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn get_node(&self, node: NodeID) -> Result<NodeID, GraphError> {
        Query::check_node_range(node, self.rank)
    }

    fn all_nodes(&self) -> Self::NodeTraverser {
        0..self.rank
    }

    fn count_nodes(&self) -> usize {
        self.matrix.len()
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

    fn get_edge<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<EdgeID, GraphError> {
        match edge.into() {
            EdgeQuery::EdgeID(v) => {
                let max = self.edges.len();
                if v < max { Ok(v) } else { Err(GraphError::edge_out_of_range(v, max)) }
            }
            EdgeQuery::Dynamic(v) => self.find_first_edge(v.from as u32, v.goto as u32),
            EdgeQuery::Directed(v) => self.find_first_edge(v.from as u32, v.goto as u32),
            EdgeQuery::Undirected(v) => v.as_unsupported(),
        }
    }

    fn all_edges(&self) -> Self::EdgeTraverser {
        0..self.count_edges()
    }

    fn count_edges(&self) -> usize {
        self.edges.len()
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

#[derive(Clone, Debug)]
pub struct DiGraphBridges<'a> {
    graph: &'a DiGraphAM,
    index: usize,
}

impl<'a> Iterator for DiGraphBridges<'a> {
    type Item = IndeterminateEdge;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> DoubleEndedIterator for DiGraphBridges<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
