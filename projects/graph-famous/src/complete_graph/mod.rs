use graph_derive::Graph;
use graph_types::{
    errors::GraphError,
    placeholder::{PlaceholderEdgeIterator, PlaceholderNodeIterator},
    Edge, EdgeID, EdgeQuery, GraphEngine, GraphKind, IndeterminateEdge, NodeID, NodeQuery, NodeRangeVisitor, NodesVisitor,
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

impl GraphEngine for CompleteGraph {
    type NodeIterator = PlaceholderNodeIterator;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeIterator = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        match self.mask < 0 {
            true => GraphKind::Undirected,
            false => GraphKind::Directed,
        }
    }

    fn get_node_id<Q: Into<NodeQuery>>(&self, node: Q) -> Result<NodeID, GraphError> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        self.rank()
    }

    fn all_node_ids(&self) -> Self::NodeIterator {
        todo!()
    }

    fn get_edge_id<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<EdgeID, GraphError> {
        todo!()
    }

    fn all_edge_ids(&self) -> Self::EdgeIterator {
        todo!()
    }

    fn get_bridges<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<IndeterminateEdge, GraphError> {
        todo!()
    }

    fn all_bridges(&self) -> Self::BridgeIterator {
        todo!()
    }

    fn count_edges(&self) -> usize {
        let rank = self.mask as usize;
        rank * (rank - 1) * 2
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
    pub fn check<G: GraphEngine>(graph: &G) -> Option<Self> {
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
