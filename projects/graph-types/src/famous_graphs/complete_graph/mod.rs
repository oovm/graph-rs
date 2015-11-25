use crate::{DirectedEdge, Graph};
use std::borrow::Cow;

/// [CompleteGraph](https://reference.wolfram.com/language/ref/CompleteGraph.html)
/// represents a graph where every node is connected to every other node.
///
/// # Examples
///
/// ```
/// use graph_theory::CompleteGraph;
/// let graph = CompleteGraph::new(3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompleteGraph {
    directed: bool,
    rank: usize,
}

impl Graph for CompleteGraph {
    type NodeIndex = usize;
    type EdgeIndex = usize;

    fn get_node_id(&self, index: Self::NodeIndex) -> Option<usize> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        todo!()
    }

    fn get_edge(&self, index: usize) -> Option<Cow<Self::EdgeIndex>> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        todo!()
    }
}

impl CompleteGraph {
    /// Creates a new complete graph with the given rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::CompleteGraph;
    /// let graph = CompleteGraph::new(3);
    /// ```
    pub fn new(rank: usize) -> Self {
        Self { rank }
    }
}
