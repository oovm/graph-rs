use graph_derive::Graph;
use graph_types::{Edge, EdgeQuery, EdgesVisitor, GraphEngine, GraphKind, NodesVisitor};
use std::mem::size_of;

/// https://reference.wolfram.com/language/ref/CycleGraph.html
#[repr(C)]
#[derive(Graph)]
pub struct CycleGraph {
    #[easy_graph(default = 5, wolfram = "CycleGraph")]
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
        (node_id < self.count_nodes()).then_some(node_id)
    }

    fn count_nodes(&self) -> usize {
        self.rank()
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        NodesVisitor::range(self, 0..self.count_nodes())
    }

    /// In a indirected graph, edges ids are counted from 0 to `self.count_edges() - 1`.
    ///
    /// And in a directed graph, edges ids are counted from 0 to `self.count_edges() - 1`.
    fn has_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        match edge.into() {
            EdgeQuery::EdgeID(edge_id) => (edge_id < self.count_edges()).then_some(edge_id),
            EdgeQuery::Directed(v) if v.max_index() < self.count_nodes() => {
                // if back, edge id + 1
                let dir = if v.goto > v.from { 0 } else { 1 };
                // adjacent nodes
                if v.delta_index() == 1 {
                    return Some(v.min_index() * 2 + dir);
                }
                // last edge
                else if self.count_nodes() == v.delta_index() + 1 {
                    return Some(v.max_index() * 2 + dir);
                }
            }
            EdgeQuery::Undirected(v) => match self.graph_kind() {
                GraphKind::Undirected if v.max_index() < self.count_nodes() => {
                    // adjacent nodes
                    if v.delta_index() == 1 {
                        return Some(v.min_index());
                    }
                    // last edge
                    else if self.count_nodes() == v.delta_index() + 1 {
                        return Some(v.max_index());
                    }
                }
                _ => {}
            },
            _ => {}
        }
        None
    }

    fn traverse_edges(&self) -> EdgesVisitor<Self> {
        EdgesVisitor::range(self, 0..self.count_edges())
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
