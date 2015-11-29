use crate::Graph;
use std::{
    cmp::{max, min},
    ops::Range,
};

pub mod directed;
pub mod insert_action;
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
/// use graph_theory::Graph;
/// ```
pub trait Edge {
    /// Whether the edge is bidirectional
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{DirectedEdge, InsertionEdge, UndirectedEdge};
    /// ```
    fn bidirectional(&self) -> bool;
    /// The index of the node the edge is coming from
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::Graph;
    /// ```
    fn from(&self) -> usize;
    /// The index of the node the edge is going to
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::Graph;
    /// ```
    fn goto(&self) -> usize;
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
/// use graph_theory::Graph;
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
    /// use graph_theory::Graph;
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
    /// use graph_theory::Graph;
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
    /// use graph_theory::Graph;
    /// ```
    Reverse,
}
