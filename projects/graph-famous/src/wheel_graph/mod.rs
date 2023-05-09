use crate::StarGraph;
use graph_types::{Edge, EdgeInsertID, EdgeQuery, EdgesVisitor, GraphEngine, GraphKind, NodesVisitor};
use std::{fmt::Debug, mem::size_of};

// https://reference.wolfram.com/language/ref/WheelGraph.html
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WheelGraph {
    mask: i32,
}

impl GraphEngine for WheelGraph {
    fn graph_kind(&self) -> GraphKind {
        match self.mask < 0 {
            true => GraphKind::Undirected,
            false => GraphKind::Directed,
        }
    }

    fn has_node(&self, node_id: usize) -> Option<usize> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        self.mask as usize
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        NodesVisitor::range(self, 0..self.count_nodes())
    }

    fn has_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        match edge.into() {
            EdgeQuery::EdgeID(i) => {
                if i < self.count_edges() {
                    Some(i)
                }
                else {
                    None
                }
            }
            EdgeQuery::Directed(_) => {
                todo!()
            }
            EdgeQuery::Undirected(_) => {
                todo!()
            }
        }
    }

    fn traverse_edges(&self) -> EdgesVisitor<Self> {
        EdgesVisitor::range(self, 0..=self.count_edges())
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

impl WheelGraph {
    pub fn one_way(rank: usize) -> Self {
        Self { mask: rank as i32 }
    }
    pub fn two_way(rank: usize) -> Self {
        Self { mask: -(rank as i32) }
    }
    pub fn rank(&self) -> usize {
        self.mask.abs() as usize
    }
}
