pub mod get_iter;
pub mod indexed;
pub mod mut_iter;

use crate::{GraphEngine, GraphError, Query, ValueProvider};
use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    ops::{Bound, Deref, Range, RangeBounds},
};

/// Represents a node in a graph
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
pub trait Node {
    type Name: AsRef<str>;
    type Weight: PartialEq + PartialOrd;

    /// The index of the node, all kinds of nodes must have an index
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn index(&self) -> usize;
    /// The weight of the node, only weighted graph nodes has a weight
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn name(&self) -> Self::Name;
    /// The weight of the node, only weighted graph nodes has a weight
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn weight(&self) -> Option<Self::Weight> {
        None
    }
}
