use super::*;

/// [DirectedEdge](https://reference.wolfram.com/language/ref/DirectedEdge.html)
/// represents an bidirectional edge between two nodes.
///
/// Also known as an arc.
///
/// # Examples
///
/// ```
/// use graph_theory::Graph;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectedEdge {
    /// The index of the node that the edge is coming from.
    pub from: usize,
    /// The index of the node that the edge is going to.
    pub goto: usize,
}

impl Edge for DirectedEdge {
    fn bidirectional(&self) -> bool {
        false
    }

    fn from(&self) -> usize {
        self.from
    }

    fn goto(&self) -> usize {
        self.goto
    }
}

impl From<(usize, usize)> for DirectedEdge {
    fn from(ordinal: (usize, usize)) -> Self {
        Self { from: ordinal.0 - 1, goto: ordinal.1 - 1 }
    }
}
