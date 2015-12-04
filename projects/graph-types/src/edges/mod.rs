use crate::{DirectedEdge, GraphEngine};
use std::{
    cmp::{max, min},
    ops::Range,
};

pub mod directed;
pub mod remove_action;
pub mod undirected;

mod simple;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;

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
    /// use graph_theory::{DirectedEdge, EdgeRemoveAction, UndirectedEdge};
    /// ```
    fn bidirectional(&self) -> bool;
    /// The index of the node the edge is coming from
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn from(&self) -> usize;
    /// The index of the node the edge is going to
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn goto(&self) -> usize;
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
        max(self.from(), self.goto())
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
        min(self.from(), self.goto())
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
    TwoWay,
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
    Forward,
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
    Reverse,
}
