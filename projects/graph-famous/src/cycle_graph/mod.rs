use graph_types::{Edge, EdgeInsertID, EdgeQuery, GetEdgesVisitor, GraphEngine, GraphKind, NodesVisitor};
use std::{fmt::Debug, mem::size_of};

/// https://reference.wolfram.com/language/ref/CycleGraph.html
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CycleGraph {
    mask: i32,
}

impl GraphEngine for CycleGraph {
    fn graph_kind(&self) -> GraphKind {
        match self.mask < 0 {
            true => GraphKind::Undirected,
            false => GraphKind::Directed,
        }
    }

    fn has_node(&self, node_id: usize) -> Option<usize> {
        (node_id < self.rank()).then_some(node_id)
    }

    fn count_nodes(&self) -> usize {
        self.rank()
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        NodesVisitor::range(self, 0..self.count_nodes())
    }

    fn has_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        todo!()
    }

    fn traverse_edges(&self) -> GetEdgesVisitor<Self> {
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

impl CycleGraph {
    pub fn one_way(rank: usize) -> Self {
        Self { mask: rank as i32 }
    }
    pub fn two_way(rank: usize) -> Self {
        Self { mask: -(rank as i32) }
    }
    pub fn rank(&self) -> usize {
        self.mask.abs() as usize
    }
    pub fn is_two_way(&self) -> bool {
        self.mask < 0
    }
}