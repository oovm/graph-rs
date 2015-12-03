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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InsertionEdge {
    /// Whether the edge is bidirectional
    pub bidi: bool,
    /// The index of the node the edge is coming from
    pub from: usize,
    /// The index of the node the edge is going to
    pub goto: usize,
}

impl From<UndirectedEdge> for InsertionEdge {
    fn from(edge: UndirectedEdge) -> Self {
        Self { bidi: true, from: edge.from, goto: edge.goto }
    }
}

impl From<DirectedEdge> for InsertionEdge {
    fn from(edge: DirectedEdge) -> Self {
        Self { bidi: false, from: edge.from, goto: edge.goto }
    }
}

impl Edge for InsertionEdge {
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
