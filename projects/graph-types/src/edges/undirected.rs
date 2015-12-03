use super::*;
use std::fmt::{Display, Formatter};

/// [UndirectedEdge](https://reference.wolfram.com/language/ref/UndirectedEdge.html)
/// represents an bidirectional edge between two nodes.
///
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UndirectedEdge {
    /// The index of the node that the edge is coming from, usually the smaller index.
    pub from: usize,
    /// The index of the node that the edge is going to, usually the larger index.
    pub goto: usize,
}

impl Display for UndirectedEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ↔ {}", self.from + 1, self.goto + 1)
    }
}

impl Edge for UndirectedEdge {
    fn bidirectional(&self) -> bool {
        true
    }

    fn from(&self) -> usize {
        self.min_index()
    }

    fn goto(&self) -> usize {
        self.max_index()
    }
}

impl From<(usize, usize)> for UndirectedEdge {
    /// Creates a new edge from the given ordinals (start from 1).
    fn from(ordinal: (usize, usize)) -> Self {
        Self { from: ordinal.0 - 1, goto: ordinal.1 - 1 }
    }
}

impl UndirectedEdge {
    /// Creates a new edge from the given indices (start from 0).
    pub fn new(from: usize, goto: usize) -> Self {
        Self { from, goto }
    }
    /// Use the edge as a range.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::UndirectedEdge;
    /// let range = UndirectedEdge::new(0, 3).as_range();
    /// assert_eq!(range, 0..3);
    /// ```
    pub fn as_range(&self) -> Range<usize> {
        self.min_index()..self.max_index()
    }
}
