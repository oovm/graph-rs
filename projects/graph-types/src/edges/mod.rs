use std::{
    cmp::{max, min},
    ops::Range,
};

use crate::{DynamicEdge, UndirectedEdge};
use std::fmt::{Display, Formatter};

pub mod actions;
pub mod typed_edges;

mod simple;

pub mod get_iter;
pub mod mut_iter;

/// used to determine the direction of an edge
pub type EdgeID = usize;

/// Marker trait for edges
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
pub trait Edge: Display {
    /// Whether the edge is bidirectional
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::DirectedEdge;
    /// ```
    fn direction(&self) -> EdgeDirection;
    /// The left side node id of the edge
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn lhs(&self) -> usize;
    /// The right side node id of the edge
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn rhs(&self) -> usize;
    /// The smaller of the two indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{Edge, UndirectedEdge};
    /// assert_eq!(UndirectedEdge::new(1, 2).max_index(), 2);
    /// assert_eq!(UndirectedEdge::new(2, 1).max_index(), 2);
    /// ```
    fn max_index(&self) -> usize {
        max(self.lhs(), self.rhs())
    }
    /// The smaller of the two indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{Edge, UndirectedEdge};
    /// assert_eq!(UndirectedEdge::new(1, 2).min_index(), 1);
    /// assert_eq!(UndirectedEdge::new(2, 1).min_index(), 1);
    /// ```
    fn min_index(&self) -> usize {
        min(self.lhs(), self.rhs())
    }

    /// The smaller of the two indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{Edge, UndirectedEdge};
    /// assert_eq!(UndirectedEdge::new(1, 3).delta_index(), 2);
    /// assert_eq!(UndirectedEdge::new(3, 1).delta_index(), 2);
    /// ```
    fn delta_index(&self) -> usize {
        self.max_index() - self.min_index()
    }

    /// Creates a new edge with the indices swapped.
    fn as_dynamic(&self) -> DynamicEdge {
        DynamicEdge { bidi: self.direction(), from: self.lhs(), goto: self.rhs() }
    }
}

/// Determines the direction between two nodes
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirection {
    /// Two nodes that are not connected.
    Disconnect,
    /// This edge is bidirectional
    TwoWay,
    /// This edge is unidirectional
    Forward,
    /// This edge is unidirectional and goes in the opposite direction
    Reverse,
}
