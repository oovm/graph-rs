use crate::{DirectedEdge, Graph};
use std::borrow::Cow;
mod display;

/// [CompleteGraph](https://reference.wolfram.com/language/ref/CompleteGraph.html)
/// represents a graph where every node is connected to every other node.
///
/// ![](https://reference.wolfram.com/language/ref/Files/CompleteGraph.zh/O_1.png)
///
/// # Examples
///
/// ```
/// use graph_theory::{CompleteGraph, Graph};
/// let graph = CompleteGraph::new(3);
/// assert_eq!(graph.count_nodes(), 3);
/// assert_eq!(graph.count_edges(), 3 * 2);
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
        self.rank
    }

    fn get_edge(&self, index: usize) -> Option<Cow<Self::EdgeIndex>> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        if self.directed { self.rank * (self.rank - 1) * 2 } else { self.rank * (self.rank - 1) }
    }
}

impl CompleteGraph {
    /// Creates a new complete graph with the given rank.
    ///
    /// ![](https://raw.githubusercontent.com/oovm/graph-rs/dev/projects/graph-types/src/famous_graphs/complete_graph/k-complete.svg)
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::CompleteGraph;
    /// let graph = CompleteGraph::new(3);
    /// ```
    pub fn new(rank: usize) -> Self {
        Self { directed: false, rank }
    }
    /// Creates a new directed complete graph with the given rank.
    ///
    /// ![](https://raw.githubusercontent.com/oovm/graph-rs/dev/projects/graph-types/src/famous_graphs/complete_graph/d-complete.svg)
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::CompleteGraph;
    /// let graph = CompleteGraph::directed(3);
    /// ```
    pub fn directed(rank: usize) -> Self {
        Self { directed: true, rank }
    }
    /// Check if the given graph is a complete graph, and if so, return it.
    pub fn check<G: Graph>(graph: &G) -> Option<Self> {
        let nodes = graph.count_nodes();
        let edges = graph.count_edges();
        if edges == nodes * (nodes - 1) {
            if is_directed(graph, nodes) {
                return Some(Self { directed: false, rank: nodes });
            }
        }
        else if edges == nodes * (nodes - 1) * 2 {
            if is_undirected(graph) {
                return Some(Self { directed: true, rank: nodes });
            }
        }
        None
    }
}

/// Add nodes degree is rank -1
fn is_directed<G>(graph: &G, rank: usize) -> bool
where
    G: Graph,
{
    let _ = (graph, rank);
    todo!()
}

fn is_undirected<G>(graph: &G) -> bool
where
    G: Graph,
{
    let _ = (graph,);
    todo!()
}
