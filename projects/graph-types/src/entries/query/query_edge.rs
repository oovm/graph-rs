use crate::{DirectedEdge, UndirectedEdge};

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeQuery {
    /// No need to remove anything
    EdgeID(usize),
    /// Removed one node, return the node id
    Directed(DirectedEdge),
    /// Removed two nodes, return these node ids
    Undirected(UndirectedEdge),
}

impl From<usize> for EdgeQuery {
    fn from(value: usize) -> Self {
        Self::EdgeID(value)
    }
}

impl From<UndirectedEdge> for EdgeQuery {
    fn from(edge: UndirectedEdge) -> Self {
        Self::Undirected(edge)
    }
}

impl From<DirectedEdge> for EdgeQuery {
    fn from(edge: DirectedEdge) -> Self {
        Self::Directed(edge)
    }
}
