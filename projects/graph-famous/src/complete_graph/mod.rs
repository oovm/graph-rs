use graph_types::{Edge, EdgeInsertID, EdgeQuery, GetEdgesVisitor, GraphEngine, NodesVisitor};
use std::{
    fmt::{Debug, Display, Formatter},
    mem::size_of,
};

mod display;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;

/// [CompleteGraph](https://reference.wolfram.com/language/ref/CompleteGraph.html)
/// represents a graph where every node is connected to every other node.
///
/// ![](https://reference.wolfram.com/language/ref/Files/CompleteGraph.zh/O_1.png)
///
/// # Examples
///
/// ```
/// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
/// let graph = CompleteGraph::new(3);
/// assert_eq!(graph.count_nodes(), 3);
/// assert_eq!(graph.count_edges(), 12);
/// ```
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompleteGraph {
    rank: u32,
}

impl GraphEngine for CompleteGraph {
    fn has_node(&self, node_id: usize) -> bool {
        node_id < self.rank as usize
    }

    fn count_nodes(&self) -> usize {
        self.rank as usize
    }

    #[track_caller]
    fn remove_node_with_edges(&mut self, _: usize) {
        self.exception("remove node")
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        NodesVisitor::range(self, 0..self.count_nodes())
    }

    fn get_edges(&self) -> GetEdgesVisitor<Self> {
        todo!()
    }

    #[track_caller]
    fn insert_edge_with_nodes<E: Edge>(&mut self, _edge: E) -> EdgeInsertID {
        self.exception("insert edge")
    }

    #[track_caller]
    fn remove_edge<E>(&mut self, _: E)
    where
        E: Into<EdgeQuery>,
    {
        self.exception("remove edge")
    }

    fn count_edges(&self) -> usize {
        let rank = self.rank as usize;
        rank * (rank - 1) * 2
    }

    /// Takes O(1) space, in fact it's always takes 32 bits.
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// assert_eq!(CompleteGraph::new(3).size_hint(), 4);
    /// assert_eq!(CompleteGraph::new(4).size_hint(), 4);
    /// assert_eq!(CompleteGraph::new(5).size_hint(), 4);
    /// ```
    fn size_hint(&self) -> usize {
        size_of::<CompleteGraph>()
    }
}

impl CompleteGraph {
    /// Creates a new complete graph with the given rank.
    ///
    /// ![](https://raw.githubusercontent.com/oovm/graph-rs/dev/projects/graph-types/src/famous_graphs/complete_graph/k-complete.svg)
    ///
    /// ![](https://raw.githubusercontent.com/oovm/graph-rs/dev/projects/graph-types/src/famous_graphs/complete_graph/d-complete.svg)
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::CompleteGraph;
    /// let graph = CompleteGraph::new(3);
    /// ```
    pub fn new(rank: usize) -> Self {
        Self { rank: rank as u32 }
    }
    pub fn directed(rank: usize) -> Self {
        Self { rank: rank as u32 }
    }
    /// Check if the given graph is a complete graph, and if so, return it.
    pub fn check<G: GraphEngine>(graph: &G) -> Option<Self> {
        let nodes = graph.count_nodes();
        let edges = graph.count_edges();
        if edges == nodes * (nodes - 1) {
            if is_directed(graph, nodes) {
                return Some(Self { rank: nodes as u32 });
            }
        }
        None
    }
}

/// Add nodes degree is rank -1
fn is_directed<G>(graph: &G, rank: usize) -> bool
where
    G: GraphEngine,
{
    let _ = (graph, rank);
    todo!()
}

fn is_undirected<G>(graph: &G) -> bool
where
    G: GraphEngine,
{
    let _ = (graph,);
    todo!()
}
