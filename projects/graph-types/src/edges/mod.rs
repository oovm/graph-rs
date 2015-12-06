use crate::{DirectedEdge, GraphEngine};
use std::{
    cmp::{max, min},
    ops::Range,
};

use std::fmt::{Display, Formatter};
use crate::{DynamicEdge, UndirectedEdge};

pub mod typed_edges;
pub mod actions;


mod simple;


pub mod get_iter;
pub mod mut_iter;

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
pub trait Edge {
    /// Whether the edge is bidirectional
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{DirectedEdge};
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirection {
    /// A directed edge that goes from the smaller index to the larger index.
    Dynamic,
    /// A directed edge that goes from the smaller index to the larger index.
    TwoWay,
    /// A directed edge that goes from the smaller index to the larger index.
    Forward,
    /// A directed edge that goes from the larger index to the smaller index.
    Reverse,
}
