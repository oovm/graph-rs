use super::*;
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
