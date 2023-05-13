use graph_derive::Graph;
use graph_types::{EdgeQuery, GraphEngine, GraphKind, NodeRangeVisitor, NodesVisitor};
use std::mem::size_of;

/// https://reference.wolfram.com/language/ref/WheelGraph.html
#[repr(C)]
#[derive(Graph)]
pub struct WheelGraph {
    #[easy_graph(default = 5, wolfram = "WheelGraph")]
    mask: i32,
}

impl GraphEngine for WheelGraph {
    fn graph_kind(&self) -> GraphKind {
        match self.mask < 0 {
            true => GraphKind::Undirected,
            false => GraphKind::Directed,
        }
    }

    fn get_node_id(&self, node_id: usize) -> Option<usize> {
        (node_id < self.count_nodes()).then_some(node_id)
    }

    fn count_nodes(&self) -> usize {
        self.rank()
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        NodesVisitor::range(self, 0..self.count_nodes())
    }

    /// Edges ids are counted from 0 to `self.count_edges() - 1`.
    fn get_edge_id<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        match edge.into() {
            EdgeQuery::EdgeID(edge_id) => (edge_id < self.count_edges()).then_some(edge_id),
            EdgeQuery::Directed(_) => {
                todo!()
            }
            EdgeQuery::Undirected(_) => match self.graph_kind() {
                GraphKind::Directed => None,
                GraphKind::Undirected => {
                    todo!()
                }
            },
        }
    }

    fn traverse_edges(&self) -> NodeRangeVisitor<Self> {
        NodeRangeVisitor::new(self, 0..self.count_edges())
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
