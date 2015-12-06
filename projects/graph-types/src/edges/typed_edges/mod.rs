use super::*;

mod display;
mod constructors;

#[cfg(feature = "wolfram_wxf")]
mod wolfram;

/// [DynamicEdge](https://reference.wolfram.com/language/ref/DirectedEdge.html)
/// represents an bidirectional edge between two nodes.
///
/// Also known as an arc.
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynamicEdge {
    pub bidi: EdgeDirection,
    /// The index of the node that the edge is coming from.
    pub from: usize,
    /// The index of the node that the edge is going to.
    pub goto: usize,
}

/// [DirectedEdge](https://reference.wolfram.com/language/ref/DirectedEdge.html)
/// represents an bidirectional edge between two nodes.
///
/// Also known as an arc.
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectedEdge {
    /// The index of the node that the edge is coming from.
    pub from: usize,
    /// The index of the node that the edge is going to.
    pub goto: usize,
}


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

impl Edge for DynamicEdge {
    fn direction(&self) -> EdgeDirection {
        self.bidi
    }

    fn lhs(&self) -> usize {
        self.from
    }

    fn rhs(&self) -> usize {
        self.goto
    }
}


impl Edge for DirectedEdge {
    fn direction(&self) -> EdgeDirection {
        EdgeDirection::Forward
    }

    fn lhs(&self) -> usize {
        self.from
    }

    fn rhs(&self) -> usize {
        self.goto
    }
}

impl Edge for UndirectedEdge {
    fn direction(&self) -> EdgeDirection {
        EdgeDirection::TwoWay
    }

    fn lhs(&self) -> usize {
        self.from
    }

    fn rhs(&self) -> usize {
        self.goto
    }
}
