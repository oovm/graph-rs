use graph_types::{Edge, EdgeInsertID, EdgeQuery, GetEdgesVisitor, GraphEngine, NodesVisitor};
use std::{fmt::Debug, mem::size_of};

// https://reference.wolfram.com/language/ref/StarGraph.html
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StarGraph {
    rank: u32,
}

impl GraphEngine for StarGraph {
    fn has_node(&self, node_id: usize) -> bool {
        node_id < self.rank as usize
    }

    fn count_nodes(&self) -> usize {
        self.rank as usize
    }

    fn remove_node_with_edges(&mut self, _: usize) {
        self.exception("remove node")
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        NodesVisitor::range(self, 0..self.count_nodes())
    }

    fn get_edges(&self) -> GetEdgesVisitor<Self> {
        todo!()
    }

    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertID {
        todo!()
    }

    fn remove_edge<E>(&mut self, _: E)
    where
        E: Into<EdgeQuery>,
    {
        self.exception("remove edge")
    }

    fn count_edges(&self) -> usize {
        self.rank as usize * 2
    }

    /// Takes O(1) space, in fact it's always takes 32 bits.
    ///
    /// ```
    /// use graph_theory::{graph_engines::StarGraph, GraphEngine};
    /// assert_eq!(StarGraph::new(3).size_hint(), 4);
    /// assert_eq!(StarGraph::new(4).size_hint(), 4);
    /// assert_eq!(StarGraph::new(5).size_hint(), 4);
    /// ```
    fn size_hint(&self) -> usize {
        size_of::<StarGraph>()
    }
}

impl StarGraph {
    pub fn new(rank: u32) -> Self {
        Self { rank }
    }
}
