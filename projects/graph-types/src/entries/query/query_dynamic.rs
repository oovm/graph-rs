use crate::{edges::typed_edges::IndeterminateEdge, errors::GraphError, DirectedEdge, GraphEntry, NodeID, UndirectedEdge};

/// # Arguments
///
/// * `index`:
///
/// returns: Option<Cow<Self::Node>>
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Clone, Copy, Debug)]
pub enum Query {
    NodeID(usize),
    /// No need to remove anything
    EdgeID(usize),
    /// Removed one node, return the node id
    Directed(DirectedEdge),
    /// Removed two nodes, return these node ids
    Undirected(UndirectedEdge),
    /// Account for the fact that the graph is dynamic
    Indeterminate(IndeterminateEdge),
}

impl Query {
    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Option<Cow<Self::Node>>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn as_entry(&self) -> GraphEntry {
        match self {
            Self::NodeID(_) => GraphEntry::Node,
            Self::EdgeID(_) => GraphEntry::Edge,
            Self::Directed(_) => GraphEntry::Edge,
            Self::Undirected(_) => GraphEntry::Edge,
            Self::Indeterminate(_) => GraphEntry::Edge,
        }
    }
    /// Returns the node as a node ID.
    pub fn check_node_range(index: NodeID, count: usize) -> Result<NodeID, GraphError> {
        if index < count { Ok(index) } else { Err(GraphError::node_out_of_range(index, count)) }
    }
}
