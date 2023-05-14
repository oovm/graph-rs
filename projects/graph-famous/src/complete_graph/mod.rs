use graph_derive::Graph;
use graph_types::{
    errors::GraphError,
    placeholder::{PlaceholderEdgeIterator, PlaceholderNodeIterator},
    Edge, EdgeID, EdgeQuery, GraphEngine, GraphKind, IndeterminateEdge, NodeID,
};
use std::mem::size_of;

/// [CompleteGraph](https://reference.wolfram.com/language/ref/CompleteGraph.html)
/// represents a graph where every node is connected to every other node.
///
/// ![](https://reference.wolfram.com/language/ref/Files/CompleteGraph.zh/O_1.png)
///
/// # Examples
///
/// ```
/// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
/// let graph = CompleteGraph::one_way(3);
/// assert_eq!(graph.count_nodes(), 3);
/// assert_eq!(graph.count_edges(), 12);
/// ```
#[repr(C)]
#[derive(Graph)]
pub struct CompleteGraph {
    #[easy_graph(constructor = false, default = 5, wolfram = "CompleteGraph")]
    mask: i32,
}

impl<'a> GraphEngine<'a> for CompleteGraph {
    type NeighborIterator = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;
    type NodeTraverser = PlaceholderNodeIterator;
    type EdgeTraverser = PlaceholderNodeIterator;
    type BridgeTraverser = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        match self.mask < 0 {
            true => GraphKind::Undirected,
            false => GraphKind::Directed,
        }
    }

    fn get_node(&self, node: NodeID) -> Result<NodeID, GraphError> {
        todo!()
    }

    fn all_nodes(&self) -> Self::NodeTraverser {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        self.rank()
    }

    fn all_neighbors(&'a self, node: NodeID) -> Self::NeighborIterator {
        todo!()
    }

    fn get_edge(&self, edge: EdgeID) -> Result<EdgeID, GraphError> {
        todo!()
    }

    fn all_edges(&self) -> Self::EdgeTraverser {
        todo!()
    }

    fn count_edges(&self) -> usize {
        let rank = self.mask as usize;
        rank * (rank - 1) * 2
    }

    fn get_bridge(&self, edge: EdgeID) -> Result<IndeterminateEdge, GraphError> {
        todo!()
    }

    fn get_bridges(&'a self, from: NodeID, goto: NodeID) -> Self::BridgeIterator {
        todo!()
    }

    fn all_bridges(&self) -> Self::BridgeIterator {
        todo!()
    }

    /// Takes O(1) space, in fact it's always takes 32 bits.
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// assert_eq!(CompleteGraph::one_way(3).size_hint(), 4);
    /// assert_eq!(CompleteGraph::one_way(4).size_hint(), 4);
    /// assert_eq!(CompleteGraph::one_way(5).size_hint(), 4);
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
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// let graph = CompleteGraph::one_way(3);
    /// assert_eq!(graph.count_nodes(), 3);
    /// ```
    pub fn one_way(rank: usize) -> Self {
        Self { mask: rank as i32 }
    }
    /// Creates a new complete graph with the given rank.
    ///
    /// ![](https://raw.githubusercontent.com/oovm/graph-rs/dev/projects/graph-types/src/famous_graphs/complete_graph/d-complete.svg)
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// let graph = CompleteGraph::two_way(3);
    /// assert_eq!(graph.count_nodes(), 6);
    /// ```
    pub fn two_way(rank: usize) -> Self {
        Self { mask: -(rank as i32) }
    }
    /// Check if the given graph is a complete graph, and if so, return it.
    pub fn check<'a, G: GraphEngine<'a>>(graph: &'a G) -> Option<Self> {
        let nodes = graph.count_nodes();
        let edges = graph.count_edges();
        if edges == nodes * (nodes - 1) {
            if is_directed(graph, nodes) {
                return Some(Self::one_way(nodes));
            }
        }
        else if edges == nodes * (nodes - 1) * 2 {
            if is_undirected(graph) {
                return Some(Self::two_way(nodes));
            }
        }
        None
    }
}

/// Add nodes degree is rank -1
fn is_directed<'a, G>(graph: &'a G, rank: usize) -> bool
where
    G: GraphEngine<'a>,
{
    let _ = (graph, rank);
    todo!()
}

fn is_undirected<'a, G>(graph: &'a G) -> bool
where
    G: GraphEngine<'a>,
{
    let _ = (graph,);
    todo!()
}
