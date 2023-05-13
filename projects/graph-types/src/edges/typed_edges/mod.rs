use super::*;
use crate::{errors::GraphError, Query};
#[cfg(feature = "datasize")]
use datasize::DataSize;

mod constructors;
mod display;

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
pub struct IndeterminateEdge {
    /// The index of the node that the edge is coming from.
    pub from: usize,
    /// The index of the node that the edge is going to.
    pub goto: usize,
}
#[cfg(feature = "datasize")]
impl DataSize for IndeterminateEdge {
    const IS_DYNAMIC: bool = false;
    const STATIC_HEAP_SIZE: usize = 0;

    fn estimate_heap_size(&self) -> usize {
        self.from.estimate_heap_size() + self.goto.estimate_heap_size()
    }
}

impl Display for IndeterminateEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Edge for IndeterminateEdge {
    fn direction(&self) -> EdgeDirection {
        EdgeDirection::TwoWay
    }

    fn lhs(&self) -> usize {
        todo!()
    }

    fn rhs(&self) -> usize {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynamicEdge {
    /// The direction of the edge.
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
impl DirectedEdge {
    /// Returns the edge as a directed edge, with the smaller index as the `from` node.
    pub fn as_unsupported<T>(&self) -> Result<T, GraphError> {
        Err(GraphError::not_support(Query::Directed(*self)))
    }
}
impl UndirectedEdge {
    /// Returns the edge as a directed edge, with the smaller index as the `from` node.
    pub fn as_unsupported<T>(&self) -> Result<T, GraphError> {
        Err(GraphError::not_support(Query::Undirected(*self)))
    }
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
