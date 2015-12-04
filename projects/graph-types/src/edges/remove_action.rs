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
    NodeID(usize),
    Directed(DirectedEdge),
    Undirected(UndirectedEdge),
}

impl From<usize> for EdgeRemoveAction {
    fn from(value: usize) -> Self {
        Self::NodeID(value)
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

impl Edge for EdgeRemoveAction {
    fn bidirectional(&self) -> bool {
        self.bidi
    }

    fn from(&self) -> usize {
        self.from
    }
    fn goto(&self) -> usize {
        self.goto
    }
}
