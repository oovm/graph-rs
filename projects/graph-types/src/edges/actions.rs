use super::*;

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
pub enum EdgeInsertID {
    /// No need to insert anything
    Nothing,
    /// Inserted one node, return the node id
    OneEdge(usize),
    /// Inserted two nodes, return these node ids
    TwoEdges(usize, usize),
}

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
pub enum EdgeRemoveAction {
    EdgeID(usize),
    Directed(DirectedEdge),
    Undirected(UndirectedEdge),
}

impl From<usize> for EdgeRemoveAction {
    fn from(value: usize) -> Self {
        Self::EdgeID(value)
    }
}

impl From<UndirectedEdge> for EdgeRemoveAction {
    fn from(edge: UndirectedEdge) -> Self {
        Self::Undirected(edge)
    }
}

impl From<DirectedEdge> for EdgeRemoveAction {
    fn from(edge: DirectedEdge) -> Self {
        Self::Directed(edge)
    }
}
