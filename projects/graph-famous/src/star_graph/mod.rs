use graph_derive::Graph;
use graph_types::{EdgeQuery, EdgesVisitor, GraphEngine, GraphKind, NodesVisitor};
use std::mem::size_of;

// https://reference.wolfram.com/language/ref/StarGraph.html
#[repr(C)]
#[derive(Graph)]
pub struct StarGraph {
    #[easy_graph(!constructor)]
    mask: i32,
}

impl GraphEngine for StarGraph {
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

    fn traverse_edges(&self) -> EdgesVisitor<Self> {
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
